use crate::ray::Ray;
use crate::vecry::{Vec3, dot};
use crate::interval::Interval;

#[derive(Clone, Debug)]
pub struct HitRecord {
    pub p: Vec3,            // Ponto da interseção
    pub normal: Vec3,       // Vetor normal na interseção
    pub time: f64,          // Parâmetro t da equação da reta
    pub front_face: bool,   // Se a interseção está voltada para frente
}

pub trait Hittable {
  fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    /// Cria um novo registro de hit com valores padrão.
    pub fn new(p: Vec3, normal: Vec3, time: f64, front_face: bool) -> Self {
      Self {
          p,
          normal,
          time,
          front_face,
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