use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

use nalgebra::Vector3;

#[derive(Copy, Clone)]
pub struct AABB {
    pub minimum: Vector3<f64>,
    pub maximum: Vector3<f64>,
}

impl AABB {
    pub fn new(minimum: Vector3<f64>, maximum: Vector3<f64>) -> AABB {
        AABB { minimum, maximum }
    }

    pub fn zeros() -> AABB {
        AABB::new(Vector3::zeros(), Vector3::zeros())
    }

    pub fn surrounding_box(a: &AABB, b: &AABB) -> AABB {
        let min = Vector3::new(
            f64::min(a.minimum.x, b.minimum.x),
            f64::min(a.minimum.y, b.minimum.y),
            f64::min(a.minimum.z, b.minimum.z),
        );

        let max = Vector3::new(
            f64::max(a.maximum.x, b.maximum.x),
            f64::max(a.maximum.y, b.maximum.y),
            f64::max(a.maximum.z, b.maximum.z),
        );

        AABB::new(min, max)
    }
}

impl Hittable for AABB {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, _hit: &mut HitRecord) -> bool {
        for axis in 0..3 {
            let inv_d = 1.0 / ray.direction[axis];

            let mut t0 = (self.minimum[axis] - ray.origin[axis]) * inv_d;
            let mut t1 = (self.maximum[axis] - ray.origin[axis]) * inv_d;

            if inv_d < 0.0 {
                (t0, t1) = (t1, t0);
            }

            let t_min = f64::max(t0, t_min);
            let t_max = f64::min(t1, t_max);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}
