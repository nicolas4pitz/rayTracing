use std::{fs::File, io, ops::Mul};

use crate::{color::{self, Color}, hittable::{self, Hittable}, interval::Interval, ray::{self, Ray}, rtweekend::{random_double, INFINITYCONST}, vecry::{self, random_on_hemisphere, unit_vector, Point3, Vec3}};
use std::io::Write;
use indicatif::{ProgressBar, ProgressStyle};



pub struct Camera {
  pub aspect_ratio: f64,
  pub image_width: u32,
  pub samples_per_pixel: u32,
  pub max_depth: u32,
  pixel_sample_scale: f64,
  image_height: u32,
  center: Point3,
  pixelhundred_loc: Point3,
  pixel_delta_u: Vec3,
  pixel_delta_v: Vec3,
}

impl Camera {
  pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32, max_depth: u32) -> Self {
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
    let pixel_sample_scale = 1.0 / samples_per_pixel as f64;
    Self {
      aspect_ratio,
      image_width,
      samples_per_pixel,
      pixel_sample_scale,
      max_depth,
      image_height,
      center,
      pixelhundred_loc,
      pixel_delta_u,
      pixel_delta_v,
    }
  } //Caso começar a dar merda, implementar o Initialize

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
        let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
        for _ in 0..self.samples_per_pixel {
          let ray: Ray = self.get_ray(i, j);
          pixel_color += Camera::ray_color(&ray, self.max_depth, world);
        }
        color::write_color(&mut file, &pixel_color.mul(self.pixel_sample_scale));
        progressbar.inc(1);
      }
  }

  println!("Arquivo de imagem gerado: image.ppm");
  Ok(())

  }

  fn get_ray(&self, i: u32, j: u32) -> Ray {
    let offset: Vec3 = Camera::sample_square();
    let pixel_sample: Vec3 = self.pixelhundred_loc + (self.pixel_delta_u * (i as f64 + offset.x()) ) + (self.pixel_delta_v * (j as f64+offset.y()));

    let ray_origin: Vec3 = self.center;
    let ray_direction: Vec3 = pixel_sample - ray_origin;

    Ray::new(ray_origin, ray_direction)
  }

  fn sample_square() -> Vec3 {
    Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
  }

  fn ray_color(r: &ray::Ray, depth: u32, world: &dyn hittable::Hittable) -> Color {

    if depth <= 0 {
      return Color::new(0.0, 0.0, 0.0);
    }

    let normal: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    let mut rec = hittable::HitRecord::new(Vec3::new(0.0, 0.0, 0.0), normal, 0.0, false);

    if world.hit(r, Interval::new(0.001, INFINITYCONST), &mut rec){
        
        let direction: Vec3 = random_on_hemisphere(rec.normal);
        return Self::ray_color(&Ray::new(rec.p, direction), depth-1, world) * 0.5;
    }
    
    let unit_direction: Vec3 = unit_vector(r.direction());
    
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
  }

}