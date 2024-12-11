use crate::vecry;
use std::io::Write;
use crate::vecry::Vec3;
use crate::interval::Interval;

pub type Color = Vec3;

pub fn write_color(file: &mut std::fs::File, pixel_color: &vecry::Vec3) {
  let red = pixel_color.x();
  let green = pixel_color.y();
  let blue = pixel_color.z();

  static COLOR_INTENSITY: std::sync::LazyLock<Interval> = std::sync::LazyLock::new(|| Interval::new(0.0, 0.999));
  let redbyte = (255.999 * COLOR_INTENSITY.clamp(red)) as u8;
  let greenbyte = (255.999 * COLOR_INTENSITY.clamp(green)) as u8;
  let bluebyte = (255.999 * COLOR_INTENSITY.clamp(blue)) as u8;

  writeln!(file, "{} {} {}", redbyte, greenbyte, bluebyte).unwrap();

}