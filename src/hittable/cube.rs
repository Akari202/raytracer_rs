use std::rc::Rc;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::scene::Hittable;
use crate::vec::Vec3;

pub struct Cube {
    center: Vec3,
    width: Vec3,
    height: Vec3,
    depth: Vec3,
    material: Rc<dyn Material>
}

impl Cube {
    pub fn new(center: Vec3, width: Vec3, height: Vec3, depth: Vec3, material: Rc<dyn Material>) -> Cube {
        Cube {
            center,
            width,
            height,
            depth,
            material
        }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let normal: Vec3 = self.width.cross(&self.height);
        let t: f32 = (self.center - ray.get_origin()).dot(&normal) / ray.get_direction().dot(&normal);
        if t < t_min || t > t_max {
            return None;
        }
        let point: Vec3 = ray.at(t);
        let to_point: Vec3 = point - self.center;
        let width: f32 = self.width.dot(&to_point);
        let height: f32 = self.height.dot(&to_point);
        if width < 0.0 || width > self.width.length_squared() || height < 0.0 || height > self.height.length_squared() {
            return None;
        }
        Some(HitRecord::new(point, normal, t, &self.material))
    }
}
