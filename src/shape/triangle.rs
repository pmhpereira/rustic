use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;

use std::sync::Arc;

use nalgebra::Vector3;

pub struct Triangle {
    vertices: Vec<Vector3<f64>>,
    normals: Vec<Vector3<f64>>,
    indices: Vec<usize>,
    material: Arc<dyn Material>,
}

impl Triangle {
    pub fn arc_normal(
        vertices: Vec<Vector3<f64>>,
        normals: Vec<Vector3<f64>>,
        material: Arc<dyn Material>,
    ) -> Arc<Triangle> {
        let determinant = Vector3::dot(&-vertices[0], &normals[0]);

        let abs_normal = normals[0].abs();
        let max_abs_normal = abs_normal.max();

        let mut indices;
        if max_abs_normal == abs_normal.x {
            indices = [0, 1, 2].to_vec();
        } else if max_abs_normal == abs_normal.y {
            indices = [1, 2, 0].to_vec();
        } else {
            indices = [2, 0, 1].to_vec();
        }

        Arc::new(Triangle {
            vertices,
            normals,
            indices,
            material,
        })
    }

    pub fn arc(vertices: Vec<Vector3<f64>>, material: Arc<dyn Material>) -> Arc<Triangle> {
        let normal =
            Vector3::cross(&(vertices[1] - vertices[0]), &(vertices[2] - vertices[0])).normalize();

        let normals = [normal, normal, normal].to_vec();
        Triangle::arc_normal(vertices, normals, material)
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit: &mut HitRecord) -> bool {
        let ab = self.vertices[1] - self.vertices[0];
        let ac = self.vertices[2] - self.vertices[0];

        let n = ab.cross(&ac);
        let d = n.dot(&ray.direction);

        if d == 0.0 {
            return false;
        }

        let ap = ray.origin - self.vertices[0];
        let t = ap.dot(&n);

        if t < 0.0 && d < 0.0 {
            return false;
        }

        if t > 0.0 && d > 0.0 {
            return false;
        }

        let d = d.abs();

        let e = -ray.direction.cross(&ap);

        let mut v;
        let mut w;
        let toi;
        let normal;

        if t < 0.0 {
            v = -ac.dot(&e);

            if v < 0.0 || v > d {
                return false;
            }

            w = ab.dot(&e);

            if w < 0.0 || v + w > d {
                return false;
            }

            let invd = 1.0 / d;
            toi = -t * invd;
            normal = -n.normalize();
            v = v * invd;
            w = w * invd;
        } else {
            v = ac.dot(&e);

            if v < 0.0 || v > d {
                return false;
            }

            w = -ab.dot(&e);

            if w < 0.0 || v + w > d {
                return false;
            }

            let invd = 1.0 / d;
            toi = t * invd;
            normal = n.normalize();
            v = v * invd;
            w = w * invd;
        }

        hit.t = toi;
        hit.point = ray.at(toi);
        hit.normal = normal;
        hit.material = Arc::clone(&self.material);

        hit.set_face_normal(ray.direction, hit.normal);

        true
    }

    fn bounding_box(&self, output_box: &mut AABB) -> bool {
        let x0 = f64::min(
            self.vertices[0].x,
            f64::min(self.vertices[1].x, self.vertices[2].x),
        );
        let y0 = f64::min(
            self.vertices[0].y,
            f64::min(self.vertices[1].y, self.vertices[2].y),
        );
        let z0 = f64::min(
            self.vertices[0].z,
            f64::min(self.vertices[1].z, self.vertices[2].z),
        );

        let x1 = f64::max(
            self.vertices[0].x,
            f64::max(self.vertices[1].x, self.vertices[2].x),
        );
        let y1 = f64::max(
            self.vertices[0].y,
            f64::max(self.vertices[1].y, self.vertices[2].y),
        );
        let z1 = f64::max(
            self.vertices[0].z,
            f64::max(self.vertices[1].z, self.vertices[2].z),
        );

        let epsilon = Vector3::new(0.0001, 0.0001, 0.0001);

        *output_box = AABB::new(
            Vector3::new(x0, y0, z0) - epsilon,
            Vector3::new(x1, y1, z1) + epsilon,
        );

        true
    }
}
