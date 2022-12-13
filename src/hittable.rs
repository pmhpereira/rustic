use crate::aabb::AABB;
use crate::material::*;
use crate::ray::Ray;
use crate::texture::SolidColorTexture;

use std::sync::Arc;

use nalgebra::Vector3;

pub struct HitRecord {
    pub point: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
    pub uv: (f64, f64),
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            point: Vector3::zeros(),
            normal: Vector3::zeros(),
            t: f64::MAX,
            front_face: false,
            material: LambertianMaterial::arc(SolidColorTexture::arc(Vector3::new(1.0, 1.0, 1.0))),
            uv: (0.0, 0.0),
        }
    }

    pub fn set_face_normal(&mut self, direction: Vector3<f64>, normal: Vector3<f64>) {
        if Vector3::dot(&direction, &normal) > 0.0 {
            self.normal = -normal;
            self.front_face = false;
        } else {
            self.front_face = true;
        }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit: &mut HitRecord) -> bool;
    fn bounding_box(&self, _t0: f64, _t1: f64, _output_box: &mut AABB) -> bool {
        true
    }
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, boxed_object: Arc<dyn Hittable>) {
        self.objects.push(boxed_object);
    }
}

impl Hittable for HittableList {
    fn hit<'a>(&self, ray: &Ray, t_min: f64, t_max: f64, hit: &mut HitRecord) -> bool {
        let mut hit_anything = false;

        let mut closest_so_far = t_max;

        for object in &self.objects {
            let mut temp_hit = HitRecord::new();
            if object.hit(ray, t_min, closest_so_far, &mut temp_hit) {
                hit_anything = true;
                closest_so_far = temp_hit.t;
                *hit = temp_hit;
            }
        }

        return hit_anything;
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut temp_box = AABB::zeros();
        let mut first_box = true;

        for object in &self.objects {
            if false == object.bounding_box(t0, t1, &mut temp_box) {
                return false;
            }

            if true == first_box {
                *output_box = temp_box;
            } else {
                *output_box = AABB::surrounding_box(output_box, &temp_box);
            }

            first_box = false;
        }

        true
    }
}
