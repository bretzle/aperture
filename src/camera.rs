use crate::{
    math::{AnimatedTransform, Matrix, Point, Ray, Transform, Vector},
    matrix,
};

pub struct Camera {
    cam_world: AnimatedTransform,
    raster_screen: Transform,
    proj_div_inv: Transform,
    shutter_open: f32,
    shutter_close: f32,
    shutter_size: f32,
    fov: f32, // TODO: support animations
    scaling: Vector,
    active_at: usize, // frame when camera activates
}

impl Camera {
    pub fn new(
        cam_world: AnimatedTransform,
        fov: f32,
        dims: (usize, usize),
        shutter_size: f32,
        active_at: usize,
    ) -> Self {
        let aspect_ratio = (dims.0 as f32) / (dims.1 as f32);
        let screen = if aspect_ratio > 1.0 {
            [-aspect_ratio, aspect_ratio, -1.0, 1.0]
        } else {
            [-1.0, 1.0, -1.0 / aspect_ratio, 1.0 / aspect_ratio]
        };
        let screen_raster = Transform::scale(&Vector::new(dims.0 as f32, dims.1 as f32, 1.0))
            * Transform::scale(&Vector::new(
                1.0 / (screen[1] - screen[0]),
                1.0 / (screen[2] - screen[3]),
                1.0,
            ))
            * Transform::translate(&Vector::new(-screen[0], -screen[3], 0.0));
        let raster_screen = screen_raster.inverse();

        let far = 1.0;
        let near = 1000.0;
        let proj_div = matrix! {
            1.0, 0.0, 0.0, 0.0;
            0.0, 1.0, 0.0, 0.0;
            0.0, 0.0, far / (far - near), -far * near / (far - near);
            0.0, 0.0, 1.0, 0.0;
        };

        let tan_fov = (fov.to_radians() / 2.0).tan();
        let scaling = Vector::new(tan_fov, tan_fov, 1.0);

        Self {
            cam_world,
            raster_screen,
            proj_div_inv: Transform::from_matrix(proj_div).inverse(),
            shutter_open: 0.0,
            shutter_close: 0.0,
            shutter_size,
            fov,
            scaling,
            active_at,
        }
    }

    pub fn generate_ray(&self, px: &(f32, f32), time: f32) -> Ray {
        // Take the raster space position -> camera space
        let px_pos =
            self.scaling * (self.proj_div_inv * self.raster_screen * Point::new(px.0, px.1, 0.0));
        let d = Vector::new(px_pos.x, px_pos.y, px_pos.z).normalize();
        // Compute the time being sampled for this frame based on shutter open/close times
        let frame_time = (self.shutter_close - self.shutter_open) * time + self.shutter_open;
        self.cam_world.transform(frame_time) * Ray::new(Point::broadcast(0.0), d, frame_time)
    }
}
