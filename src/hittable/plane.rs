pub(crate) use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::vec::Vec3;

pub struct Plane {
    normal: Vec3,
    point: Vec3,
    material: Box<dyn Material>
}

impl Plane {
    pub fn new(normal: Vec3, point: Vec3, material: Box<dyn Material>) -> Plane {
        Plane {
            normal: normal.get_normalized(),
            point,
            material
        }
    }

    pub fn from_points(a: Vec3, b: Vec3, c: Vec3, material: Box<dyn Material>) -> Plane {
        let normal: Vec3 = (b - a).cross(&(c - a)).get_normalized();
        Plane {
            normal,
            point: a,
            material
        }
    }

    pub fn get_normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn get_point(&self) -> &Vec3 {
        &self.point
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let denominator: f32 = ray.get_direction().dot(&self.normal);
        if denominator.abs() > 0.0001 {
            let t: f32 = (self.point - *ray.get_origin()).dot(&self.normal) / denominator;
            if t < t_max && t > t_min {
                let point: Vec3 = ray.at(t);
                return Some(HitRecord::new(point, self.normal, t));
            }
        }
        None
    }
}
