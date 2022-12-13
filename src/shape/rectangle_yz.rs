use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;

use std::sync::Arc;

use nalgebra::Vector3;

pub struct RectangleYZ {
    y: (f64, f64),
    z: (f64, f64),
    k: f64,
    material: Arc<dyn Material>,
}

impl RectangleYZ {
    pub fn arc(
        y: (f64, f64),
        z: (f64, f64),
        k: f64,
        material: Arc<dyn Material>,
    ) -> Arc<RectangleYZ> {
        Arc::new(RectangleYZ { y, z, k, material })
    }
}

impl Hittable for RectangleYZ {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if t < t_min || t > t_max {
            return false;
        }

        let point = ray.origin + t * ray.direction;
        if point.y < self.y.0 || point.y > self.y.1 || point.z < self.z.0 || point.z > self.z.1 {
            return false;
        }

        hit.t = t;
        hit.point = point;
        hit.normal = Vector3::new(1.0, 0.0, 0.0);
        hit.material = Arc::clone(&self.material);
        hit.uv.0 = (point.y - self.y.0) / (self.y.1 - self.y.0);
        hit.uv.1 = (point.z - self.z.0) / (self.z.1 - self.z.0);

        hit.set_face_normal(ray.direction, hit.normal);

        return true;
    }

    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Vector3::new(self.k - 0.0001, self.y.0, self.z.0),
            Vector3::new(self.k + 0.0001, self.y.1, self.z.1),
        );

        true
    }
}
