mod ray;
mod hitable;
mod sphere;
mod camera;

use glam::DVec3;
use indicatif::ProgressIterator;
use itertools::Itertools;
use std::{fs, io, vec};

use crate::{camera::Camera, hitable::{HitableList, Material}, ray::Ray, sphere::Sphere, camera::CameraNew};



fn main() -> io::Result<()> {

  let mut world: HitableList = HitableList {objects: vec![]};

  // let R = (std::f64::consts::PI / 4.).cos();

  // let material_left = Material::Lambertian { albedo: DVec3::new(0., 0., 1.) };
  // let material_rigth = Material::Lambertian { albedo: DVec3::new(1., 0., 0.) };

  // world.add(Sphere {
  //   center: DVec3::new(-R, 0., -1.),
  //   radius: R,
  //   material: material_left.clone()
  // });

  // world.add(Sphere {
  //   center: DVec3::new(R, 0., -1.),
  //   radius: R,
  //   material: material_rigth.clone()
  // });


  // 1. CHÃO - Material difuso amarelado
  let material_ground = Material::Lambertian { 
      albedo: DVec3::new(0.8, 0.8, 0.0)  // RGB: Amarelo opaco
  };

  // 2. ESFERA CENTRAL - Material difuso roxo
  let material_center = Material::Lambertian { 
      albedo: DVec3::new(0.5, 0.1, 0.6)  // RGB: Roxo
  };

  // 3. VIDRO - Material dielétrico (refração)
  let material_left = Material::Dielectric { 
      index_of_refraction: 1.5  // Índice de refração do vidro
  };

  // 4. BOLHA DE AR - Vidro com índice invertido
  let material_buble = Material::Dielectric { 
      index_of_refraction: 1.0/1.5  // Cria efeito de bolha
  };

  // 5. METAL - Material reflexivo dourado
  let material_rigth = Material::Metal { 
      albedo: DVec3::new(0.8, 0.6, 0.2),  // RGB: Dourado
      fuzz: 0.0  // Sem difusão (espelho perfeito)
  };

  // CHÃO - Esfera gigante simulando superfície plana
  world.add(Sphere {
      center: DVec3::new(0.0, -100.5, -1.0),
      radius: 100.0,
      material: material_ground
  });

  // ESFERA CENTRAL - Objeto principal roxo
  world.add(Sphere {
      center: DVec3::new(0.0, 0.0, -1.0),
      radius: 0.5,
      material: material_center
  });

  // ESFERA DE VIDRO - Esquerda
  world.add(Sphere {
      center: DVec3::new(-1.0, 0.0, -1.0),
      radius: 0.5,
      material: material_left.clone()
  });

  // BOLHA INTERNA - Raio negativo inverte as normais!
  world.add(Sphere {
      center: DVec3::new(-1.0, 0.0, -1.0),
      radius: -0.4,  // TRUQUE: raio negativo = oco por dentro
      material: material_left
  });

  // ESFERA METÁLICA - Direita
  world.add(Sphere {
      center: DVec3::new(1.0, 0.0, -1.0),
      radius: 0.5,
      material: material_rigth
  });

    let camera = Camera::new(CameraNew { 
      image_width: 400,              // Resolução horizontal
      aspect_ratio: 16.0/9.0,        // Proporção widescreen
      look_from: Some(DVec3::new(-2., 2., 1.)),  // Posição da câmera
      look_at: Some(DVec3::new(0., 0., -1.)),    // Ponto focal
      vup: Some(DVec3::Y),           // Vetor "para cima"
      focus_dist: Some(3.4),         // Distância focal
      defocus_angle: Some(0.0)       // Ângulo de desfoque (DOF desligado)
    });

    camera.render_to_disk(world)?;

    Ok(())
}



