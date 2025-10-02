use glam::DVec3;

use crate::hit_sphere;

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

  fn sense(&self, time: f64) -> DVec3{
    self.origin + time * self.direction
  }

  pub fn color(&self) -> DVec3{

    if hit_sphere(&DVec3::new(0., 0., -1.), 0.5, self){
      return DVec3::new(1., 0., 0.);
    };

    let unit_direction: DVec3 = self.direction.normalize();

    let alpha = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - alpha) * DVec3::new(1.0, 1.0, 1.0) + alpha * DVec3::new(0.5, 0.7, 1.0);
  }

}