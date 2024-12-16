use std::sync::Arc;
use crate::ray::Ray;
use crate::vecry::{Vec3, Point3, dot};
use crate::interval::Interval;
use crate::material::Material;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<Arc<dyn Material>>,
    pub time: f64,
    pub front_face: bool,
}

impl HitRecord {
    /// Cria um novo registro de hit com valores padrão.
    pub fn new(p: Vec3, normal: Vec3, time: f64, front_face: bool) -> Self {
        Self {
            p,
            normal,
            time,
            front_face,
            mat: None,
        }
    }

    /// Define a orientação do vetor normal com base no raio e no vetor normal externo.
    /// O vetor normal será ajustado para apontar na direção oposta ao raio se necessário.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // Define se a face está voltada para frente
        self.front_face = dot(r.direction(), &outward_normal) < 0.0;

        // Ajusta o vetor normal
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}