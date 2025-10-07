mod ray;
mod hitable;
mod sphere;
mod camera;

use glam::DVec3;
use indicatif::ProgressIterator;
use itertools::Itertools;
use std::{fs, io, vec};

use crate::{camera::Camera, hitable::HitableList, ray::Ray, sphere::Sphere};



fn main() -> io::Result<()> {

  let mut world: HitableList = HitableList {objects: vec![]};

  world.add(Sphere {
    center: DVec3::new(0.0, 0.0, -1.0),
    radius: 0.5,
  });

  world.add(Sphere {
    center: DVec3::new(0.0, -100.5, -1.),
    radius: 100.,
  });

    let camera = Camera::new(400, 16.0/9.0);
    camera.render_to_disk(world)?;

    Ok(())
}



