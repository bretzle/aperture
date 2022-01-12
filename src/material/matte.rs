use super::Material;
use crate::texture::Texture;
use std::sync::Arc;

pub struct Matte {
    _diffuse: Arc<dyn Texture + Send + Sync>,
    _roughness: Arc<dyn Texture + Send + Sync>,
}

impl Matte {
    pub fn new<D, R>(diffuse: &Arc<D>, roughness: &Arc<R>) -> Self
    where
        D: Texture + Send + Sync + 'static,
        R: Texture + Send + Sync + 'static,
    {
        Self {
            _diffuse: diffuse.clone(),
            _roughness: roughness.clone(),
        }
    }
}

impl Material for Matte {}
