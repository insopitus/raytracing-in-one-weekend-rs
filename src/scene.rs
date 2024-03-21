use std::f32::INFINITY;

use lib_rs::ray::{HitRecord, Hitable, Ray};

use crate::renderer::Material;

pub struct Scene {
    entities: Vec<(Box<dyn Hitable + Sync>, Material)>,
}

impl Scene {
    pub fn new() -> Self {
        Self { entities: vec![] }
    }
    pub fn ray_cast(&self, ray: Ray) -> Option<(HitRecord, Material)> {
        let mut iter = self.entities.iter();
        let mut nearest: Option<(HitRecord, Material)> = None;
        while let Some((s, m)) = iter.next() {
            if let Some(r) = ray.hit(s, 0.001..INFINITY) {
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
    pub fn add(&mut self, g: impl Hitable + 'static + Sync, m: Material) {
        self.entities.push((Box::new(g), m));
    }
}
