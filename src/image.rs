use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<[u8; 4]>,
    rays: u32
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            width,
            height,
            pixels: vec![[0, 0, 0, 255]; (width * height) as usize],
            rays: 0
        }
    }

    fn correct_gamma(&mut self) {
        for pixel in &mut self.pixels {
            pixel[0] = (pixel[0] as f32).sqrt() as u8;
            pixel[1] = (pixel[1] as f32).sqrt() as u8;
            pixel[2] = (pixel[2] as f32).sqrt() as u8;
            pixel[3] = 255;
        }
    }

    pub fn save_as_png(&mut self, path: impl AsRef<std::path::Path>) -> Result<(), Box<dyn Error>>{
        // self.correct_gamma();
        let mut buffer: Vec<u8> = Vec::new();
        for pixel in &self.pixels {
            buffer.push(pixel[0]);
            buffer.push(pixel[1]);
            buffer.push(pixel[2]);
            buffer.push(pixel[3]);
        }
        let file: File = File::create(path)?;
        let w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&buffer)?;
        Ok(())
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: [u8; 4]) {
        self.pixels[(y * self.width + x) as usize] = color;
    }

    pub fn increment_ray_count(&mut self) {
        self.rays += 1;
    }

    pub fn set_ray_count(&mut self, rays: u32) {
        self.rays = rays;
    }

    pub fn jump_ray_count(&mut self, rays: u32) {
        self.rays += rays;
    }

    pub fn get_ray_count(&self) -> u32 {
        self.rays
    }

    pub fn set_pixels(&mut self, pixels: Vec<[u8; 4]>) {
        self.pixels = pixels;
    }
}

