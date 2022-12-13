use crate::hittable::{HitRecord, Hittable};

use std::sync::Arc;

use nalgebra::Vector3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Ray {
        Ray { origin, direction }
    }

    pub fn zeros() -> Ray {
        Ray::new(Vector3::zeros(), Vector3::zeros())
    }

    pub fn at(self, t: f64) -> Vector3<f64> {
        self.origin + t * self.direction
    }

    pub fn ray_color(
        ray: &Ray,
        world: &Arc<dyn Hittable>,
        background_color: &Vector3<f64>,
        depth: u64,
    ) -> Vector3<f64> {
        let mut hit = HitRecord::new();

        if depth == 0 {
            return Vector3::zeros();
        }

        if world.hit(ray, 0.0001, f64::MAX, &mut hit) == false {
            return *background_color;
        }

        let mut scattered = Ray::zeros();
        let mut attenuation = Vector3::zeros();
        let emitted = hit.material.emitted(hit.uv, &hit.point);

        if hit
            .material
            .scatter(&ray, &hit, &mut attenuation, &mut scattered)
            == false
        {
            return emitted;
        }

        return emitted
            + attenuation.component_mul(&Self::ray_color(
                &scattered,
                world,
                background_color,
                depth - 1,
            ));
    }
}
