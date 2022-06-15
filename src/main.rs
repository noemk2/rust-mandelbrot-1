extern crate image;
extern crate num;

use mandelbrot::{get_range, if_flah, mandelbrot, rainbow, sin_to_dec, transpose_coordinate, Args};

#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate serde;

use num::complex::Complex64;
use std::fs::File;
use std::path::Path;

use docopt::Docopt;
//use serde::de::{Deserialize, Deserializer, Error, Visitor};
type Rgba = image::Rgba<u8>;

// Write the Docopt usage string

fn main() {
    // command line options .. todo
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if_flah(&args);

    let step: f64 = args.flag_step;
    let max_iterations: u32 = args.flag_i;
    let y_m_min: f64 = args.flag_y0;
    let y_m_max: f64 = args.flag_y1;
    let x_m_min: f64 = args.flag_x0;
    let x_m_max: f64 = args.flag_x1;

    let w_h: (u32, u32) = get_range_r(
        &step,
        &max_iterations,
        &y_m_min,
        &y_m_min,
        &x_m_max,
        &x_m_min,
    );

    //make an image

    let mut img = image::RgbaImage::new(w_h.0, w_h.1);

    let black = Rgba {
        data: [0, 0, 0, 255],
    };

    let colours = rainbow(max_iterations);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let depth = mandelbrot(
            transpose_coordinate(x, x_m_min, step),
            transpose_coordinate(y, y_m_min, step),
            max_iterations,
        );
        //set the image colour according to depth.
        *pixel = if depth < max_iterations {
            colours[depth as usize]
        } else {
            black
        };
    }

    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("fractal.png")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageRgba8(img).save(fout, image::PNG);
    println!("All done!");
}
