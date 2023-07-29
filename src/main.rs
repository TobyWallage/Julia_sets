use image::{ImageBuffer, Rgb};
use num::complex::Complex;
use std::cmp::Ordering;
use std::time;


fn main() {
    let now = time::Instant::now();

    let (width, height):(u32, u32) = (20000,10000);
    let scale = 2.0;
    let (x_aspect, y_aspect):(f64, f64) = get_aspects(width, height, scale);

    let scalex = x_aspect/width as f64;
    let scaley = y_aspect/height as f64;

    
    let mut image_buffer = ImageBuffer::new(width, height);
    
    
    let c = Complex::new(-0.74543,  0.11301);
    let max_iteration = 500;
    

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut(){
        let julia_val = calc_julia_val(x, scalex, x_aspect, y, scaley, y_aspect, c, &max_iteration);
        let julia_val = ((julia_val as f64/max_iteration as f64) * 255.0) as u8;

        *pixel = Rgb([(0.4*julia_val as f64) as u8,(0.7*julia_val as f64) as u8, (1.0*julia_val as f64) as u8]);
    };

    image_buffer.save("Julia_fractal.png").unwrap();

    let elapsed = now.elapsed();
    println!("Time taken: {:.2?}", elapsed);

}

fn calc_julia_val(x: u32, scalex: f64, x_aspect: f64, y: u32, scaley: f64, y_aspect: f64, c: Complex<f64>, max_iterations: &u32) -> u32 {
    let z = Complex::new(
        (x as f64 * scalex) - x_aspect/2.0,
        (y as f64 * scaley) - y_aspect/2.0
    );
    let julia_val = julia(z, &c, &max_iterations, &2.0);
    return julia_val;
}

fn julia(z: Complex<f64>, c: &Complex<f64>, max_iterations: &u32, max_r: &f64) -> u32 {
    let mut i:u32 = 0;

    let mut z = z.clone();

    for iteration in 0..*max_iterations{
        if {z.re*z.re + z.im*z.im} > *max_r{
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
