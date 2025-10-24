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
use serde::{Deserialize, Serialize};

use crate::{camera::Camera, renderer::Geometry, scene::Scene};

mod camera;
mod parser;
mod renderer;
mod scene;
#[derive(Deserialize)]
struct CameraConfig {
    position: [f32; 3],
    look_at: [f32; 3],
    fov: f32,
    width: u32,
    height: u32,
}
#[derive(Deserialize)]
struct MeshConfig {
    geometry: Geometry,
    material: Material,
}
#[derive(Deserialize)]
struct SceneConfig {
    camera: CameraConfig,
    samples: u32,
    scene: Vec<MeshConfig>,
}

fn main() {
    let scene_config_path = std::env::args()
        .skip(1)
        .next()
        .expect("scene desc is needed.");
    let scene_config: SceneConfig =
        serde_json::from_str(&std::fs::read_to_string(scene_config_path).unwrap()).unwrap();
    let mut camera = Camera::new(
        scene_config.camera.width,
        scene_config.camera.height,
        scene_config.camera.fov,
    );
    camera.move_to(vec3(
        scene_config.camera.position[0],
        scene_config.camera.position[1],
        scene_config.camera.position[2],
    ));
    camera.look_at(vec3(
        scene_config.camera.look_at[0],
        scene_config.camera.look_at[1],
        scene_config.camera.look_at[2],
    ));
    let mut scene = Scene::from_list(
        scene_config
            .scene
            .into_iter()
            .map(|mesh| (mesh.geometry, mesh.material))
            .collect(),
    );
    // let mut scene = Scene::new();

    let renderer = Renderer::new(&camera, &scene, scene_config.samples);
    let time = Instant::now();
    // cornell_box(&mut scene);
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

    use renderer::Geometry;
    scene.add(
        Geometry::Parallelogram(Parallelogram::new(
            vec3(555., 0., 0.),
            vec3(0., 555., 0.),
            vec3(0., 0., 555.),
        )),
        green,
    );
    scene.add(
        Geometry::Parallelogram(Parallelogram::new(
            vec3(0., 0., 0.),
            vec3(0., 555., 0.),
            vec3(0., 0., 555.),
        )),
        red,
    );
    scene.add(
        // Parallelogram::new(vec3(343.,544.,332.),vec3(-130.,0.,0.),vec3(0.,0.,-105.)),
        Geometry::Sphere(Sphere::new(vec3(278.0, 544.0, 278.0), 70.0)),
        light,
    );
    scene.add(
        Geometry::Parallelogram(Parallelogram::new(
            vec3(0., 0., 0.),
            vec3(555., 0., 0.),
            vec3(0., 0., 555.),
        )),
        white,
    );
    scene.add(
        Geometry::Parallelogram(Parallelogram::new(
            vec3(555., 555., 555.),
            vec3(-555., 0., 0.),
            vec3(0., 0., -555.),
        )),
        white,
    );
    scene.add(
        Geometry::Parallelogram(Parallelogram::new(
            vec3(0., 0., 555.),
            vec3(555., 0., 0.),
            vec3(0., 555., 0.),
        )),
        white,
    );

    scene.add(
        Geometry::AxisAlignedBox(AxisAlignedBox::new(
            vec3(130., 0., 65.),
            vec3(295., 165., 230.),
        )),
        white,
    );
    scene.add(
        Geometry::AxisAlignedBox(AxisAlignedBox::new(
            vec3(265., 0., 295.),
            vec3(430., 330., 460.),
        )),
        white,
    );
}

// fn simple_light_scene(scene: &mut Scene) {
//     scene.add(
//         Sphere::new(vec3(0.0, -1000.0, 0.0), 1000.0),
//         Material {
//             kind: MaterialKind::Lambertian,
//             color: Color::WHITE,
//         },
//     );
//     scene.add(
//         Sphere::new(vec3(0.0, 2.0, 0.0), 2.0),
//         Material {
//             kind: MaterialKind::Lambertian,
//             color: Color::WHITE,
//         },
//     );
//     scene.add(
//         Sphere::new(vec3(0.0, 7.0, 0.0), 1.0),
//         Material {
//             kind: MaterialKind::DiffuseLight,
//             color: Color::WHITE,
//         },
//     );
//     scene.add(
//         Parallelogram::new(vec3(3., 1., -2.), vec3(2.0, 0.0, 0.0), vec3(0.0, 2.0, 0.0)),
//         Material {
//             kind: MaterialKind::DiffuseLight,
//             color: rgba(4.0, 4.0, 4.0, 1.0),
//         },
//     )
// }

// fn box_scene(scene: &mut Scene) {
//     scene.add(
//         // left red
//         Parallelogram::new(vec3(-3.0, -2., 5.), vec3(0., 0., -4.), vec3(0., 4., 0.)),
//         Material {
//             kind: MaterialKind::Lambertian,
//             color: rgba(1.0, 0.2, 0.2, 1.0),
//         },
//     );
//     scene.add(
//         // back green
//         Parallelogram::new(vec3(-2., -2., 0.), vec3(4., 0., 0.), vec3(0., 4., 0.)),
//         Material {
//             kind: MaterialKind::Lambertian,
//             color: rgba(0.2, 1.0, 0.2, 1.0),
//         },
//     );
//     scene.add(
//         // right blue
//         Parallelogram::new(vec3(3., -2., 1.), vec3(0., 0., 4.), vec3(0., 4., 0.)),
//         Material {
//             kind: MaterialKind::Lambertian,
//             color: rgba(0.2, 0.2, 1.0, 1.0),
//         },
//     );
//     scene.add(
//         // upper orange
//         Parallelogram::new(vec3(-2., 3., 1.), vec3(4., 0., 0.), vec3(0., 0., 4.)),
//         Material {
//             kind: MaterialKind::Lambertian,
//             color: rgba(1.0, 0.5, 0.0, 1.0),
//         },
//     );

//     scene.add(
//         // lower teal
//         Parallelogram::new(vec3(-2., -3., 5.), vec3(4., 0., 0.), vec3(0., 0., -4.)),
//         Material {
//             kind: MaterialKind::Lambertian,
//             color: rgba(0.2, 0.8, 0.8, 1.0),
//         },
//     );
// }
