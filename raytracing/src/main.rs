pub mod vecry;
pub mod color;
use std::fs::File;
use std::io::{self, Write};
use indicatif::{ProgressBar, ProgressStyle};


fn main() -> io::Result<()> {
    let image_width = 800;
    let image_height = 600;
    let total_pixels = (image_height * image_width) as u64;

    // Cria um arquivo chamado "image.ppm"
    let mut file = File::create("image.ppm")?;

    // Escreve o cabeçalho do arquivo PPM no arquivo
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", image_width, image_height)?;
    writeln!(file, "255")?;

    let progressbar = ProgressBar::new(total_pixels);
    progressbar.set_style(ProgressStyle::default_bar().template("[{elapsed}] [{wide_bar:.green}] {percent}% {msg}").unwrap(),);

    // Escreve os valores RGB dos pixels no arquivo
    for j in 0..image_height {
      
        for i in 0..image_width {
          let pixel_color = vecry::Vec3::new(i as f64 / (image_width - 1) as f64, j as f64 / (image_height - 1) as f64, 0 as f64);
          color::write_color(&mut file, &pixel_color);
          progressbar.inc(1);
        }
    }

    println!("Arquivo de imagem gerado: image.ppm");
    Ok(())
}