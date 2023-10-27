use std::error::Error;
use log::{debug, info};
use crate::image::Image;
use crate::vec::Vec3;
use crate::ray::Ray;
use rayon::prelude::*;
use crate::hittable::{HitRecord, Hittable};
use crate::scene::Scene;

pub struct Camera {
    position: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    sample_count: u32,
    pixel_width: u32,
    pixel_height: u32
}

impl Camera {
    pub fn new(direction: Ray, aspect_ratio: f32, sample_count: u32, pixel_width: u32) -> Camera {
        let pixel_height: u32 = (pixel_width as f32 / aspect_ratio) as u32;
        let w: Vec3 = direction.get_direction().get_normalized() * -1.0;
        let u: Vec3 = Vec3::cross(&Vec3::new(0.0, 0.0, 1.0), &w).get_normalized();
        let v: Vec3 = Vec3::cross(&w, &u) * -1.0;
        Camera {
            position: *direction.get_origin(),
            u,
            v,
            w,
            sample_count,
            pixel_width,
            pixel_height
        }
    }

    pub fn render(&self, scene: &Scene) -> Result<Image, Box<dyn Error>> {
        let mut image: Image = Image::new(self.pixel_width, self.pixel_height);
        let pixels: Vec<[u8; 4]> = (0..(self.pixel_height * self.pixel_width)).map(|i| {
            let x: u32 = i % self.pixel_width;
            let y: u32 = i / self.pixel_width;
            let ray: Ray = Ray::new(
                self.position,
                (self.w * -1.0) + (self.u * (2.0 * (x as f32 + 0.5) / self.pixel_width as f32 - 1.0)) + (self.v * (2.0 * (y as f32 + 0.5) / self.pixel_height as f32 - 1.0))
            );
            debug!("{:?}", ray);
            let hit: Option<HitRecord> = scene.hit(&ray, 0.0, 1000.0);
            let color: [u8; 4] = match hit {
                Some(hit) => {
                    let normal: Vec3 = *hit.get_normal();
                    let normal: Vec3 = Vec3::new(normal.x.abs(), normal.y.abs(), normal.z.abs());
                    let normal: Vec3 = normal * 255.0;
                    [normal.x as u8, normal.y as u8, normal.z as u8, 255u8]
                },
                None => {
                    [0, 0, 0, 255]
                }
            };
            image.increment_ray_count();
            color
        }).collect::<Vec<[u8; 4]>>();
        image.set_pixels(pixels);
        Ok(image)
    }
}
