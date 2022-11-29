use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    lens_radius: f64,
    u : Vec3,
    v : Vec3,
    w : Vec3,
}

impl Camera {
    pub fn new(look_from : Vec3, look_at : Vec3, v_up : Vec3, fov_y: f64, aspect_ratio: f64, aperture: f64, focus_distance: f64) -> Camera {
        let theta = fov_y.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalized();
        let u = Vec3::cross(v_up, w).normalized();
        let v = Vec3::cross(w, u);

        // Camera
        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - focus_distance * w;
        let lens_radius = aperture / 2.0;

        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
            lens_radius: lens_radius,
            u: u,
            v: v,
            w: w,
        }
    }

    pub fn get_ray(&self, u : f64, v: f64) -> Ray {
        let random_direction = self.lens_radius * Vec3::random_in_unit_disc();
        let offset = self.u * random_direction.x + self.v * random_direction.y;

        let direction = self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin;
        Ray::new(self.origin + offset, direction - offset)
    }
}