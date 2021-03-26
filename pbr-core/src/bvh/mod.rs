use self::node::*;
use crate::{
    bounds::{Axis, Bounds3f},
    interaction::SurfaceInteraction,
    material::{Material, TransportMode},
    paramset::ParamSet,
    primitive::{AreaLight, Primitive},
    ray::Ray,
};
use itertools as it;
use light_arena::Allocator;
use log::info;
use maths::*;
use std::{cmp::min, mem::replace, sync::Arc};

mod node;

#[derive(Copy, Clone, Debug)]
pub enum SplitMethod {
    Middle,
    EqualCounts,
    Sah,
}

/// Bounding Volume Hierarchies
#[derive(Debug)]
pub struct Bvh {
    max_prims_per_node: usize,
    primitives: Vec<Arc<dyn Primitive>>,
    nodes: Vec<LNode>,
}

impl Bvh {
    pub fn create(_prims: &[Arc<dyn Primitive>], _ps: &ParamSet) -> Bvh {
        todo!()
    }

    pub fn new(
        max_prims_per_node: usize,
        prims: &[Arc<dyn Primitive>],
        split_method: SplitMethod,
    ) -> Bvh {
        info!("Generating BVH with method {:?}:", split_method);

        // 1. Get bounds info
        info!("\tGenerating primitive info");
        let mut primitive_info: Vec<_> = prims
            .iter()
            .enumerate()
            .map(|(i, p)| PrimitiveInfo::new(i, p.world_bounds()))
            .collect();

        // 2. Build tree
        info!("\tBuilding tree for {} primitives", prims.len());
        let mut total_nodes = 0;
        let mut ordered_prims = Vec::with_capacity(prims.len());
        let root: BuildNode = Self::build(
            prims,
            &mut primitive_info,
            0,
            prims.len(),
            max_prims_per_node,
            &mut total_nodes,
            &mut ordered_prims,
            split_method,
        );

        info!("\tCreated {} nodes", total_nodes);

        // 3. Build flatten representation
        info!("\tFlattening tree");
        let mut nodes = Vec::with_capacity(total_nodes);
        Self::flatten(&root, &mut nodes);
        assert_eq!(nodes.len(), total_nodes);

        let bvh = Self {
            max_prims_per_node: min(max_prims_per_node, 255),
            primitives: ordered_prims,
            nodes,
        };
        info!(
            "BVH created with {} nodes for {} primitives",
            total_nodes,
            bvh.primitives.len()
        );

        bvh
    }

    fn build(
        primitives: &[Arc<dyn Primitive>],
        primitive_info: &mut Vec<PrimitiveInfo>,
        start: usize,
        end: usize,
        max_prims_per_node: usize,
        total_nodes: &mut usize,
        ordered_prims: &mut Vec<Arc<dyn Primitive>>,
        split_method: SplitMethod,
    ) -> BuildNode {
        *total_nodes += 1;
        let n_primitives = end - start;
        assert_ne!(start, end);
        // Compute bounds of all primitives in node
        let bounds = primitive_info[start..end]
            .iter()
            .fold(Bounds3f::new(), |b, pi| Bounds3f::union(&b, &pi.bounds));
        if n_primitives == 1 {
            // Create leaf
            let first_prim_offset = ordered_prims.len();
            for pi in primitive_info[start..end].iter() {
                let prim_num = pi.prim_number;
                ordered_prims.push(Arc::clone(&primitives[prim_num]));
            }
            BuildNode::leaf(first_prim_offset, n_primitives, bounds)
        } else {
            // Compute bounds of primitive centroids
            let centroids_bounds = primitive_info[start..end]
                .iter()
                .fold(Bounds3f::new(), |bb, pi| {
                    Bounds3f::union_point(&bb, &pi.centroid)
                });
            // Choose split dimension
            let dimension = centroids_bounds.maximum_extent();
            // Partition primitives into 2 sets and build children
            if approx!(centroids_bounds[0][dimension], == centroids_bounds[1][dimension]) {
                let first_prim_offset = ordered_prims.len();
                for pi in primitive_info[start..end].iter() {
                    let prim_num = pi.prim_number;
                    ordered_prims.push(Arc::clone(&primitives[prim_num]));
                }
                return BuildNode::leaf(first_prim_offset, n_primitives, bounds);
            }
            // Partition primitives based on split method (here split middle)
            let mut mid;
            match split_method {
                SplitMethod::Middle => {
                    let pmid =
                        0.5 * (centroids_bounds[0][dimension] + centroids_bounds[1][dimension]);
                    mid = start
                        + it::partition(primitive_info[start..end].iter_mut(), |pi| {
                            pi.centroid[dimension] < pmid
                        })
                        + start;
                    if mid == start || mid == end {
                        // If partition failed, used Split Equal method
                        primitive_info[start..end].sort_by(|p1, p2| {
                            p1.centroid[dimension]
                                .partial_cmp(&p2.centroid[dimension])
                                .unwrap()
                        });
                        mid = (start + end) / 2;
                    }
                }
                SplitMethod::EqualCounts => unimplemented!(),
                SplitMethod::Sah => {
                    // Partition primitives using approximate SAH
                    if n_primitives <= 2 {
                        // Partition primitives into equally-sized subsets
                        mid = (start + end) / 2;
                        if start != end - 1
                            && primitive_info[end - 1].centroid[dimension]
                                < primitive_info[start].centroid[dimension]
                        {
                            primitive_info.swap(start, end - 1);
                        }
                    } else {
                        const N_BUCKETS: usize = 12;
                        // Allocate `BucketInfo for SAH partition buckets
                        let mut buckets = [BucketInfo::default(); N_BUCKETS];

                        // Initialize `BucketInfo` for SAH partition buckets
                        for prim_inf in primitive_info.iter().take(end).skip(start) {
                            let mut b = (N_BUCKETS as f32
                                * centroids_bounds.offset(&prim_inf.centroid)[dimension])
                                as usize;
                            if b == N_BUCKETS {
                                b = N_BUCKETS - 1;
                            }
                            assert!(b < N_BUCKETS);
                            buckets[b].count += 1;
                            buckets[b].bounds =
                                Bounds3f::union(&buckets[b].bounds, &prim_inf.bounds);
                        }

                        // Compute costs for splitting after each bucket
                        let mut cost = [0.0; N_BUCKETS - 1];
                        for (i, cost_i) in cost.iter_mut().enumerate().take(N_BUCKETS - 1) {
                            let mut b0 = Bounds3f::new();
                            let mut b1 = Bounds3f::new();
                            let mut count0 = 0;
                            let mut count1 = 0;
                            for bucket in buckets.iter().take(i + 1) {
                                b0 = Bounds3f::union(&b0, &bucket.bounds);
                                count0 += bucket.count;
                            }
                            for bucket in buckets.iter().take(N_BUCKETS).skip(i + 1) {
                                b1 = Bounds3f::union(&b1, &bucket.bounds);
                                count1 += bucket.count;
                            }
                            *cost_i = 1.0
                                + (count0 as f32 * b0.surface_area()
                                    + count1 as f32 * b1.surface_area())
                                    / bounds.surface_area();
                        }

                        // Find bucket to split at that minimizes SAH metric
                        let mut min_cost = cost[0];
                        let mut min_cost_split_bucket = 0;
                        for (i, cost_i) in cost.iter().enumerate().take(N_BUCKETS - 1).skip(1) {
                            if *cost_i < min_cost {
                                min_cost = *cost_i;
                                min_cost_split_bucket = i;
                            }
                        }

                        // Either create leaf of split primitives at selected SAH bucket
                        let leaf_cost = n_primitives as f32;
                        if n_primitives > max_prims_per_node || min_cost < leaf_cost {
                            mid = start
                                + it::partition(primitive_info[start..end].iter_mut(), |pi| {
                                    let mut b = (N_BUCKETS as f32
                                        * centroids_bounds.offset(&pi.centroid)[dimension])
                                        as usize;
                                    if b == N_BUCKETS {
                                        b = N_BUCKETS - 1;
                                    }
                                    assert!(b < N_BUCKETS);
                                    b <= min_cost_split_bucket
                                });
                        } else {
                            // Create leaf `BVHBuildNode`
                            let first_prim_offset = ordered_prims.len();
                            for prim_inf in primitive_info.iter().take(end).skip(start) {
                                let prim_num = prim_inf.prim_number;
                                ordered_prims.push(Arc::clone(&primitives[prim_num]));
                            }
                            return BuildNode::leaf(first_prim_offset, n_primitives, bounds);
                        }
                    }
                }
            }

            let right = Box::new(Bvh::build(
                primitives,
                primitive_info,
                mid,
                end,
                max_prims_per_node,
                total_nodes,
                ordered_prims,
                split_method,
            ));
            let left = Box::new(Bvh::build(
                primitives,
                primitive_info,
                start,
                mid,
                max_prims_per_node,
                total_nodes,
                ordered_prims,
                split_method,
            ));
            BuildNode::interior(dimension, left, right)
        }
    }

    fn flatten(node: &BuildNode, nodes: &mut Vec<LNode>) -> usize {
        let offset = nodes.len();

        match *node {
            BuildNode::Leaf {
                first_prim_offset,
                num_prims,
                ..
            } => {
                let linear_node = LNode {
                    bounds: *node.bounds(),
                    data: LNodeData::Leaf {
                        num_prims,
                        primitives_offset: first_prim_offset,
                    },
                };
                nodes.push(linear_node);
            }
            BuildNode::Interior {
                split_axis,
                ref children,
                ..
            } => {
                let linear_node = LNode {
                    bounds: *node.bounds(),
                    data: LNodeData::Interior {
                        axis: split_axis,
                        second_child_offset: 0,
                    },
                };
                nodes.push(linear_node);
                Self::flatten(&*children[0], nodes);
                let second_offset = Bvh::flatten(&*children[1], nodes);
                let _prev = replace(
                    &mut nodes[offset].data,
                    LNodeData::Interior {
                        axis: split_axis,
                        second_child_offset: second_offset,
                    },
                );
            }
        }

        offset
    }
}

impl Primitive for Bvh {
    fn world_bounds(&self) -> Bounds3f {
        self.nodes[0].bounds
    }

    fn intersect(&self, ray: &mut Ray) -> Option<SurfaceInteraction<'_, '_>> {
        if self.nodes.is_empty() {
            return None;
        }
        let mut result = None;

        let mut to_visit_offset = 0;
        let mut current_node_idx = 0;
        let mut nodes_to_visit = [0; 64];
        let inv_dir = Vector3f::new(1.0 / ray.d.x, 1.0 / ray.d.y, 1.0 / ray.d.z);
        let dir_is_neg = [
            (inv_dir.x < 0.0) as usize,
            (inv_dir.y < 0.0) as usize,
            (inv_dir.z < 0.0) as usize,
        ];
        loop {
            let linear_node = &self.nodes[current_node_idx];
            if linear_node
                .bounds
                .intersect_p_fast(ray, &inv_dir, &dir_is_neg)
            {
                match linear_node.data {
                    LNodeData::Leaf {
                        num_prims,
                        primitives_offset,
                    } => {
                        for i in 0..num_prims {
                            result = self.primitives[primitives_offset + i]
                                .intersect(ray)
                                .or(result);
                        }
                        if to_visit_offset == 0 {
                            break;
                        }
                        to_visit_offset -= 1;
                        current_node_idx = nodes_to_visit[to_visit_offset];
                    }
                    LNodeData::Interior {
                        axis,
                        second_child_offset,
                        ..
                    } => {
                        let axis_num = match axis {
                            Axis::X => 0,
                            Axis::Y => 1,
                            Axis::Z => 2,
                        };
                        if dir_is_neg[axis_num] != 0 {
                            nodes_to_visit[to_visit_offset] = current_node_idx + 1;
                            to_visit_offset += 1;
                            current_node_idx = second_child_offset;
                        } else {
                            nodes_to_visit[to_visit_offset] = second_child_offset;
                            to_visit_offset += 1;
                            current_node_idx += 1;
                        }
                    }
                }
            } else {
                if to_visit_offset == 0 {
                    break;
                }
                to_visit_offset -= 1;
                current_node_idx = nodes_to_visit[to_visit_offset];
            }
        }
        result
    }

    fn intersect_p(&self, ray: &Ray) -> bool {
        if self.nodes.is_empty() {
            return false;
        }

        let mut to_visit_offset = 0;
        let mut current_node_idx = 0;
        let mut nodes_to_visit = [0; 64];
        let inv_dir = Vector3f::new(1.0 / ray.d.x, 1.0 / ray.d.y, 1.0 / ray.d.z);
        let dir_is_neg = [
            (inv_dir.x < 0.0) as usize,
            (inv_dir.y < 0.0) as usize,
            (inv_dir.z < 0.0) as usize,
        ];
        loop {
            let linear_node = &self.nodes[current_node_idx];
            if linear_node
                .bounds
                .intersect_p_fast(ray, &inv_dir, &dir_is_neg)
            {
                match linear_node.data {
                    LNodeData::Leaf {
                        num_prims,
                        primitives_offset,
                    } => {
                        for i in 0..num_prims {
                            if self.primitives[primitives_offset + i].intersect_p(ray) {
                                return true;
                            }
                        }
                        if to_visit_offset == 0 {
                            break;
                        }
                        to_visit_offset -= 1;
                        current_node_idx = nodes_to_visit[to_visit_offset];
                    }
                    LNodeData::Interior {
                        axis,
                        second_child_offset,
                        ..
                    } => {
                        let axis_num = match axis {
                            Axis::X => 0,
                            Axis::Y => 1,
                            Axis::Z => 2,
                        };
                        if dir_is_neg[axis_num] != 0 {
                            nodes_to_visit[to_visit_offset] = current_node_idx + 1;
                            to_visit_offset += 1;
                            current_node_idx = second_child_offset;
                        } else {
                            nodes_to_visit[to_visit_offset] = second_child_offset;
                            to_visit_offset += 1;
                            current_node_idx += 1;
                        }
                    }
                }
            } else {
                if to_visit_offset == 0 {
                    break;
                }
                to_visit_offset -= 1;
                current_node_idx = nodes_to_visit[to_visit_offset];
            }
        }
        false
    }

    fn area_light(&self) -> Option<Arc<dyn AreaLight>> {
        panic!("area_light() should not be called on an Aggregate Primitive!");
    }

    fn material(&self) -> Option<Arc<dyn Material>> {
        panic!("material() should not be called on an Aggregate Primitive!");
    }

    fn compute_scattering_functions<'a, 'b>(
        &self,
        _: &mut SurfaceInteraction<'a, 'b>,
        _: TransportMode,
        _: bool,
        _: &'b Allocator<'_>,
    ) {
        panic!("compute_scattering_functions() should not be called on an Aggregate Primitive!");
    }
}
