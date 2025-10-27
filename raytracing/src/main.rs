mod ray;
mod hitable;
mod sphere;
mod camera;

use glam::DVec3;
use indicatif::ProgressIterator;
use itertools::Itertools;
use std::{fs, io, vec};

use crate::{camera::Camera, hitable::{HitableList, Material}, ray::Ray, sphere::Sphere};



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


  let material_ground: Material = Material::Lambertian { albedo: DVec3 { x: 0.8, y: 0.8, z: 0.0 }, };
  let material_center: Material = Material::Lambertian { albedo: DVec3 { x: 0.5, y: 0.1, z: 0.6 }, };
  let material_left: Material = Material::Dielectric { index_of_refraction: 1.5 };
  let material_buble: Material = Material::Dielectric { index_of_refraction: 1.0/1.5 };
  let material_rigth: Material = Material::Metal { albedo: DVec3 { x: 0.8, y: 0.6, z: 0.2 }, fuzz: 0.0};

  

  world.add(Sphere {
    center: DVec3::new(0.0, -100.5, -1.),
    radius: 100.,
    material: material_ground
  });

  world.add(Sphere {
    center: DVec3::new(0.0, 0.0, -1.0),
    radius: 0.5,
    material: material_center,
  });

  world.add(Sphere {
    center: DVec3::new(-1.0, 0.0, -1.0),
    radius: 0.5,
    material: material_left.clone()
  });

  world.add(Sphere {
    center: DVec3::new(-1.0, 0.0, -1.0),
    radius: -0.4,
    material: material_left
  });

  world.add(Sphere {
    center: DVec3::new(1., 0., -1.),
    radius: 0.5,
    material: material_rigth
  });

    let camera = Camera::new(400, 16.0/9.0, Some(DVec3::new(-2., 2., 1.)), Some(DVec3::new(0., 0., -1.)), Some(DVec3::Y),);
    camera.render_to_disk(world)?;

    Ok(())
}



