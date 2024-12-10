use std::ops::Sub;

use crate::ray::Ray;
use crate::vecry::{Point3, dot};
use crate::hittable::{HitRecord, Hittable};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere{
  fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
    let oc = r.origin().sub(self.center);
    let a = r.direction().length_squared();
    let h = dot(r.direction(), &oc);
    let c = oc.length_squared() - self.radius * self.radius;

    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
        return false; // No intersection
    }

    let sqrtd = discriminant.sqrt();

    // Find the nearest root that lies within the acceptable range
    let mut root = (h - sqrtd) / a;
    if root <= ray_tmin || ray_tmax <= root {
        root = (h + sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            return false; // No valid root within range
        }
    }

    rec.time = root;
    rec.p = r.at(rec.time);
    rec.normal = (rec.p - self.center) / self.radius;

    true
}
}