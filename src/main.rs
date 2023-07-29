use image::{ImageBuffer, Rgb}; // for making image
use num::complex::Complex; // for handing complex numbers
use std::cmp::Ordering; // for match statement when getting aspect ratio
use std::sync::mpsc::channel; // for Send and Receive Channels between threads
use std::thread; // for getting number of available threads
use threadpool::ThreadPool; // for threadpool to compute julia values in parrallel

fn main() {
    // Define size of image
    let (width, height): (u32, u32) = (40000, 20000);
    // Scale is sort of like the reciprocol of a Zooming into the fractal, Smaller values = More Zoomed in
    let scale = 2.0;
    // Calculate the aspect ratio so final image does not look stretched etc.
    let (x_aspect, y_aspect): (f64, f64) = get_aspects(width, height, scale);

    // Define the scale for each axis
    let scalex = x_aspect / width as f64;
    let scaley = y_aspect / height as f64;

    // Create new image buffer that will contain final image using RGB pixels
    let mut image_buffer: ImageBuffer<Rgb<u8>, Vec<_>> = ImageBuffer::new(width, height);

    // Define Complex Seed that determines the fractal from the Julia set,
    // This should have an absolute value less than (R^2 - R), currently R is hardcoded as R=2
    // Therefore the absolute value of c Should be less than 2
    let c = Complex::new(-0.74543, 0.11301);

    // Max number of iterations to perform when calulcating pixel value
    // This is will also control how bright the final image fractal may appear
    let max_iteration = 500;

    // Get number of threads available for multithreading
    let num_cores = thread::available_parallelism().unwrap().get();
    println!("{}", num_cores); // Print for debug

    // Create the threadpool with number of threads equal to num_cores
    let pool = ThreadPool::new(num_cores);

    // Create Send and Receive Channel for sending computed values from each thread
    let (sender, receiver) = channel::<(u32, u32, u32)>();

    // Iterate through height of image, creating new thread for each row
    for y in 0..height {
        // clone sender to send to a thread
        let sender = sender.clone();

        // Executes the captured closure (lambda function) on a thread in the threadpool
        // move, moves all required values for closure (lambda function) to the thread
        // Closure (lambda function) iterates through every x value on the current row computing the value
        pool.execute(move || {
            for x in 0..width {
                let julia_val =
                    calc_julia_val(x, scalex, x_aspect, y, scaley, y_aspect, c, max_iteration);
                // normalize the value from 0 to 255 for pixel value
                let julia_val = ((julia_val as f64 / max_iteration as f64) * 255.0) as u32;
                // Send the computed pixel value and the coresponding x and y values
                sender.send((x, y, julia_val)).unwrap();
            }
        })
    }

    // loop for every pixel in the image,
    // Recieve the values from the thread and place in in the correct coordinate inside the image
    for _ in 0..(width * height) {
        // retrieve value and coordinate or if it fails for some reason, uses (x = 0, y = 0,pixl = 0)
        let (x, y, julia_val) = receiver.recv().unwrap_or((0, 0, 0));
        // compute RGB pixel value
        let pixel = Rgb([
            (0.4 * julia_val as f64) as u8,
            (0.7 * julia_val as f64) as u8,
            (1.0 * julia_val as f64) as u8,
        ]);
        // place pixel in image
        image_buffer.put_pixel(x, y, pixel);
    }
    // save the image to directory for the binary was called from
    image_buffer.save("Julia_fractal.png").unwrap();
}

/// light wrapper around actual julia iterator function
fn calc_julia_val(
    x: u32,
    scalex: f64,
    x_aspect: f64,
    y: u32,
    scaley: f64,
    y_aspect: f64,
    c: Complex<f64>,
    max_iterations: u32,
) -> u32 {

    // compute the complex value that is scaled correctly from x and y value
    // it also value so image will end up centered on (0, 0)
    let z = Complex::new(
        (x as f64 * scalex) - x_aspect / 2.0,
        (y as f64 * scaley) - y_aspect / 2.0,
    );

    // call julia iterator function with parameters
    // currently, max_r (the escape value for which iteration stops) is hardcoded, may change
    let julia_val = julia(z, &c, &max_iterations, &2.0);
    return julia_val;
}

/// function that iterates and generate a value in a julia set
fn julia(z: Complex<f64>, c: &Complex<f64>, max_iterations: &u32, max_r: &f64) -> u32 {
    let mut i: u32 = 0;

    let mut z = z.clone();

    for iteration in 0..*max_iterations {
        if z.norm() > *max_r {
            break;
        }
        z = z * z + c;
        i = iteration;
    }
    return i;
}

/// computes aspect ratios and scale
fn get_aspects(width: u32, height: u32, scale: f64) -> (f64, f64) {
    let aspect_ratio = width as f64 / height as f64;
    match width.cmp(&height) {
        Ordering::Equal => return (scale, scale),
        Ordering::Greater => return (scale * aspect_ratio, scale),
        Ordering::Less => return (scale, scale * aspect_ratio),
    }
}
