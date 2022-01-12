use std::iter;

use crate::color::Color;
use rand::{
    distributions::{IndependentSample, Range},
    Rng, StdRng,
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
        samples.clear();
        if !self.has_samples() {
            return;
        }
        if samples.len() < self.spp {
            let len = self.spp - samples.len();
            samples.extend(iter::repeat((0.0, 0.0)).take(len));
        }
        self.get_samples_2d(&mut samples[..], rng);
        for s in samples.iter_mut() {
            s.0 += self.region.current.0 as f32;
            s.1 += self.region.current.1 as f32;
        }

        self.region.current.0 += 1;
        if self.region.current.0 == self.region.end.0 {
            self.region.current.0 = self.region.start.0;
            self.region.current.1 += 1;
        }
    }

    fn get_samples_2d(&mut self, samples: &mut [(f32, f32)], rng: &mut StdRng) {
        let scramble = (
            self.scramble_range.ind_sample(rng),
            self.scramble_range.ind_sample(rng),
        );
        sample_2d(samples, scramble, 0);
        rng.shuffle(samples);
    }

    fn get_samples_1d(&mut self, samples: &mut [f32], rng: &mut StdRng) {
        todo!()
    }

    fn max_spp(&self) -> usize {
        self.spp
    }

    fn has_samples(&self) -> bool {
        self.region.current.1 != self.region.end.1
    }

    fn dimensions(&self) -> (u32, u32) {
        todo!()
    }

    fn select_block(&mut self, start: (u32, u32)) {
        self.region.select_region(start)
    }

    fn get_region(&self) -> &Region {
        &self.region
    }
}

fn sample_2d(samples: &mut [(f32, f32)], scramble: (u32, u32), offset: u32) {
    for s in samples.iter_mut().enumerate() {
        *s.1 = sample_02(s.0 as u32 + offset, scramble);
    }
}

fn sample_02(n: u32, scramble: (u32, u32)) -> (f32, f32) {
    (van_der_corput(n, scramble.0), sobol(n, scramble.1))
}

/// Generate a scrambled Van der Corput sequence value
/// as described by Kollig & Keller (2002) and in PBR
/// method is specialized for base 2
pub fn van_der_corput(mut n: u32, scramble: u32) -> f32 {
    n = (n << 16) | (n >> 16);
    n = ((n & 0x00ff00ff) << 8) | ((n & 0xff00ff00) >> 8);
    n = ((n & 0x0f0f0f0f) << 4) | ((n & 0xf0f0f0f0) >> 4);
    n = ((n & 0x33333333) << 2) | ((n & 0xcccccccc) >> 2);
    n = ((n & 0x55555555) << 1) | ((n & 0xaaaaaaaa) >> 1);
    n ^= scramble;
    f32::min(
        ((n >> 8) & 0xffffff) as f32 / ((1 << 24) as f32),
        1.0 - f32::EPSILON,
    )
}
/// Generate a scrambled Sobol' sequence value
/// as described by Kollig & Keller (2002) and in PBR
/// method is specialized for base 2
pub fn sobol(mut n: u32, mut scramble: u32) -> f32 {
    let mut i = 1 << 31;
    while n != 0 {
        if n & 0x1 != 0 {
            scramble ^= i;
        }
        n >>= 1;
        i ^= i >> 1;
    }
    f32::min(
        ((scramble >> 8) & 0xffffff) as f32 / ((1 << 24) as f32),
        1.0 - f32::EPSILON,
    )
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
