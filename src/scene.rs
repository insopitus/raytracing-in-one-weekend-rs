use std::f32::INFINITY;

use lib_rs::ray::{HitRecord, Hitable, Ray};

use crate::renderer::{Geometry, Material};

pub struct Scene {
    entities: Vec<(Geometry, Material)>,
}

impl Scene {
    pub fn new() -> Self {
        Self { entities: vec![] }
    }
    pub fn from_list(list: Vec<(Geometry, Material)>) -> Self {
        Self { entities: list }
    }
    pub fn ray_cast(&self, ray: Ray) -> Option<(HitRecord, Material)> {
        let mut iter = self.entities.iter();
        let mut nearest: Option<(HitRecord, Material)> = None;
        while let Some((s, m)) = iter.next() {
            if let Some(r) = s.hit(ray, 0.001..INFINITY) {
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
    pub fn add(&mut self, g: Geometry, m: Material) {
        self.entities.push((g, m));
    }
}
