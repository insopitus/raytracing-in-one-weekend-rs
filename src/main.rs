use std::{
    fs::File,
    io::BufWriter,
    thread,
    time::{Duration, Instant},
};

use lib_rs::{
    color::{rgba, Color},
    geometry::{self, Sphere},
    linear_algebra::{vector::vec3, Vector3},
    ray::Ray,
};
use renderer::{Material, MaterialKind, Renderer, Scene};

use crate::renderer::Camera;
mod renderer;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() {
    let bar = indicatif::ProgressBar::new((WIDTH * HEIGHT) as u64);
    let camera = Camera::new();
    // let sphere1 = Sphere::new(vec3(0.0, 0.0, -1.0), 0.5);
    // let sphere1 = Sphere::new(vec3(0.0, -100.5, -1.0), 100.0);
    // let mut geometries = vec![sphere];
    let mut scene = Scene::new();

    scene.add(
        Sphere::new(vec3(0.0, 0.0, -1.0), 0.5),
        Material {
            kind: MaterialKind::Lambertian,
            color: rgba(0.7, 0.3, 0.3, 1.0),
        },
    );
    scene.add(
        Sphere::new(vec3(-1.0, 0.0, -1.0), 0.5),
        Material {
            kind: MaterialKind::Metal,
            color: rgba(0.8, 0.8, 0.8, 1.0),
        },
    );
    scene.add(
        Sphere::new(vec3(1.0, 0.0, -1.0), 0.5),
        Material {
            kind: MaterialKind::Metal,
            color: rgba(0.8, 0.6, 0.2, 1.0),
        },
    );
    // ground needs to be added last; ray only hit one target
    scene.add(
        Sphere::new(vec3(0.0, -100.5, -1.0), 100.0),
        Material {
            kind: MaterialKind::Lambertian,
            color: rgba(0.8, 0.8, 0.0, 1.0),
        },
    );
    let renderer = Renderer::new(&camera, &scene);
    let time = Instant::now();
    let pixels = renderer.render();
    println!("Time {} secs.", time.elapsed().as_secs_f32());
    let writer = BufWriter::new(File::create("output.png").unwrap());
    renderer.write(&pixels, writer);
}
