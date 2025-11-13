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

use std::io;

use glam::DVec3;
use indicatif::ProgressIterator;
use itertools::Itertools;
use rand::Rng;

use crate::{hitable::Hittable, ray::Ray};

pub struct Camera {
    // CONFIGURAÇÕES DE IMAGEM
    image_width: u32,          // Largura em pixels
    image_heigth: u32,         // Altura calculada
    aspect_radio: f64,         // Proporção largura/altura
    max_value: u8,             // Valor máximo de cor (255)
    
    // AMOSTRAGEM E QUALIDADE
    samples_per_pixel: u32,    // Anti-aliasing (100 amostras)
    max_depth: u32,            // Profundidade de recursão (50)
    
    // POSICIONAMENTO DA CÂMERA
    lookfrom: DVec3,           // Posição no espaço 3D
    lookat: DVec3,             // Para onde aponta
    vup: DVec3,                // Orientação vertical
    center: DVec3,
    vfov: f64,
    
    // VETORES ORTONORMAIS (Base da câmera)
    u: DVec3,                  // Eixo horizontal (direita)
    v: DVec3,                  // Eixo vertical (cima)
    w: DVec3,                  // Eixo de profundidade (trás)
    
    // PROFUNDIDADE DE CAMPO
    defocus_angle: f64,        // Abertura do diafragma
    focus_dist: f64,           // Distância de foco
    defocus_disk_u: DVec3,     // Disco de desfoque horizontal
    defocus_disk_v: DVec3,     // Disco de desfoque vertical
    
    // GEOMETRIA DO VIEWPORT
    pixel_delta_u: DVec3,      // Espaçamento horizontal entre pixels
    pixel_delta_v: DVec3,      // Espaçamento vertical entre pixels
    pixel100_loc: DVec3        // Localização do primeiro pixel
}

pub struct CameraNew {
    pub image_width: u32,
    pub aspect_ratio: f64,
    pub look_from: Option<DVec3>,
    pub look_at: Option<DVec3>,
    pub vup: Option<DVec3>,
    pub focus_dist: Option<f64>,
    pub defocus_angle: Option<f64>,
}

impl Camera {
    pub fn new(config: CameraNew) -> Self {
        // Constantes para definir as dimensões da imagem
        let lookfrom = config.look_from.unwrap_or(DVec3::NEG_Z);
        let lookat = config.look_at.unwrap_or(DVec3::ZERO);
        let vup = config.vup.unwrap_or(DVec3::Y);
        let focus_dist = config.focus_dist.unwrap_or(10.);
        let defocus_angle = config.defocus_angle.unwrap_or(0.);
        
        // Calculo da dimensao
        let image_heigth: u32 = (config.image_width as f64 / config.aspect_ratio) as u32; 

        //Campo de Visão (FOV)
        let vfov: f64 = 20.0; // Angulo vertical em graus
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan(); // Altura Tangecial

        //Dimensões do Viewport
        let viewport_heigth: f64 = 2.0 * h * focus_dist;
        let viewport_width: f64 = viewport_heigth * (config.image_width as f64 / image_heigth as f64);

        let center = lookfrom;

        // SISTEMA DE COORDENADAS DA CÂMERA
        let w = (lookfrom - lookat).normalize(); // Para tras
        let u = vup.cross(w).normalize(); // Para direita
        let v = w.cross(u); // Para cima

        let max_value: u8 = 255;

        // Vetores do Viewport
        let viewport_u: DVec3 = viewport_width * u;
        let viewport_v: DVec3 = viewport_heigth * -v; // Negativo = tela cresce para baixo

        // Espaçamento entre cada pixel
        let pixel_delta_u = viewport_u / config.image_width as f64;
        let pixel_delta_v = viewport_v / image_heigth as f64;

        // Canto Superior Esquerdo
        let viewport_upper_left: DVec3 =
            center - focus_dist * w - viewport_u / 2. - viewport_v / 2.;

        // Centro do Primeiro pixel 
        let pixel100_loc: DVec3 = viewport_upper_left + 0.5 * (pixel_delta_u * pixel_delta_v);

        // Profundidade de campo
        let defocus_radius = focus_dist * (defocus_angle / 2.).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            image_width: config.image_width,
            image_heigth,
            max_value,
            aspect_radio: config.aspect_ratio,
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel100_loc,
            samples_per_pixel: 100,
            max_depth: 50,
            vfov,
            lookfrom,
            lookat,
            vup,
            u,
            v,
            w,
            defocus_angle,
            focus_dist,
            defocus_disk_u,
            defocus_disk_v
        }
    }

    pub fn render_to_disk<T>(&self, world: T) -> io::Result<()>
    where
        T: Hittable,
    {
        let pixels: String = (0..self.image_heigth)
            .cartesian_product(0..self.image_width)  // Para cada pixel (y,x)
            .progress_count(self.image_heigth as u64 * self.image_width as u64) // Barra de progresso
            .map(|(y, x)| {
                let scale_factor: f64 = (self.samples_per_pixel as f64).recip();

                // ANTI-ALIASING: Múltiplas amostras por pixel
                let multisampled_pixel_color: DVec3 = (0..self.samples_per_pixel)
                    .into_iter()
                    .map(|_| {
                        self.get_ray(x as i32, y as i32)
                            .color(self.max_depth as i32, &world)
                    })
                    .sum::<DVec3>()
                    * scale_factor;

                // Correção da Gamma, Linear -> sRGB
                let color = DVec3 {
                    x: linear_to_gamma(multisampled_pixel_color.x),
                    y: linear_to_gamma(multisampled_pixel_color.y),
                    z: linear_to_gamma(multisampled_pixel_color.z),
                }
                .clamp(DVec3::splat(0.), DVec3::splat(0.999))
                    * 256.;

                // 4. Formatar como RGB
                format!("{} {} {}", color.x, color.y, color.z)
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
",
                self.image_width, self.image_heigth, self.max_value
            ),
        )
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
      // Calcular centro do pixel
        let pixel_center: DVec3 =
            self.pixel100_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);

        // Amostragem aleatória dentro do pixel
        let pixel_sample: DVec3 = pixel_center + self.sample_square();

        // Profundidade de campo
        let ray_origin: DVec3 = if self.defocus_angle <= 0. {
          self.center // Sem DOF = origem fixa
        } else{
          self.defocus_disk_sample() // Origem aleatrória no disco
        };
        let ray_direction: DVec3 = pixel_sample - ray_origin;

        Ray {
            origin: self.center,
            direction: ray_direction,
        }
    }

    // Amostragem no disco de desfoque
    fn defocus_disk_sample(&self) -> DVec3{
      let p = random_in_unit_disk();

      self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    fn sample_square(&self) -> DVec3 {
        let mut rn = rand::rng();

        let px: f64 = -0.5 + rn.random::<f64>();
        let py: f64 = -0.5 + rn.random::<f64>();

        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
}

fn linear_to_gamma(linear: f64) -> f64 {
    linear.sqrt()
}

fn random_in_unit_disk() -> DVec3 {
  let mut rng = rand::rng();

  loop{
    let v = DVec3::new(rng.random_range(-1.0..1.), rng.random_range(-1.0..1.), 0.);

    if v.length_squared() < 1. {
      break v;
    }
  }
}