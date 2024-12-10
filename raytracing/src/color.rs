use crate::vecry;
use std::io::Write;
use crate::vecry::Vec3;

pub type Color = Vec3;

pub fn write_color(file: &mut std::fs::File, pixel_color: &vecry::Vec3) {
  let red = pixel_color.x();
  let green = pixel_color.y();
  let blue = pixel_color.z();

  let redbyte = (255.999 * red) as u8;
  let greenbyte = (255.999 * green) as u8;
  let bluebyte = (255.999 * blue) as u8;

  writeln!(file, "{} {} {}", redbyte, greenbyte, bluebyte).unwrap();

}