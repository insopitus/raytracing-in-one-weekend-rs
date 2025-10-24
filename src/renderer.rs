use std::{
    f32::{
        consts::{PI, TAU},
        INFINITY,
    },
    fs::File,
    io::{self, BufWriter},
};

use image::ImageError;
use indicatif::ProgressBar;
use lib_rs::{
    color::{self, mix, rgba, Color},
    geometry::Sphere,
    linear_algebra::{
        vector::{cross, dot, vec3},
        Vector3,
    },
    ray::{HitRecord, Hitable, Ray},
};
use rand::Rng;
use serde::Deserialize;

use crate::{camera::Camera, scene::Scene};

pub fn random_vec3(rng: &mut rand::rngs::ThreadRng) -> Vector3 {
    vec3(
        rng.gen_range(-1.0..=1.0),
        rng.gen_range(-1.0..=1.0),
        rng.gen_range(-1.0..=1.0),
    )
}
pub fn random_vec3_on_unit_sphere(rng: &mut rand::rngs::ThreadRng) -> Vector3 {
    let theta = rng.gen_range(0.0..=TAU);
    let phi = rng.gen_range(0.0..PI);
    let sin_phi = phi.sin();

    vec3(sin_phi * theta.cos(), sin_phi * theta.sin(), phi.cos())
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

pub struct Renderer<'a> {
    camera: &'a Camera,
    scene: &'a Scene,
    samples: u32,
    light_direction: Vector3,
    background: Color,
}
impl<'a> Renderer<'a> {
    pub fn new(camera: &'a Camera, scene: &'a Scene, samples: u32) -> Self {
        Self {
            camera,
            scene,
            samples,
            light_direction: vec3(1.0, 1.0, 1.0).normalize(),
            background: Color::BLACK,
        }
    }
    pub fn ray_color(&self, ray: Ray, max_depth: u32, rng: &mut rand::rngs::ThreadRng) -> Color {
        if max_depth == 0 {
            return rgba(0.0, 0.0, 0.0, 1.0);
        }
        if let Some((record, material)) = self.scene.ray_cast(ray) {
            const DEBUG_NORMAL: bool = false;
            let mut color = if DEBUG_NORMAL {
                let n = record.normal;
                rgba(n.x, n.y, n.z, 1.0)
            } else {
                // color from scatter
                let (scatter, ray_out, attenuation) = material.scatter(&ray, &record, rng);
                // color from emmision
                let emmision_color = material.emit();
                // let n = record.normal;
                // rgba(n.x+1.0,n.y+1.0,n.z+1.0,2.0)*0.5
                let scatter_color = if scatter {
                    self.ray_color(ray_out, max_depth, rng) * attenuation
                } else {
                    self.background
                };
                scatter_color + emmision_color
            };

            color.a = 1.0;
            color
        } else {
            Color::BLACK
        }
    }
    pub fn render(&self) -> Vec<Color> {
        let pixels_count = (self.camera.frame_size().0 * self.camera.frame_size().1) as usize;
        let bar = ProgressBar::new(pixels_count as u64);
        use rayon::prelude::*;
        let mut positions = Vec::with_capacity(pixels_count);
        for j in 0..self.camera.frame_size().1 {
            for i in 0..self.camera.frame_size().0 {
                positions.push((i, j));
            }
        }
        let pixels = positions
            .into_par_iter()
            .map(|(i, j)| {
                let accu_color: Color = (0..self.samples)
                    .into_iter()
                    .map(|_| {
                        let mut rng = rand::thread_rng();
                        let ray = self.camera.get_ray_at(i, j, &mut rng);
                        let color = self.ray_color(ray, 10, &mut rng);
                        color
                    })
                    .sum::<Color>()
                    / self.samples as f32;
                bar.inc(1);
                accu_color
            })
            .collect();

        bar.finish();

        pixels
    }
    pub fn write(&self, pixels: &Vec<Color>, mut writer: impl io::Write + io::Seek) {
        let mut buffer: Vec<u8> = Vec::with_capacity(pixels.len() * 4);
        for c in pixels {
            buffer.extend_from_slice(&c.linear_to_gamma(2.2).as_rgba8_bytes());
        }
        // let mut writer = BufWriter::new(File::create("output.png").unwrap());
        image::write_buffer_with_format(
            &mut writer,
            &buffer,
            self.camera.frame_size().0,
            self.camera.frame_size().1,
            image::ColorType::Rgba8,
            image::ImageFormat::Png,
        )
        .unwrap();
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powf(5.0)
}

#[derive(Clone, Copy, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MaterialKind {
    Lambertian,
    Metal { fuzz: f32 },
    Dielectric { fraction_rate: f32 },
    DiffuseLight,
}
impl MaterialKind {
    pub fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        rng: &mut rand::rngs::ThreadRng,
    ) -> (bool, Ray) {
        let (scattered, dir) = match self {
            MaterialKind::Lambertian => (
                true,
                (hit_record.normal + random_vec3_on_unit_sphere(rng)).normalize(),
            ),
            MaterialKind::Metal { fuzz } => {
                let reflected = ray_in.direction.reflect(hit_record.normal).normalize();
                let dir = (reflected + *fuzz * random_vec3(rng)).normalize();
                let scattered = dot(dir, hit_record.normal) > 0.0;
                (scattered, dir)
            }
            MaterialKind::Dielectric { fraction_rate } => {
                let refraction_ratio = if hit_record.front_face {
                    1.0 / fraction_rate
                } else {
                    *fraction_rate
                };
                let cos_theta = dot(-ray_in.direction, hit_record.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let direction = if refraction_ratio * sin_theta > 1.0
                    || reflectance(cos_theta, refraction_ratio) > rng.gen()
                {
                    ray_in.direction.reflect(hit_record.normal)
                } else {
                    ray_in
                        .direction
                        .refract(hit_record.normal, refraction_ratio)
                };

                (true, direction.normalize())
            }
            MaterialKind::DiffuseLight => (false, ray_in.direction),
        };
        (scattered, Ray::new(hit_record.point, dir))
    }
    pub fn emit(&self, color: Color) -> Color {
        match self {
            MaterialKind::DiffuseLight => color,
            _ => rgba(0.0, 0.0, 0.0, 1.0),
        }
    }
}
#[derive(Clone, Copy, Deserialize, Debug)]
pub struct Material {
    pub kind: MaterialKind,
    pub color: Color,
}
impl Material {
    pub fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        rng: &mut rand::rngs::ThreadRng,
    ) -> (bool, Ray, Color) {
        let (scatter, mut ray_out) = self.kind.scatter(ray_in, hit_record, rng);
        let ray_out_dir = ray_out.direction;
        let epsilon = 1e-8;
        if ray_out_dir.x.abs() < epsilon
            && ray_out_dir.y.abs() < epsilon
            && ray_out_dir.z.abs() < epsilon
        {
            ray_out.direction = hit_record.normal;
        }
        (scatter, ray_out, self.color)
    }
    pub fn emit(&self) -> Color {
        self.kind.emit(self.color)
    }
}
