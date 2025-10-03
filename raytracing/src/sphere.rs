// use nalgebra::Vector3;
// use crate::ray::Ray;
// use crate::hitable::{Hitable, HitRecord};
// use crate::material::Material;

use glam::DVec3;

pub struct Sphere {
    pub center: DVec3,
    pub radius: f64,
}

// impl<M: Material> Sphere<M> {
//     pub fn new(center: Vector3<f32>, radius: f32, material: M) -> Self { Sphere {center, radius, material} }
// }

// impl<M: Material> Hitable for Sphere<M> {
//     fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
//         let oc: nalgebra::Matrix<f32, nalgebra::Const<3>, nalgebra::Const<1>, nalgebra::ArrayStorage<f32, 3, 1>> = ray.origin() - self.center;
//         let a: f32 = ray.direction().dot(&ray.direction());
//         let b: f32 = oc.dot(&ray.direction());
//         let c: f32 = oc.dot(&oc) - self.radius.powi(2);
//         let discriminant = b.powi(2) - a * c;
//         if discriminant > 0.0 {
//             let sqrt_discriminant = discriminant.sqrt();
//             let t: f32 = (-b - sqrt_discriminant) / a;
//             if t < t_max && t > t_min {
//                 let p: nalgebra::Matrix<f32, nalgebra::Const<3>, nalgebra::Const<1>, nalgebra::ArrayStorage<f32, 3, 1>> = ray.point_at_parameter(t);
//                 let normal: nalgebra::Matrix<f32, nalgebra::Const<3>, nalgebra::Const<1>, nalgebra::ArrayStorage<f32, 3, 1>> = (p - self.center) / self.radius;
//                 return Some(HitRecord { t, p, normal, material: &self.material })
//             }
//             let t: f32 = (-b + sqrt_discriminant) / a;
//             if t < t_max && t > t_min {
//                 let p: nalgebra::Matrix<f32, nalgebra::Const<3>, nalgebra::Const<1>, nalgebra::ArrayStorage<f32, 3, 1>> = ray.point_at_parameter(t);
//                 let normal: nalgebra::Matrix<f32, nalgebra::Const<3>, nalgebra::Const<1>, nalgebra::ArrayStorage<f32, 3, 1>> = (p - self.center) / self.radius;
//                 return Some(HitRecord { t, p, normal, material: &self.material })
//             }
//         }
//         None
//     }
// }
