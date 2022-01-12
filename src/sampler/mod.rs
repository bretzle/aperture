use crate::color::Color;
use rand::{
    distributions::{IndependentSample, Range},
    StdRng,
};

pub mod block_queue;

pub trait Sampler {
    fn get_samples(&mut self, samples: &mut Vec<(f32, f32)>, rng: &mut StdRng);
    fn get_samples_2d(&mut self, samples: &mut [(f32, f32)], rng: &mut StdRng);
    fn get_samples_1d(&mut self, samples: &mut [f32], rng: &mut StdRng);
    fn max_spp(&self) -> usize;
    fn has_samples(&self) -> bool;
    fn dimensions(&self) -> (u32, u32);
    fn select_block(&mut self, start: (u32, u32));
    fn get_region(&self) -> &Region;
    fn report_results(&mut self, _samples: &[ImageSample]) -> bool {
        true
    }
}

pub struct ImageSample {
    pub x: f32,
    pub y: f32,
    pub color: Color,
}

impl ImageSample {
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        Self { x, y, color }
    }
}

/// Insert a 0 bit between each of the low 16 bits of x
pub fn part1_by1(mut x: u32) -> u32 {
    // x = ---- ---- ---- ---- fedc ba98 7654 3210
    x &= 0x0000ffff;
    // x = ---- ---- fedc ba98 ---- ---- 7654 3210
    x = (x ^ (x << 8)) & 0x00ff00ff;
    // x = ---- fedc ---- ba98 ---- 7654 ---- 3210
    x = (x ^ (x << 4)) & 0x0f0f0f0f;
    // x = --fe --dc --ba --98 --76 --54 --32 --10
    x = (x ^ (x << 2)) & 0x33333333;
    // x = -f-e -d-c -b-a -9-8 -7-6 -5-4 -3-2 -1-0
    (x ^ (x << 1)) & 0x55555555
}
/// Compute the Morton code for the `(x, y)` position
pub fn morton2(p: &(u32, u32)) -> u32 {
    (part1_by1(p.1) << 1) + part1_by1(p.0)
}

pub struct LowDiscrepancy {
    region: Region,
    spp: usize,
    scramble_range: Range<u32>,
}

impl LowDiscrepancy {
    /// Create a low discrepancy sampler to sample the image in `dim.0 * dim.1` sized blocks
    pub fn new(dim: (u32, u32), mut spp: usize) -> LowDiscrepancy {
        if !spp.is_power_of_two() {
            spp = spp.next_power_of_two();
            warn!(
                "LowDiscrepancy sampler requires power of two samples per pixel, rounding up to {}",
                spp
            );
        }
        Self {
            region: Region::new((0, 0), dim),
            spp,
            scramble_range: Range::new(0, u32::MAX),
        }
    }
}

impl Sampler for LowDiscrepancy {
    fn get_samples(&mut self, samples: &mut Vec<(f32, f32)>, rng: &mut StdRng) {
        todo!()
    }

    fn get_samples_2d(&mut self, samples: &mut [(f32, f32)], rng: &mut StdRng) {
        todo!()
    }

    fn get_samples_1d(&mut self, samples: &mut [f32], rng: &mut StdRng) {
        todo!()
    }

    fn max_spp(&self) -> usize {
        todo!()
    }

    fn has_samples(&self) -> bool {
        todo!()
    }

    fn dimensions(&self) -> (u32, u32) {
        todo!()
    }

    fn select_block(&mut self, start: (u32, u32)) {
        todo!()
    }

    fn get_region(&self) -> &Region {
        todo!()
    }
}

pub struct Region {
    pub current: (u32, u32),
    pub start: (u32, u32),
    pub end: (u32, u32),
    pub dim: (u32, u32),
}

impl Region {
    pub fn new(start: (u32, u32), dim: (u32, u32)) -> Self {
        Self {
            current: start,
            start,
            end: (start.0 + dim.0, start.1 + dim.1),
            dim,
        }
    }

    pub fn select_region(&mut self, start: (u32, u32)) {
        self.start.0 = start.0 * self.dim.0;
        self.start.1 = start.1 * self.dim.1;
        self.end.0 = self.start.0 + self.dim.0;
        self.end.1 = self.start.1 + self.dim.1;
        self.current.0 = self.start.0;
        self.current.1 = self.start.1;
    }
}
