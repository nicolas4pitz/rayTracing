use glam::DVec3;

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

  pub fn color<T>(&self, world: &T) -> DVec3 where T: Hittable{
    if let Some(rec) = world.hit(&self, (0.)..f64::INFINITY){
      return 0.5 * (rec.normal + DVec3::new(1., 1., 1.))
    }

    let unit_direction: DVec3 = self.direction.normalize();

    let a = 0.5 * (unit_direction.y + 1.0);

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