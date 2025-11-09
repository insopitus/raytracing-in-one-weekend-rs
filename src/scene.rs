use lib_rs::{
    linear_algebra::Transform,
    ray::{HitRecord, Ray},
};

use crate::renderer::{Geometry, Material};

pub struct Scene {
    /// entity defined as geometry,material,and rotation_y(will be transform in the future)
    entities: Vec<(Geometry, Material, Option<Transform>)>,
}

impl Scene {
    pub fn _new() -> Self {
        Self { entities: vec![] }
    }
    pub fn from_list(list: Vec<(Geometry, Material, Option<Transform>)>) -> Self {
        Self { entities: list }
    }
    pub fn ray_cast(&self, ray: Ray) -> Option<(HitRecord, Material)> {
        let iter = self.entities.iter();
        let mut nearest: Option<(HitRecord, Material)> = None;
        for (s, m, t) in iter {
            if let Some(r) = s.hit(ray, 0.001..f32::INFINITY, *t) {
                if let Some((near, _)) = nearest {
                    if r.t < near.t {
                        nearest = Some((r, *m));
                    }
                } else {
                    nearest = Some((r, *m))
                }
            };
        }
        nearest
    }
    // pub fn _add(&mut self, g: Geometry, m: Material) {
    //     self.entities.push((g, m));
    // }
}
