pub mod vecry;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod rtweekend;
pub mod interval;
pub mod camera;

use std::sync::Arc;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vecry::Point3;
use crate::camera::Camera;

fn main(){
  
  //World

  let mut world = HittableList::new();

  // Adiciona uma esfera à lista

  world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, 1.0), 0.5)));
  world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
  

  //Camera

  let camera:Camera = Camera::new(16.0/9.0, 400, 100);

  camera.render(&world).unwrap();
}
