use std::ops::Sub;
use crate::vecry::{dot};

use crate::ray::Ray;
use crate::Vec3;
use crate::Point3;
use crate::hittable::{HitRecord, Hittable};

pub struct Sphere {
  center: Point3,
  radius: f64,
}

impl Hittable for Sphere {
  fn hit(&self,r: &Ray,ray_tmin: f64,ray_tmax: f64,rec: &mut HitRecord,) -> bool {
    let oc: Vec3 = r.origin().sub(self.center);

    let a: f64 = r.direction().length_squared();
    let h: f64 = dot(r.direction(), &oc);
    let c: f64 = oc.length_squared() - self.radius * self.radius;

    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
        false; // Não há interseção
    }

    let sqrtd = discriminant.sqrt();

    // Encontra a raiz mais próxima dentro do intervalo aceitável
    let mut root = (h - sqrtd) / a;
    if root < ray_tmin || root > ray_tmax {
      root = (h + sqrtd) / a;
      if root < ray_tmin || root > ray_tmax {
        return false;
      }
    }

    rec.time = root;
    rec.p = r.at(rec.time);
    rec.normal = (rec.p - self.center) / self.radius;

    true
  }
}