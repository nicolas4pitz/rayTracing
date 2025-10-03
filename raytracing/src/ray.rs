use glam::DVec3;

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
    let t: f64 = hit_sphere(&DVec3::new(0., 0., -1.), 0.5, &self);
    if t > 0.0{
      let normal: DVec3 = (self.sense(t) - DVec3::new(0., 0., -1.)).normalize();
      return 0.5 * (normal + 1.);
    }

    let unit_direction: DVec3 = self.direction.normalize();

    let alpha = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - alpha) * DVec3::new(1.0, 1.0, 1.0) + alpha * DVec3::new(0.5, 0.7, 1.0);
  }

}

//Calcula onde a esfera estaria, e verifica se o ray bate onde a esfera pode estar
fn hit_sphere(center: &DVec3, radius: f64, ray: &Ray) -> f64{
    let distanceOriginCenter:DVec3 = ray.origin - *center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * distanceOriginCenter.dot(ray.direction);
    let c = distanceOriginCenter.dot(distanceOriginCenter) - radius * radius;
    let discriminant = b * b - 4. * a *c;
    if(discriminant < 0.){
      -1.0
    } else{
      (-b - discriminant.sqrt()) / (2.0*a)
    }

}