use crate::{vecry::{Point3, Vec3}, Ray};

pub struct HitRecord {
  pub p: Point3,
  pub normal: Vec3,
  pub time: f64,
}

pub trait Hittable {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

