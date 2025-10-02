mod ray;
// mod hitable;
// mod material;
// mod sphere;
// mod camera;

// use std::f32;
// use nalgebra::Vector3;
// use rand::Rng;
// use rayon::prelude::*;
// use crate::ray::Ray;
// use crate::material::{Lambertian, Metal, Dielectric};
// use crate::hitable::{Hitable, HitableList};
// use crate::sphere::Sphere;
// use crate::camera::Camera;

use itertools::Itertools;
use std::{fmt::format, fs, io};
use indicatif::ProgressIterator;
use glam::DVec3;


const ASPECT_RADIO: f64 = 16.0 / 9.0;
// Constantes para definir as dimensões da imagem e valor máximo de cor
const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGTH: u32 = ((IMAGE_WIDTH as f64 / ASPECT_RADIO) as u32);



const MAX_VALUE: u8 = 255;





fn main() -> io::Result<()> {
    // Gera uma imagem gradiente usando o produto cartesiano de coordenadas y e x
    let pixels: String = (0..IMAGE_HEIGTH)
        .cartesian_product(0..IMAGE_WIDTH) // Cria todas as combinações possíveis de (y, x)
        .progress_count(
            IMAGE_HEIGTH as u64 * IMAGE_WIDTH as u64
        ) 
        .map(|(y, x)| {
            // Calcula os valores RGB normalizados (0.0 a 1.0) baseados na posição do pixel
            let red: f64 = x as f64 / (IMAGE_HEIGTH as f64 - 1.0);   // Gradiente horizontal (vermelho)
            let green: f64 = y as f64 / (IMAGE_HEIGTH as f64 - 1.0); // Gradiente vertical (verde)
            let blue: f64 = 0.0;                                      // Sem componente azul
            
            // Converte os valores normalizados para a escala 0-255 e formata como string
            format!("{} {} {}", red * 255.0, green * 255.0, blue * 255.0)
        })
        // Agrupa os pixels em linhas (chunks) de acordo com a largura da imagem
        .chunks(IMAGE_WIDTH as usize)
        .into_iter()
        // Junta cada linha de pixels com espaços e depois junta todas as linhas com quebras de linha
        .map(|chunk| chunk.into_iter().join(" "))
        .join("\n");
    
    // Exibe os pixels no console (para debug)
    //println!("{}", pixels);
    
    // Escreve a imagem no formato PPM (Portable Pixmap Format)
    // P3 = formato ASCII, seguido por largura, altura, valor máximo e dados dos pixels
    fs::write(
        "output.ppm",
        format!(
            "P3 
{IMAGE_WIDTH} {IMAGE_HEIGTH} 
{MAX_VALUE} 
{pixels}
"
        ),
    )?;
    Ok(())
}
