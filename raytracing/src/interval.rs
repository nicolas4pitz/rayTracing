


pub struct Interval {
  min: f64,
  max: f64,
}

impl Interval {
  pub fn new(min: f64, max: f64) -> Self {
    Self { min, max }
  }

  pub fn contains(&self, value: f64) -> bool {
    value >= self.min && value <= self.max
  }

  pub fn size(&self) -> f64 {
    self.max - self.min
  }

  pub fn surronunds(&self, other: f64) -> bool {
    self.min < other && self.max > other 
  }

  pub fn clamp(&self, x: f64) -> f64 {
    if x < self.min {
      return self.min;
    }

    if x > self.max {
      return self.max;
    }

    x
  }

  pub fn get_max(&self) -> f64 {
    self.max
  }

  pub fn get_min(&self) -> f64 {
    self.min
  }
}