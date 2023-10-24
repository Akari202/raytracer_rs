#![allow(dead_code, unused_imports)]
use std::error::Error;
use std::time::{Duration, Instant};
use log::{info, LevelFilter};
use viuer::print_from_file;
use rayon::prelude::*;
use crate::camera::Camera;
use crate::hittable::sphere::Sphere;
use crate::image::Image;
use crate::ray::Ray;
use crate::scene::Hittable;
use crate::vec::Vec3;

mod vec;
mod util;
mod image;
mod ray;
mod camera;
mod scene;
mod hittable;
mod color;

fn main() -> Result<(), Box<dyn Error>>{
    env_logger::builder().filter_level(LevelFilter::Info).init();
    // env_logger::init();
    let perf_start = Instant::now();

    let camera = Camera::new(
        Ray::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0)
        ),
        90.0,
        16.0 / 9.0,
        1,
        512
    );
    let mut world = scene::Scene::new();
    // world.add_objects(vec![
    //     Box::new(Sphere::new(Vec3::new(0.0, 30.0, 75.0), 5.0)),
    //     Box::new(Sphere::new(Vec3::new(0.0, 15.0, 75.0), 5.0)),
    //     Box::new(Sphere::new(Vec3::new(0.0, 0.0, 75.0), 5.0)),
    //     Box::new(Sphere::new(Vec3::new(0.0, -15.0, 75.0), 5.0)),
    //     Box::new(Sphere::new(Vec3::new(0.0, -30.0, 75.0), 5.0))
    // ]);
    for i in 0..10 {
        world.add_object(
            Box::new(
                Sphere::new(
                    Vec3::new(0.0, 60.0 - 30.0 * i as f32, 75.0),
                    5.0
                )
            )
        );
    }
    let image: Image = camera.render(&world)?;
    let image_filename: &str = "test.png";
    image.save_as_png(image_filename)?;

    let perf_time: Duration = perf_start.elapsed();
    info!("Cast {} rays in {:?} averaging {:?} per ray", image.get_ray_count(), perf_time, perf_time / image.get_ray_count());

    // print_from_file(image_filename, &viuer::Config{..Default::default()})?;
    Ok(())
}
