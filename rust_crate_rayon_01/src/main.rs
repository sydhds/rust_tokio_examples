use rayon::prelude::*;
use std::io::{LineWriter, Write};
use std::path::PathBuf;

fn main() {
    // Tell rayon to only use 4 cores
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .unwrap();

    let width = 320;
    let height = 240;
    let path_1 = PathBuf::from("image.ppm");
    let path_2_1 = PathBuf::from("image_gray_std.ppm");
    let path_2_2 = PathBuf::from("image_gray_rayon.ppm");

    println!("Generating image data...");
    let mut img = generate_image_data(width, height).expect("Unable to generate image data");

    println!("Writing {:?}...", path_1);
    write_image(width, height, &img, &path_1)
        .unwrap_or_else(|_| panic!("Unable to write image {:?}", path_1));
    let mut img2 = img.clone();
    let to_grayscale = |chunk: &mut [u8]| {
        let r = chunk[0];
        let g = chunk[1];
        let b = chunk[2];

        // rgb to grayscale using luminosity method
        let gray_ = 0.3 * f64::from(r) + 0.59 * f64::from(g) + 0.11 * f64::from(b);

        chunk[0] = gray_ as u8;
        chunk[1] = gray_ as u8;
        chunk[2] = gray_ as u8;
    };

    // std chunks iterator
    let st = std::time::Instant::now();
    img.chunks_exact_mut(3).for_each(to_grayscale);
    let elapsed = st.elapsed();
    println!(
        "std chunks exact mut - elapsed {} microseconds",
        elapsed.as_micros()
    );

    // par_chunks is a // rayon iterator
    let st2 = std::time::Instant::now();
    img2.par_chunks_exact_mut(3).for_each(to_grayscale);
    let elapsed2 = st2.elapsed();
    println!(
        "rayon chunks exact mut - elapsed {} microseconds",
        elapsed2.as_micros()
    );

    println!("Writing {:?}...", path_2_1);
    write_image(width, height, &img, &path_2_1)
        .unwrap_or_else(|_| panic!("Unable to write image {:?}", path_2_1));

    println!("Writing {:?}...", path_2_2);
    write_image(width, height, &img2, &path_2_2)
        .unwrap_or_else(|_| panic!("Unable to write image {:?}", path_2_2));
}

fn generate_image_data(width: u16, height: u16) -> anyhow::Result<Vec<u8>> {
    // let data_size = width * height * 3;
    let data_size = usize::try_from(u32::from(width) * u32::from(height) * 3)?;
    eprintln!("Image size: {}", data_size);
    let mut data: Vec<u8> = Vec::with_capacity(data_size);

    for j in 0..height {
        for i in 0..width {
            let r = f64::from(i) / (f64::from(width) - 1.0);
            let g = f64::from(j) / (f64::from(height) - 1.0);
            let b = 0.0;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            data.push(ir);
            data.push(ig);
            data.push(ib);
        }
    }

    Ok(data)
}

fn write_image(width: u16, height: u16, data: &[u8], path: &PathBuf) -> anyhow::Result<()> {
    let file = std::fs::File::create(path)?;
    let mut writer = LineWriter::new(file);

    let color_count = 3;
    let line_size = u32::from(width) * color_count;

    writer.write_all(b"P3\n")?;
    writeln!(writer, "{} {}", width, height)?;
    writer.write_all(b"255\n")?;

    for j in 0..height {
        for i in 0..width {
            let index_ = (u32::from(j) * line_size) + (u32::from(i) * 3);
            let index = usize::try_from(index_)?;

            let r = data[index];
            let g = data[index + 1];
            let b = data[index + 2];
            writeln!(writer, "{} {} {}", r, g, b)?;
        }
    }

    Ok(())
}
