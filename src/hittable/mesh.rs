use std::cmp::min_by;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;
use crate::hittable::triangle::Triangle;
use crate::material::Material;
use crate::scene::Hittable;
use crate::vec::Vec3;

pub struct Mesh {
    triangles: Vec<Triangle>
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>) -> Mesh {
        Mesh {
            triangles
        }
    }

    pub fn load(path: impl AsRef<Path>, material: Rc<dyn Material>) -> Result<Mesh, Box<dyn Error>> {
        let file: File = File::open(path)?;
        let mut reader: BufReader<File> = BufReader::new(file);
        let mut triangles: Vec<Triangle> = Vec::new();
        let mut line: String = String::new();
        reader.read_line(&mut line)?;
        let triangle_count: usize = line.trim().parse()?;
        (0..triangle_count).for_each(|_| {
            line.clear();
            reader.read_line(&mut line).unwrap();
            let mut points: Vec<Vec3> = Vec::new();
            (0..3).for_each(|_| {
                line.clear();
                reader.read_line(&mut line).unwrap();
                let mut words: Vec<&str> = line.split_whitespace().collect();
                let x: f32 = words[0].parse().unwrap();
                let y: f32 = words[1].parse().unwrap();
                let z: f32 = words[2].parse().unwrap();
                points.push(Vec3::new(x, y, z));
            });
            let a: Vec3 = points[0];
            let b: Vec3 = points[1];
            let c: Vec3 = points[2];
            let triangle: Triangle = Triangle::new(a, b, c, material.clone());
            triangles.push(triangle);
        });
        Ok(Mesh::new(triangles))
    }

    pub fn translate(&mut self, translation: Vec3) {
        self.triangles.iter_mut().for_each(|triangle| {
            triangle.translate(translation);
        });
    }

    pub fn rotate(&mut self, rotation: Vec3) {
        self.triangles.iter_mut().for_each(|triangle| {
            triangle.rotate(rotation);
        });
    }

    pub fn scale(&mut self, scale: Vec3) {
        self.triangles.iter_mut().for_each(|triangle| {
            triangle.scale(scale);
        });
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<crate::hittable::HitRecord> {
        self.triangles
            .iter()
            .filter_map(|triangle| {
                triangle.hit(ray, t_min, t_max)
            })
            .min_by(|a, b| {
                a.get_t().partial_cmp(&b.get_t()).unwrap()
            })
    }

    fn get_object_count(&self) -> usize {
        self.triangles.len()
    }
}
