use super::Material;

use crate::texture::Texture;

use std::sync::Arc;

use nalgebra::Vector3;

#[derive(Clone)]
pub struct DiffuseLight {
    emission: Arc<dyn Texture>,
    scale: f64,
}

impl DiffuseLight {
    pub fn new(emission: Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight {
            emission: emission,
            scale: 1.0,
        }
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, uv: (f64, f64), point: &Vector3<f64>) -> Vector3<f64> {
        self.emission.get_color(uv, point) * self.scale
    }
}
