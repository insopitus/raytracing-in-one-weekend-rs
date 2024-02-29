use std::{thread, time::Duration};

use lib_rs::{
    color::{rgba, Color},
    geometry::{self, Sphere},
    linear_algebra::{vector::vec3, Vector3},
    ray::Ray,
};
mod image;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() {
    let bar = indicatif::ProgressBar::new((WIDTH * HEIGHT) as u64);
    let camera = Vector3::ZERO;
    let u_dir = Vector3::UNIT_X;
    let v_dir = -Vector3::UNIT_Y;
    let focal_len = 1.0;
    let aspect_ratio: f32 = 16.0 / 9.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    // width of a pixel
    let delta_u = u_dir * viewport_width / (WIDTH as f32);
    // height of a pixel
    let delta_v = v_dir * viewport_height / (HEIGHT as f32);
    // center of the top left pixel
    let top_left_ray_dir = camera + vec3(0.0, 0.0, focal_len)
        - u_dir * viewport_width / 2.0
        - v_dir * viewport_height / 2.0
        + delta_u / 2.0
        + delta_v / 2.0;
    dbg!(top_left_ray_dir);
    let sphere1 = Sphere::new(vec3(0.0, 0.0, 1.0), 0.5);
    let sphere2 = Sphere::new(vec3(0.0, -100.5, 1.0), 100.0);
    let geometies = vec![sphere1, sphere2];
    // let mut geometries = vec![sphere];
    let mut image =
        crate::image::Image::new(WIDTH, HEIGHT, Vec::with_capacity((WIDTH * HEIGHT) as usize));
    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let ray_dir =
                (top_left_ray_dir + (i as f32) * delta_u + (j as f32) * delta_v).normalize();
            let ray = Ray::new(camera, ray_dir);
            let d = ray.hit(sphere1, 0.0, 5.0);
            let mut iter = geometies.iter();

            let r = loop {
                if let Some(s) = iter.next() {
                    if let Some(r) = ray.hit(*s, 0.0, 5.0) {
                        let normal = r.normal;
                        break Some(rgba(
                            normal.x * 0.5 + 0.5,
                            normal.y * 0.5 + 0.5,
                            normal.z * 0.5 + 0.5,
                            1.0,
                        ));
                    };
                } else {
                    break None;
                }
            };

            let color = r.unwrap_or(rgba(1.0, 1.0, 1.0, 1.0));

            image.push_pixel(color);
            bar.inc(1);
        }
    }
    image.write_to_file();
    bar.finish();
}
