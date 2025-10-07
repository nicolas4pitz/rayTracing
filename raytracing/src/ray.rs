use glam::DVec3;
use rand::Rng;

use crate::hitable::Hittable;

// pub struct Ray {
//     a: Vector3<f32>,
//     b: Vector3<f32>
// }

// impl Ray {
//     pub fn new (a: Vector3<f32>, b: Vector3<f32>) -> Self {
//         Ray { a, b }
//     }

//     pub fn origin(&self) -> Vector3<f32> { self.a }
//     pub fn direction(&self) -> Vector3<f32> { self.b }
//     pub fn point_at_parameter(&self, t: f32) -> Vector3<f32> { self.a + t * self.b }
// }

pub struct Ray{
  pub origin: DVec3,
  pub direction: DVec3
}

impl Ray{

  pub fn at(&self, time: f64) -> DVec3{
    self.origin + time * self.direction
  }

  pub fn color<T>(&self, depth:i32, world: &T) -> DVec3 where T: Hittable{

    if depth <= 0{
      return DVec3::new(0., 0., 0.);
    }

    if let Some(rec) = world.hit(&self, (0.001)..f64::INFINITY){
      let direction: DVec3 = rec.normal + random_unit_vector();
      let ray = Ray {origin: rec.point, direction};

      return 0.5 * ray.color(depth - 1, world);
    }

    let unit_direction: DVec3 = self.direction.normalize();

    let a: f64 = 0.5 * (unit_direction.y + 1.0);

    return (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0);
  }

}

//Calcula onde a esfera estaria, e verifica se o ray bate onde a esfera pode estar
// fn hit_sphere(center: &DVec3, radius: f64, ray: &Ray) -> f64{
//     let distanceOriginCenter:DVec3 = ray.origin - *center;
//     let a: f64 = ray.direction.length_squared();
//     let half_b:f64 = distanceOriginCenter.dot(ray.direction);
//     //let b = 2.0 * distanceOriginCenter.dot(ray.direction);
//     let c:f64 = distanceOriginCenter.length_squared() - radius*radius;
    
//     let discriminant = half_b*half_b - a*c;
    
//     if(discriminant < 0.){
//       -1.0
//     } else{
//       (-half_b - discriminant.sqrt()) / a
//     }

// }

fn random_in_unit_sphere() -> DVec3{
  let mut rng = rand::rng();

  loop{
    let vec = DVec3::new(rng.random_range(-1.0..1.), rng.random_range(-1.0..1.), rng.random_range(-1.0..1.));

    if vec.length_squared() < 1. {
      break vec;
    }
  }
}

fn random_unit_vector() -> DVec3{
  return random_in_unit_sphere().normalize();
}

fn random_on_hemisphere(normal: &DVec3) -> DVec3{
  let on_unit_sphere = random_unit_vector();
  if on_unit_sphere.dot(*normal) > 0.0 {
    on_unit_sphere
  } else {
    -on_unit_sphere
  }
}

