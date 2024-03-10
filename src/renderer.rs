use std::{
    f32::INFINITY,
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

pub fn random_vec3(rng: &mut rand::rngs::ThreadRng) -> Vector3 {
    vec3(
        rng.gen_range(-1.0..=1.0),
        rng.gen_range(-1.0..=1.0),
        rng.gen_range(-1.0..=1.0),
    )
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

const VIEWPORT_HEIGHT_3D: f32 = 2.0;

#[derive(Debug)]
pub struct Camera {
    aspect_ratio: f32,
    frame_size: (u32, u32),
    position: Vector3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    pixel_0_loc: Vector3,
    /// near plane
    viewport_size: (f32, f32),
}
impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        let aspect_ratio = width as f32 / height as f32;
        let center = Vector3::ZERO;
        let view_dir = vec3(0.0, 0.0, -1.0);

        // near plane
        let (pixel_0_loc, pixel_delta_u, pixel_delta_v, viewport_size) =
            Self::calc_near_plane_values(center, view_dir, aspect_ratio, (width, height));
        Self {
            aspect_ratio,
            frame_size: (width, height),
            position: center,
            pixel_0_loc,
            pixel_delta_u,
            pixel_delta_v,
            viewport_size,
        }
    }
    fn calc_near_plane_values(
        center: Vector3,
        view_dir: Vector3,
        aspect_ratio: f32,
        frame_size: (u32, u32),
    ) -> (Vector3, Vector3, Vector3, (f32, f32)) {
        let dir = view_dir;
        let left = cross(vec3(0.0, 1.0, 0.0), dir).normalize();
        // up direction of the frame in 3d space;
        let up = cross(dir, left);
        let viewport_size = (VIEWPORT_HEIGHT_3D * aspect_ratio, VIEWPORT_HEIGHT_3D);
        let pixel_size = 2.0 / (frame_size.1 as f32);
        let viewport_top_left = center + dir + viewport_size.0/2.0 * left + viewport_size.1/2.0 * up;
        let pixel_delta_u = pixel_size * -left;
        let pixel_delta_v = pixel_size * -up;
        let pixel_0_loc = viewport_top_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        (pixel_0_loc, pixel_delta_u, pixel_delta_v, viewport_size)
    }
    pub fn move_to(&mut self, pos: Vector3) {
        self.pixel_0_loc = self.pixel_0_loc + (pos - self.position);
        self.position = pos;
    }
    pub fn look_at(&mut self, target: Vector3) {
        let dir = (target - self.position).normalize();
        let (loc_0,delta_u,delta_v,_) = Self::calc_near_plane_values(self.position, dir, self.aspect_ratio, self.frame_size);
        self.pixel_0_loc = loc_0;
        self.pixel_delta_u = delta_u;
        self.pixel_delta_v = delta_v;
        // left direction of the frame in 3d space;
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
            - self.position;

        Ray {
            origin: self.position,
            direction: dir.normalize(),
        }
    }
    pub fn frame_size(&self) -> (u32, u32) {
        self.frame_size
    }
}

pub struct Scene {
    entities: Vec<(Box<dyn Hitable + Sync>, Material)>,
}

impl Scene {
    pub fn new() -> Self {
        Self { entities: vec![] }
    }
    pub fn ray_cast(&self, ray: Ray) -> Option<(HitRecord, Material)> {
        let mut iter = self.entities.iter();
        while let Some((s, m)) = iter.next() {
            if let Some(r) = ray.hit(s, 0.001..INFINITY) {
                return Some((r, *m));
            };
        }
        None
    }
    pub fn add(&mut self, g: impl Hitable + 'static + Sync, m: Material) {
        self.entities.push((Box::new(g), m));
    }
}

pub struct Renderer<'a> {
    camera: &'a Camera,
    scene: &'a Scene,
    samples: u32,
    light_direction: Vector3,
}
impl<'a> Renderer<'a> {
    pub fn new(camera: &'a Camera, scene: &'a Scene, samples: u32) -> Self {
        Self {
            camera,
            scene,
            samples,
            light_direction: vec3(1.0, 1.0, 1.0).normalize(),
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
                let (scatter, ray_out, attenuation) = material.scatter(&ray, &record, rng);
                // let n = record.normal;
                // rgba(n.x+1.0,n.y+1.0,n.z+1.0,2.0)*0.5
                if scatter {
                    self.ray_color(ray_out, max_depth, rng) * attenuation
                } else {
                    rgba(0.0, 0.0, 0.0, 1.0)
                }
            };

            color.a = 1.0;
            color
        } else {
            let d = ray.direction;
            let a = 0.5 * (d.y + 1.0);
            mix(rgba(1.0, 1.0, 1.0, 1.0), rgba(0.5, 0.7, 1.0, 1.0), a)
        }
    }
    pub fn render(&self) -> Vec<Color> {
        let mut pixels =
            Vec::with_capacity((self.camera.frame_size.0 * self.camera.frame_size.1) as usize);
        let bar = ProgressBar::new(pixels.capacity() as u64);
        use rayon::prelude::*;

        for j in 0..self.camera.frame_size.1 {
            for i in 0..self.camera.frame_size.0 {
                let accu_color: Color = (0..self.samples)
                    .into_par_iter()
                    .map(|_| {
                        let mut rng = rand::thread_rng();
                        let ray = self.camera.get_ray_at(i, j, &mut rng);
                        let color = self.ray_color(ray, 10, &mut rng);
                        color
                    })
                    .sum::<Color>()
                    / self.samples as f32;

                pixels.push(accu_color);
                bar.inc(1);
            }
        }
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
            self.camera.frame_size.0,
            self.camera.frame_size.1,
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

#[derive(Clone, Copy)]
pub enum MaterialKind {
    Lambertian,
    Metal { fuzz: f32 },
    Dielectric { fraction_rate: f32 },
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
                (hit_record.normal + random_vec3(rng).normalize()).normalize(),
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
        };
        (scattered, Ray::new(hit_record.point, dir))
    }
}
#[derive(Clone, Copy)]
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
}
