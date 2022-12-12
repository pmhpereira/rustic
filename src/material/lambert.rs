use super::Material;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vector3_traits::Helpers;

use std::sync::Arc;

use nalgebra::Vector3;

#[derive(Clone)]
pub struct LambertianMaterial {
    albedo: Arc<dyn Texture>,
}

impl LambertianMaterial {
    pub fn new(albedo: Arc<dyn Texture>) -> LambertianMaterial {
        LambertianMaterial { albedo: albedo }
    }
}

impl Material for LambertianMaterial {
    fn scatter(
        &self,
        _ray: &Ray,
        hit: &HitRecord,
        attenuation: &mut Vector3<f64>,
        scattered: &mut Ray,
    ) -> bool {
        let scattered_direction = hit.normal + Vector3::new_random_in_unit_sphere().normalize();
        *scattered = Ray::new(hit.point, scattered_direction);
        *attenuation = self.albedo.get_color(hit.uv, &hit.point);

        return true;
    }
}
