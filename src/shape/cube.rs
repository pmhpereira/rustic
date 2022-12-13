use super::RectangleXY;
use super::RectangleXZ;
use super::RectangleYZ;

use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::hittable::HittableList;
use crate::material::Material;
use crate::ray::Ray;

use std::sync::Arc;

use nalgebra::Vector3;

pub struct Cube {
    minimum: Vector3<f64>,
    maximum: Vector3<f64>,
    sides: HittableList,
    material: Arc<dyn Material>,
}

impl Cube {
    pub fn arc(
        minimum: Vector3<f64>,
        maximum: Vector3<f64>,
        material: Arc<dyn Material>,
    ) -> Arc<Cube> {
        let mut sides = HittableList::new();

        sides.add(RectangleXY::arc(
            (minimum.x, maximum.x),
            (minimum.y, maximum.y),
            maximum.z,
            Arc::clone(&material),
        ));
        sides.add(RectangleXY::arc(
            (minimum.x, maximum.x),
            (minimum.y, maximum.y),
            minimum.z,
            Arc::clone(&material),
        ));

        sides.add(RectangleXZ::arc(
            (minimum.x, maximum.x),
            (minimum.z, maximum.z),
            maximum.y,
            Arc::clone(&material),
        ));
        sides.add(RectangleXZ::arc(
            (minimum.x, maximum.x),
            (minimum.z, maximum.z),
            minimum.y,
            Arc::clone(&material),
        ));

        sides.add(RectangleYZ::arc(
            (minimum.y, maximum.y),
            (minimum.z, maximum.z),
            maximum.x,
            Arc::clone(&material),
        ));
        sides.add(RectangleYZ::arc(
            (minimum.y, maximum.y),
            (minimum.z, maximum.z),
            minimum.x,
            Arc::clone(&material),
        ));

        Arc::new(Cube {
            minimum,
            maximum,
            sides,
            material,
        })
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit: &mut HitRecord) -> bool {
        self.sides.hit(ray, t_min, t_max, hit)
    }

    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(self.minimum, self.maximum);
        true
    }
}
