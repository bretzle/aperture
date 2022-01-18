//! Provides a Gaussian reconstruction filter.

use std::f32;

use crate::film::filter::Filter;

use super::Filters;

/// A Gaussian reconstruction filter.
/// Recommended parameters to try: w = 2.0, h = 2.0, alpha = 2.0
#[derive(Copy, Clone, Debug)]
pub struct Gaussian {
    w: f32,
    h: f32,
    inv_w: f32,
    inv_h: f32,
    alpha: f32,
    exp_x: f32,
    exp_y: f32,
}

impl Gaussian {
    pub fn new(w: f32, h: f32, alpha: f32) -> Filters {
        Filters::Gaussian(Self {
            w,
            h,
            inv_w: 1.0 / w,
            inv_h: 1.0 / h,
            alpha,
            exp_x: f32::exp(-alpha * w * w),
            exp_y: f32::exp(-alpha * h * h),
        })
    }

    fn weight_1d(&self, x: f32, e: f32) -> f32 {
        f32::max(0.0, f32::exp(-self.alpha * x * x) - e)
    }
}

impl Filter for Gaussian {
    fn weight(&self, x: f32, y: f32) -> f32 {
        self.weight_1d(x, self.exp_x) * self.weight_1d(y, self.exp_y)
    }

    fn width(&self) -> f32 {
        self.w
    }

    fn inv_width(&self) -> f32 {
        self.inv_w
    }

    fn height(&self) -> f32 {
        self.h
    }

    fn inv_height(&self) -> f32 {
        self.inv_h
    }
}
