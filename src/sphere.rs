use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;

        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = discriminant.sqrt();

        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        hit.t = root;
        hit.point = ray.at(root);
        hit.normal = (hit.point - self.center) / self.radius;
        hit.material = dyn_clone::clone_box(&*self.material);

        if Vec3::dot(ray.direction, hit.normal) > 0.0 {
            hit.normal = -hit.normal;
            hit.front_face = false;
        } else {
            hit.front_face = true;
        }

        return true;
    }
}
