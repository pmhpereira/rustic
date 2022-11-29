use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;
use crate::vec3::Vec3;

use dyn_clone::DynClone;
use rand::Rng;

pub trait Material : DynClone {
    fn scatter(&self, ray : &Ray, hit : &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

#[derive(Clone)]
pub struct LambertianMaterial {
    albedo: Color
}

impl LambertianMaterial {
    pub fn new(albedo : Color) -> LambertianMaterial {
        LambertianMaterial { albedo: albedo }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, _ray : &Ray, hit : &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let scattered_direction = hit.normal + Vec3::random_in_unit_sphere().normalized();
        *scattered = Ray::new(hit.point, scattered_direction);
        *attenuation = self.albedo;
        
        return true;
    }
}

#[derive(Clone)]
pub struct MetalMaterial {
    albedo: Color,
    fuzz : f64,
}

impl MetalMaterial {
    pub fn new(albedo : Color, fuzz : f64) -> MetalMaterial {
        MetalMaterial { albedo: albedo, fuzz: fuzz }
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, ray : &Ray, hit : &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected_direction = Vec3::reflect(ray.direction.normalized(), hit.normal);
        *scattered = Ray::new(hit.point, reflected_direction + self.fuzz * Vec3::random_in_unit_sphere());
        *attenuation = self.albedo;
        
        return true;
    }
}

#[derive(Clone)]
pub struct DielectricMaterial {
    ir: f64,
}

impl DielectricMaterial {
    pub fn new(ir : f64) -> DielectricMaterial {
        DielectricMaterial { ir: ir }
    }

    fn reflectance(cosine: f64, ref_index: f64) -> f64 {
        let r0 = (1.0 - ref_index) / (1.0 + ref_index);
        let r0 = r0 * r0;

        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
    }
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray : &Ray, hit : &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);

        let refraction_ratio = if hit.front_face { 1.0 / self.ir } else { self.ir };

        let direction_normalized = ray.direction.normalized();

        let cos_theta = f64::min(Vec3::dot(-direction_normalized, hit.normal), 1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;

        let direction: Vec3;
        if cannot_refract == true || DielectricMaterial::reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen() {
            direction = Vec3::reflect(direction_normalized, hit.normal);
        }
        else {
            direction = Vec3::refract(direction_normalized, hit.normal, refraction_ratio);
        }

        *scattered = Ray::new(hit.point, direction);
        
        return true;
    }
}
