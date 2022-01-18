//! The film module provides color types and a render target that the image
//! is written too.

pub use self::{
    animated_color::{AnimatedColor, ColorKeyframe},
    camera::Camera,
    color::Colorf,
    image::Image,
    render_target::{ImageSample, RenderTarget},
};

pub mod animated_color;
pub mod camera;
pub mod color;
pub mod filter;
pub mod image;
pub mod render_target;

/// Struct to store various parameters for the frame timing
#[derive(Debug, Copy, Clone)]
pub struct FrameInfo {
    /// Total number of frames for the scene
    pub frames: usize,
    /// Total time for all the entire scene (ie. for all frames)
    pub time: f32,
    /// Frame number to start rendering at
    pub start: usize,
    /// Frame number to stop rendering at
    pub end: usize,
}

impl FrameInfo {
    pub fn new(frames: usize, time: f32, start: usize, end: usize) -> Self {
        Self {
            frames,
            time,
            start,
            end,
        }
    }
}
