// use std::intrinsics::sqrtf32;
use crate::hittable::HitRecord;
use crate::material::{Color, Material};
use crate::ray::Ray;
use crate::vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Dielectric {
    refraction_index: f32,
    color: Color
}

impl Dielectric {
    pub fn new(refraction_index: f32, color: Color) -> Dielectric {
        Dielectric {
            refraction_index,
            color
        }
    }

    pub fn get_refraction_index(&self) -> f32 {
        self.refraction_index
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
        let r0: f32 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Color)> {
        let refraction_ratio: f32 = if hit.get_normal().dot(ray.get_direction()) > 0.0 {
            self.refraction_index
        } else {
            1.0 / self.refraction_index
        };
        let unit_direction: Vec3 = ray.get_direction().get_normalized();
        let cos_theta: f32 = (-unit_direction).dot(hit.get_normal()).min(1.0);
        let sin_theta: f32 = 1.0 - cos_theta * cos_theta;
        let sin_theta: f32 = sin_theta.sqrt();
        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3 = if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rand::random::<f32>() {
            unit_direction.reflect(hit.get_normal())
        } else {
            unit_direction.refract(hit.get_normal(), refraction_ratio)
        };
        let scattered: Ray = Ray::from_to(*hit.get_point(), direction);
        let attenuation: Color = self.color;
        Some((scattered, attenuation))
    }
}
