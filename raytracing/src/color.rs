use crate::vecry::Vec3;
use crate::interval::Interval;
use std::io::Write;

pub type Color = Vec3;

/// Aplica uma transformação linear para gama (gama 2)
fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

/// Escreve a cor do pixel no arquivo
pub fn write_color(file: &mut std::fs::File, pixel_color: &Color) {
    let mut red = pixel_color.x();
    let mut green = pixel_color.y();
    let mut blue = pixel_color.z();

    // Aplica a transformação linear para gama
    red = linear_to_gamma(red);
    green = linear_to_gamma(green);
    blue = linear_to_gamma(blue);

    // Traduz os valores dos componentes [0,1] para o intervalo de bytes [0,255]
    static COLOR_INTENSITY: std::sync::LazyLock<Interval> = std::sync::LazyLock::new(|| Interval::new(0.0, 0.999));
    let red_byte = (256.0 * COLOR_INTENSITY.clamp(red)) as u8;
    let green_byte = (256.0 * COLOR_INTENSITY.clamp(green)) as u8;
    let blue_byte = (256.0 * COLOR_INTENSITY.clamp(blue)) as u8;

    // Escreve os componentes de cor do pixel no arquivo
    writeln!(file, "{} {} {}", red_byte, green_byte, blue_byte).unwrap();
}