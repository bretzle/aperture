//! Provides the Sampler trait which is implemented by the various samplers
//! to provide stratified, low-discrepancy, adaptive sampling methods and so
//! on through a simple trait interface

use crate::film::ImageSample;
use rand::StdRng;

pub use self::{adaptive::Adaptive, block_queue::BlockQueue, ld::LowDiscrepancy, uniform::Uniform};

pub mod adaptive;
pub mod block_queue;
pub mod ld;
pub mod morton;
pub mod uniform;

/// Provides the interface for all samplers to implement. Defines functions for
/// getting samples from the sampler and checking the sampler has finished sampling
/// the region
#[enum_dispatch(Samplers)]
pub trait Sampler {
    /// Fill the vector with 2D pixel coordinate samples for a single pixel
    /// in the region being sampled. If the sampler doesn't have any more samples
    /// for the region the vector will be empty
    /// Samplers that use randomness to compute samples will use the thread rng
    fn get_samples(&mut self, samples: &mut Vec<(f32, f32)>, rng: &mut StdRng);
    /// Fill the slice with 2D samples from the sampler
    fn get_samples_2d(&mut self, samples: &mut [(f32, f32)], rng: &mut StdRng);
    /// Fill the slice with 1D samples from the sampler
    fn get_samples_1d(&mut self, samples: &mut [f32], rng: &mut StdRng);
    /// Get the max number of samples this sampler will take per pixel
    fn max_spp(&self) -> usize;
    /// Check if the sampler has more samples for the region being sampled
    fn has_samples(&self) -> bool;
    /// Get the dimensions of the region being sampled in pixels
    fn dimensions(&self) -> (u32, u32);
    /// Move to a new block of the image to sample with this sampler by specifying
    /// the starting `(x, y)` block index for the new block. The block starting
    /// position will be calculated as `dimensions * start`
    fn select_block(&mut self, start: (u32, u32));
    /// Get the region being samples
    fn get_region(&self) -> &Region;
    /// Let the sampler inspect the results of sampling the pixel so it can
    /// determine if more samples should be taken. Returns true if these samples
    /// are ok to use, false if more need to be taken. The default implementation
    /// just returns true.
    fn report_results(&mut self, _samples: &[ImageSample]) -> bool {
        true
    }
}

#[enum_dispatch]
pub enum Samplers {
    Adaptive,
    LowDiscrepancy,
    Uniform,
}

/// Provides a simple way to pass around a 3 component sample consisting of one 2D and
/// one 1D sample
#[derive(Debug)]
pub struct Sample {
    /// The 2D sample
    pub two_d: (f32, f32),
    /// The 1D sample
    pub one_d: f32,
}

impl Sample {
    /// Create a new sample taking the 2D sample values from the slice
    pub fn new(two_d: &(f32, f32), one_d: f32) -> Self {
        Self {
            two_d: *two_d,
            one_d,
        }
    }
}

/// Defines a region of the image being sampled in pixel coordinates
#[derive(Clone, Copy, Debug)]
pub struct Region {
    /// Current coordinates of the pixel to sample (x, y)
    pub current: (u32, u32),
    /// Coordinates of the start of region being sampled (x, y)
    pub start: (u32, u32),
    /// Coordinates of the end of the region being sampled (x, y)
    pub end: (u32, u32),
    /// Dimensions of the region being sampled
    pub dim: (u32, u32),
}

impl Region {
    /// Create a new region starting at `start` with dimension `dim`
    pub fn new(start: (u32, u32), dim: (u32, u32)) -> Self {
        Self {
            current: start,
            start,
            end: (start.0 + dim.0, start.1 + dim.1),
            dim,
        }
    }

    /// Select a new region starting at region indices `start` with the same dimensions
    /// eg. with blocks of width 8 the 2nd region along x is at 16 so to get
    /// this block you'd set start.0 = 2
    pub fn select_region(&mut self, start: (u32, u32)) {
        self.start.0 = start.0 * self.dim.0;
        self.start.1 = start.1 * self.dim.1;
        self.end.0 = self.start.0 + self.dim.0;
        self.end.1 = self.start.1 + self.dim.1;
        self.current.0 = self.start.0;
        self.current.1 = self.start.1;
    }
}
