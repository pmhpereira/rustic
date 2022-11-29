use std::ops::{Add, Sub, Mul, Div, Neg};

use rand::Rng;

#[derive(Copy, Clone)]
pub struct Vec3
{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z}
    }

    pub fn dot(u:  Vec3, v: Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u:  Vec3, v: Vec3) -> Vec3 {
        Vec3::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x,
        )
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        let mut length_squared = 0.0;

        length_squared += self.x * self.x;
        length_squared += self.y * self.y;
        length_squared += self.z * self.z;

        length_squared
    }

    pub fn normalized(self) -> Vec3 {
        let length = self.length();
        return self / length
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let x = rand::thread_rng().gen_range(-1.0..=1.0);
            let y = rand::thread_rng().gen_range(-1.0..=1.0);
            let z = rand::thread_rng().gen_range(-1.0..=1.0);

            let vec = Vec3::new(x, y, z);
            let len = vec.length_squared();
            if len <= 1.0 {
                return vec
            }
        }
    }

    pub fn random_in_unit_disc() -> Vec3 {
        loop {
            let x = rand::thread_rng().gen_range(-1.0..=1.0);
            let y = rand::thread_rng().gen_range(-1.0..=1.0);

            let vec = Vec3::new(x, y, 0.0);
            let len = vec.length_squared();
            if len <= 1.0 {
                return vec
            }
        }
    }

    pub fn reflect(v : Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * Vec3::dot(v, n) * n
    }

    pub fn refract(uv : Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(Vec3::dot(-uv, n), 1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta*n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new( 
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z 
        )
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new( 
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z 
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new( 
            self.x * rhs,
            self.y * rhs,
            self.z * rhs 
        )
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new( 
            self.x / rhs,
            self.y / rhs,
            self.z / rhs 
        )
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}
