use super::Texture;

use std::sync::Arc;

use nalgebra::Vector3;

pub struct SolidColorTexture {
    color: Vector3<f64>,
}

impl SolidColorTexture {
    pub fn arc(color: Vector3<f64>) -> Arc<SolidColorTexture> {
        Arc::new(SolidColorTexture { color })
    }
}

impl Texture for SolidColorTexture {
    fn get_color(&self, (_u, _v): (f64, f64), _point: &Vector3<f64>) -> Vector3<f64> {
        self.color
    }
}
