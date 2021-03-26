use crate::{
    camera::CameraSample,
    utils::{Rng, ONE_MINUS_EPSILON},
};
use log::info;
use maths::*;
use num::Zero;

pub trait Sampler: Send + Sync {
    fn start_pixel(&mut self, p: Point2i);
    fn get_1d(&mut self) -> f32;
    fn get_2d(&mut self) -> Point2f;
    fn get_camera_sample(&mut self, p_raster: Point2i) -> CameraSample;
    fn request_1d_array(&mut self, n: usize);
    fn request_2d_array(&mut self, n: usize);
    fn round_count(&self, count: usize) -> usize;
    fn get_1d_array(&mut self, n: usize) -> Option<&[f32]>;
    fn get_2d_array(&mut self, n: usize) -> Option<&[Point2f]>;
    fn start_next_sample(&mut self) -> bool;
    fn reseed(&mut self, seed: u64);
    fn spp(&self) -> usize;
    fn box_clone(&self) -> Box<dyn Sampler>;
    fn current_sample_number(&self) -> usize;
}

#[derive(Clone)]
pub struct ZeroTwoSequence {
    spp: usize,
    current_pixel: Point2i,
    current_pixel_sample_index: usize,
    sample_1d_array_sizes: Vec<usize>,
    sample_2d_array_sizes: Vec<usize>,
    sample_array_1d: Vec<Vec<f32>>,
    sample_array_2d: Vec<Vec<Point2f>>,
    array_1d_offset: usize,
    array_2d_offset: usize,
    // Pixel sampler data
    samples_1d: Vec<Vec<f32>>,
    samples_2d: Vec<Vec<Point2f>>,
    current_1d_dimension: usize,
    current_2d_dimension: usize,
    rng: Rng,
}

impl ZeroTwoSequence {
    pub fn new(spp: usize, n_sampled_dimensions: usize) -> ZeroTwoSequence {
        let spp = spp.next_power_of_two();
        let mut samples1d = Vec::with_capacity(n_sampled_dimensions);
        let mut samples2d = Vec::with_capacity(n_sampled_dimensions);
        for _ in 0..n_sampled_dimensions {
            samples1d.push(vec![0.0; spp]);
            samples2d.push(vec![Point2f::new(0.0, 0.0); spp]);
        }

        ZeroTwoSequence {
            spp,
            current_pixel: Point2i::new(0, 0),
            current_pixel_sample_index: 0,
            sample_1d_array_sizes: Vec::new(),
            sample_2d_array_sizes: Vec::new(),
            sample_array_1d: Vec::new(),
            sample_array_2d: Vec::new(),
            array_1d_offset: 0,
            array_2d_offset: 0,
            samples_1d: samples1d,
            samples_2d: samples2d,
            current_1d_dimension: 0,
            current_2d_dimension: 0,
            rng: Rng::new(),
        }
    }
}

impl Sampler for ZeroTwoSequence {
    fn start_pixel(&mut self, p: Point2i) {
        // Generate 1D and 2D pixel sample components using (0, 2)-sequence
        for i in 0..self.samples_1d.len() {
            van_der_corput(
                1,
                self.spp as u32,
                &mut self.samples_1d[i][..],
                &mut self.rng,
            );
        }
        for i in 0..self.samples_2d.len() {
            sobol_2d(
                1,
                self.spp as u32,
                &mut self.samples_2d[i][..],
                &mut self.rng,
            );
        }

        // generate 1d and 2d array samples
        for i in 0..self.sample_1d_array_sizes.len() {
            van_der_corput(
                self.sample_1d_array_sizes[i] as u32,
                self.spp as u32,
                &mut self.sample_array_1d[i][..],
                &mut self.rng,
            );
        }
        for i in 0..self.sample_2d_array_sizes.len() {
            sobol_2d(
                self.sample_2d_array_sizes[i] as u32,
                self.spp as u32,
                &mut self.sample_array_2d[i][..],
                &mut self.rng,
            );
        }

        self.current_pixel = p;
        self.current_pixel_sample_index = 0;
        self.array_1d_offset = 0;
        self.array_2d_offset = 0;
    }

    fn start_next_sample(&mut self) -> bool {
        self.array_1d_offset = 0;
        self.array_2d_offset = 0;
        self.current_1d_dimension = 0;
        self.current_2d_dimension = 0;
        self.current_pixel_sample_index += 1;
        self.current_pixel_sample_index < self.spp
    }

    fn request_1d_array(&mut self, n: usize) {
        self.sample_1d_array_sizes.push(n);
        let mut vec = Vec::new();
        vec.resize(n * self.spp, 0.0);
        self.sample_array_1d.push(vec);
    }

    fn request_2d_array(&mut self, n: usize) {
        info!("Requesting 2d array of {} samples", n);
        self.sample_2d_array_sizes.push(n);
        let mut vec = Vec::new();
        vec.resize(n * self.spp, Point2f::zero());
        self.sample_array_2d.push(vec);
    }

    fn get_1d_array(&mut self, n: usize) -> Option<&[f32]> {
        if self.array_1d_offset == self.sample_array_1d.len() {
            return None;
        }
        assert_eq!(self.sample_1d_array_sizes[self.array_1d_offset], n);
        assert!(self.current_pixel_sample_index < self.spp);
        let res =
            &self.sample_array_1d[self.array_1d_offset][(self.current_pixel_sample_index * n)..];
        self.array_1d_offset += 1;
        Some(res)
    }

    fn get_2d_array(&mut self, n: usize) -> Option<&[Point2f]> {
        if self.array_2d_offset == self.sample_array_2d.len() {
            return None;
        }
        assert_eq!(self.sample_2d_array_sizes[self.array_2d_offset], n);
        assert!(self.current_pixel_sample_index < self.spp);
        let res =
            &self.sample_array_2d[self.array_2d_offset][(self.current_pixel_sample_index * n)..];
        self.array_2d_offset += 1;
        Some(res)
    }

    fn get_1d(&mut self) -> f32 {
        if self.current_1d_dimension < self.samples_1d.len() {
            let res = self.samples_1d[self.current_1d_dimension][self.current_pixel_sample_index];
            self.current_1d_dimension += 1;
            res
        } else {
            self.rng.uniform_f32()
        }
    }

    fn get_2d(&mut self) -> Point2f {
        if self.current_2d_dimension < self.samples_2d.len() {
            let res = self.samples_2d[self.current_2d_dimension][self.current_pixel_sample_index];
            self.current_2d_dimension += 1;
            res
        } else {
            let x = self.rng.uniform_f32();
            let y = self.rng.uniform_f32();
            // For some reason, the C++ version evaluates its second argument first...
            // Replicating the same behaviour helps with debuggability.
            Point2f::new(y, x)
        }
    }

    fn get_camera_sample(&mut self, p_raster: Point2i) -> CameraSample {
        let film = Point2f::from(p_raster) + self.get_2d();
        let time = self.get_1d();
        let lens = self.get_2d();

        CameraSample { film, lens, time }
    }

    fn round_count(&self, count: usize) -> usize {
        count.next_power_of_two()
    }

    fn reseed(&mut self, seed: u64) {
        self.rng.set_sequence(seed);
    }

    fn spp(&self) -> usize {
        self.spp
    }

    fn box_clone(&self) -> Box<dyn Sampler> {
        Box::new(self.clone())
    }

    fn current_sample_number(&self) -> usize {
        self.current_pixel_sample_index
    }
}

fn van_der_corput(
    n_samples_per_pixel_sample: u32,
    n_pixel_samples: u32,
    samples: &mut [f32],
    rng: &mut Rng,
) {
    let scramble = rng.uniform_u32();
    let total_samples = n_samples_per_pixel_sample * n_pixel_samples;
    gray_code_sample(&CVAN_DER_CORPUT, total_samples, scramble, &mut samples[..]);
    // Randomly shuffle 1D points
    for i in 0..n_pixel_samples {
        shuffle(
            &mut samples[(i as usize * n_samples_per_pixel_sample as usize)..],
            n_samples_per_pixel_sample,
            1,
            rng,
        );
    }
    shuffle(
        &mut samples[..],
        n_pixel_samples,
        n_samples_per_pixel_sample,
        rng,
    );
}

fn sobol_2d(
    n_samples_per_pixel_sample: u32,
    n_pixel_samples: u32,
    samples: &mut [Point2f],
    rng: &mut Rng,
) {
    let scramble = Point2i::new(rng.uniform_u32() as i32, rng.uniform_u32() as i32);

    gray_code_sample_2d(
        &CSOBOL[0],
        &CSOBOL[1],
        n_samples_per_pixel_sample * n_pixel_samples,
        scramble,
        &mut samples[..],
    );
    // Randomly shuffle 2D points
    for i in 0..n_pixel_samples {
        shuffle(
            &mut samples[(i as usize * n_samples_per_pixel_sample as usize)..],
            n_samples_per_pixel_sample,
            1,
            rng,
        );
    }
    shuffle(
        &mut samples[..],
        n_pixel_samples,
        n_samples_per_pixel_sample,
        rng,
    );
}

// fn radical_inverse(base: u32, a: u64) -> f32 {
//     match base {
//         0 => reverse_bits_64(a) as f32 * 5.4210108624275222e-20,
//         1 => radical_inverse_specialized(3, a),
//         2 => radical_inverse_specialized(5, a),
//         3 => radical_inverse_specialized(7, a),
//         4 => radical_inverse_specialized(11, a),
//         5 => radical_inverse_specialized(13, a),
//         _ => unimplemented!(),
//     }
// }

// fn reverse_bits_32(n: u32) -> u32 {
//     let mut n = n;
//     n = (n << 16) | (n >> 16);
//     n = ((n & 0x00ff00ff) << 8) | ((n & 0xff00ff00) >> 8);
//     n = ((n & 0x0f0f0f0f) << 4) | ((n & 0xf0f0f0f0) >> 4);
//     n = ((n & 0x33333333) << 2) | ((n & 0xcccccccc) >> 2);
//     n = ((n & 0x55555555) << 1) | ((n & 0xaaaaaaaa) >> 1);
//     n
// }

// fn reverse_bits_64(n: u64) -> u64 {
//     let n0 = reverse_bits_32(n as u32);
//     let n1 = reverse_bits_32((n >> 32) as u32);
//     (u64::from(n0) << 32) | u64::from(n1)
// }

// fn radical_inverse_specialized(base: u32, a: u64) -> f32 {
//     let mut a = a;
//     let inv_base: f32 = 1.0 / base as f32;
//     let mut reversed_digits: u64 = 0;
//     let mut inv_base_n = 1.0;
//     while a != 0 {
//         let next = a / u64::from(base);
//         let digit = a - next * u64::from(base);
//         reversed_digits = reversed_digits * u64::from(base) + digit;
//         inv_base_n *= inv_base;
//         a = next;
//     }
//     assert!(reversed_digits as f32 * inv_base_n < 1.00001);
//     f32::min(reversed_digits as f32 * inv_base_n, ONE_MINUS_EPSILON)
// }

fn gray_code_sample(c: &[u32], n: u32, scramble: u32, p: &mut [f32]) {
    let mut v = scramble;
    for i in 0..n {
        p[i as usize] = (v as f32 * 2.3283064365386963E-10f32).min(ONE_MINUS_EPSILON);
        v ^= c[(i + 1).trailing_zeros() as usize];
    }
}

fn gray_code_sample_2d(c0: &[u32], c1: &[u32], n: u32, scramble: Point2i, p: &mut [Point2f]) {
    let mut v = [scramble.x as u32, scramble.y as u32];
    for i in 0..n {
        p[i as usize].x = (v[0] as f32 * 2.3283064365386963E-10f32).min(ONE_MINUS_EPSILON);
        p[i as usize].y = (v[1] as f32 * 2.3283064365386963E-10f32).min(ONE_MINUS_EPSILON);
        v[0] ^= c0[(i + 1).trailing_zeros() as usize];
        v[1] ^= c1[(i + 1).trailing_zeros() as usize];
    }
}

fn shuffle<T>(samp: &mut [T], count: u32, n_dimensions: u32, rng: &mut Rng) {
    for i in 0..count {
        let other = i + rng.uniform_u32_bounded(count - i);
        for j in 0..n_dimensions {
            samp.swap(
                (n_dimensions * i + j) as usize,
                (n_dimensions * other + j) as usize,
            );
        }
    }
}

const CVAN_DER_CORPUT: [u32; 32] = [
    0b_10000000000000000000000000000000,
    0b_1000000000000000000000000000000,
    0b_100000000000000000000000000000,
    0b_10000000000000000000000000000,
    0b_1000000000000000000000000000,
    0b_100000000000000000000000000,
    0b_10000000000000000000000000,
    0b_1000000000000000000000000,
    0b_100000000000000000000000,
    0b_10000000000000000000000,
    0b_1000000000000000000000,
    0b_100000000000000000000,
    0b_10000000000000000000,
    0b_1000000000000000000,
    0b_100000000000000000,
    0b_10000000000000000,
    0b_1000000000000000,
    0b_100000000000000,
    0b_10000000000000,
    0b_1000000000000,
    0b_100000000000,
    0b_10000000000,
    0b_1000000000,
    0b_100000000,
    0b_10000000,
    0b_1000000,
    0b_100000,
    0b_10000,
    0b_1000,
    0b_100,
    0b_10,
    0b_1,
];
/// Generator matrices for Sobol 2D
const CSOBOL: [[u32; 32]; 2] = [
    [
        0x80000000, 0x40000000, 0x20000000, 0x10000000, 0x8000000, 0x4000000, 0x2000000, 0x1000000,
        0x800000, 0x400000, 0x200000, 0x100000, 0x80000, 0x40000, 0x20000, 0x10000, 0x8000, 0x4000,
        0x2000, 0x1000, 0x800, 0x400, 0x200, 0x100, 0x80, 0x40, 0x20, 0x10, 0x8, 0x4, 0x2, 0x1,
    ],
    [
        0x80000000, 0xc0000000, 0xa0000000, 0xf0000000, 0x88000000, 0xcc000000, 0xaa000000,
        0xff000000, 0x80800000, 0xc0c00000, 0xa0a00000, 0xf0f00000, 0x88880000, 0xcccc0000,
        0xaaaa0000, 0xffff0000, 0x80008000, 0xc000c000, 0xa000a000, 0xf000f000, 0x88008800,
        0xcc00cc00, 0xaa00aa00, 0xff00ff00, 0x80808080, 0xc0c0c0c0, 0xa0a0a0a0, 0xf0f0f0f0,
        0x88888888, 0xcccccccc, 0xaaaaaaaa, 0xffffffff,
    ],
];
