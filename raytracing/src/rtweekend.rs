use std::f64::consts::PI;
use std::f64;
use rand::Rng;

// Constantes
pub const INFINITYCONST: f64 = f64::INFINITY;
pub const PICONST: f64 = PI;

// Funções utilitárias

/// Converte graus para radianos
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PICONST / 180.0
}

/// Retorna um número real aleatório no intervalo [0, 1)
pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

/// Retorna um número real aleatório no intervalo [min, max)
pub fn random_double_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
