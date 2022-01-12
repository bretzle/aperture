#![feature(box_syntax)]

use aperture::{
    camera::Camera,
    color::Color,
    filter::MitchellNetravali,
    material::Matte,
    math::{AnimatedTransform, Point, Transform, Vector},
    shapes::Sphere,
    texture::{ConstantColor, ConstantScalar},
    RenderTarget,
};
use std::sync::Arc;

// const PBRT_PATH: &str = "scenes/cornell-box/scene.pbrt";
// const PBRT_PATH: &str = "hello.pbrt";

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .parse_filters("info")
        .init();

    let filter = box MitchellNetravali::new(2.0, 2.0, 1.0 / 3.0, 1.0 / 3.0);
    let rt = RenderTarget::new((WIDTH, HEIGHT), (20, 20), filter);
    let transform = AnimatedTransform::unanimated(&Transform::look_at(
        &Point::new(0.0, 0.0, -10.0),
        &Point::new(0.0, 0.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
    ));

    let camera = Camera::new(transform, 40.0, rt.dimensions(), 0.5, 0);
    let sphere = Arc::new(Sphere::new(1.5));
    let geometry_lock = Arc::new(sphere);
    let texture = Arc::new(ConstantColor::new(Color::new(0.740063, 0.742313, 0.733934)));
    let roughness = Arc::new(ConstantScalar::new(1.0));
    let white_wall = Matte::new(texture, roughness);
    let material_lock = Arc::new(white_wall);
    let position_transform =
        AnimatedTransform::unanimated(&Transform::translate(&Vector::new(0.0, 2.0, 0.0)));

    Ok(())
}
