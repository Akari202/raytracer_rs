use crate::ray::Ray;
use crate::vec::Vec3;

pub mod sphere;
pub mod plane;


pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Debug)]
pub struct HitRecord {
    point: Vec3,
    normal: Vec3,
    t: f32
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f32) -> HitRecord {
        HitRecord {
            point,
            normal,
            t
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
}
