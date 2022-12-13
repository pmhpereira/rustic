use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;

use nalgebra::Vector3;

pub struct RectangleXZ {
    x: (f64, f64),
    z: (f64, f64),
    k: f64,
    material: Box<dyn Material>,
}

impl RectangleXZ {
    pub fn new(x: (f64, f64), z: (f64, f64), k: f64, material: Box<dyn Material>) -> RectangleXZ {
        RectangleXZ { x, z, k, material }
    }
}

impl Hittable for RectangleXZ {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit: &mut HitRecord) -> bool {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t > t_max {
            return false;
        }

        let point = ray.origin + t * ray.direction;
        if point.x < self.x.0 || point.x > self.x.1 || point.z < self.z.0 || point.z > self.z.1 {
            return false;
        }

        hit.t = t;
        hit.point = point;
        hit.normal = Vector3::new(0.0, 1.0, 0.0);
        hit.material = dyn_clone::clone_box(&*self.material);
        hit.uv.0 = (point.x - self.x.0) / (self.x.1 - self.x.0);
        hit.uv.1 = (point.z - self.z.0) / (self.z.1 - self.z.0);

        hit.set_face_normal(ray.direction, hit.normal);

        return true;
    }

    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Vector3::new(self.x.0, self.k - 0.0001, self.z.0),
            Vector3::new(self.x.1, self.k + 0.0001, self.z.1),
        );

        true
    }
}
