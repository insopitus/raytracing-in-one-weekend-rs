use lib_rs::ray::{HitRecord, Ray};

use crate::renderer::{Geometry, Material};

pub struct Scene {
    entities: Vec<(Geometry, Material)>,
}

impl Scene {
    pub fn _new() -> Self {
        Self { entities: vec![] }
    }
    pub fn from_list(list: Vec<(Geometry, Material)>) -> Self {
        Self { entities: list }
    }
    pub fn ray_cast(&self, ray: Ray) -> Option<(HitRecord, Material)> {
        let iter = self.entities.iter();
        let mut nearest: Option<(HitRecord, Material)> = None;
        for (s, m) in iter {
            if let Some(r) = s.hit(ray, 0.001..f32::INFINITY) {
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
    pub fn _add(&mut self, g: Geometry, m: Material) {
        self.entities.push((g, m));
    }
}
