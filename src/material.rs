use crate::texture::Texture;
use std::sync::Arc;

pub trait Material {}

pub struct Matte<D, R> {
    diffuse: Arc<D>,
    roughness: Arc<R>,
}

impl<D, R> Matte<D, R>
where
    D: Texture + Send + Sync,
    R: Texture + Send + Sync,
{
    pub fn new(diffuse: Arc<D>, roughness: Arc<R>) -> Self {
        Self {
            diffuse: diffuse.clone(),
            roughness: roughness.clone(),
        }
    }
}

impl<D, R> Material for Matte<D, R> {}
