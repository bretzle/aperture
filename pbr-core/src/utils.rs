use crate::{bounds::Bounds2i, spectrum::Spectrum};
use anyhow::Result;
use log::debug;
use maths::Point2i;
use std::{
    num::Wrapping,
    path::Path,
    sync::atomic::{AtomicU32, Ordering},
};

// TODO
pub fn resolve_filename(filename: &str) -> String {
    debug!("Resolving filename {}", filename);
    filename.to_owned()
}

const PCG32_DEFAULT_STATE: Wrapping<u64> = Wrapping(0x853c49e6748fea9b);
const PCG32_DEFAULT_STREAM: Wrapping<u64> = Wrapping(0xda3e39cb94b95bdb);
const PCG32_MULT: Wrapping<u64> = Wrapping(0x5851f42d4c957f2d);
pub const ONE_MINUS_EPSILON: f32 = 0.99999994f32;

#[derive(Copy, Clone)]
pub struct Rng {
    state: Wrapping<u64>,
    inc: Wrapping<u64>,
}

impl Rng {
    pub fn new() -> Self {
        Rng {
            state: PCG32_DEFAULT_STATE,
            inc: PCG32_DEFAULT_STREAM,
        }
    }

    pub fn uniform_u32(&mut self) -> u32 {
        let oldstate = self.state;
        self.state = oldstate * PCG32_MULT + self.inc;
        let xorshifted = Wrapping((((oldstate >> 18) ^ oldstate) >> 27).0 as u32);
        let rot = (oldstate >> 59).0 as u32;

        (xorshifted.0 >> rot) | (xorshifted.0 << ((!Wrapping(rot) + Wrapping(1)).0 & 31))
    }

    pub fn uniform_u32_bounded(&mut self, b: u32) -> u32 {
        let threshold = (!b + 1) & b;
        loop {
            let r = self.uniform_u32();
            if r >= threshold {
                return r % b;
            }
        }
    }

    pub fn uniform_f32(&mut self) -> f32 {
        ((self.uniform_u32() as f64 * 2.3283064365386963E-10) as f32).min(ONE_MINUS_EPSILON)
    }

    pub fn set_sequence(&mut self, seed: u64) {
        self.state = Wrapping(0);
        self.inc = Wrapping((seed << 1) | 1);
        let _ = self.uniform_u32();
        self.state += PCG32_DEFAULT_STATE;
        let _ = self.uniform_u32();
    }
}

impl Default for Rng {
    fn default() -> Rng {
        Rng::new()
    }
}

#[derive(Default)]
pub struct AtomicFloat {
    bits: AtomicU32,
}

impl AtomicFloat {
    pub fn as_float(&self) -> f32 {
        f32::from_bits(self.bits.load(Ordering::Relaxed))
    }

    pub fn add(&mut self, v: f32) {
        let f = self.as_float() + v;
        let bits = f.to_bits();
        self.bits.store(bits, Ordering::Relaxed)
    }
}

pub fn write_image<P: AsRef<Path>>(
    _name: P,
    _rgb: &[f32],
    _output_bounds: &Bounds2i,
    _total_resolution: Point2i,
) -> Result<()> {
    todo!()
}

pub fn has_extension<P: AsRef<Path>>(filename: P, extension: &str) -> bool {
    filename
        .as_ref()
        .extension()
        .map(|e| e == extension)
        .unwrap_or(false)
}

pub fn read_image<P: AsRef<Path>>(_path: P) -> Result<(Vec<Spectrum>, Point2i)> {
    todo!()
}
