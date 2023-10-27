pub(crate) use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Vec3;


pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Box<dyn Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material
        }
    }
}

impl Hittable for Sphere {
   fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = ray.get_origin() - self.center;
        let a: f32 = ray.get_direction().dot(ray.get_direction());
        let b: f32 = oc.dot(ray.get_direction());
        let c: f32 = oc.dot(&oc) - self.radius * self.radius;
        let discriminant: f32 = b * b - a * c;
        if discriminant > 0.0 {
            let t: f32 = (-b - discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let point: Vec3 = ray.at(t);
                let normal: Vec3 = (point - self.center) / self.radius;
                return Some(HitRecord::new(point, normal, t));
            }
            let t: f32 = (-b + discriminant.sqrt()) / a;
            if t < t_max && t > t_min {
                let point: Vec3 = ray.at(t);
                let normal: Vec3 = (point - self.center) / self.radius;
                return Some(HitRecord::new(point, normal, t));
            }
        }
        None
   }
}
