use std::{thread, time::Duration};

use lib_rs::{
    color::{rgba, Color},
    geometry::{self, Sphere},
    linear_algebra::{vector::vec3, Vector3},
    ray::Ray,
};
use renderer::{Renderer, Scene};

use crate::renderer::Camera;
mod renderer;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() {
    let bar = indicatif::ProgressBar::new((WIDTH * HEIGHT) as u64);
    let camera = Camera::new();
    dbg!(&camera);
    let sphere1 = Sphere::new(vec3(0.0, 0.0, -1.0), 0.5);
    let sphere2 = Sphere::new(vec3(0.0, -100.5, -1.0), 100.0);
    // let mut geometries = vec![sphere];
    let mut scene = Scene::new();
    scene.add(Box::new(sphere1));
    scene.add(Box::new(sphere2));
    let renderer = Renderer::new(&camera,&scene);
    renderer.render();
}
