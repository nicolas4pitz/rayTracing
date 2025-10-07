// use std::f32;
// use nalgebra::Vector3;
// use rand::Rng;
// use crate::ray::Ray;

// fn random_in_unit_disk() -> Vector3<f32> {
//     let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
//     let unit: nalgebra::Matrix<f32, nalgebra::Const<3>, nalgebra::Const<1>, nalgebra::ArrayStorage<f32, 3, 1>> = Vector3::new(1.0, 1.0, 0.0);
//     loop {
//         let p: nalgebra::Matrix<f32, nalgebra::Const<3>, nalgebra::Const<1>, nalgebra::ArrayStorage<f32, 3, 1>> = 2.0 * Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0) - unit;
//         if p.dot(&p) < 1.0 {
//             return p
//         }
//     }
// }

use std::fs;

use std::{io};

use glam::DVec3;
use indicatif::ProgressIterator;
use itertools::Itertools;
use rand::Rng;

use crate::{hitable::Hittable, ray::Ray};

pub struct Camera {
    image_width: u32,
    image_heigth: u32,
    max_value: u8,
    aspect_radio: f64,
    center: DVec3,
    pixel_delta_u: DVec3,
    pixel_delta_v: DVec3,
    //viewport_upper_left: DVec3,
    pixel100_loc: DVec3,
    samples_per_pixel: u32,
    max_depth: u32,
}

impl Camera {
    pub fn new(image_width: u32, aspect_radio: f64) -> Self {

        let max_value: u8 = 255;

        // Constantes para definir as dimensões da imagem e valor máximo de cor
        
        let image_heigth: u32 = if ((image_width as f64 / aspect_radio) as u32) < 1 {
            1
        } else {
            (image_width as f64 / aspect_radio) as u32
        }; //talvez reverter

        let viewport_heigth: f64 = 2.0;
        let viewport_width: f64 = viewport_heigth * (image_width as f64 / image_heigth as f64);

        let focal_length: f64 = 1.0;
        let center: DVec3 = DVec3::ZERO;

        let viewport_u: DVec3 = DVec3::new(viewport_width, 0., 0.);
        let viewport_v: DVec3 = DVec3::new(0., -viewport_heigth, 0.);

        let max_value: u8 = 255;

        let pixel_delta_u: DVec3 = viewport_u / image_width as f64;
        let pixel_delta_v: DVec3 = viewport_v / image_heigth as f64;

        let viewport_upper_left: DVec3 = center
        - DVec3::new(0., 0., focal_length)
        - viewport_u / 2.
        - viewport_v / 2.;


        let pixel100_loc: DVec3 = viewport_upper_left + 0.5 * (pixel_delta_u * pixel_delta_v);

        Self { image_width, image_heigth, max_value, aspect_radio, center, pixel_delta_u, pixel_delta_v, pixel100_loc, samples_per_pixel: 100, max_depth: 50 }
    }

    pub fn render_to_disk<T>(&self, world: T) -> io::Result<()> where T: Hittable {
        let pixels: String = (0..self.image_heigth)
        .cartesian_product(0..self.image_width)  // Para cada pixel (y,x)
        .progress_count(self.image_heigth as u64 * self.image_width as u64  )                // Barra de progresso
        .map(|(y, x)| {
            let scale_factor: f64 = (self.samples_per_pixel as f64).recip();
            
            let multisampled_pixel_color: DVec3 = (0..self.samples_per_pixel).into_iter().map(|_| {
              let color = self.get_ray(x as i32, y as i32).color(self.max_depth as i32, &world) * 255.0 * scale_factor;
              DVec3 {
                x: linear_to_gamma(color.x),
                y: linear_to_gamma(color.y),
                z: linear_to_gamma(color.z),
              }
            }).sum::<DVec3>();
            
            // 4. Formatar como RGB
            // Converte os valores normalizados para a escala 0-255 e formata como string
            format!("{} {} {}", multisampled_pixel_color.x, multisampled_pixel_color.y, multisampled_pixel_color.z)
        })
        // Agrupa os pixels em linhas (chunks) de acordo com a largura da imagem
        .chunks(self.image_width as usize)
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
{} {}
{}
{pixels}
", self.image_width, self.image_heigth, self.max_value),) 

} 

  fn get_ray(&self, i: i32, j: i32) -> Ray {

    let pixel_center: DVec3 = self.pixel100_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);

    let pixel_sample: DVec3 = pixel_center + self.sample_square();

    let ray_origin: DVec3 = self.center;
    let ray_direction: DVec3 = pixel_sample - ray_origin;

    Ray { origin: self.center, direction: ray_direction }

  }

  fn sample_square(&self) -> DVec3{
    let mut rn = rand::rng();

    let px: f64 = -0.5 + rn.random::<f64>();
    let py: f64 = -0.5 + rn.random::<f64>();
    
    (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
  }


}
        


fn linear_to_gamma(linear: f64) -> f64{
  linear.sqrt()
}
