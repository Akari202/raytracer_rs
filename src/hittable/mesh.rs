use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::hittable::triangle::Triangle;
use crate::material::Material;
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

    pub fn load(path: impl AsRef<Path>, material: Box<dyn Material>) -> Result<Mesh, Box<dyn Error>> {
        todo!()
    }
}
