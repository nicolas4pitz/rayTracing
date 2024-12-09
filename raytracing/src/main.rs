pub mod vecry;
pub mod color;
pub mod ray;
use std::fs::File;
use std::io::{self, Write};
use std::ops::{Mul, Sub};
use indicatif::{ProgressBar, ProgressStyle};
use ray::Ray;
use vecry::{dot, unit_vector, Point3, Vec3};

fn main() -> io::Result<()> {
  //Imagem
  let aspect_ratio = 16.0 / 9.0;
  let image_width: u32 = 400;

  //Calcula a altura da imagem e garante que seja ao menos 1
  let image_height = ((image_width as f64 / aspect_ratio) as u32).max(1);

  //Camera

  let focal_length = 1.0;
  let viewport_height = 2.0;
  let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
  let camera_center = Point3::new(0.0, 0.0, 0.0);

  //Calcula o vetor por meio da horizontal e abaixa o ponto de visao vertical
  let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
  let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

  // Calculate the horizontal and vertical delta vectors from pixel to pixel.
  let pixel_delta_u: Vec3 = viewport_u / image_width as f64;
  let pixel_delta_v: Vec3 = viewport_v / image_height as f64;

  //Calculate the location of the upper left pixel
  let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
  let pixelhundred_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

  // Render
    let total_pixels = (image_height * image_width) as u64;

    // Cria um arquivo chamado "image.ppm"
    let mut file = File::create("image.ppm")?;

    // Escreve o cabeçalho do arquivo PPM no arquivo
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", image_width, image_height)?;
    writeln!(file, "255")?;

    let progressbar = ProgressBar::new(total_pixels);
    progressbar.set_style(ProgressStyle::default_bar().template("[{elapsed}] [{wide_bar:.green}] {percent}% {msg}").unwrap(),);

    // Escreve os valores RGB dos pixels no arquivo
    for j in 0..image_height {
      
        for i in 0..image_width {
          let pixel_center: Vec3 = pixelhundred_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
          let ray_direction: Vec3 = pixel_center - camera_center;
          let camera_ray: Ray = ray::Ray::new(camera_center, ray_direction);

          let pixel_color: Vec3 = ray_color(&camera_ray);

          color::write_color(&mut file, &pixel_color);
          progressbar.inc(1);
        }
    }

    println!("Arquivo de imagem gerado: image.ppm");
    Ok(())
}

fn ray_color(r: &ray::Ray) -> vecry::Vec3 {
  // Verifica se o raio atinge a esfera
  let t: f64 = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r);
  if t > 0.0 {
      // Calcula a normal no ponto de interseção
      let normal: Vec3 = unit_vector(&(r.at(t).sub(Point3::new(0.0, 0.0, -1.0))));
      // Retorna a cor baseada na normal
      return Vec3::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0) * 0.5;
  }
  // Calcula a direção unitária do raio
  let unit_direction: Vec3 = unit_vector(r.direction());
  // Interpola entre branco e azul com base na direção y
  let t = 0.5 * (unit_direction.y() + 1.0);
  Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> f64 {

  // Vetor do raio de origem ao centro da esfera
  let oc: Vec3 = r.origin().sub(*center);

  // Coeficientes da equação quadrática
  let a: f64 = r.direction().length_squared();
  let h: f64 = vecry::dot(r.direction(), &oc);
  let c: f64 = &oc.length_squared() - radius * radius;

  // Discriminante da equação quadrática
  let discriminant: f64 = h*h - a*c;

  if discriminant < 0.0 {
      -1.0 // Não há interseção
  } else {
      (-h - discriminant.sqrt()) / (a) // Menor raiz
  }
}