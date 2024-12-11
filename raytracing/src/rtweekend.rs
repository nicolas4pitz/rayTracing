use std::f64::consts::PI; // Para constantes matemáticas
use std::f64; // Para acessar infinity
use rand::Rng;

pub const INFINITYCONST: f64 = f64::INFINITY;
pub const PICONST: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
  degrees * PI / 180.0
}

pub fn random_double() -> f64 {
  let mut rng = rand::thread_rng();
  rng.gen::<f64>()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
  let mut rng = rand::thread_rng();
  rng.gen_range(min..max) 
}