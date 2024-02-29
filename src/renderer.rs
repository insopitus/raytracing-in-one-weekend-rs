#[derive(Debug)]
pub struct Camera {
    aspect_ratio: f32,
    pub frame_size: (u32, u32),
    center: Vector3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    pixel_0_loc: Vector3,
    /// near plane
    viewport_size: (f32, f32),
}
impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let frame_width = 1280;
        let frame_height = 720;
        let center = Vector3::ZERO;
        // near plane
        let viewport_size = (2.0*aspect_ratio, 2.0);
        let pixel_size = 2.0 / frame_height as f32;
        let viewport_top_left =
            center + vec3(0.0, 0.0, -1.0) + vec3(-viewport_size.0 / 2.0, viewport_size.1 / 2.0, 0.0);
        let pixel_delta_u = vec3(pixel_size, 0.0, 0.0);
        let pixel_delta_v = vec3(0.0, -pixel_size, 0.0);
        let pixel_0_loc = viewport_top_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        Self {
            aspect_ratio,
            frame_size: (frame_width, frame_height),
            center,
            pixel_0_loc,
            pixel_delta_u,
            pixel_delta_v,
            viewport_size,
        }
    }
    pub fn get_ray_at(&self, u: u32, v: u32) -> Ray {
        let dir =
            self.pixel_0_loc + (u as f32) * self.pixel_delta_u + (v as f32) * self.pixel_delta_v
                - self.center;
        Ray {
            origin: self.center,
            direction: dir.normalize(),
        }
    }
}

use std::{fs::File, io::BufWriter};

use image::ImageError;
use indicatif::ProgressBar;
use lib_rs::{
    color::{rgba, Color},
    linear_algebra::{vector::vec3, Vector3},
    ray::{HitRecord, Hitable, Ray},
};
pub struct Scene {
    geometries: Vec<Box<dyn Hitable>>,
}

impl Scene {
    pub fn new() -> Self {
        Self { geometries: vec![] }
    }
    pub fn ray_cast(&self, ray: Ray) -> Option<HitRecord> {
        let mut iter = self.geometries.iter();
        loop {
            if let Some(s) = iter.next() {
                if let Some(r) = ray.hit(s, 0.0, 5.0) {
                    return Some(r);
                };
            } else {
                break;
            }
        }
        return None;
    }
    pub fn add(&mut self,g:Box<dyn Hitable>){
        self.geometries.push(g);
    }
}

pub struct Renderer<'a> {
    camera: &'a Camera,
    scene: &'a Scene,
}
impl<'a> Renderer<'a> {
    pub fn new(camera: &'a Camera, scene: &'a Scene) -> Self {
        Self { camera, scene }
    }
    pub fn render(&self) -> Result<(), ImageError> {
        let mut pixels =
            Vec::with_capacity((self.camera.frame_size.0 * self.camera.frame_size.1) as usize);
        let bar = ProgressBar::new(pixels.capacity() as u64);
        for j in 0..self.camera.frame_size.1 {
            for i in 0..self.camera.frame_size.0 {
                let ray = self.camera.get_ray_at(i, j);
                let d = self.scene.ray_cast(ray);

                let color = if let Some(r) = d {
                    rgba(
                        r.normal.x * 0.5 + 0.5,
                        r.normal.y * 0.5 + 0.5,
                        r.normal.z * 0.5 + 0.5,
                        1.0,
                    )
                } else {
                    rgba(1.0, 1.0, 1.0, 1.0)
                };

                pixels.push(color);
                bar.inc(1);
            }
        }
        bar.finish();

        let mut buffer: Vec<u8> = Vec::with_capacity(pixels.len() * 4);
        for c in pixels {
            buffer.extend_from_slice(&c.as_rgba8_bytes());
        }
        let mut writer = BufWriter::new(File::create("output.png").unwrap());
        image::write_buffer_with_format(
            &mut writer,
            &buffer,
            self.camera.frame_size.0,
            self.camera.frame_size.1,
            image::ColorType::Rgba8,
            image::ImageFormat::Png,
        )
        .unwrap();
        Ok(())
    }
}
