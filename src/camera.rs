use crate::{
    film::Film,
    geometry::{Bounds2f, Matrix, Point2f, Point3f, Ray, RayDifferential, Vector3f},
    sampling,
    transform::Transform,
};
use num::Zero;
use std::fmt::Debug;

pub trait Camera: Debug + Send + Sync {
    fn get_film(&self) -> &Film;
    fn generate_ray(&self, sample: CameraSample) -> Ray;
    fn generate_ray_differential(&self, sample: CameraSample) -> Ray;
}

pub struct CameraSample {
    pub film: Point2f,
    pub lens: Point2f,
    pub time: f32,
}

// TODO
pub struct ProjectiveCamera {}

// TODO
pub struct OrthographicCamera {}

// TODO
#[derive(Debug)]
pub struct PerspectiveCamera {
    film: Box<Film>,
    camera_to_world: Transform,
    camera_to_screen: Matrix,
    raster_to_camera: Transform,
    lens_radius: f32,
    focal_distance: f32,
    dx_camera: Vector3f,
    dy_camera: Vector3f,
}

impl PerspectiveCamera {
    pub fn new(
        camera_to_world: Transform,
        screen_window: Bounds2f,
        lens_radius: f32,
        focal_distance: f32,
        fov: f32,
        film: Box<Film>,
    ) -> Self {
        let camera_to_screen = Transform::perspective(fov, 1e-2, 1000.0);
        let screen_to_raster = Transform::scale(
            film.full_resolution.x as f32,
            film.full_resolution.y as f32,
            1.0,
        ) * Transform::scale(
            1.0 / (screen_window.p_max.x - screen_window.p_min.x),
            1.0 / (screen_window.p_min.y - screen_window.p_max.y),
            1.0,
        ) * Transform::translate(&Vector3f::new(
            -screen_window.p_min.x,
            -screen_window.p_max.y,
            0.0,
        ));

        let raster_to_screen = screen_to_raster.inverse();
        let raster_to_camera = camera_to_screen.inverse() * raster_to_screen;

        // compute differential changes in origin for perspective camera rays
        let dx_camera = (&raster_to_camera * &Point3f::new(1.0, 0.0, 0.0))
            - (&raster_to_camera * &Point3f::new(0.0, 0.0, 0.0));
        let dy_camera = (&raster_to_camera * &Point3f::new(0.0, 1.0, 0.0))
            - (&raster_to_camera * &Point3f::new(0.0, 0.0, 0.0));

        Self {
            film,
            camera_to_world,
            camera_to_screen: camera_to_screen.m,
            raster_to_camera,
            lens_radius,
            focal_distance,
            dx_camera,
            dy_camera,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn get_film(&self) -> &Film {
        &self.film
    }

    fn generate_ray(&self, sample: CameraSample) -> Ray {
        let p_film = Point3f::new(sample.film.x, sample.film.y, 0.0);
        let p_camera: Point3f = &self.raster_to_camera * &p_film;

        let mut ray = Ray::new(Point3f::zero(), Vector3f::from(p_camera).normalize());
        // modify ray for depth of field
        if self.lens_radius > 0.0 {
            // Sample point on lens
            let p_lens = self.lens_radius * sampling::concentric_sample_disk(sample.lens);
            // Compute point on plane of focus
            let ft = self.focal_distance / ray.d.z;
            let p_focus = ray.at(ft);
            // Update ray for effect of lens
            ray.o = Point3f::new(p_lens.x, p_lens.y, 0.0);
            ray.d = (p_focus - ray.o).normalize();
        }
        ray.transform(&self.camera_to_world).0
    }

    fn generate_ray_differential(&self, sample: CameraSample) -> Ray {
        let p_film = Point3f::new(sample.film.x, sample.film.y, 0.0);
        let p_camera = &self.raster_to_camera * &p_film;

        let mut ray = Ray::new(Point3f::zero(), Vector3f::from(p_camera).normalize());
        // modify ray for depth of field
        if self.lens_radius > 0.0 {
            // Sample point on lens
            let p_lens = self.lens_radius * sampling::concentric_sample_disk(sample.lens);
            // Compute point on plane of focus
            let ft = self.focal_distance / ray.d.z;
            let p_focus = ray.at(ft);
            // Update ray for effect of lens
            ray.o = Point3f::new(p_lens.x, p_lens.y, 0.0);
            ray.d = (p_focus - ray.o).normalize();
        }
        // compute offset rays for PerspectiveCamera ray differentials
        let diff = if self.lens_radius > 0.0 {
            // Sample point on lens
            let p_lens = self.lens_radius * sampling::concentric_sample_disk(sample.lens);
            let origin = Point3f::new(p_lens.x, p_lens.y, 0.0);

            // ray differential in x direction
            let dx = Vector3f::from(p_camera + self.dx_camera).normalize();
            let ft_x = self.focal_distance / dx.z;
            let p_focus_x = Point3f::from(ft_x * dx);
            let rx_dir = (p_focus_x - origin).normalize();

            // ray differential in x direction
            let dy = Vector3f::from(p_camera + self.dy_camera).normalize();
            let ft_y = self.focal_distance / dy.z;
            let p_focus_y = Point3f::from(ft_y * dy);
            let ry_dir = (p_focus_y - origin).normalize();

            RayDifferential {
                rx_origin: origin,
                ry_origin: origin,
                rx_direction: rx_dir,
                ry_direction: ry_dir,
            }
        } else {
            RayDifferential {
                rx_origin: ray.o,
                ry_origin: ray.o,
                rx_direction: (Vector3f::from(p_camera) + self.dx_camera).normalize(),
                ry_direction: (Vector3f::from(p_camera) + self.dy_camera).normalize(),
            }
        };

        ray.differential = Some(diff);

        ray.transform(&self.camera_to_world).0
    }
}

// TODO
pub struct EnvironmentCamera {}
