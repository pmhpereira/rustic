mod dielectric;
mod lambert;
mod metal;

pub use dielectric::DielectricMaterial;
pub use lambert::LambertianMaterial;
pub use metal::MetalMaterial;

use crate::hittable::HitRecord;
use crate::ray::Ray;

pub use dyn_clone::DynClone;
pub use nalgebra::Vector3;
pub use rand::Rng;

pub trait Material: DynClone + Sync + Send {
    fn scatter(
        &self,
        _ray: &Ray,
        _hit: &HitRecord,
        _attenuation: &mut Vector3<f64>,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }

    }
}
