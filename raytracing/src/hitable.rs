use std::ops::Range;

use glam::DVec3;
use rand::{ Rng};

use crate::ray::random_unit_vector;

// use nalgebra::Vector3;
use crate::{ray::Ray, sphere::Sphere};
// use crate::material::Material;

// pub struct HitRecord<'a> {
//     pub t: f32,
//     pub p: Vector3<f32>,
//     pub normal: Vector3<f32>,
//     pub material: &'a dyn Material
// }

pub trait Hittable {
    fn hit(
        &self, 
        ray: &Ray,
        interval: Range<f64>, 
        //t_min: f32, 
        //t_max: f32
    ) -> Option<HitRecord>;
}


#[non_exhaustive]
#[derive(Clone)]
pub enum Material {
  Lambertian { albedo: DVec3 },
  Metal { albedo: DVec3, fuzz: f64 },
  Dielectric { index_of_refraction: f64 }
}

pub struct Scattered {
  pub attenuation: DVec3,
  pub scattered: Ray,
}

impl Material {
  pub fn scatter(&self, r_in: &Ray, hit_record: HitRecord) -> Option<Scattered>{
    match self {
      Material::Lambertian { albedo } =>{
        let mut scatter_direction = hit_record.normal + random_unit_vector();

        if scatter_direction.abs_diff_eq(DVec3::new(0., 0., 0.), 1e-8,){
          scatter_direction = hit_record.normal;
        }

        let scattered = Ray {
          origin: hit_record.point,
          direction: scatter_direction,
        };

        Some(Scattered { attenuation: *albedo, scattered})
      }

      Material::Metal { albedo, fuzz } => {
        let reflected: DVec3 = reflect(r_in.direction.normalize(), hit_record.normal,);

        let scattered = Ray {
          origin: hit_record.point, direction: reflected + *fuzz * random_unit_vector(),
        };

        if scattered.direction.dot(hit_record.normal) > 0. {
          Some(Scattered { attenuation: *albedo, scattered, })
        } else{
          None
        }

      }

      Material::Dielectric { index_of_refraction } => {
        let attenuation = DVec3::splat(1.0);

        let mut rng = rand::rng();

        let ri:f64;

        if hit_record.front_face {
          ri = 1.0/index_of_refraction;
        } else{
          ri = *index_of_refraction;
        }

        let unit_direction: DVec3 = r_in.direction.normalize();
        // let refracted = refract(unit_direction, hit_record.normal, ri);

        let cos_theta: f64 = (-unit_direction).dot(hit_record.normal).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract: bool = ri * sin_theta > 1.0;

        let direction: DVec3;

        if cannot_refract || reflectance(cos_theta, ri) > rng.random::<f64>() {
          direction = reflect(unit_direction, hit_record.normal);
        } else{
          direction = refract(unit_direction, hit_record.normal, ri);
        };


        Some(Scattered { attenuation, scattered: Ray {
          origin: hit_record.point,
          direction: direction
        } })
      }

      _ => None,

    }
  }
}


fn refract(uv: DVec3, n: DVec3, etai_over_tat: f64) -> DVec3{
  let cos_theta: f64 = -((uv).dot(n).min(1.0));
  let r_out_perp: DVec3 = etai_over_tat * (uv + cos_theta * n);
  let r_out_parallel: DVec3 = ((1.0 - r_out_perp.length_squared()).abs()).sqrt() * n;
  return r_out_perp + r_out_parallel;
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64{
  let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
  r0 = r0 * r0;
  return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
}


#[derive(Clone)]
pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    t: f64,
    front_face: bool,
    pub material: Material
}

impl HitRecord {
    fn with_face_normal(point: DVec3, outward_normal: DVec3, t: f64, ray: &Ray, material: Material) -> Self {
        let (front_face, normal) = HitRecord::calc_face_normal(ray, &outward_normal);
        HitRecord {
            point,
            normal,
            t,
            front_face,
            material
        }
    }

    fn calc_face_normal(ray: &Ray, outward_normal: &DVec3) -> (bool, DVec3) {
        let front_face: bool = ray.direction.dot(*outward_normal) < 0.;
        let normal: DVec3 = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
        (front_face, normal)
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &Ray, 
        interval: Range<f64>,
        //t_min: f32, 
        //t_max: f32
    ) -> Option<HitRecord> {
        let distance_origin_center: DVec3 = ray.origin - self.center;
        let a: f64 = ray.direction.length_squared();
        let half_b: f64 = distance_origin_center.dot(ray.direction);
        //let b = 2.0 * distanceOriginCenter.dot(ray.direction);
        let c: f64 = distance_origin_center.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !interval.contains(&root) {
            root = (-half_b + sqrtd) / a;
            if !interval.contains(&root) {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;

        let rec: HitRecord = HitRecord::with_face_normal(point, outward_normal, t, ray, self.material.clone());

        Some(rec)
    }
}

pub struct HitableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HitableList {
    fn clear(&mut self) {
        self.objects = vec![]
    }

    pub fn add<T>(&mut self, object: T)
    where
        T: Hittable + 'static,
    {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HitableList {
    fn hit(&self, ray: &Ray, interval: Range<f64>) -> Option<HitRecord> {
        let (_closest, hit_record) = self.objects.iter().fold((interval.end, None), |acc, item| {
            if let Some(temp_rec) = item.hit(ray, interval.start..acc.0) {
                (temp_rec.t, Some(temp_rec))
            } else {
                acc
            }
        });

        hit_record
    }
}


fn reflect(v: DVec3, n: DVec3) -> DVec3{
  return v-2. * v.dot(n)*n;
}