mod sphere;
use crate::sphere::Sphere;

mod ray;
use crate::ray::Ray;

mod hittable;
use crate::hittable::{Hittable, HittableList};

mod camera;
use crate::camera::Camera;

mod material;
use crate::material::{DielectricMaterial, LambertianMaterial, Material, MetalMaterial};

mod vector3_traits;
use crate::vector3_traits::Helpers;

mod aabb;

mod bvh;
use bvh::BVH;

use nalgebra::Vector3;
use rand::Rng;
use rayon::prelude::*;

use std::sync::Arc;

fn save_image(file_path: &str, width: u32, height: u32, pixels: Vec<f64>) -> std::io::Result<()> {
    let transformed_pixels: Vec<u8> = pixels
        .into_iter()
        .map(|pixel| (255.0 * pixel) as u8)
        .collect();

    image::save_buffer(
        file_path,
        &transformed_pixels,
        width,
        height,
        image::ColorType::Rgb8,
    )
    .unwrap();

    Ok(())
}

fn random_world() -> impl Hittable {
    let mut world = HittableList::new();

    let material_ground = Box::new(LambertianMaterial::new(Vector3::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Vector3::new(0.0, -1000.0, -1.0),
        1000.0,
        material_ground,
    )));

    for x in -11..11 {
        for y in -11..11 {
            let radius = 0.2;

            let center = Vector3::new(
                x as f64 + 0.9 * rand::thread_rng().gen::<f64>(),
                radius,
                y as f64 + 0.9 * rand::thread_rng().gen::<f64>(),
            );

            if (center - Vector3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                let material_random_value: f64 = rand::thread_rng().gen();

                let material_sphere: Box<dyn Material>;

                if material_random_value < 0.8 {
                    // diffuse
                    let albedo = Vector3::new_random().component_mul(&Vector3::new_random());
                    material_sphere = Box::new(LambertianMaterial::new(albedo));
                } else if material_random_value < 0.95 {
                    // metal
                    let albedo = Vector3::new_random_in_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                    material_sphere = Box::new(MetalMaterial::new(albedo, fuzz));
                } else {
                    // glass
                    material_sphere = Box::new(DielectricMaterial::new(1.5));
                }

                world.add(Arc::new(Sphere::new(center, radius, material_sphere)));
            }
        }
    }

    let material_left = Box::new(LambertianMaterial::new(Vector3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        material_left,
    )));

    let material_center = Box::new(DielectricMaterial::new(1.5));
    world.add(Arc::new(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        material_center,
    )));

    let material_right = Box::new(MetalMaterial::new(Vector3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        material_right,
    )));

    BVH::new(&mut world.objects, (0.0, 1.0))
}

fn main() {
    // World
    let world = random_world();

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 600;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u64 = 16;
    const MAX_DEPTH: u64 = 5;
    const GAMMA: f64 = 2.0;

    // Camera
    let look_from = Vector3::new(13.0, 2.0, 3.0);
    let look_at = Vector3::new(0.0, 0.0, 0.0);
    let v_up = Vector3::new(0.0, 1.0, 0.0);
    let focus_distance = 10.0;

    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        20.0,
        ASPECT_RATIO,
        0.1,
        focus_distance,
    );

    // Render
    let instant = std::time::Instant::now();
    let image = (0..IMAGE_HEIGHT)
        .into_par_iter()
        .rev()
        .flat_map(|y| {
            (0..IMAGE_WIDTH)
                .into_par_iter()
                .flat_map(|x| {
                    let mut pixel_color = Vector3::zeros();

                    for _s in 0..SAMPLES_PER_PIXEL {
                        let u =
                            (x as f64 + rand::thread_rng().gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                        let v = (y as f64 + rand::thread_rng().gen::<f64>())
                            / (IMAGE_HEIGHT - 1) as f64;

                        let ray = camera.get_ray(u, v);

                        pixel_color = pixel_color + Ray::ray_color(&ray, &world, MAX_DEPTH);
                    }

                    pixel_color = pixel_color / SAMPLES_PER_PIXEL as f64;
                    pixel_color = pixel_color.gamma(GAMMA);

                    [pixel_color.x, pixel_color.y, pixel_color.z]
                })
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<f64>>();

    let _ = save_image("render.png", IMAGE_WIDTH, IMAGE_HEIGHT, image);

    eprintln!();
    eprintln!("Done. Took {} seconds.", instant.elapsed().as_secs_f64());
}
