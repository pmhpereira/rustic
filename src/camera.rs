use crate::{ray::Ray, vector3_traits::Helpers};

use nalgebra::Vector3;

pub struct Camera {
    origin: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
    lens_radius: f64,
    u: Vector3<f64>,
    v: Vector3<f64>,
}

impl Camera {
    pub fn new(
        look_from: Vector3<f64>,
        look_at: Vector3<f64>,
        v_up: Vector3<f64>,
        fov_y: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Camera {
        let theta = fov_y.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = Vector3::cross(&v_up, &w).normalize();
        let v = Vector3::cross(&w, &u);

        // Camera
        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_distance * w;
        let lens_radius = aperture / 2.0;

        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
            lens_radius: lens_radius,
            u: u,
            v: v,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let random_direction = self.lens_radius * Vector3::new_random_in_unit_disc();
        let offset = self.u * random_direction.x + self.v * random_direction.y;

        let direction =
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;
        Ray::new(self.origin + offset, direction - offset)
    }
}
