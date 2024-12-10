use std::f64::consts::PI; // Para constantes matemáticas
use std::f64; // Para acessar infinity

pub const INFINITYCONST: f64 = f64::INFINITY;
pub const PICONST: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
  degrees * PI / 180.0
}