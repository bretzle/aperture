use crate::{
    math::{pnt3_offset_ray_origin, Normal3, Point3, Ray, Vector3},
    medium::{HenyeyGreenStein, Medium, MediumInterface},
};
use std::{cell::Cell, f32::INFINITY, sync::Arc};

pub trait Interaction {
    fn is_surface_interaction(&self) -> bool;
    fn is_medium_interaction(&self) -> bool;
    fn spawn_ray(&self, d: Vector3<f32>) -> Ray;
    fn get_common(&self) -> &InteractionBase;
    fn get_p(&self) -> &Point3<f32>;
    fn get_time(&self) -> f32;
    fn get_p_error(&self) -> &Vector3<f32>;
    fn get_wo(&self) -> &Vector3<f32>;
    fn get_n(&self) -> &Normal3<f32>;
    fn get_medium_interface(&self) -> Option<Arc<MediumInterface>>;
    // fn get_bsdf(&self) -> Option<&Bsdf>;
    fn get_shading_n(&self) -> Option<&Normal3<f32>>;
    fn get_phase(&self) -> Option<Arc<HenyeyGreenStein>>;
}

#[derive(Default, Clone)]
pub struct InteractionBase {
    p: Point3<f32>,
    time: f32,
    p_error: Vector3<f32>,
    wo: Vector3<f32>,
    n: Normal3<f32>,
    medium_interface: Option<Arc<MediumInterface>>,
}

impl InteractionBase {
    pub fn spawn_ray(&self, dir: Vector3<f32>) -> Ray {
        todo!()
    }

    pub fn spawn_ray_to_point(&self, point: Point3<f32>) -> Ray {
        todo!()
    }

    pub fn spawn_ray_to(&self, it: &Self) -> Ray {
        todo!()
    }

    pub fn get_medium(&self, w: Vector3<f32>) -> Option<Arc<MediumInterface>> {
        todo!()
    }
}

#[derive(Default, Clone)]
pub struct MediumInteraction {
    pub base: InteractionBase,
    pub phase: Option<Arc<HenyeyGreenStein>>,
}

impl MediumInteraction {
    pub fn new(
        p: Point3<f32>,
        wo: Vector3<f32>,
        time: f32,
        medium: Option<Arc<Medium>>,
        phase: Option<Arc<HenyeyGreenStein>>,
    ) -> Self {
        let medium_interface = medium.map(|marc| {
            let inside = Some(marc.clone());
            let outside = Some(marc);
            Arc::new(MediumInterface::new(inside, outside))
        });

        let base = InteractionBase {
            p,
            time,
            wo,
            medium_interface,
            ..Default::default()
        };

        Self { base, phase }
    }

    pub fn get_medium(&self, w: Vector3<f32>) -> Option<Arc<Medium>> {
        todo!()
    }

    pub fn is_valid(&self) -> bool {
        self.phase.is_some()
    }
}

impl Interaction for MediumInteraction {
    fn is_surface_interaction(&self) -> bool {
        self.base.n == Normal3::default()
    }

    fn is_medium_interaction(&self) -> bool {
        !self.is_surface_interaction()
    }

    fn spawn_ray(&self, direction: Vector3<f32>) -> Ray {
        let origin =
            pnt3_offset_ray_origin(&self.base.p, &self.base.p_error, &self.base.n, &direction);
        Ray {
            origin,
            direction,
            t_max: Cell::new(INFINITY),
            time: self.base.time,
            differential: None,
            medium: self.get_medium(direction),
        }
    }

    fn get_common(&self) -> &InteractionBase {
        &self.base
    }

    fn get_p(&self) -> &Point3<f32> {
        &self.base.p
    }

    fn get_time(&self) -> f32 {
        self.base.time
    }

    fn get_p_error(&self) -> &Vector3<f32> {
        &self.base.p_error
    }

    fn get_wo(&self) -> &Vector3<f32> {
        &self.base.wo
    }

    fn get_n(&self) -> &Normal3<f32> {
        &self.base.n
    }

    fn get_medium_interface(&self) -> Option<Arc<MediumInterface>> {
        self.base.medium_interface.as_ref().cloned()
    }

    fn get_shading_n(&self) -> Option<&Normal3<f32>> {
        None
    }

    fn get_phase(&self) -> Option<Arc<HenyeyGreenStein>> {
        self.phase.as_ref().cloned()
    }
}
