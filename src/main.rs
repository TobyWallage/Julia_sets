use image::{ImageBuffer, Rgb};
use num::complex::Complex;
use std::cmp::Ordering;
use std::sync::mpsc::channel;
use std::thread;
use threadpool::ThreadPool;

fn main() {
    let (width, height):(u32, u32) = (40000, 20000);
    let scale = 2.0;
    let (x_aspect, y_aspect):(f64, f64) = get_aspects(width, height, scale);

    let scalex = x_aspect/width as f64;
    let scaley = y_aspect/height as f64;

    
    let mut image_buffer: ImageBuffer<Rgb<u8>, Vec<_>> = ImageBuffer::new(width, height);
    
    
    let c = Complex::new(-0.74543,  0.11301);
    let max_iteration = 500;
    
    let num_cores = thread::available_parallelism().unwrap().get();
    println!("{}", num_cores);

    let pool = ThreadPool::new(num_cores);

    let (sender, receiver) = channel::<(u32, u32, u32)>();

    for y in 0..height{
        let sender = sender.clone();

        pool.execute(move || for x in 0..width {
            let julia_val = calc_julia_val(x, scalex, x_aspect, y, scaley, y_aspect, c, max_iteration);
            let julia_val = ((julia_val as f64/max_iteration as f64) * 255.0) as u32;
            sender.send((x, y, julia_val)).unwrap();
        })
    }

    for _ in 0..(width*height){
        let (x, y, julia_val) = receiver.recv().unwrap_or((0, 0, 0));
        let pixel = Rgb([(0.4*julia_val as f64) as u8,(0.7*julia_val as f64) as u8, (1.0*julia_val as f64) as u8]);
        image_buffer.put_pixel(x, y, pixel);
    }

    image_buffer.save("Julia_fractal.png").unwrap();

}

fn calc_julia_val(x: u32, scalex: f64, x_aspect: f64, y: u32, scaley: f64, y_aspect: f64, c: Complex<f64>, max_iterations: u32) -> u32 {
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
