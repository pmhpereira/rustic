mod dielectric;
mod emissive;
mod lambert;
mod metal;

pub use dielectric::DielectricMaterial;
pub use emissive::EmissiveMaterial;
pub use lambert::LambertianMaterial;
pub use metal::MetalMaterial;

use crate::hittable::HitRecord;
use crate::ray::Ray;

pub use nalgebra::Vector3;
pub use rand::Rng;

pub trait Material: Sync + Send {
    fn scatter(
        &self,
        _ray: &Ray,
        _hit: &HitRecord,
        _attenuation: &mut Vector3<f64>,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, _uv: (f64, f64), _point: &Vector3<f64>) -> Vector3<f64> {
        Vector3::zeros()
    }
}
