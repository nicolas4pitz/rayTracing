use std::{fs::File, io};

use crate::{color::{self, Color}, hittable::{self, Hittable}, interval::Interval, ray::{self, Ray}, rtweekend::INFINITYCONST, vecry::{self, unit_vector, Point3, Vec3}};
use std::io::Write;
use indicatif::{ProgressBar, ProgressStyle};



pub struct Camera {
  pub aspect_ratio: f64,
  pub image_width: u32,
  image_height: u32,
  center: Point3,
  pixelhundred_loc: Point3,
  pixel_delta_u: Vec3,
  pixel_delta_v: Vec3,
}

impl Camera {
  pub fn new(aspect_ratio: f64, image_width: u32) -> Self {
    let image_height = ((image_width as f64 / aspect_ratio) as u32).max(1);
    let center = Point3::new(0.0, 0.0, 0.0);
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, viewport_height, 0.0);
    let pixel_delta_u: Vec3 = viewport_u / image_width as f64;
    let pixel_delta_v: Vec3 = viewport_v / image_height as f64;
    let viewport_upper_left = center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixelhundred_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;
    Self {
      aspect_ratio,
      image_width,
      image_height,
      center,
      pixelhundred_loc,
      pixel_delta_u,
      pixel_delta_v,
    }
  }

  pub fn render(&self, world: &dyn hittable::Hittable) -> io::Result<()>{

  let total_pixels = (self.image_height * self.image_width) as u64;

  // Cria um arquivo chamado "image.ppm"
  let mut file = File::create("image.ppm")?;

  // Escreve o cabeçalho do arquivo PPM no arquivo
  writeln!(file, "P3")?;
  writeln!(file, "{} {}", self.image_width, self.image_height)?;
  writeln!(file, "255")?;

  let progressbar = ProgressBar::new(total_pixels);
  progressbar.set_style(ProgressStyle::default_bar().template("[{elapsed}] [{wide_bar:.green}] {percent}% {msg}").unwrap());

  // Escreve os valores RGB dos pixels no arquivo
  for j in 0..self.image_height {
    
      for i in 0..self.image_width {
        let pixel_center: Vec3 = self.pixelhundred_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
        let ray_direction: Vec3 = pixel_center - self.center;
        let camera_ray: Ray = ray::Ray::new(self.center, ray_direction); // = r na doc

        let pixel_color: Vec3 = Camera::ray_color(&camera_ray, world);

        color::write_color(&mut file, &pixel_color);
        progressbar.inc(1);
      }
  }

  println!("Arquivo de imagem gerado: image.ppm");
  Ok(())

  }

  fn ray_color(r: &ray::Ray, world: &dyn hittable::Hittable) -> vecry::Vec3 {
    let normal: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    let mut rec = hittable::HitRecord::new(Vec3::new(0.0, 0.0, 0.0), normal, 0.0, false);

    if world.hit(r, Interval::new(0.0, INFINITYCONST), &mut rec){
        // Retorna a cor baseada na normal
        return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }
    // Calcula a direção unitária do raio
    let unit_direction: Vec3 = unit_vector(r.direction());
    // Interpola entre branco e azul com base na direção y
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
  }

}