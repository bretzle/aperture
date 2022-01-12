use crate::math;

pub trait Filter {
    fn wieght(&self, x: f32, y: f32) -> f32;
    fn width(&self) -> f32;
    fn inv_width(&self) -> f32;
    fn height(&self) -> f32;
    fn inv_height(&self) -> f32;
}

pub struct MitchellNetravali {
    w: f32,
    h: f32,
    inv_w: f32,
    inv_h: f32,
    b: f32,
    c: f32,
}

impl MitchellNetravali {
    pub fn new(w: f32, h: f32, b: f32, c: f32) -> Self {
        if b < 0.0 || b > 1.0 {
            warn!(
                "Mitchell-Netravali b param = {} is out of bounds, clamping in range",
                b
            );
        }
        if c < 0.0 || c > 1.0 {
            warn!(
                "Mitchell-Netravali c param = {} is out of bounds, clamping in range",
                c
            );
        }

        Self {
            w,
            h,
            inv_w: 1.0 / w,
            inv_h: 1.0 / h,
            b: math::clamp(b, 0.0, 1.0),
            c: math::clamp(c, 0.0, 1.0),
        }
    }
}

impl Filter for MitchellNetravali {
    fn wieght(&self, x: f32, y: f32) -> f32 {
        todo!()
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
