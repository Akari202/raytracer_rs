mod mesh;

use std::clone;
use std::error::Error;
use std::path::Path;
use std::rc::Rc;
use serde::Deserialize;
use crate::hittable::Hittable;
use crate::vec::Vec3;

#[derive(Deserialize, Debug)]
struct Camera {
    name: String,
    from: Vec3,
    to: Vec3,
    pixel_size: [u32; 2],
    samples: u32,
    max_depth: u32
}

#[derive(Deserialize, Debug)]
struct Diffuse {
    name: String,
    color: Vec3,
    albedo: f32
}

#[derive(Deserialize, Debug)]
struct Reflective {
    name: String,
    albedo: Vec3,
    fuzz: f32
}

#[derive(Deserialize, Debug)]
struct Dielectric {
    name: String,
    index: f32,
    color: Vec3
}

#[derive(Deserialize, Debug)]
struct Sphere {
    center: Vec3,
    radius: f32,
    material: String
}

#[derive(Deserialize, Debug)]
struct Plane {
    point: Vec3,
    normal: Vec3,
    material: String
}

#[derive(Deserialize, Debug)]
struct Rectangle {
    point: Vec3,
    width: Vec3,
    height: Vec3,
    material: String
}

#[derive(Deserialize, Debug)]
struct Cube {
    center: Vec3,
    width: Vec3,
    height: Vec3,
    depth: Vec3,
    material: String
}

#[derive(Deserialize, Debug)]
struct Triangle {
    vertices: [Vec3; 3],
    material: String
}

#[derive(Deserialize, Debug)]
struct Mesh {
    path: String,
    material: String,
    translation: Option<Vec3>,
    rotation: Option<Vec3>,
    scale: Option<Vec3>
}

#[derive(Deserialize, Debug)]
struct Objects {
    spheres: Vec<Sphere>,
    planes: Vec<Plane>,
    rectangles: Vec<Rectangle>,
    cubes: Vec<Cube>,
    triangles: Vec<Triangle>,
    meshes: Vec<Mesh>
}

#[derive(Deserialize, Debug)]
struct Materials {
    diffuse: Vec<Diffuse>,
    reflective: Vec<Reflective>,
    dielectric: Vec<Dielectric>
}

#[derive(Deserialize, Debug)]
struct Scene {
    camera: Camera,
    materials: Materials,
    objects: Objects,
}

pub fn load_scene(path: impl AsRef<Path>) -> Result<(crate::camera::Camera, crate::scene::Scene), Box<dyn Error>> {
    let contents: String = std::fs::read_to_string(path)?;
    let scene: Scene = toml::from_str(&contents)?;
    let materials = scene.materials;
    let objects = scene.objects
        .flatten()
        .iter()
        .map(|i| {
            i.build(&materials)
        })
        .collect::<Result<Vec<Box<dyn Hittable>>, Box<dyn Error>>>()?;
    let camera = scene.camera.build()?;
    let mut scene = crate::scene::Scene::new();
    scene.add_objects(objects);
    Ok((camera, scene))
}

impl Objects {
    fn flatten(&self) -> Vec<&dyn HittableEntry> {
        let mut objects: Vec<&dyn HittableEntry> = Vec::new();
        for sphere in &self.spheres {
            objects.push(sphere);
        }
        for plane in &self.planes {
            objects.push(plane);
        }
        for rectangle in &self.rectangles {
            objects.push(rectangle);
        }
        for cube in &self.cubes {
            objects.push(cube);
        }
        for triangle in &self.triangles {
            objects.push(triangle);
        }
        for mesh in &self.meshes {
            objects.push(mesh);
        }
        objects
    }
}

impl Materials {
    fn get(&self, name: &str) -> Option<&dyn MaterialEntry> {
        for diffuse in &self.diffuse {
            if diffuse.name == name {
                return Some(diffuse);
            }
        }
        for reflective in &self.reflective {
            if reflective.name == name {
                return Some(reflective);
            }
        }
        for dielectric in &self.dielectric {
            if dielectric.name == name {
                return Some(dielectric);
            }
        }
        None
    }
}

impl Camera {
    fn build(&self) -> Result<crate::camera::Camera, Box<dyn Error>> {
        Ok(
            crate::camera::Camera::new(
                crate::ray::Ray::from_to(
                    self.from,
                    self.to
                ),
                self.pixel_size[0] as f32 / self.pixel_size[1] as f32,
                self.samples,
                self.max_depth,
                self.pixel_size[0]
            )
        )
    }
}

impl HittableEntry for Sphere {
    fn build(&self, materials: &Materials) -> Result<Box<dyn Hittable>, Box<dyn Error>> {
        Ok(
            Box::new(
                crate::hittable::sphere::Sphere::new(
                    self.center,
                    self.radius,
                    materials.get(&self.material).ok_or("Material not found")?.build()?
                )
            )
        )
    }
}

impl HittableEntry for Plane {
    fn build(&self, materials: &Materials) -> Result<Box<dyn Hittable>, Box<dyn Error>> {
        Ok(Box::new(crate::hittable::plane::Plane::new(
            self.normal,
            self.point,
            materials.get(&self.material).ok_or("Material not found")?.build()?
        )))
    }
}

impl HittableEntry for Rectangle {
    fn build(&self, materials: &Materials) -> Result<Box<dyn Hittable>, Box<dyn Error>> {
        Ok(Box::new(crate::hittable::rectangle::Rectangle::new(
            self.point,
            self.width,
            self.height,
            materials.get(&self.material).ok_or("Material not found")?.build()?
        )))
    }
}

impl HittableEntry for Cube {
    fn build(&self, materials: &Materials) -> Result<Box<dyn Hittable>, Box<dyn Error>> {
        Ok(Box::new(crate::hittable::cube::Cube::new(
            self.center,
            self.width,
            self.height,
            self.depth,
            materials.get(&self.material).ok_or("Material not found")?.build()?
        )))
    }
}

impl HittableEntry for Triangle {
    fn build(&self, materials: &Materials) -> Result<Box<dyn Hittable>, Box<dyn Error>> {
        Ok(Box::new(crate::hittable::triangle::Triangle::new(
            self.vertices[0],
            self.vertices[1],
            self.vertices[2],
            materials.get(&self.material).ok_or("Material not found")?.build()?
        )))
    }
}

impl HittableEntry for Mesh {
    fn build(&self, materials: &Materials) -> Result<Box<dyn Hittable>, Box<dyn Error>> {
        let mut mesh = crate::hittable::mesh::Mesh::load(
            self.path.clone(),
            materials.get(&self.material).ok_or("Material not found")?.build()?,
        )?;
        if let Some(translation) = &self.translation {
            mesh.translate(translation.clone());
        }
        if let Some(rotation) = &self.rotation {
            mesh.rotate(rotation.clone());
        }
        if let Some(scale) = &self.scale {
            mesh.scale(scale.clone());
        }
        Ok(Box::new(mesh))
    }
}

impl MaterialEntry for Diffuse {
    fn build(&self) -> Result<Rc<dyn crate::material::Material>, Box<dyn Error>> {
        Ok(Rc::new(crate::material::diffuse::Diffuse::new(
            self.color,
            self.albedo
        )))
    }
}

impl MaterialEntry for Reflective {
    fn build(&self) -> Result<Rc<dyn crate::material::Material>, Box<dyn Error>> {
        Ok(Rc::new(crate::material::reflective::Reflective::new(
            self.albedo,
            self.fuzz
        )))
    }
}

impl MaterialEntry for Dielectric {
    fn build(&self) -> Result<Rc<dyn crate::material::Material>, Box<dyn Error>> {
        Ok(Rc::new(crate::material::dielectric::Dielectric::new(
            self.index,
            self.color
        )))
    }
}

trait MaterialEntry {
    fn build(&self) -> Result<Rc<dyn crate::material::Material>, Box<dyn Error>>;
}

trait HittableEntry {
    fn build(&self, materials: &Materials) -> Result<Box<dyn Hittable>, Box<dyn Error>>;
}
