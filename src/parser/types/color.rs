#[derive(Debug, Clone)]
pub struct Blackbody {
    pub temperature: f32,
    pub scale: f32,
}

#[derive(Debug, Clone)]
pub struct RGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl RGB {
    pub fn color(v: f32) -> RGB {
        RGB { r: v, g: v, b: v }
    }
}

/// PBRT spectrum type
#[derive(Debug, Clone)]
pub enum Spectrum {
    RGB(RGB),
    Blackbody(Blackbody),
    Texture(String),
    Spectrum(String),
    Mapname(String),
}
