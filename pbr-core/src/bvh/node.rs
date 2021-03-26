use maths::*;

use crate::bounds::{Axis, Bounds3f};

pub struct PrimitiveInfo {
    pub prim_number: usize,
    pub centroid: Point3f,
    pub bounds: Bounds3f,
}

impl PrimitiveInfo {
    pub fn new(pn: usize, bb: Bounds3f) -> PrimitiveInfo {
        PrimitiveInfo {
            prim_number: pn,
            centroid: 0.5 * bb[0] + 0.5 * bb[1],
            bounds: bb,
        }
    }
}

pub enum BuildNode {
    Interior {
        bounds: Bounds3f,
        children: [Box<BuildNode>; 2],
        split_axis: Axis,
    },
    Leaf {
        bounds: Bounds3f,
        first_prim_offset: usize,
        num_prims: usize,
    },
}

impl BuildNode {
    pub fn interior(axis: Axis, child1: Box<BuildNode>, child2: Box<BuildNode>) -> BuildNode {
        let bbox = Bounds3f::union(child1.bounds(), child2.bounds());
        BuildNode::Interior {
            bounds: bbox,
            children: [child1, child2],
            split_axis: axis,
        }
    }

    pub fn leaf(first_prim_offset: usize, num_prims: usize, bbox: Bounds3f) -> BuildNode {
        BuildNode::Leaf {
            bounds: bbox,
            first_prim_offset,
            num_prims,
        }
    }

    pub fn bounds(&self) -> &Bounds3f {
        match self {
            BuildNode::Interior { ref bounds, .. } | BuildNode::Leaf { ref bounds, .. } => bounds,
        }
    }
}

#[derive(Debug)]
pub enum LNodeData {
    Interior {
        second_child_offset: usize,
        axis: Axis,
    },
    Leaf {
        primitives_offset: usize,
        num_prims: usize,
    },
}

#[derive(Debug)]
pub struct LNode {
    pub bounds: Bounds3f,
    pub(super) data: LNodeData,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct BucketInfo {
    pub count: usize,
    pub bounds: Bounds3f,
}
