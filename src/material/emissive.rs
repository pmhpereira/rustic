use super::Material;

use crate::texture::Texture;

use std::sync::Arc;

use nalgebra::Vector3;

#[derive(Clone)]
pub struct EmissiveMaterial {
    emission: Arc<dyn Texture>,
    scale: f64,
}

impl EmissiveMaterial {
    pub fn arc(emission: Arc<dyn Texture>) -> Arc<EmissiveMaterial> {
        Arc::new(EmissiveMaterial {
            emission: emission,
            scale: 1.0,
        })
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
    }
}

impl Material for EmissiveMaterial {
    fn emitted(&self, uv: (f64, f64), point: &Vector3<f64>) -> Vector3<f64> {
        self.emission.get_color(uv, point) * self.scale
    }
}
