use std::error::Error;
use log::{debug, info};
use crate::image::Image;
use crate::vec::Vec3;
use crate::ray::Ray;
use crate::scene::{HitRecord, Hittable, Scene};
use rayon::prelude::*;

pub struct Camera {
    position: Vec3,
    upper_left_corner: Vec3,
    lower_right_corner: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    sample_count: u32,
    pixel_width: u32,
    pixel_height: u32
}

impl Camera {
    pub fn new(direction: Ray, vertical_fov: f32, aspect_ratio: f32, sample_count: u32, pixel_height: u32) -> Camera {
        let theta: f32 = vertical_fov.to_radians();
        let half_height: f32 = (theta / 2.0).tan();
        let half_width: f32 = aspect_ratio * half_height;
        let w: Vec3 = *direction.get_direction();
        let u: Vec3 = w.cross(&Vec3::new(0.0, 1.0, 0.0)).get_normalized();
        let v: Vec3 = u.cross(&w);
        let position: Vec3 = *direction.get_origin();
        let upper_left_corner: Vec3 = (position - u * half_width + v * half_height - w).get_normalized();
        let lower_right_corner: Vec3 = (position + u * half_width - v * half_height - w).get_normalized();
        Camera {
            position,
            upper_left_corner,
            lower_right_corner,
            u,
            v,
            w,
            sample_count,
            pixel_width: (pixel_height as f32 * aspect_ratio) as u32,
            pixel_height
        }
    }

    pub fn render(&self, scene: &Scene) -> Result<Image, Box<dyn Error>> {
        let mut image: Image = Image::new(self.pixel_width, self.pixel_height);
        let x_step_size: f32 = (self.lower_right_corner - self.upper_left_corner).get_length() / self.pixel_width as f32;
        let y_step_size: f32 = (self.upper_left_corner - self.lower_right_corner).get_length() / self.pixel_height as f32;
        for i in 0..self.pixel_width {
            for j in 0..self.pixel_height {
                let ray: Ray = Ray::new(
                    self.position,
                    (self.upper_left_corner + self.u * (i as f32 * x_step_size) - self.v * (j as f32 * y_step_size) - self.position).get_normalized()
                );
                debug!("{:?}", ray);
                let hit: Option<HitRecord> = scene.hit(&ray, 0.0, 1000.0);
                let color: [u8; 4] = match hit {
                    None => [128, 172, 255, 255],
                    Some(hit) => {
                        debug!("{:?}", hit);
                        [0, 0, 0, 255]
                    }
                };
                image.set_pixel(i, j, color);
                image.increment_ray_count();
            }
        }
        Ok(image)
    }
}
