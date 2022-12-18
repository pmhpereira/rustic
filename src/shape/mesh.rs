use super::Triangle;

use crate::aabb::AABB;
use crate::bvh::BVH;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::hittable::HittableList;
use crate::material::Material;
use crate::obj_loader::ObjLoader;
use crate::ray::Ray;

use std::sync::Arc;

use nalgebra::Vector3;

pub struct Mesh {
    faces: Arc<dyn Hittable>,
    material: Arc<dyn Material>,
    minimum: Vector3<f64>,
    maximum: Vector3<f64>,
}

impl Mesh {
    pub fn arc(file_path: String, material: Arc<dyn Material>) -> Arc<Mesh> {
        let result = ObjLoader::load(file_path).unwrap();

        let mut faces = HittableList::new();

        let mut minimum = Vector3::new(f64::MAX, f64::MAX, f64::MAX);
        let mut maximum = Vector3::new(f64::MIN, f64::MIN, f64::MIN);

        for f in result.faces.chunks(3) {
            let vertices = [f[0].0, f[1].0, f[2].0].to_vec();
            let normals = [f[0].1, f[1].1, f[2].1].to_vec();

            minimum.x = minimum
                .x
                .min(vertices[0].x.min(vertices[1].x.min(vertices[2].x)));
            minimum.y = minimum
                .y
                .min(vertices[0].y.min(vertices[1].y.min(vertices[2].y)));
            minimum.z = minimum
                .z
                .min(vertices[0].z.min(vertices[1].z.min(vertices[2].z)));

            maximum.x = maximum
                .x
                .max(vertices[0].x.max(vertices[1].x.max(vertices[2].x)));
            maximum.y = maximum
                .y
                .max(vertices[0].y.max(vertices[1].y.max(vertices[2].y)));
            maximum.z = maximum
                .z
                .max(vertices[0].z.max(vertices[1].z.max(vertices[2].z)));

            faces.add(Triangle::arc_normal(
                vertices,
                normals,
                Arc::clone(&material),
            ));
        }

        let faces = BVH::arc(&mut faces.objects);
        Arc::new(Mesh {
            faces,
            material,
            minimum,
            maximum,
        })
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit: &mut HitRecord) -> bool {
        self.faces.hit(ray, t_min, t_max, hit)
    }

    fn bounding_box(&self, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(self.minimum, self.maximum);
        true
    }
}
