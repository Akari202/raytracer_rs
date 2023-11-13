#![allow(dead_code, unused_imports)]
use std::error::Error;
use std::path::Path;
use std::time::{Duration, Instant};
use log::{info, LevelFilter};
use notify::{Event, EventHandler, recommended_watcher, Watcher};
use notify::RecursiveMode::NonRecursive;
use rayon::prelude::*;
use crate::camera::Camera;
use crate::config::load_scene;
use crate::hittable::cube::Cube;
use crate::hittable::plane::Plane;
use crate::hittable::rectangle::Rectangle;
use crate::hittable::sphere::Sphere;
use crate::image::Image;
use crate::material::Color;
use crate::material::diffuse::Diffuse;
use crate::material::reflective::Reflective;
use crate::ray::Ray;
use crate::scene::{Hittable, Scene};
use crate::vec::Vec3;

mod vec;
mod util;
mod image;
mod ray;
mod camera;
mod scene;
mod hittable;
mod material;
mod config;

struct FileRender {
    path: String
}

impl FileRender {
    fn new(path: impl AsRef<Path>) -> FileRender {
        FileRender {
            path: path.as_ref().to_str().unwrap().to_owned()
        }
    }
}

impl EventHandler for FileRender {
    fn handle_event(&mut self, _event: notify::Result<Event>) {
        let (camera, scene): (Camera, Scene) = load_scene(self.path.clone()).expect("Failed to load scene");
        let name = format!("{}.png", self.path.split(".").next().unwrap());
        camera.render_and_save(&scene, &name).expect("Failed to render scene");
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let path = "test_scene.toml";
    info!("Watching for changes to {}", path);
    let renderer = FileRender::new(path);
    let mut watcher = recommended_watcher(renderer)?;
    watcher.watch(path.as_ref(), NonRecursive)?;

    loop {}

    // Ok(())
}
