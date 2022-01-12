#![feature(box_syntax)]

#[macro_use]
extern crate log;

use aperture::{
    camera::Camera,
    color::Color,
    filter::MitchellNetravali,
    geometry::instance::Instance,
    material::Matte,
    math::{AnimatedTransform, Point, Transform, Vector},
    sampler::{block_queue::BlockQueue, ImageSample, LowDiscrepancy, Sampler},
    shapes::Sphere,
    texture::{ConstantColor, ConstantScalar},
    RenderTarget,
};
use rand::StdRng;
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

    info!("Starting...");

    let filter = box MitchellNetravali::new(2.0, 2.0, 1.0 / 3.0, 1.0 / 3.0);
    let rt = RenderTarget::new((WIDTH, HEIGHT), (20, 20), filter);
    let transform = AnimatedTransform::unanimated(&Transform::look_at(
        &Point::new(0.0, 0.0, -10.0),
        &Point::new(0.0, 0.0, 0.0),
        &Vector::new(0.0, 1.0, 0.0),
    ));

    let camera = Camera::new(transform, 40.0, rt.dimensions(), 0.5, 0);
    let sphere = Sphere::new(1.5);
    let geometry_lock = Arc::new(sphere);
    let texture = Arc::new(ConstantColor::new(Color::new(0.740063, 0.742313, 0.733934)));
    let roughness = Arc::new(ConstantScalar::new(1.0));
    let white_wall = Matte::new(texture, roughness);
    let material_lock = Arc::new(white_wall);
    let position_transform =
        AnimatedTransform::unanimated(&Transform::translate(&Vector::new(0.0, 2.0, 0.0)));

    let instance = Instance::receiver(
        geometry_lock,
        material_lock,
        position_transform,
        "single_sphere".to_string(),
    );

    info!("Created Instance.");

    let dim = rt.dimensions();

    let block_queue = BlockQueue::new((dim.0 as u32, dim.1 as u32), (8, 8), (0, 0));
    let block_dim = block_queue.block_dim();
    let mut sampler = LowDiscrepancy::new(block_dim, 2);
    let mut sample_pos = Vec::with_capacity(sampler.max_spp());
    let mut block_samples =
        Vec::with_capacity(sampler.max_spp() * (block_dim.0 * block_dim.1) as usize);

    let mut rng = StdRng::new()?;

    info!("Rendering...");

    for b in block_queue.iter() {
        sampler.select_block(b);
        while sampler.has_samples() {
            sampler.get_samples(&mut sample_pos, &mut rng);
            for s in &sample_pos[..] {
                let mut ray = camera.generate_ray(s, 0.0);
                if let Some(_) = instance.intersect(&mut ray) {
                    block_samples.push(ImageSample::new(s.0, s.1, Color::WHITE));
                } else {
                    // For correct filtering we also MUST set a background color of some kind
                    // if we miss, otherwise the pixel weights will be wrong and we'll see object
                    // fringes and artifacts at object boundaries w/ nothing. Try removing this
                    // line and rendering again.
                    block_samples.push(ImageSample::new(s.0, s.1, Color::BLACK));
                }
            }
        }
        // We write all samples at once so we don't need to lock the render target tiles as often
        rt.write(&block_samples, sampler.get_region());
        block_samples.clear();
    }

    info!("Saving...");

    // Get the sRGB8 render buffer from the floating point framebuffer and save it
    let img = rt.get_render();
    match image::save_buffer(
        "sphere.png",
        &img[..],
        dim.0 as u32,
        dim.1 as u32,
        image::RGB(8),
    ) {
        Ok(_) => {}
        Err(e) => println!("Error saving image, {}", e),
    };

    Ok(())
}
