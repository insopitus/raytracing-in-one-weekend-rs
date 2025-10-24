use std::{
    fs::File,
    io::BufWriter,
    thread,
    time::{Duration, Instant},
};

use lib_rs::{
    color::{rgba, Color},
    geometry::{self, AxisAlignedBox, Parallelogram, Sphere},
    linear_algebra::vector::vec3,
};
use renderer::{Material, MaterialKind, Renderer};

use crate::{camera::Camera, scene::Scene};

mod camera;
mod parser;
mod renderer;
mod scene;

// fn main() {
//     // let bar = indicatif::ProgressBar::new((WIDTH * HEIGHT) as u64);
//     let mut camera = Camera::new(1280, 760,90.0);
//     camera.move_to(vec3(0.8, 0.4, 0.4));
//     camera.look_at(vec3(0.0, 0.0, -0.5));
//     // let sphere1 = Sphere::new(vec3(0.0, 0.0, -1.0), 0.5);
//     // let sphere1 = Sphere::new(vec3(0.0, -100.5, -1.0), 100.0);
//     // let mut geometries = vec![sphere];
//     let mut scene = Scene::new();
//     // return;
//     // glass ball
//     // scene.add(
//     //     Sphere::new(vec3(-0.3, -0.2, -0.8), 0.2),
//     //     Material {
//     //         kind: MaterialKind::Dielectric {
//     //             fraction_rate: 1.5,
//     //         },
//     //         color: rgba(1., 1., 1., 1.0),
//     //     },
//     // );

//     scene.add(
//         Sphere::new(vec3(0.0, 0.0, 0.0), 0.5),
//         Material {
//             kind: MaterialKind::Metal { fuzz: 0.2 },
//             color: rgba(0.7, 0.3, 0.3, 1.0),
//         },
//     );
//     // // metal ball
//     // scene.add(
//     //     Sphere::new(vec3(1.0, 0.0, -1.0), 0.5),
//     //     Material {
//     //         kind: MaterialKind::Metal { fuzz: 0.2 },
//     //         color: rgba(0.8, 0.8, 0.8, 1.0),
//     //     },
//     // );

//     scene.add(
//         AxisAlignedBox::new(vec3(0.4, -0.2, -0.8), vec3(0.8, 0.2, -0.4)),
//         Material {
//             kind: MaterialKind::Dielectric { fraction_rate: 1.5 },
//             color: rgba(1.0, 1.0, 1.0, 1.0),
//         },
//     );
//     scene.add(
//         Sphere::new(vec3(0.0, 0.0, -2.0), 0.5),
//         Material {
//             kind: MaterialKind::Lambertian,
//             color: rgba(0.4, 0.8, 0.3, 1.0),
//         },
//     );

//     // ground needs to be added last; ray only hit one target
//     scene.add(
//         Sphere::new(vec3(0.0, -100.5, -1.0), 100.0),
//         Material {
//             kind: MaterialKind::Lambertian,
//             color: rgba(0.8, 0.8, 0.0, 1.0),
//         },
//     );
//     let renderer = Renderer::new(&camera, &scene, 50);
//     let time = Instant::now();
//     let pixels = renderer.render();
//     println!("Time {} secs.", time.elapsed().as_secs_f32());
//     let writer = BufWriter::new(File::create("output.png").unwrap());
//     renderer.write(&pixels, writer);
// }

fn main() {
    // let bar = indicatif::ProgressBar::new((WIDTH * HEIGHT) as u64);
    let mut camera = Camera::new(640, 640, 40.0);
    camera.move_to(vec3(278.0, 278.0, -800.0));
    camera.look_at(vec3(278.0, 278.0, 0.0));
    // let sphere1 = Sphere::new(vec3(0.0, 0.0, -1.0), 0.5);
    // let sphere1 = Sphere::new(vec3(0.0, -100.5, -1.0), 100.0);
    // let mut geometries = vec![sphere];
    let mut scene = Scene::new();

    cornell_box(&mut scene);
    // simple_light_scene(&mut scene);
    // box_scene(&mut scene);

    let renderer = Renderer::new(&camera, &scene, 500);
    let time = Instant::now();
    let pixels = renderer.render();
    println!("Time {} secs.", time.elapsed().as_secs_f32());
    let writer = BufWriter::new(File::create("output.png").unwrap());
    renderer.write(&pixels, writer);
}

fn cornell_box(scene: &mut Scene) {
    let white = Material {
        kind: MaterialKind::Lambertian,
        color: rgba(0.73, 0.73, 0.73, 1.0),
    };
    let red = Material {
        kind: MaterialKind::Lambertian,
        color: rgba(0.65, 0.05, 0.05, 1.0),
    };
    let green = Material {
        kind: MaterialKind::Lambertian,
        color: rgba(0.12, 0.45, 0.15, 1.0),
    };
    let light = Material {
        kind: MaterialKind::DiffuseLight,
        color: rgba(15., 15., 15., 1.0),
    };
    let metal = Material {
        kind: MaterialKind::Metal { fuzz: 0.0 },
        color: Color::WHITE,
    };
    scene.add(
        Parallelogram::new(vec3(555., 0., 0.), vec3(0., 555., 0.), vec3(0., 0., 555.)),
        green,
    );
    scene.add(
        Parallelogram::new(vec3(0., 0., 0.), vec3(0., 555., 0.), vec3(0., 0., 555.)),
        red,
    );
    scene.add(
        // Parallelogram::new(vec3(343.,544.,332.),vec3(-130.,0.,0.),vec3(0.,0.,-105.)),
        Sphere::new(vec3(278.0, 544.0, 278.0), 70.0),
        light,
    );
    scene.add(
        Parallelogram::new(vec3(0., 0., 0.), vec3(555., 0., 0.), vec3(0., 0., 555.)),
        white,
    );
    scene.add(
        Parallelogram::new(
            vec3(555., 555., 555.),
            vec3(-555., 0., 0.),
            vec3(0., 0., -555.),
        ),
        white,
    );
    scene.add(
        Parallelogram::new(vec3(0., 0., 555.), vec3(555., 0., 0.), vec3(0., 555., 0.)),
        white,
    );

    scene.add(
        AxisAlignedBox::new(vec3(130., 0., 65.), vec3(295., 165., 230.)),
        white,
    );
    scene.add(
        AxisAlignedBox::new(vec3(265., 0., 295.), vec3(430., 330., 460.)),
        white,
    );
}

fn simple_light_scene(scene: &mut Scene) {
    scene.add(
        Sphere::new(vec3(0.0, -1000.0, 0.0), 1000.0),
        Material {
            kind: MaterialKind::Lambertian,
            color: Color::WHITE,
        },
    );
    scene.add(
        Sphere::new(vec3(0.0, 2.0, 0.0), 2.0),
        Material {
            kind: MaterialKind::Lambertian,
            color: Color::WHITE,
        },
    );
    scene.add(
        Sphere::new(vec3(0.0, 7.0, 0.0), 1.0),
        Material {
            kind: MaterialKind::DiffuseLight,
            color: Color::WHITE,
        },
    );
    scene.add(
        Parallelogram::new(vec3(3., 1., -2.), vec3(2.0, 0.0, 0.0), vec3(0.0, 2.0, 0.0)),
        Material {
            kind: MaterialKind::DiffuseLight,
            color: rgba(4.0, 4.0, 4.0, 1.0),
        },
    )
}

fn box_scene(scene: &mut Scene) {
    scene.add(
        // left red
        Parallelogram::new(vec3(-3.0, -2., 5.), vec3(0., 0., -4.), vec3(0., 4., 0.)),
        Material {
            kind: MaterialKind::Lambertian,
            color: rgba(1.0, 0.2, 0.2, 1.0),
        },
    );
    scene.add(
        // back green
        Parallelogram::new(vec3(-2., -2., 0.), vec3(4., 0., 0.), vec3(0., 4., 0.)),
        Material {
            kind: MaterialKind::Lambertian,
            color: rgba(0.2, 1.0, 0.2, 1.0),
        },
    );
    scene.add(
        // right blue
        Parallelogram::new(vec3(3., -2., 1.), vec3(0., 0., 4.), vec3(0., 4., 0.)),
        Material {
            kind: MaterialKind::Lambertian,
            color: rgba(0.2, 0.2, 1.0, 1.0),
        },
    );
    scene.add(
        // upper orange
        Parallelogram::new(vec3(-2., 3., 1.), vec3(4., 0., 0.), vec3(0., 0., 4.)),
        Material {
            kind: MaterialKind::Lambertian,
            color: rgba(1.0, 0.5, 0.0, 1.0),
        },
    );

    scene.add(
        // lower teal
        Parallelogram::new(vec3(-2., -3., 5.), vec3(4., 0., 0.), vec3(0., 0., -4.)),
        Material {
            kind: MaterialKind::Lambertian,
            color: rgba(0.2, 0.8, 0.8, 1.0),
        },
    );
}
