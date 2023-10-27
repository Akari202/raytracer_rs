use crate::hittable::HitRecord;
use crate::material::{Color, Material};
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Diffuse {
    color: Color,
    albedo: f32
}

impl Diffuse {
    pub fn new(color: Color, albedo: f32) -> Diffuse {
        Diffuse {
            color,
            albedo
        }
    }
}

impl Material for Diffuse {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let target: Vec3 = hit.get_point() + hit.get_normal() + Vec3::random_in_unit_sphere();
        let scattered: Ray = Ray::from_to(*hit.get_point(), target);
        let attenuation: Color = self.color * self.albedo;
        Some((scattered, attenuation))
    }
}
