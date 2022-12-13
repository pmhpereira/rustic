use super::Material;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vector3_traits::Helpers;

use std::sync::Arc;

use nalgebra::Vector3;
use rand::Rng;

#[derive(Clone)]
pub struct DielectricMaterial {
    ir: f64,
}

impl DielectricMaterial {
    pub fn arc(ir: f64) -> Arc<DielectricMaterial> {
        Arc::new(DielectricMaterial { ir: ir })
    }

    fn reflectance(cosine: f64, ref_index: f64) -> f64 {
        let r0 = (1.0 - ref_index) / (1.0 + ref_index);
        let r0 = r0 * r0;

        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
    }
}

impl Material for DielectricMaterial {
    fn scatter(
        &self,
        ray: &Ray,
        hit: &HitRecord,
        attenuation: &mut Vector3<f64>,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vector3::new(1.0, 1.0, 1.0);

        let refraction_ratio = if hit.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let direction_normalized = ray.direction.normalize();

        let cos_theta = f64::min(Vector3::dot(&-direction_normalized, &hit.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;

        let direction: Vector3<f64>;
        if cannot_refract == true
            || DielectricMaterial::reflectance(cos_theta, refraction_ratio)
                > rand::thread_rng().gen()
        {
            direction = Vector3::reflection(direction_normalized, hit.normal);
        } else {
            direction = Vector3::refraction(direction_normalized, hit.normal, refraction_ratio);
        }

        *scattered = Ray::new(hit.point, direction);

        return true;
    }
}
