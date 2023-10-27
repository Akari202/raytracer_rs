use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::Vec3;

pub mod diffuse;

pub type Color = Vec3;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)>;
}
