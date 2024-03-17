use std::{
    fs::File,
    io::BufWriter,
    thread,
    time::{Duration, Instant},
};

use lib_rs::{
    color::{rgba},
    geometry::{self, AxisAlignedBox, Sphere},
    linear_algebra::{vector::vec3,},
};
use renderer::{Material, MaterialKind, Renderer, Scene};

use crate::camera::Camera;

mod renderer;
mod camera;



fn main() {
    // let bar = indicatif::ProgressBar::new((WIDTH * HEIGHT) as u64);
    let mut camera = Camera::new(1280, 760);
    camera.move_to(vec3(0.8,0.4,0.4));
    camera.look_at(vec3(0.0,0.0,-0.5));
    // let sphere1 = Sphere::new(vec3(0.0, 0.0, -1.0), 0.5);
    // let sphere1 = Sphere::new(vec3(0.0, -100.5, -1.0), 100.0);
    // let mut geometries = vec![sphere];
    let mut scene = Scene::new();
    // return;
    // glass ball
    // scene.add(
    //     Sphere::new(vec3(-0.3, -0.2, -0.8), 0.2),
    //     Material {
    //         kind: MaterialKind::Dielectric {
    //             fraction_rate: 1.5,
    //         },
    //         color: rgba(1., 1., 1., 1.0),
    //     },
    // );

    scene.add(
        Sphere::new(vec3(0.0, 0.0, 0.0), 0.5),
        Material {
            kind: MaterialKind::Metal { fuzz: 0.2 },
            color: rgba(0.7, 0.3, 0.3, 1.0),
        },
    );
    // // metal ball
    // scene.add(
    //     Sphere::new(vec3(1.0, 0.0, -1.0), 0.5),
    //     Material {
    //         kind: MaterialKind::Metal { fuzz: 0.2 },
    //         color: rgba(0.8, 0.8, 0.8, 1.0),
    //     },
    // );

    scene.add(
        AxisAlignedBox::new(
            vec3(0.4, -0.2, -0.8),
            vec3(0.8, 0.2, -0.4),
        ),
        Material {
            kind: MaterialKind::Dielectric { fraction_rate: 1.5 },
            color: rgba(1.0, 1.0, 1.0, 1.0),
        },
    );
    scene.add(
        Sphere::new(vec3(0.0, 0.0, -2.0), 0.5),
        Material {
            kind: MaterialKind::Lambertian,
            color: rgba(0.4, 0.8, 0.3, 1.0),
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
    let renderer = Renderer::new(&camera, &scene, 250);
    let time = Instant::now();
    let pixels = renderer.render();
    println!("Time {} secs.", time.elapsed().as_secs_f32());
    let writer = BufWriter::new(File::create("output.png").unwrap());
    renderer.write(&pixels, writer);
}
