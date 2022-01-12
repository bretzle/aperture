use crate::texture::Texture;
use std::sync::Arc;

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
