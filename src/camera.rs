use lib_rs::{linear_algebra::{vector::{cross, vec3}, Vector3}, ray::Ray};

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
