mod ray;

use glam::DVec3;
use indicatif::ProgressIterator;
use itertools::Itertools;
use std::{fmt::format, fs, io};

use crate::ray::Ray;

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

    let pixel_delta_u: DVec3 = VIEWPORT_U / IMAGE_WIDTH as f64;
    let pixel_delta_v: DVec3 = VIEWPORT_V / IMAGE_HEIGTH as f64;

    let viewport_upper_left: DVec3 = CAMERA_CENTER
        - DVec3::new(0., 0., FOCAL_LENGTH)
        - VIEWPORT_U / 2.
        - VIEWPORT_V / 2.;
    
    let pixel100_loc: DVec3 = viewport_upper_left + 0.5 * (pixel_delta_u * pixel_delta_v);

    // Gera uma imagem gradiente usando o produto cartesiano de coordenadas y e x
    let pixels: String = (0..IMAGE_HEIGTH)
        .cartesian_product(0..IMAGE_WIDTH) // Cria todas as combinações possíveis de (y, x)
        .progress_count(IMAGE_HEIGTH as u64 * IMAGE_WIDTH as u64)
        .map(|(y, x)| {
            
            let pixel_center: DVec3 = pixel100_loc + (x as f64 * pixel_delta_u) + (y as f64 * pixel_delta_v);
            let ray_direction: DVec3 = pixel_center - CAMERA_CENTER;

            let ray = Ray{
                origin: CAMERA_CENTER,
                direction: ray_direction,
            };

            let pixel_color = ray.color() * 255.0;

            // Converte os valores normalizados para a escala 0-255 e formata como string
            format!("{} {} {}", pixel_color.x, pixel_color.y, pixel_color.z)
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


fn hit_sphere(center: &DVec3, radius: f64, ray: &Ray) -> bool{
    let distanceOriginCenter:DVec3 = ray.origin - *center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * distanceOriginCenter.dot(ray.direction);
    let c = distanceOriginCenter.dot(distanceOriginCenter) - radius * radius;
    let discriminant = b * b - 4. * a *c;
    discriminant >= 0.

}
