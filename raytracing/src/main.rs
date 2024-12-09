use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    // Cria um arquivo chamado "image.ppm"
    let mut file = File::create("image.ppm")?;

    // Escreve o cabeçalho do arquivo PPM no arquivo
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", image_width, image_height)?;
    writeln!(file, "255")?;

    // Escreve os valores RGB dos pixels no arquivo
    for j in 0..image_height {
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            writeln!(file, "{} {} {}", ir, ig, ib)?;
        }
    }

    println!("Arquivo de imagem gerado: image.ppm");
    Ok(())
}