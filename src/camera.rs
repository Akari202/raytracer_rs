use std::error::Error;
use std::time::{Duration, Instant};
use log::{debug, info};
use crate::image::Image;
use crate::vec::Vec3;
use crate::ray::Ray;
use rayon::prelude::*;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Color;
use crate::scene::Scene;

pub struct Camera {
    position: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    sample_count: u32,
    max_depth: u32,
    far_clip: f32,
    pixel_width: u32,
    pixel_height: u32
}

const GAMMA: f32 = 1.5;

impl Camera {
    pub fn new(direction: Ray, aspect_ratio: f32, sample_count: u32, max_depth: u32, pixel_width: u32) -> Camera {
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
            max_depth,
            far_clip: 500.0,
            pixel_width,
            pixel_height
        }
    }

    fn ray_color(&self, scene: &Scene, ray: &Ray, depth: u32) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let hit: Option<HitRecord> = scene.hit(&ray, 0.001, self.far_clip);
        match hit {
            Some(hit) => {
                let mut color: Color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.sample_count {
                    if let Some((scattered, attenuation)) = hit.get_material().scatter(&ray, &hit) {
                        color = color + self.ray_color(scene, &scattered, depth - 1) * attenuation;
                    }
                }
                color * GAMMA
            },
            None => {
                let a: f32 = (ray.get_direction().z + 1.0) / 2.0;
                Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
            }
        }
    }

    pub fn render(&self, scene: &Scene) -> Result<Image, Box<dyn Error>> {
        info!("Rendering {}x{} image with {} samples per pixel", self.pixel_width, self.pixel_height, self.sample_count);
        info!("Scene has {} objects", scene.get_object_count());
        let perf_start: Instant = Instant::now();

        let mut image: Image = Image::new(self.pixel_width, self.pixel_height);
        let pixels: Vec<[u8; 4]> = (0..(self.pixel_height * self.pixel_width)).map(|i| {
            let x: u32 = i % self.pixel_width;
            let y: u32 = i / self.pixel_width;
            let colors: Vec<Color> = (0..self.sample_count).into_iter().map(|_| {
                let ray: Ray;
                if self.sample_count > 1 {
                    ray = Ray::new(
                        self.position,
                        (self.w * -1.0) + (self.u * (2.0 * (x as f32 + rand::random::<f32>()) / self.pixel_width as f32 - 1.0)) + (self.v * (2.0 * (y as f32 + rand::random::<f32>()) / self.pixel_height as f32 - 1.0))
                    );
                } else {
                    ray = Ray::new(
                        self.position,
                        (self.w * -1.0) + (self.u * (2.0 * (x as f32 + 0.5) / self.pixel_width as f32 - 1.0)) + (self.v * (2.0 * (y as f32 + 0.5) / self.pixel_height as f32 - 1.0))
                    );
                }
                debug!("{:?}", ray);
                self.ray_color(scene, &ray, self.max_depth)
            }).collect();
            image.jump_ray_count(self.sample_count);
            if x == 0 && y % 100 == 0 {
                info!("Rendered row {} of {}", y, self.pixel_height);
            }
            // accumulate and average color samples
            let mut color: Color = Color::new(0.0, 0.0, 0.0);
            for c in colors {
                color = color + c;
            }
            color = color / self.sample_count as f32;
            [
                (color.x * 255.0) as u8,
                (color.y * 255.0) as u8,
                (color.z * 255.0) as u8,
                255
            ]
        }).collect::<Vec<[u8; 4]>>();
        image.set_pixels(pixels);

        let perf_time: Duration = perf_start.elapsed();
        info!("Cast {} rays in {:?} averaging {:?} per ray and {:?} per pixel", image.get_ray_count(), perf_time, perf_time / image.get_ray_count(), perf_time / (self.pixel_width * self.pixel_height));
        Ok(image)
    }

    pub fn render_and_save(&self, scene: &Scene, path: &str) -> Result<(), Box<dyn Error>> {
        let mut image: Image = self.render(scene)?;
        image.save_as_png(path)?;
        Ok(())
    }
}
