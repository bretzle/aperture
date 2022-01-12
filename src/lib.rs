#![allow(clippy::return_self_not_must_use)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate log;

use filter::Filter;
use sampler::{ImageSample, Region};

pub mod camera;
pub mod color;
pub mod filter;
pub mod geometry;
pub mod material;
pub mod math;
pub mod rand;
pub mod sampler;
pub mod shapes;
pub mod texture;

pub struct RenderTarget<F> {
    width: usize,
    height: usize,
    lock_size: (i32, i32),
    filter: Box<F>,
}

impl<F> RenderTarget<F>
where
    F: Filter + Send + Sync,
{
    pub fn new(image_dim: (usize, usize), lock_size: (usize, usize), filter: Box<F>) -> Self {
        let (width, height) = image_dim;
        if width % lock_size.0 != 0 || height % lock_size.1 != 0 {
            panic!(
                "Image with dimension {:?} not evenly divided by blocks of {:?}",
                image_dim, lock_size
            );
        }

        Self {
            width,
            height,
            lock_size: (lock_size.0 as i32, lock_size.1 as i32),
            filter,
        }
    }

    pub fn write(&self, samples: &[ImageSample], region: &Region) {
        todo!()
    }

    pub fn get_render(&self) -> Vec<u8> {
        todo!()
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}
