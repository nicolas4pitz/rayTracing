mod ray;
mod hitable;
mod sphere;

use glam::DVec3;
use indicatif::ProgressIterator;
use itertools::Itertools;
use std::{fs, io, vec};

use crate::{hitable::HitableList, ray::Ray, sphere::Sphere};

const ASPECT_RADIO: f64 = 16.0 / 9.0;
// Constantes para definir as dimensões da imagem e valor máximo de cor
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGTH: u32 = if ((IMAGE_WIDTH as f64 / ASPECT_RADIO) as u32) < 1 {
    1
} else {
    (IMAGE_WIDTH as f64 / ASPECT_RADIO) as u32
}; //talvez reverter

const VIEWPORT_HEIGTH: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGTH * (IMAGE_WIDTH as f64 / IMAGE_HEIGTH as f64);

const FOCAL_LENGTH: f64 = 1.0;
const CAMERA_CENTER: DVec3 = DVec3::ZERO;

const VIEWPORT_U: DVec3 = DVec3::new(VIEWPORT_WIDTH, 0., 0.);
const VIEWPORT_V: DVec3 = DVec3::new(0., -VIEWPORT_HEIGTH, 0.);

const MAX_VALUE: u8 = 255;

fn main() -> io::Result<()> {

  let mut world = HitableList {objects: vec![]};

  world.add(Sphere {
    center: DVec3::new(0.0, 0.0, -1.0),
    radius: 0.5,
  });

  world.add(Sphere {
    center: DVec3::new(0.0, -100.5, -1.),
    radius: 100.,
  });

    let pixel_delta_u: DVec3 = VIEWPORT_U / IMAGE_WIDTH as f64;
    let pixel_delta_v: DVec3 = VIEWPORT_V / IMAGE_HEIGTH as f64;

    let viewport_upper_left: DVec3 = CAMERA_CENTER
        - DVec3::new(0., 0., FOCAL_LENGTH)
        - VIEWPORT_U / 2.
        - VIEWPORT_V / 2.;
    
    let pixel100_loc: DVec3 = viewport_upper_left + 0.5 * (pixel_delta_u * pixel_delta_v);

    // Gera uma imagem gradiente usando o produto cartesiano de coordenadas y e x
    let pixels: String = (0..IMAGE_HEIGTH)
        .cartesian_product(0..IMAGE_WIDTH)  // Para cada pixel (y,x)
        .progress_count(IMAGE_HEIGTH as u64 * IMAGE_WIDTH as u64)                // Barra de progresso
        .map(|(y, x)| {
            // 1. Calcular posição 3D deste pixel no viewport
            let pixel_center: DVec3 = pixel100_loc 
                + (x as f64 * pixel_delta_u)    // Move horizontalmente
                + (y as f64 * pixel_delta_v);   // Move verticalmente
            
            // 2. Criar raio da câmera através deste pixel
            let ray_direction: DVec3 = pixel_center - CAMERA_CENTER;
            let ray = Ray{
                origin: CAMERA_CENTER,
                direction: ray_direction,
            };
            
            // 3. Calcular cor baseada no que o raio "vê"
            let pixel_color = ray.color(&world) * 255.0;
            
            // 4. Formatar como RGB
            // Converte os valores normalizados para a escala 0-255 e formata como string
            format!("{} {} {}", pixel_color.x, pixel_color.y, pixel_color.z)
        })
        // Agrupa os pixels em linhas (chunks) de acordo com a largura da imagem
        .chunks(IMAGE_WIDTH as usize)
        .into_iter()
        // Junta cada linha de pixels com espaços e depois junta todas as linhas com quebras de linha
        .map(|chunk| chunk.into_iter().join(" "))
        .join("\n");

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



