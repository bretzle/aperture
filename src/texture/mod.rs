use std::{collections::HashMap, fmt::Debug, sync::Arc};

use log::error;

use crate::{
    geometry::Vector3f, interaction::SurfaceInteraction, paramset::ParamSet, spectrum::Spectrum,
    transform::Transform,
};

pub type TextureFloat = dyn Texture<f32>;

pub trait Texture<T>: Debug + Send + Sync {
    fn evaluate(&self, si: &SurfaceInteraction) -> T;
}

#[derive(Debug)]
pub struct ConstantTexture<T> {
    value: T,
}

impl<T: Copy> ConstantTexture<T> {
    pub fn new(value: T) -> ConstantTexture<T> {
        ConstantTexture { value }
    }
}

impl ConstantTexture<f32> {
    pub fn create_float(_tex2world: &Transform, tp: &TextureParams<'_>) -> ConstantTexture<f32> {
        ConstantTexture::new(tp.find_float("value", 1.0))
    }
}

impl ConstantTexture<Spectrum> {
    pub fn create_spectrum(
        _tex2world: &Transform,
        tp: &TextureParams<'_>,
    ) -> ConstantTexture<Spectrum> {
        ConstantTexture::new(tp.find_spectrum("value", Spectrum::white()))
    }
}

impl<T: Copy + Debug + Send + Sync> Texture<T> for ConstantTexture<T> {
    fn evaluate(&self, _si: &SurfaceInteraction<'_, '_>) -> T {
        self.value
    }
}

pub struct TextureParams<'a> {
    geom_params: &'a ParamSet,
    material_params: &'a ParamSet,
    float_textures: &'a HashMap<String, Arc<dyn Texture<f32>>>,
    spectrum_textures: &'a HashMap<String, Arc<dyn Texture<Spectrum>>>,
}

impl<'a> TextureParams<'a> {
    pub fn new(
        gp: &'a ParamSet,
        mp: &'a ParamSet,
        ft: &'a HashMap<String, Arc<dyn Texture<f32>>>,
        st: &'a HashMap<String, Arc<dyn Texture<Spectrum>>>,
    ) -> TextureParams<'a> {
        TextureParams {
            geom_params: gp,
            material_params: mp,
            float_textures: ft,
            spectrum_textures: st,
        }
    }

    pub fn find_int(&self, n: &str, d: i32) -> i32 {
        let d = self.material_params.find_one_int(n, d);
        self.geom_params.find_one_int(n, d)
    }

    pub fn find_string(&self, n: &str, d: &str) -> String {
        let mat_string = self.material_params.find_one_string(n, d.to_owned());
        self.geom_params.find_one_string(n, mat_string)
    }

    pub fn find_filename(&self, n: &str, d: &str) -> String {
        let mat_string = self.material_params.find_one_filename(n, d.to_owned());
        self.geom_params.find_one_filename(n, mat_string)
    }

    pub fn find_bool(&self, n: &str, d: bool) -> bool {
        let d = self.material_params.find_one_bool(n, d);
        self.geom_params.find_one_bool(n, d)
    }

    pub fn find_float(&self, n: &str, d: f32) -> f32 {
        let d = self.material_params.find_one_float(n, d);
        self.geom_params.find_one_float(n, d)
    }

    pub fn find_vector3f(&self, n: &str, d: Vector3f) -> Vector3f {
        let d = self.material_params.find_one_vector3f(n, d);
        self.geom_params.find_one_vector3f(n, d)
    }

    pub fn find_spectrum(&self, n: &str, d: Spectrum) -> Spectrum {
        let d = self.material_params.find_one_spectrum(n, d);
        self.geom_params.find_one_spectrum(n, d)
    }

    pub fn get_spectrum_texture(&self, n: &str, default: &Spectrum) -> Arc<dyn Texture<Spectrum>> {
        let mut name = self.geom_params.find_texture(n, "".to_owned());
        if &name == "" {
            name = self.material_params.find_texture(n, "".to_owned());
        }
        if &name != "" {
            if let Some(tex) = self.spectrum_textures.get(&name) {
                return Arc::clone(tex);
            } else {
                error!(
                    "Couldn't find spectrum texture {} for parameter {}",
                    name, n
                );
            }
        }
        // If texture wasn't found
        let val = self.material_params.find_one_spectrum(n, *default);
        let val = self.geom_params.find_one_spectrum(n, val);
        Arc::new(ConstantTexture::new(val))
    }

    pub fn get_float_texture(&self, n: &str, default: f32) -> Arc<dyn Texture<f32>> {
        let mut name = self.geom_params.find_texture(n, "".to_owned());
        if &name == "" {
            name = self.material_params.find_texture(n, "".to_owned());
        }
        if &name != "" {
            if let Some(tex) = self.float_textures.get(&name) {
                return Arc::clone(tex);
            } else {
                error!("Couldn't find float texture {} for parameter {}", name, n);
            }
        }
        // If texture wasn't found
        let val = self.material_params.find_one_float(n, default);
        let val = self.geom_params.find_one_float(n, val);
        Arc::new(ConstantTexture::new(val))
    }

    pub fn get_float_texture_or_none(&self, n: &str) -> Option<Arc<dyn Texture<f32>>> {
        let mut name = self.geom_params.find_texture(n, "".to_owned());
        if &name == "" {
            name = self.material_params.find_texture(n, "".to_owned());
        }
        if &name != "" {
            if let Some(tex) = self.float_textures.get(&name) {
                return Some(Arc::clone(tex));
            } else {
                error!("Couldn't find float texture {} for parameter {}", name, n);
                return None;
            }
        }
        // If texture wasn't found
        self.geom_params
            .find_float(n)
            .or_else(|| self.material_params.find_float(n))
            .map(|val| {
                let tex: Arc<dyn Texture<f32>> = Arc::new(ConstantTexture::new(val[0]));
                tex
            })
    }
}
