use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vector3_traits::Helpers;

use std::sync::Arc;

use nalgebra::Vector3;

pub struct RotateY {
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: AABB,
    hittable: Arc<dyn Hittable>,
}

impl RotateY {
    pub fn arc(degrees: f64, hittable: Arc<dyn Hittable>) -> Arc<RotateY> {
        let radians = degrees.to_radians();

        let mut bounding_box = AABB::zeros();
        hittable.bounding_box(0.0, 1.0, &mut bounding_box);

        let mut min = Vector3::infinity();
        let mut max = -Vector3::infinity();

        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        for i in 0..=1 {
            for j in 0..=1 {
                for k in 0..=1 {
                    let x = i as f64 * bounding_box.maximum.x
                        + (1.0 - i as f64) * bounding_box.minimum.x;
                    let y = j as f64 * bounding_box.maximum.y
                        + (1.0 - j as f64) * bounding_box.minimum.y;
                    let z = k as f64 * bounding_box.maximum.z
                        + (1.0 - k as f64) * bounding_box.minimum.z;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vector3::new(new_x, y, new_z);
                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }

        let bounding_box = AABB::new(min, max);

        Arc::new(RotateY {
            sin_theta,
            cos_theta,
            hittable,
            bounding_box,
        })
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit: &mut HitRecord) -> bool {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin[0] = self.cos_theta * ray.origin.x - self.sin_theta * ray.origin.z;
        origin[2] = self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.z;

        direction[0] = self.cos_theta * ray.direction.x - self.sin_theta * ray.direction.z;
        direction[2] = self.sin_theta * ray.direction.x + self.cos_theta * ray.direction.z;

        let ray_rotated = Ray::new(origin, direction);
        if self.hittable.hit(&ray_rotated, t_min, t_max, hit) == false {
            return false;
        }

        let mut point = hit.point;
        point[0] = self.cos_theta * hit.point.x + self.sin_theta * hit.point.z;
        point[2] = -self.sin_theta * hit.point.x + self.cos_theta * hit.point.z;

        let mut normal = hit.normal;
        normal[0] = self.cos_theta * hit.normal.x + self.sin_theta * hit.normal.z;
        normal[2] = -self.sin_theta * hit.normal.x + self.cos_theta * hit.normal.z;

        hit.point = point;
        hit.normal = normal;
        hit.set_face_normal(ray_rotated.direction, hit.normal);

        true
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        if self.hittable.bounding_box(t0, t1, output_box) {
            return false;
        }

        *output_box = self.bounding_box;

        true
    }
}
