use crate::{
    film::Colorf,
    linalg::lerp,
    texture::{Image, Texture, Textures},
};

/// An `AnimatedImage` texture is a `Texture` whose samples come
/// from a series of `Image`s which are played through over time.
pub struct AnimatedImage {
    // I wonder how much sense it would make, and it what it
    // would look like to do a B-spline interpolation between the
    // images
    frames: Vec<(f32, Image)>,
}

impl AnimatedImage {
    pub fn new_texture(frames: Vec<(f32, Textures)>) -> Textures {
        assert!(frames.len() >= 2);
        let frames = frames
            .into_iter()
            .map(|(x, t)| {
                if let Textures::Image(img) = t {
                    (x, img)
                } else {
                    panic!();
                }
            })
            .collect::<Vec<_>>();
        Textures::AnimatedImage(Self { frames })
    }

    pub fn active_keyframes(&self, time: f32) -> (usize, Option<usize>) {
        match self
            .frames
            .binary_search_by(|&(t, _)| t.partial_cmp(&time).unwrap())
        {
            Ok(i) => (i, None),
            Err(i) => {
                if i == self.frames.len() {
                    (i - 1, None)
                } else if i == 0 {
                    (0, None)
                } else {
                    (i - 1, Some(i))
                }
            }
        }
    }
}

impl Texture for AnimatedImage {
    fn sample_f32(&self, u: f32, v: f32, time: f32) -> f32 {
        match self.active_keyframes(time) {
            (lo, None) => self.frames[lo].1.sample_f32(u, v, time),
            (lo, Some(hi)) => {
                let x = (time - self.frames[lo].0) / (self.frames[hi].0 - self.frames[lo].0);
                lerp(
                    x,
                    &self.frames[lo].1.sample_f32(u, v, time),
                    &self.frames[hi].1.sample_f32(u, v, time),
                )
            }
        }
    }

    fn sample_color(&self, u: f32, v: f32, time: f32) -> Colorf {
        match self.active_keyframes(time) {
            (lo, None) => self.frames[lo].1.sample_color(u, v, time),
            (lo, Some(hi)) => {
                let x = (time - self.frames[lo].0) / (self.frames[hi].0 - self.frames[lo].0);
                lerp(
                    x,
                    &self.frames[lo].1.sample_color(u, v, time),
                    &self.frames[hi].1.sample_color(u, v, time),
                )
            }
        }
    }
}
