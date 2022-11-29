mod color;
use crate::color::Color;

mod vec3;
use crate::vec3::Vec3;

mod sphere;
use crate::sphere::Sphere;

mod ray;
use crate::ray::Ray;

mod hittable;
use crate::hittable::{HittableList};

mod camera;
use crate::camera::Camera;

mod material;
use crate::material::Material;
use crate::material::LambertianMaterial;
use crate::material::MetalMaterial;
use crate::material::DielectricMaterial;

use rand::Rng;

fn random_world() -> HittableList {
    let mut world = HittableList::new();
    
    let material_ground = Box::new(LambertianMaterial::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, -1.0), 1000.0, material_ground)));

    for x in -11..11 {
        for y in -11..11 {
            let center = Vec3::new(x as f64 + 0.9 * rand::thread_rng().gen::<f64>(), 0.2, y as f64 + 0.9 * rand::thread_rng().gen::<f64>());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material_random_value : f64 = rand::thread_rng().gen();
                
                let material_sphere : Box<dyn Material>;
                
                if material_random_value < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    material_sphere = Box::new(LambertianMaterial::new(albedo));
                }
                else if material_random_value < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                    material_sphere = Box::new(MetalMaterial::new(albedo, fuzz));
                }
                else {
                    // glass
                    material_sphere = Box::new(DielectricMaterial::new(1.5));
                }

                world.add(Box::new(Sphere::new(center, 0.2, material_sphere)));
            }
        }
    }

    let material_left = Box::new(LambertianMaterial::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material_left)));
    
    let material_center = Box::new(DielectricMaterial::new(1.5));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material_center)));
    
    let material_right = Box::new(MetalMaterial::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material_right)));

    world
}

fn main() {
    // World
    let world = random_world();

    // Image
    const ASPECT_RATIO : f64 = 16.0 / 9.0;
    const IMAGE_WIDTH : u64 = 100;
    const IMAGE_HEIGHT : u64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL : u64 = 16;
    const MAX_DEPTH : u64 = 5;
    
    // Camera
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let focus_distance = 10.0;

    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        20.0,
        ASPECT_RATIO,
        0.1,
        focus_distance
    );

    // Render
    let mut image = Vec::<Color>::new();
    image.resize((IMAGE_WIDTH * IMAGE_HEIGHT) as usize, Color::new(0.0, 0.0, 0.0));

    for y in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {y}");
        
        for x in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (x as f64 + rand::thread_rng().gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (y as f64 + rand::thread_rng().gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;

                let ray = camera.get_ray(u, v);

                pixel_color = pixel_color + Ray::ray_color(&ray, &world, MAX_DEPTH);
            }
            
            image[(x + y * IMAGE_WIDTH) as usize] = pixel_color / SAMPLES_PER_PIXEL as f64;
        }
    }

    
    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");
    for y in (0..IMAGE_HEIGHT).rev() {
        for x in 0..IMAGE_WIDTH {
            let pixel_color = image[(x + y * IMAGE_WIDTH) as usize];
            Color::write_color(std::io::stdout(), pixel_color, 2.0);
        }
    }

    eprintln!();
    eprintln!("Done.");
}