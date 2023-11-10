use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::scene::Hittable;
use crate::vec::Vec3;

pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    normal: Vec3,
    material: Box<dyn Material>
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, material: Box<dyn Material>) -> Triangle {
        let normal: Vec3 = (b - a).cross(&(c - a)).unit();
        Triangle {
            a,
            b,
            c,
            normal,
            material
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t: f32 = (self.a - ray.get_origin()).dot(&self.normal) / ray.get_direction().dot(&self.normal);
        if t < t_min || t > t_max {
            return None;
        }
        let point: Vec3 = ray.at(t);
        let ab: Vec3 = self.b - self.a;
        let bc: Vec3 = self.c - self.b;
        let ca: Vec3 = self.a - self.c;
        let ap: Vec3 = point - self.a;
        let bp: Vec3 = point - self.b;
        let cp: Vec3 = point - self.c;
        let ab_normal: Vec3 = ab.cross(&self.normal);
        let bc_normal: Vec3 = bc.cross(&self.normal);
        let ca_normal: Vec3 = ca.cross(&self.normal);
        if ab_normal.dot(&ap) < 0.0 || bc_normal.dot(&bp) < 0.0 || ca_normal.dot(&cp) < 0.0 {
            return None;
        }
        Some(HitRecord::new(point, self.normal, t, &self.material))
    }
}
