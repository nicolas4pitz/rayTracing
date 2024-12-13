use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vecry::Vec3;

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    /// Cria uma nova lista de objetos hittable vazia.
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }

    /// Cria uma nova lista de objetos hittable com um objeto inicial.
    pub fn with_object(object: Arc<dyn Hittable>) -> Self {
        let mut list = Self::new();
        list.add(object);
        list
    }

    /// Limpa todos os objetos da lista.
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    /// Adiciona um objeto à lista.
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
  fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
    let mut temp_rec = HitRecord::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0, false);
    let mut hit_anything = false;
    let mut closest_so_far = ray_t.get_max();

    for object in &self.objects {
        if object.hit(r, Interval::new(ray_t.get_min(), closest_so_far),&mut temp_rec) {
            hit_anything = true;
            closest_so_far = temp_rec.time;
            *rec = temp_rec.clone();
        }
    }

    hit_anything
}
}