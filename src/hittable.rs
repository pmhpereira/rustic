use crate::color::Color;
use crate::material::{LambertianMaterial, Material};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Box<dyn Material>,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: f64::MAX,
            front_face: false,
            material: Box::new(LambertianMaterial::new(Color::new(1.0, 1.0, 1.0))),
        }
    }
}

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit: &mut HitRecord) -> bool;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, boxed_object: Box<dyn Hittable>) {
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
}
