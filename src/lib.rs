#[derive(Debug, Deserialize)]
pub struct Args {
    pub flag_h: bool,
    pub flag_i: u32,
    pub flag_step: f64,
    pub flag_x0: f64,
    pub flag_x1: f64,
    pub flag_y0: f64,
    pub flag_y1: f64,
}

pub fn rainbow(c: u32) -> Vec<Rgba> {
    (0..c)
        .map(|i| Rgba {
            data: [
                sin_to_dec(c, i, 0.0 * std::f64::consts::PI * 2.0 / 3.0),
                sin_to_dec(c, i, 2.0 * std::f64::consts::PI * 2.0 / 3.0),
                sin_to_dec(c, i, 1.0 * std::f64::consts::PI * 2.0 / 3.0),
                255,
            ],
        })
        .collect()
}

pub fn sin_to_dec(c: u32, i: u32, phase: f64) -> u8 {
    let s = (std::f64::consts::PI / (c as f64) * 2.0 * (i as f64) + phase).sin();
    (((s * 127.0) + 128.0).floor()) as u8
}

pub fn transpose_coordinate(i: u32, min: f64, step: f64) -> f64 {
    min + (step * (i as f64))
}

pub fn get_range(min: f64, max: f64, step: f64) -> u32 {
    (((max - min) / step).floor()) as u32
}

pub fn mandelbrot(x: f64, y: f64, imax: u32) -> u32 {
    let a = Complex64::new(x, y);
    let mut i: u32 = 0;
    let mut z = a.clone();
    //while abs(z) < 2.0 && i < imax {
    while z.norm_sqr() < 4.0 && i < imax {
        i += 1;
        z = z * z + a;
    }
    i
}

const USAGE: &'static str = "
Usage: mandelbrot [options]

Options:
    -h                  Display this usage statement
    -i ITER             Maximum iterations [default: 30]
    --step STEP         A pixel is drawn for each step [default: 0.003]
    --x0 FROMX          the left hand x coordinate to start from [default: -2.0]
    --x1 TOX            the right hand x coordinate to go to [default: 1.0]
    --y0 FROMY          the top y coordinate (-ve is UP) [default: -1.2]
    --y1 TOY            the bottom y coordiante (+ve is DOWN) [default: 1.2]
";

pub fn if_flah(args: &Args) {
    if args.flag_h {
        println!("{}", USAGE);
        return;
    }
}

pub fn get_range_r(
    step: &f64,
    max_iterations: &u32,
    y_m_min: &f64,
    y_m_max: &f64,
    x_m_min: &f64,
    x_m_max: &f64,
) -> (u32, u32) {
    let h = get_range(*y_m_min, *y_m_max, *step);
    let w = get_range(*x_m_min, *x_m_max, *step);
    println!("max iterations: {}", max_iterations);
    println!("Generating mandelbrot W:{} H:{}", w, h);
    return (w, h);
}
