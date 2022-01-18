//! The multithreaded module provides a multithreaded execution for rendering
//! the image.

use crate::{
    exec::{Config, Exec},
    film::{Colorf, ImageSample, RenderTarget},
    geometry::{Emitter, Instance},
    integrator::Integrator,
    sampler::{self, BlockQueue, Sampler, Samplers},
    scene::Scene,
};
use light_arena;
use rand::StdRng;
use scoped_threadpool::Pool;
use std::{iter, time::SystemTime};

/// The `MultiThreaded` execution uses a configurable number of threads in
/// a threadpool to render each frame
pub struct MultiThreaded {
    pool: Pool,
}

impl MultiThreaded {
    /// Create a new multithreaded renderer which will use `num_threads` to render the image
    pub fn new(num_threads: u32) -> MultiThreaded {
        MultiThreaded {
            pool: Pool::new(num_threads),
        }
    }
    /// Launch a rendering job in parallel across the threads and wait for it to finish
    fn render_parallel(&mut self, scene: &Scene, rt: &RenderTarget, config: &Config) {
        let dim = rt.dimensions();
        let block_queue =
            BlockQueue::new((dim.0 as u32, dim.1 as u32), (8, 8), config.select_blocks);
        let light_list: Vec<_> = scene
            .bvh
            .iter()
            .filter_map(|x| match *x {
                Instance::Emitter(ref e) => Some(e),
                _ => None,
            })
            .collect();
        assert!(!light_list.is_empty(), "At least one light is required");
        let n = self.pool.thread_count();
        self.pool.scoped(|scope| {
            for _ in 0..n {
                let b = &block_queue;
                let r = &rt;
                let l = &light_list;
                scope.execute(move || {
                    thread_work(config.spp, b, scene, r, l);
                });
            }
        });
    }
}

impl Exec for MultiThreaded {
    fn render(&mut self, scene: &mut Scene, rt: &mut RenderTarget, config: &Config) {
        println!(
            "Rendering using {} threads\n--------------------",
            self.pool.thread_count()
        );
        let time_step = config.frame_info.time / config.frame_info.frames as f32;
        let frame_start_time = config.current_frame as f32 * time_step;
        let frame_end_time = (config.current_frame as f32 + 1.0) * time_step;
        scene.update_frame(config.current_frame, frame_start_time, frame_end_time);

        println!(
            "Frame {}: rendering for {} to {}",
            config.current_frame, frame_start_time, frame_end_time
        );
        let scene_start = SystemTime::now();
        self.render_parallel(scene, rt, config);
        let time = scene_start.elapsed().expect("Failed to get render time?");
        println!(
            "Frame {}: rendering took {:4}s",
            config.current_frame,
            time.as_secs() as f64 + time.subsec_nanos() as f64 * 1e-9
        );
    }
}

fn thread_work(
    spp: usize,
    queue: &BlockQueue,
    scene: &Scene,
    target: &RenderTarget,
    light_list: &[&Emitter],
) {
    let mut sampler: Samplers = sampler::LowDiscrepancy::new(queue.block_dim(), spp).into();
    let mut sample_pos = Vec::with_capacity(sampler.max_spp());
    let mut time_samples: Vec<_> = iter::repeat(0.0).take(sampler.max_spp()).collect();
    let block_dim = queue.block_dim();
    let mut block_samples =
        Vec::with_capacity(sampler.max_spp() * (block_dim.0 * block_dim.1) as usize);
    let mut rng = match StdRng::new() {
        Ok(r) => r,
        Err(e) => {
            println!("Failed to get StdRng, {}", e);
            return;
        }
    };
    let mut arena = light_arena::MemoryArena::new(8);
    let camera = scene.active_camera();
    // Grab a block from the queue and start working on it, submitting samples
    // to the render target thread after each pixel
    for b in queue.iter() {
        sampler.select_block(b);
        let mut pixel_samples = 0;
        while sampler.has_samples() {
            // Get samples for a pixel and render them
            sampler.get_samples(&mut sample_pos, &mut rng);
            sampler.get_samples_1d(&mut time_samples[..], &mut rng);
            for (s, t) in sample_pos.iter().zip(time_samples.iter()) {
                let alloc = arena.allocator();
                let mut ray = camera.generate_ray(s, *t);
                if let Some(hit) = scene.intersect(&mut ray) {
                    let c = scene
                        .integrator
                        .illumination(
                            scene,
                            light_list,
                            &ray,
                            &hit,
                            &mut sampler,
                            &mut rng,
                            &alloc,
                        )
                        .clamp();
                    block_samples.push(ImageSample::new(s.0, s.1, c));
                } else {
                    block_samples.push(ImageSample::new(s.0, s.1, Colorf::black()));
                }
            }
            // If the samples are ok the samples for the next pixel start at the end of the current
            // pixel's samples
            if sampler.report_results(&block_samples[pixel_samples..]) {
                pixel_samples = block_samples.len();
            }
        }
        target.write(&block_samples, sampler.get_region());
        block_samples.clear();
    }
}
