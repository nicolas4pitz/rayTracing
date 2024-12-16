use std::sync::Arc;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vecry::{Point3, dot};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self { center, radius: radius.max(0.0), mat }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - *r.origin();
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
        if !ray_t.surronunds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surronunds(root) {
                return false; // No valid root within range
            }
        }

        rec.time = root;
        rec.p = r.at(rec.time);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = Some(self.mat.clone());

        true
    }
}