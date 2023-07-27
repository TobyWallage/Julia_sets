use image::{ImageBuffer, Rgb};
use num::complex::Complex;
use std::cmp::Ordering;
// use rayon::prelude::*;

fn main() {
    let (width, height):(u32, u32) = (40000, 20000);
    let scale = 2.0;
    let (x_aspect, y_aspect):(f64, f64) = get_aspects(width, height, scale);

    let scalex = x_aspect/width as f64;
    let scaley = y_aspect/height as f64;

    let mut image_buffer = ImageBuffer::new(width, height);
    
    let c = Complex::new(-0.74543,  0.11301);

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut(){
        let z = Complex::new(
            (x as f64 * scalex) - x_aspect/2.0,
            (y as f64 * scaley) - y_aspect/2.0
        );

        let mut julia_val = julia(z, &c, &510, &2.0);
        julia_val = julia_val/2;
        *pixel = Rgb([(0.4*julia_val as f64) as u8,(0.7*julia_val as f64) as u8, (1.0*julia_val as f64) as u8]);
    };

    image_buffer.save("Julia_fractal.png").unwrap();

}

fn julia(z: Complex<f64>, c: &Complex<f64>, max_iterations: &u32, max_r: &f64) -> u32 {
    let mut i:u32 = 0;

    let mut z = z.clone();

    for iteration in 0..*max_iterations{
        if z.norm() > *max_r{
            break;
        }
        z = z * z + c;
        i = iteration;
    }
    return i;
}

fn get_aspects(width:u32, height:u32, scale:f64)-> (f64, f64){
    let aspect_ratio = width as f64/height as f64;
    match width.cmp(&height){
        Ordering::Equal => {return (scale, scale)},
        Ordering::Greater => {return (scale*aspect_ratio, scale)},
        Ordering::Less => {return (scale, scale*aspect_ratio)},
    }
}
