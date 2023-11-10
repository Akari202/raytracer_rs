use crate::hittable::HitRecord;
use crate::material::{Color, Material};
use crate::ray::Ray;
use crate::vec::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Reflective {
    albedo: Color,
    fuzz: f32
}

impl Reflective {
    pub fn new(albedo: Color, fuzz: f32) -> Reflective {
        Reflective {
            albedo,
            fuzz
        }
    }
}

impl Material for Reflective {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let reflected: Vec3 = ray.get_direction().reflect(hit.get_normal());
        let scattered: Ray = Ray::from_to(*hit.get_point(), *hit.get_point() + reflected + Vec3::random_in_unit_sphere() * self.fuzz);
        let attenuation: Color = self.albedo;
        if scattered.get_direction().dot(hit.get_normal()) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
