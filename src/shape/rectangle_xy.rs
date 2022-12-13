use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;

use std::sync::Arc;

use nalgebra::Vector3;

pub struct RectangleXY {
    x: (f64, f64),
    y: (f64, f64),
    k: f64,
    material: Arc<dyn Material>,
}

impl RectangleXY {
    pub fn new(x: (f64, f64), y: (f64, f64), k: f64, material: Arc<dyn Material>) -> RectangleXY {
        RectangleXY { x, y, k, material }
    }
}

impl Hittable for RectangleXY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t > t_max {
            return false;
        }

        let point = ray.origin + t * ray.direction;
        if point.x < self.x.0 || point.x > self.x.1 || point.y < self.y.0 || point.y > self.y.1 {
            return false;
        }

        hit.t = t;
        hit.point = point;
        hit.normal = Vector3::new(0.0, 0.0, 1.0);
        hit.material = Arc::clone(&self.material);
        hit.uv.0 = (point.x - self.x.0) / (self.x.1 - self.x.0);
        hit.uv.1 = (point.y - self.y.0) / (self.y.1 - self.y.0);

        hit.set_face_normal(ray.direction, hit.normal);

        return true;
    }

    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Vector3::new(self.x.0, self.y.0, self.k - 0.0001),
            Vector3::new(self.x.1, self.y.1, self.k + 0.0001),
        );

        true
    }
}
