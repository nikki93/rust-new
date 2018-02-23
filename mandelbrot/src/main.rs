extern crate num;

use num::Complex;

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

use std::str::FromStr;

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[0..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",20", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<f64>("10.2x20.5", 'x'), Some((10.2, 20.5)));
    assert_eq!(parse_pair::<f64>("10.2,20.5", 'x'), None);
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("10.2,"), None);
    assert_eq!(parse_complex("10.2,20.5"), Some(Complex { re: 10.2, im: 20.5 }));
}

fn pixel_to_point(bounds: (usize, usize),
                  pixel: (usize, usize),
                  upper_left: Complex<f64>,
                  lower_right: Complex<f64>) -> Complex<f64> {
    Complex {
        re: upper_left.re + (pixel.0 as f64) * (lower_right.re - upper_left.re) / bounds.0 as f64,
        im: upper_left.im - (pixel.1 as f64) * (upper_left.im - lower_right.im) / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 100), (25, 75),
                              Complex { re: -1.0, im: 1.0 },
                              Complex { re: 1.0, im: -1.0 }),
               Complex { re: -0.5, im: -0.5 });
}

fn render(pixels: &mut [u8],
          bounds: (usize, usize),
          upper_left: Complex<f64>,
          lower_right: Complex<f64>)
{
    assert_eq!(pixels.len(), bounds.0 * bounds.1);

    let mut pixel_index = 0;
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[pixel_index] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
            pixel_index += 1;
        }
    }
}

extern crate image;

use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
    Ok(())
}

extern crate crossbeam;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 5 {
        let bounds = parse_pair(&args[2], 'x').expect("parse bounds");
        let upper_left = parse_complex(&args[3]).expect("parse upper left");
        let lower_right = parse_complex(&args[4]).expect("parse lower right");

        let mut pixels = vec![0; bounds.0 * bounds.1];
        {
            let threads = 16;
            let rows_per_band = bounds.1 / threads + 1;
            let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
            crossbeam::scope(|spawner| {
                for (i, band) in bands.into_iter().enumerate() {
                    let band_top = rows_per_band * i;
                    let band_height = band.len() / bounds.0;
                    let band_bounds = (bounds.0, band_height);
                    let band_upper_left = pixel_to_point(bounds, (0, band_top), upper_left, lower_right);
                    let band_lower_right = pixel_to_point(bounds, (bounds.0, band_top + band_height), upper_left, lower_right);

                    spawner.spawn(move || {
                        render(band, band_bounds, band_upper_left, band_lower_right);
                    });
                }
            });
        }

        write_image(&args[1], &pixels, bounds).expect("write image");
    }
}
