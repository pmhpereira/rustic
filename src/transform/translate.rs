use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;

use std::sync::Arc;

use nalgebra::Vector3;

pub struct Translate {
    offset: Vector3<f64>,
    hittable: Arc<dyn Hittable>,
}

impl Translate {
    pub fn arc(offset: Vector3<f64>, hittable: Arc<dyn Hittable>) -> Arc<Translate> {
        Arc::new(Translate { offset, hittable })
    }
}
impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit: &mut HitRecord) -> bool {
        let ray_offset = Ray::new(ray.origin - self.offset, ray.direction);

        if self.hittable.hit(&ray_offset, t_min, t_max, hit) == false {
            return false;
        }

        hit.point += self.offset;
        hit.set_face_normal(ray_offset.direction, hit.normal);

        true
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        if self.hittable.bounding_box(t0, t1, output_box) {
            return false;
        }

        *output_box = AABB::new(
            output_box.minimum + self.offset,
            output_box.maximum + self.offset,
        );

        true
    }
}
