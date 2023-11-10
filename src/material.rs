use std::fmt::Debug;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::Vec3;

pub mod diffuse;
pub mod reflective;
pub mod dielectric;

pub type Color = Vec3;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)>;
}

impl Debug for dyn Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Material")
            .finish()
    }
}
