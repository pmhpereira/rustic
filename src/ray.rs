use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn ray_color(ray: &Ray, world: &impl Hittable, depth: u64) -> Color {
        let mut hit = HitRecord::new();

        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if world.hit(ray, 0.0001, f64::MAX, &mut hit) == true {
            let mut scattered = Ray::new(Vec3::ZERO, Vec3::ZERO);
            let mut attenuation = Color::new(0.0, 0.0, 0.0);

            if hit
                .material
                .as_ref()
                .scatter(&ray, &hit, &mut attenuation, &mut scattered)
            {
                return attenuation * Self::ray_color(&scattered, world, depth - 1);
            }

            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = ray.direction.normalized();
        let t = 0.5 * (unit_direction.y + 1.0);
        Color::lerp(t, Color::new(1.0, 1.0, 1.0), Color::new(0.5, 0.7, 1.0))
    }
}
