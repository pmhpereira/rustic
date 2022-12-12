mod shape;

mod ray;
use crate::ray::Ray;

mod hittable;

mod camera;

mod material;

mod vector3_traits;
use crate::vector3_traits::Helpers;

mod aabb;

mod bvh;

mod texture;

mod scene;
use crate::scene::Scene;

use nalgebra::Vector3;
use rand::Rng;
use rayon::prelude::*;

pub const ASPECT_RATIO: f64 = 2.0;
pub const IMAGE_WIDTH: u32 = 400;
pub const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
pub const SAMPLES_PER_PIXEL: u64 = 50;
pub const MAX_DEPTH: u64 = 50;
pub const GAMMA: f64 = 2.0;
pub const BACKGROUND_COLOR: Vector3<f64> = Vector3::new(0.9, 0.9, 0.9);

fn main() {
    // World
    let scene = Scene::random();

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

                        let ray = scene.camera.get_ray(u, v);

                        pixel_color = pixel_color
                            + Ray::ray_color(&ray, &scene.world, &BACKGROUND_COLOR, MAX_DEPTH);
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
