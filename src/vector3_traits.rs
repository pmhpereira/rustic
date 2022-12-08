use nalgebra::Vector3;
use rand::Rng;

pub trait Helpers {
    fn gamma(self, gamma: f64) -> Vector3<f64>;
    fn reflection(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64>;
    fn refraction(uv: Vector3<f64>, n: Vector3<f64>, etai_over_etat: f64) -> Vector3<f64>;

    fn new_random_in_range(start: f64, end: f64) -> Vector3<f64>;
    fn new_random_in_unit_sphere() -> Vector3<f64>;
    fn new_random_in_unit_disc() -> Vector3<f64>;
}

impl Helpers for Vector3<f64> {
    fn gamma(self, gamma: f64) -> Self {
        let gamma_r = self.x.powf(1.0 / gamma);
        let gamma_g = self.y.powf(1.0 / gamma);
        let gamma_b = self.z.powf(1.0 / gamma);

        Vector3::new(gamma_r, gamma_g, gamma_b)
    }

    fn reflection(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
        v - 2.0 * Vector3::dot(&v, &n) * n
    }

    fn refraction(uv: Vector3<f64>, n: Vector3<f64>, etai_over_etat: f64) -> Vector3<f64> {
        let cos_theta = f64::min(Vector3::dot(&-uv, &n), 1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.magnitude_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }

    fn new_random_in_range(start: f64, end: f64) -> Vector3<f64> {
        let mut rng = rand::thread_rng();

        Vector3::new(
            rng.gen_range(start..=end),
            rng.gen_range(start..=end),
            rng.gen_range(start..=end),
        )
    }

    fn new_random_in_unit_sphere() -> Vector3<f64> {
        loop {
            let vec = Vector3::new_random_in_range(-1.0, 1.0);
            let len = vec.magnitude_squared();
            if len <= 1.0 {
                return vec;
            }
        }
    }

    fn new_random_in_unit_disc() -> Vector3<f64> {
        let mut vec = Vector3::new_random_in_range(-1.0, 1.0);
        vec.z = 0.0;

        vec
    }
}
