use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Vec3;

pub mod sphere;
pub mod plane;
pub mod rectangle;
pub mod cube;
pub mod triangle;
pub mod mesh;


pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Debug, Clone)]
pub struct HitRecord<'a> {
    point: Vec3,
    normal: Vec3,
    t: f32,
    material: &'a Box<dyn Material>
}

impl HitRecord<'_> {
    pub fn new(point: Vec3, normal: Vec3, t: f32, material: &Box<dyn Material>) -> HitRecord {
        HitRecord {
            point,
            normal,
            t,
            material
        }
    }

    pub fn get_point(&self) -> &Vec3 {
        &self.point
    }

    pub fn get_normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn get_t(&self) -> f32 {
        self.t
    }

    pub fn get_material(&self) -> &Box<dyn Material> {
        self.material
    }
}
