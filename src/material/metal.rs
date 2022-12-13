use super::Material;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vector3_traits::Helpers;

use std::sync::Arc;

use nalgebra::Vector3;

#[derive(Clone)]
pub struct MetalMaterial {
    albedo: Arc<dyn Texture>,
    fuzz: f64,
}

impl MetalMaterial {
    pub fn arc(albedo: Arc<dyn Texture>, fuzz: f64) -> Arc<MetalMaterial> {
        Arc::new(MetalMaterial {
            albedo: albedo,
            fuzz: fuzz,
        })
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
        *attenuation = self.albedo.get_color(hit.uv, &hit.point);

        return true;
    }
}
