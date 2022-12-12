use super::Texture;

use nalgebra::Vector3;

pub struct CheckerTexture {
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(even: Box<dyn Texture>, odd: Box<dyn Texture>) -> CheckerTexture {
        CheckerTexture { even, odd }
    }
}

impl Texture for CheckerTexture {
    fn get_color(&self, uv: (f64, f64), point: &Vector3<f64>) -> Vector3<f64> {
        let sines = f64::sin(10.0 * point.x) * f64::sin(10.0 * point.y) * f64::sin(10.0 * point.z);
        if sines > 0.0 {
            return self.even.get_color(uv, point);
        } else {
            return self.odd.get_color(uv, point);
        }
    }
}
