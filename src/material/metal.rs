use super::Material;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vector3_traits::Helpers;

use nalgebra::Vector3;

#[derive(Clone)]
pub struct MetalMaterial {
    albedo: Vector3<f64>,
    fuzz: f64,
}

impl MetalMaterial {
    pub fn new(albedo: Vector3<f64>, fuzz: f64) -> MetalMaterial {
        MetalMaterial {
            albedo: albedo,
            fuzz: fuzz,
        }
    }
}

impl Material for MetalMaterial {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &HitRecord,
        attenuation: &mut Vector3<f64>,
        scattered: &mut Ray,
    ) -> bool {
        let reflected_direction = Vector3::reflection(ray.direction.normalize(), hit.normal);
        *scattered = Ray::new(
            hit.point,
            reflected_direction + self.fuzz * Vector3::new_random_in_unit_sphere(),
        );
        *attenuation = self.albedo;

        return true;
    }
}
