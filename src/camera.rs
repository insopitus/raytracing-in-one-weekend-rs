use std::f32::{self, consts::PI};

use lib_rs::{linear_algebra::{vector::{cross, vec3}, Vector3}, ray::Ray};


#[derive(Debug)]
pub struct Camera {
    aspect_ratio: f32,
    frame_size: (u32, u32),
    position: Vector3,
    pixel_delta_u: Vector3,
    /// distance per pixel on v axis
    pixel_delta_v: Vector3,
    /// center of the top left pixel
    pixel_0_loc: Vector3,
    // vertical field of view in degrees
    fov:f32,
    /// near plane
    viewport_size: (f32, f32),
}
impl Camera {
    pub fn new(width: u32, height: u32, fov:f32) -> Self {
        let aspect_ratio = width as f32 / height as f32;
        let center = Vector3::ZERO;
        let view_dir = vec3(0.0, 0.0, -1.0);

        // near plane
        let (pixel_0_loc, pixel_delta_u, pixel_delta_v, viewport_size) =
            Self::calc_near_plane_values(center, view_dir, aspect_ratio, fov,(width, height));
        Self {
            aspect_ratio,
            frame_size: (width, height),
            position: center,
            pixel_0_loc,
            pixel_delta_u,
            pixel_delta_v,
            viewport_size,
            fov
        }
    }

    fn calc_near_plane_values(
        center: Vector3,
        view_dir: Vector3,
        aspect_ratio: f32,
        fov:f32,
        frame_size: (u32, u32),
    ) -> (Vector3, Vector3, Vector3, (f32, f32)) {
        let dir = view_dir;
        let left = cross(vec3(0.0, 1.0, 0.0), dir).normalize();
        // up direction of the frame in 3d space;
        let up = cross(dir, left);
        const NEAR:f32 = 1.0;
        let viewport_height = 2.0*NEAR* (fov*0.5/180.0*PI).tan();
        let viewport_size = (viewport_height * aspect_ratio, viewport_height);
        let pixel_size = viewport_height / (frame_size.1 as f32);// distance per pixel
        let viewport_top_left = center + dir*NEAR + viewport_size.0/2.0 * left + viewport_size.1/2.0 * up;
        let pixel_delta_u = pixel_size * -left; 
        let pixel_delta_v = pixel_size * -up;
        // center of the top left pixel
        let pixel_0_loc = viewport_top_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        (pixel_0_loc, pixel_delta_u, pixel_delta_v, viewport_size)
    }
    pub fn move_to(&mut self, pos: Vector3) {
        self.pixel_0_loc = self.pixel_0_loc + (pos - self.position);
        self.position = pos;
    }
    pub fn look_at(&mut self, target: Vector3) {
        let dir = (target - self.position).normalize();
        let (loc_0,delta_u,delta_v,_) = Self::calc_near_plane_values(self.position, dir, self.aspect_ratio, self.fov,self.frame_size);
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
