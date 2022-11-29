use std::{io::Write, ops::{Add, Mul, Div}};

use crate::vec3::Vec3;

use rand::Rng;

#[derive(Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {r, g, b}
    }

    pub fn random() -> Color {
        let r = rand::thread_rng().gen::<f64>();
        let g = rand::thread_rng().gen::<f64>();
        let b = rand::thread_rng().gen::<f64>();

        Color {r, g, b}
    }

    pub fn random_range(min: f64, max: f64) -> Color {
        let r = rand::thread_rng().gen_range(min..max);
        let g = rand::thread_rng().gen_range(min..max);
        let b = rand::thread_rng().gen_range(min..max);

        Color {r, g, b}
    }

    pub fn write_color(mut stream : impl Write, color : Color, gamma : f64) {
        let gamma_color = color.gamma(gamma);

        let ur: u64 = (255.0 * gamma_color.r) as u64;
        let ug: u64 = (255.0 * gamma_color.g) as u64;
        let ub: u64 = (255.0 * gamma_color.b) as u64;

        writeln!(stream, "{} {} {}", ur, ug, ub);
    }

    pub fn lerp(t : f64, a : Color, b : Color) -> Color {
        return (1.0 - t) * a + t * b;
    }

    pub fn gamma(self, gamma : f64) -> Color {
        let gamma_r = self.r.powf(1.0 / gamma);
        let gamma_g = self.g.powf(1.0 / gamma);
        let gamma_b = self.b.powf(1.0 / gamma);

        Color::new(gamma_r, gamma_g, gamma_b)
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(
            self.r + rhs.r,
            self.g + rhs.g,
            self.b + rhs.b,
        )
    }
}

impl Add<Vec3> for Color {
    type Output = Color;

    fn add(self, rhs: Vec3) -> Self::Output {
        Color::new(
            self.r + rhs.x,
            self.g + rhs.y,
            self.b + rhs.z,
        )
    }
}

impl Add<Color> for Vec3 {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(
            rhs.r + self.x,
            rhs.g + self.y,
            rhs.b + self.z,
        )
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(
            self.r * rhs,
            self.g * rhs,
            self.b * rhs
        )
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}


impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(
            self.r * rhs.r,
            self.g * rhs.g,
            self.b * rhs.b
        )
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}
