use std::{f32::INFINITY, fs::File, io::BufWriter};

use image::ImageError;
use indicatif::ProgressBar;
use lib_rs::{
    color::{self, mix, rgba, Color}, geometry::Sphere, linear_algebra::{
        vector::{dot, vec3},
        Vector3,
    }, ray::{HitRecord, Hitable, Ray}
};
use rand::Rng;

pub fn random_vec3(rng: &mut rand::rngs::ThreadRng) -> Vector3 {
    vec3(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0))
}
// pub fn random_vec3_in_unit_sphere(rng:&mut rand::rngs::ThreadRng)->Vector3{
//    loop {
//         let p = random_vec3(rng);
//         if p.length_squared() < 1.0{
//             return p;

//         }
//     }
// }
pub fn random_vec3_min_max(rng: &mut rand::rngs::ThreadRng, min: f32, max: f32) -> Vector3 {
    vec3(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}
pub fn random_vec3_on_semisphere(rng: &mut rand::rngs::ThreadRng, normal: Vector3) -> Vector3 {
    let dir = random_vec3(rng).normalize();
    if dot(dir, normal) >= 0.0 {
        dir
    } else {
        -dir
    }
}

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
        let viewport_size = (2.0 * aspect_ratio, 2.0);
        let pixel_size = 2.0 / frame_height as f32;
        let viewport_top_left = center
            + vec3(0.0, 0.0, -1.0)
            + vec3(-viewport_size.0 / 2.0, viewport_size.1 / 2.0, 0.0);
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
    pub fn get_ray_at(&self, u: u32, v: u32, rng: &mut rand::rngs::ThreadRng) -> Ray {
        // let dir =
        //     self.pixel_0_loc + (u as f32) * self.pixel_delta_u + (v as f32) * self.pixel_delta_v
        //         - self.center;
        // randomized version (multisample anti-alias)
        use rand::Rng;
        let rand_x = rng.gen::<f32>() - 0.5;
        let rand_y = rng.gen::<f32>() - 0.5;
        let dir = self.pixel_0_loc
            + (u as f32 + rand_x) * self.pixel_delta_u
            + (v as f32 + rand_y) * self.pixel_delta_v
            - self.center;

        Ray {
            origin: self.center,
            direction: dir.normalize(),
        }
    }
}

pub struct Scene {
    geometries: Vec<Sphere>,
}

impl Scene {
    pub fn new() -> Self {
        Self { geometries: vec![] }
    }
    pub fn ray_cast(&self, ray: Ray) -> Option<HitRecord> {
        let mut iter = self.geometries.iter();
        while let Some(s) = iter.next() {
            if let Some(r) = ray.hit(*s, 0.001..INFINITY) {
                return Some(r);
            };
        }
        None
    }
    pub fn add(&mut self, g: Sphere) {
        self.geometries.push(g);
    }
}

pub struct Renderer<'a> {
    camera: &'a Camera,
    scene: &'a Scene,
    light_direction: Vector3,
}
impl<'a> Renderer<'a> {
    pub fn new(camera: &'a Camera, scene: &'a Scene) -> Self {
        Self {
            camera,
            scene,
            light_direction: vec3(1.0, 1.0, 1.0).normalize(),
        }
    }
    pub fn ray_color(&self, ray: Ray, rng: &mut rand::rngs::ThreadRng) -> Color {
        let record: Option<HitRecord> = self.scene.ray_cast(ray);

        if let Some(record) = record {
            // let n = record.normal;
            // rgba(n.x+1.0,n.y+1.0,n.z+1.0,2.0)*0.5
            let dir = random_vec3_on_semisphere(rng, record.normal);
            (self.ray_color(Ray::new(record.point, dir), rng) * 0.5f32).set_a(1.0)
        } else {
            let d = ray.direction;
            let a = 0.5 * (d.y + 1.0);
            mix(rgba(1.0, 1.0, 1.0, 1.0),rgba(0.5, 0.7, 1.0, 1.0),a)
        }
    }
    pub fn render(&self) -> Result<(), ImageError> {
        let mut pixels =
            Vec::with_capacity((self.camera.frame_size.0 * self.camera.frame_size.1) as usize);
        let bar = ProgressBar::new(pixels.capacity() as u64);
        let mut rng = rand::thread_rng();
        const SAMPLES: usize = 12;
        for j in 0..self.camera.frame_size.1 {
            for i in 0..self.camera.frame_size.0 {
                let mut accu_color = rgba(0.0, 0.0, 0.0, 1.0);
                for _ in 0..SAMPLES {
                    let ray = self.camera.get_ray_at(i, j, &mut rng);
                    let color = self.ray_color(ray, &mut rng);
                    accu_color += color;
                }
                accu_color /= SAMPLES as f32;

                pixels.push(accu_color);
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
