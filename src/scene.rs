use crate::bvh::BVH;
use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList};
use crate::shape::*;
use crate::texture::*;
use crate::transform::*;
use crate::vector3_traits::Helpers;
use crate::material::*;
use crate::ASPECT_RATIO;

use std::sync::Arc;

use nalgebra::Vector3;

pub struct Scene {
    pub camera: Camera,
    pub world: Box<dyn Hittable>,
}

impl Scene {
    pub fn random() -> Scene {
        // Camera
        let look_from = Vector3::new(13.0, 2.0, 3.0);
        let look_at = Vector3::new(0.0, 0.0, 0.0);
        let v_up = Vector3::new(0.0, 1.0, 0.0);
        let focus_distance = 10.0;

        let camera = Camera::new(
            look_from,
            look_at,
            v_up,
            40.0,
            ASPECT_RATIO,
            0.1,
            focus_distance,
        );

        let mut world = HittableList::new();

        let checker_texture = CheckerTexture::new(
            Box::new(SolidColorTexture::new(Vector3::new(0.2, 0.3, 0.1))),
            Box::new(SolidColorTexture::new(Vector3::new(0.9, 0.9, 0.9))),
        );

        let material_ground = Box::new(LambertianMaterial::new(Arc::new(checker_texture)));

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
                        material_sphere = Box::new(LambertianMaterial::new(Arc::new(
                            SolidColorTexture::new(albedo),
                        )));
                    } else if material_random_value < 0.95 {
                        // metal
                        let albedo = Vector3::new_random_in_range(0.5, 1.0);
                        let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                        material_sphere = Box::new(MetalMaterial::new(
                            Arc::new(SolidColorTexture::new(albedo)),
                            fuzz,
                        ));
                    } else {
                        // glass
                        material_sphere = Box::new(DielectricMaterial::new(1.5));
                    }

                    world.add(Arc::new(Sphere::new(center, radius, material_sphere)));
                }
            }
        }

        let material_left = Box::new(DielectricMaterial::new(1.5));
        world.add(Arc::new(Sphere::new(
            Vector3::new(-4.0, 1.0, 0.0),
            1.0,
            material_left,
        )));

        let material_center = Box::new(LambertianMaterial::new(Arc::new(ImageTexture::new(
            "resources/earth.jpg".to_string(),
        ))));
        world.add(Arc::new(Sphere::new(
            Vector3::new(0.0, 1.0, 0.0),
            1.0,
            material_center,
        )));

        let material_right = Box::new(MetalMaterial::new(
            Arc::new(SolidColorTexture::new(Vector3::new(0.7, 0.6, 0.5))),
            0.0,
        ));
        world.add(Arc::new(Sphere::new(
            Vector3::new(4.0, 1.0, 0.0),
            1.0,
            material_right,
        )));

        let mut diffuse_light = Box::new(EmissiveMaterial::new(Arc::new(ImageTexture::new(
            "resources/earth.jpg".to_string(),
        ))));
        diffuse_light.set_scale(3.0);
        world.add(Arc::new(Sphere::new(
            Vector3::new(8.0, 1.0, 0.0),
            1.0,
            diffuse_light,
        )));

        Scene {
            camera: camera,
            world: Box::new(BVH::new(&mut world.objects, (0.0, 1.0))),
        }
    }

    pub fn cornell_box() -> Scene {
        // Camera
        let look_from = Vector3::new(278.0, 278.0, -800.0);
        let look_at = Vector3::new(278.0, 278.0, 0.0);
        let v_up = Vector3::new(0.0, 1.0, 0.0);
        let focus_distance = 10.0;

        let camera = Camera::new(
            look_from,
            look_at,
            v_up,
            40.0,
            ASPECT_RATIO,
            0.1,
            focus_distance,
        );

        let mut world = HittableList::new();

        let red_material = Box::new(LambertianMaterial::new(Arc::new(SolidColorTexture::new(
            Vector3::new(0.65, 0.05, 0.05),
        ))));
        let white_material = Box::new(LambertianMaterial::new(Arc::new(SolidColorTexture::new(
            Vector3::new(0.73, 0.73, 0.73),
        ))));
        let white_material2 = Box::new(LambertianMaterial::new(Arc::new(SolidColorTexture::new(
            Vector3::new(0.73, 0.73, 0.73),
        ))));
        let white_material3 = Box::new(LambertianMaterial::new(Arc::new(SolidColorTexture::new(
            Vector3::new(0.73, 0.73, 0.73),
        ))));
        let white_material4 = Box::new(LambertianMaterial::new(Arc::new(SolidColorTexture::new(
            Vector3::new(0.73, 0.73, 0.73),
        ))));
        let white_material5 = Box::new(LambertianMaterial::new(Arc::new(SolidColorTexture::new(
            Vector3::new(0.73, 0.73, 0.73),
        ))));
        let green_material = Box::new(LambertianMaterial::new(Arc::new(SolidColorTexture::new(
            Vector3::new(0.12, 0.45, 0.15),
        ))));
        let light_material = Box::new(EmissiveMaterial::new(Arc::new(SolidColorTexture::new(
            Vector3::new(15.0, 15.0, 15.0),
        ))));

        world.add(Arc::new(RectangleYZ::new(
            (0.0, 555.0),
            (0.0, 555.0),
            555.0,
            green_material,
        )));

        world.add(Arc::new(RectangleYZ::new(
            (0.0, 555.0),
            (0.0, 555.0),
            0.0,
            red_material,
        )));

        world.add(Arc::new(RectangleXZ::new(
            (213.0, 343.0),
            (227.0, 332.0),
            554.0,
            light_material,
        )));

        world.add(Arc::new(RectangleXZ::new(
            (0.0, 555.0),
            (0.0, 555.0),
            0.0,
            white_material,
        )));

        world.add(Arc::new(RectangleXZ::new(
            (0.0, 555.0),
            (0.0, 555.0),
            555.0,
            white_material2,
        )));

        world.add(Arc::new(RectangleXY::new(
            (0.0, 555.0),
            (0.0, 555.0),
            555.0,
            white_material3,
        )));

        world.add(Arc::new(Translate::new(Vector3::new(265.0, 0.0, 295.0), Box::new(RotateY::new(15.0, Box::new(Cube::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(165.0, 330.0, 165.0),
            white_material4,
        )))))));
        
        world.add(Arc::new(Translate::new(Vector3::new(130.0, 0.0, 65.0), Box::new(RotateY::new(-18.0, Box::new(Cube::new(
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(165.0, 165.0, 165.0),
            white_material5,
        )))))));

        Scene {
            camera: camera,
            world: Box::new(BVH::new(&mut world.objects, (0.0, 1.0))),
        }
    }
}
