use crate::hittable::{HitRecord, Hittable};

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

    pub fn at(self, t: f64) -> Vector3<f64> {
        self.origin + t * self.direction
    }

    pub fn ray_color(ray: &Ray, world: &impl Hittable, depth: u64) -> Vector3<f64> {
        let mut hit = HitRecord::new();

        if depth == 0 {
            return Vector3::zeros();
        }

        if world.hit(ray, 0.0001, f64::MAX, &mut hit) == true {
            let mut scattered = Ray::new(Vector3::zeros(), Vector3::zeros());
            let mut attenuation = Vector3::zeros();

            if hit
                .material
                .as_ref()
                .scatter(&ray, &hit, &mut attenuation, &mut scattered)
            {
                return attenuation.component_mul(&Self::ray_color(&scattered, world, depth - 1));
            }

            return Vector3::zeros();
        }

        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vector3::lerp(
            &Vector3::new(1.0, 1.0, 1.0),
            &Vector3::new(0.5, 0.7, 1.0),
            t,
        )
    }
}
