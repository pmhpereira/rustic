use nalgebra::Vector3;

use std::fs;

pub struct ObjLoader {}

pub struct Obj {
    pub faces: Vec<(Vector3<f64>, Vector3<f64>)>,
}

impl ObjLoader {
    pub fn load(file_path: String) -> std::io::Result<Obj> {
        let contents = fs::read_to_string(file_path)?;

        let mut vertices = Vec::<Vector3<f64>>::new();
        let mut normals = Vec::<Vector3<f64>>::new();
        let mut faces = Vec::<(Vector3<f64>, Vector3<f64>)>::new();

        vertices.push(Vector3::zeros());
        normals.push(Vector3::zeros());

        let lines = contents.split("\n");

        for line in lines {
            if line.is_empty() {
                continue;
            }

            let elems = line.split(" ").collect::<Vec<&str>>();

            match elems[0] {
                "#" => continue,
                "v" => vertices.push(Vector3::new(
                    elems[1].parse().unwrap(),
                    elems[2].parse().unwrap(),
                    elems[3].parse().unwrap(),
                )),
                "vn" => normals.push(Vector3::new(
                    elems[1].parse().unwrap(),
                    elems[2].parse().unwrap(),
                    elems[3].parse().unwrap(),
                )),
                "s" => continue,
                "f" => {
                    for i in 1..=3 {
                        let pair: Vec<&str> = elems[i].split("/").collect();
                        let (vertex_index, normal_index): (usize, usize) =
                            (pair[0].parse().unwrap(), pair[2].parse().unwrap());
                        faces.push((vertices[vertex_index], normals[normal_index]));
                    }
                }
                _ => continue,
            }
        }

        Ok(Obj { faces })
    }
}
