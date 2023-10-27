use std::cmp::min_by;
use crate::ray::Ray;
use crate::vec::Vec3;
use rayon::prelude::*;
pub(crate) use crate::hittable::{HitRecord, Hittable};

pub struct Scene {
    objects: Vec<Box<dyn Hittable>>
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new()
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn add_objects(&mut self, objects: Vec<Box<dyn Hittable>>) {
        self.objects.extend(objects);
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_t: f32 = t_max;
        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t_min, closest_t) {
                closest_t = hit.get_t();
                closest_hit = Some(hit);
            }
        }
        closest_hit
    }
}

