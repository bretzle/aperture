use super::Material;
use crate::texture::Texture;
use std::sync::Arc;

pub struct Matte<D, R> {
    _diffuse: Arc<D>,
    _roughness: Arc<R>,
}

impl<D, R> Matte<D, R>
where
    D: Texture + Send + Sync,
    R: Texture + Send + Sync,
{
    pub fn new(diffuse: &Arc<D>, roughness: &Arc<R>) -> Self {
        Self {
            _diffuse: diffuse.clone(),
            _roughness: roughness.clone(),
        }
    }
}

impl<D, R> Material for Matte<D, R> {}
