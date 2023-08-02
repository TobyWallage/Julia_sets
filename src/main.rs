use image::{ImageBuffer, Rgb}; // for making image

use num::complex::Complex; // for handing complex numbers

use threadpool::ThreadPool; // for threadpool to compute julia values in parrallel

use std::cmp::Ordering; // for match statement when getting aspect ratio
use std::error::Error;
use std::fs::DirBuilder;
use std::path::Path;
use std::sync::mpsc::channel; // for Send and Receive Channels between threads
use std::thread; // for getting number of available threads
use std::time;
use std::ffi::OsStr;
use std::io::Error as IOError;
use std::io::ErrorKind;


fn main() -> Result<(), Box<dyn Error>> {
    
    let file_path = Path::new("/home/toby/images/fractal_image.png");
    
    // Size of the image
    let (width, height) = (7680,4320);
    
    // / Scale is sort of like the reciprocol of a Zooming into the fractal, Smaller values = More Zoomed in
    let scale = 2.0;
    
    // Max number of iterations to perform when calulcating pixel value
    // This is will also control how bright the final image fractal may appear
    let max_interations = 1000;
    
    // check that file path is valid
    match file_path.parent() {
        Some(parent_path) => {
            if !parent_path.exists() {
                DirBuilder::new().recursive(true).create(parent_path)?
            }
        }
        None => (),
    };

    // check that file is of type png
    match file_path.extension(){
        Some(extension_type) => if extension_type != OsStr::new("png"){
            return Err(Box::new(IOError::new(ErrorKind::InvalidInput, "Wrong file type, must end in .png")));
        },
        None => return Err(Box::new(IOError::new(ErrorKind::InvalidInput, "Wrong file type, must end in .png")))
    };
    
    // Define Complex Seed that determines the fractal from the Julia set,
    // This should have an absolute value less than (R^2 - R), currently R is hardcoded as R=2
    // Therefore the absolute value of c Should be less than 2
    let c = Complex::new(-0.74543, 0.11301);
    
    let now = time::Instant::now();

    make_fractal_image(
        file_path,
        width,
        height,
        scale,
        c,
        max_interations,
    )?;

    let elapsed = now.elapsed();
    println!("Time taken: {:.2?}", elapsed);

    Ok(())
}

fn make_fractal_image(
    file_path: &Path,
    width: u32,
    height: u32,
    scale: f64,
    c: Complex<f64>,
    max_iterations: u32,
) -> Result<(), Box<dyn Error>> {
    // Calculate the aspect ratio so final image does not look stretched etc.
    let (x_aspect, y_aspect): (f64, f64) = get_aspects(width, height, scale);

    // Define the scale for each axis
    let scalex = x_aspect / width as f64;
    let scaley = y_aspect / height as f64;

    // Create new image buffer that will contain final image using RGB pixels
    let mut image_buffer: ImageBuffer<Rgb<u8>, Vec<_>> = ImageBuffer::new(width, height);

    // Get number of threads available for multithreading
    let num_cores = thread::available_parallelism()?.get();

    // Create the threadpool with number of threads equal to num_cores
    let pool = ThreadPool::new(num_cores);

    // Create Send and Receive Channel for sending computed values from each thread
    let (sender, receiver) = channel::<(u32, Vec<u32>)>();

    // Iterate through height of image, creating new thread for each row
    for y in 0..height {
        // clone sender to send to a thread
        let sender = sender.clone();

        // Executes the captured closure (lambda function) on a thread in the threadpool
        // move, moves all required values for closure (lambda function) to the thread
        // Closure (lambda function) iterates through every x value on the current row computing the value
        pool.execute(move || {
            // Create vector with length equal to number of pixels in image row
            let mut row_values = vec![0 as u32; width as usize];

            // loop through each position in row and calculate julia value
            for x in 0..width {
                let julia_val =
                    calc_julia_val(x, scalex, x_aspect, y, scaley, y_aspect, c, max_iterations);
                // normalize the value from 0 to 255 for pixel value
                let julia_val = ((julia_val as f64 / max_iterations as f64) * 255.0) as u32;

                // store julia value in corresponding position in row vector
                row_values[x as usize] = julia_val;
            }
            // Send the computed pixel values and the coresponding y values
            sender
                .send((y, row_values))
                .expect("Unable to send results between threads!");
        })
    }

    // loop for every row of pixels in the image,
    // Recieve the values from the thread and place in in the correct coordinate inside the image
    for _ in 0..height {
        // retrieve value and coordinate or if it fails for some reason, uses (y=0,pixl = 0..)
        let (y, row_values) = receiver
            .recv()
            .unwrap_or((0, vec![0 as u32; width as usize]));
        // compute RGB pixel value
        for (x, julia_val) in row_values.iter().enumerate() {
            let julia_val = *julia_val;
            let x = x as u32;

            // maps the value to corresponding color according to colormap
            let pixel = colormap(julia_val);
            // place pixel in image
            image_buffer.put_pixel(x, y, pixel);
        }
    }

    // save the image to file path specified
    image_buffer.save(file_path)?;

    return Ok(());
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
        if { z.re * z.re + z.im * z.im } > *max_r * *max_r {
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

fn colormap(val: u32) -> Rgb<u8> {
    let colour_one: (u8, u8, u8) = (76, 201, 240);
    let colour_two: (u8, u8, u8) = (67, 97, 238);
    let colour_three: (u8, u8, u8) = (58, 12, 163);
    let colour_four: (u8, u8, u8) = (114, 9, 183);
    let colour_five: (u8, u8, u8) = (247, 37, 133);

    match val {
        (0..=63) => {
            return Rgb([
                interpolate(val as f32 / 63.0, colour_one.0, colour_two.0),
                interpolate(val as f32 / 63.0, colour_one.1, colour_two.1),
                interpolate(val as f32 / 63.0, colour_one.2, colour_two.2),
            ])
        }
        (64..=126) => {
            return Rgb([
                interpolate((val - 64) as f32 / 63.0, colour_two.0, colour_three.0),
                interpolate((val - 64) as f32 / 63.0, colour_two.1, colour_three.1),
                interpolate((val - 64) as f32 / 63.0, colour_two.2, colour_three.2),
            ])
        }
        (127..=189) => {
            return Rgb([
                interpolate((val - 127) as f32 / 63.0, colour_three.0, colour_four.0),
                interpolate((val - 127) as f32 / 63.0, colour_three.1, colour_four.1),
                interpolate((val - 127) as f32 / 63.0, colour_three.2, colour_four.2),
            ])
        }
        (190..=253) => {
            return Rgb([
                interpolate((val - 190) as f32 / 63.0, colour_four.0, colour_five.0),
                interpolate((val - 190) as f32 / 63.0, colour_four.1, colour_five.1),
                interpolate((val - 190) as f32 / 63.0, colour_four.2, colour_five.2),
            ])
        }
        (254..) => return Rgb([colour_five.0, colour_five.1, colour_five.2]),
    }
}

fn interpolate(val: f32, val_one: u8, val_two: u8) -> u8 {
    let val_dif = val_two as f32 - val_one as f32;
    let interped_val = val_one as f32 + (val_dif * val);
    return interped_val as u8;
}
