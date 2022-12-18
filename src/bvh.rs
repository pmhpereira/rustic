use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

use std::cmp::Ordering;
use std::sync::Arc;

use nalgebra::Vector3;
use rand::Rng;

pub struct BVH {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    aabb: AABB,
}

impl BVH {
    pub fn arc(objects: &mut Vec<Arc<dyn Hittable>>) -> Arc<BVH> {
        let mut rng = rand::thread_rng();
        let axis = rng.gen_range(0..3);

        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;

        match objects.len() {
            1 => {
                left = Arc::clone(&objects[0]);
                right = Arc::clone(&objects[0]);
            }
            _ => {
                objects.sort_by(|a, b| Self::box_compare(a, b, axis));

                let mid = objects.len() / 2;
                left = BVH::arc(&mut objects[..mid].to_vec());
                right = BVH::arc(&mut objects[mid..].to_vec());
            }
        }

        let mut box_left = AABB::zeros();
        let mut box_right = AABB::zeros();

        left.bounding_box(&mut box_left);
        right.bounding_box(&mut box_right);

        Arc::new(BVH {
            left: left,
            right: right,
            aabb: AABB::surrounding_box(&box_left, &box_right),
        })
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
        let mut box_a = AABB::zeros();
        let mut box_b = AABB::zeros();

        a.bounding_box(&mut box_a);
        b.bounding_box(&mut box_b);

        box_a.minimum[axis]
            .partial_cmp(&box_b.minimum[axis])
            .unwrap()
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit: &mut HitRecord) -> bool {
        if false == self.aabb.hit(ray, t_min, t_max, hit) {
            return false;
        }

        let hit_left = self.left.hit(ray, t_min, t_max, hit);
        let hit_right = self.right.hit(ray, t_min, hit.t, hit);

        hit_left || hit_right
    }

    fn bounding_box(&self, output_box: &mut AABB) -> bool {
        *output_box = self.aabb;
        true
    }
}
