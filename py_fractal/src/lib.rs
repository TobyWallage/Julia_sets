use std::{cmp::Ordering, num::NonZeroUsize};

use std::sync::mpsc::channel;// for Send and Receive Channels between threads
use threadpool::ThreadPool; // for threadpool to compute julia values in parrallel
use std::thread; // for getting number of available threads

use pyo3::prelude::*;

use numpy::ndarray::{Array, Array2, prelude::*};
use numpy::{PyArray2, IntoPyArray};

use num::complex::Complex;

// /// Formats the sum of two numbers as string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }

#[pyfunction]
#[pyo3(name = "gen_julia_fractal")]
fn gen_julia_fractal_py(py:Python, width:u32, height:u32, scale:f64, c:Complex<f64>, max_iterations:u32) -> &PyArray2<u32>{

    gen_julia_fractal(width, height, scale, c, max_iterations).into_pyarray(py)
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_fractal(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(gen_julia_fractal_py, m)?)?;
    Ok(())
}

fn gen_julia_fractal(width:u32, height:u32, scale:f64, c:Complex<f64>, max_iterations:u32)->Array2<u32>{
    // Calculate the aspect ratio so final fractal is not stretched
    let (x_aspect, y_aspect): (f64, f64) = get_aspects(width, height, scale);

    // Define the scale for each axis
    let scalex = x_aspect / width as f64;
    let scaley = y_aspect / height as f64;

    let mut fractal_array = Array::<u32,Ix2>::zeros((height as usize, width as usize).f());

    // Get number of threads available for multithreading
    let num_cores = thread::available_parallelism().unwrap_or(NonZeroUsize::new(1).unwrap()).get();

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
            let mut row_values = Vec::with_capacity(width as usize);

            // loop through each position in row and calculate julia value
            for x in 0..width {
                let z = Complex::new((x as f64 * scalex) - x_aspect / 2.0,(y as f64 * scaley) - y_aspect / 2.0,);

                let julia_val = julia(z, c, max_iterations, 2.0);

                // store julia value in corresponding position in row vector
                row_values.push(julia_val);
            }
            // Send the computed pixel values and the coresponding y values
            sender
                .send((y, row_values))
                .expect("Unable to send results between threads!");
        })
    }

    // loop for every row of values in fractal array
    for _ in 0..height{
        // retrieve value and coordinate or if it fails for some reason, skips that row
        let (y,row_values) = match receiver.recv(){
            Ok(ok_val) => ok_val,
            _ => continue
        };
        fractal_array.row_mut(y as usize).assign(&Array::from(row_values))
    }

    return fractal_array;
}

/// computes aspect ratios and scale
fn get_aspects(width: u32, height: u32, scale: f64) -> (f64, f64) {
    let aspect_ratio = width as f64 / height as f64;
    match width.cmp(&height) {
        Ordering::Equal => return (scale, scale),
        Ordering::Greater => return (scale * aspect_ratio, scale),
        Ordering::Less => return (scale, scale / aspect_ratio),
    }
}

/// function that iterates and generate a value in a julia set
fn julia(z: Complex<f64>, c: Complex<f64>, max_iterations: u32, max_r: f64) -> u32 {
    let mut i: u32 = 0;

    let mut z = z.clone();

    for iteration in 0..max_iterations {
        if { z.re * z.re + z.im * z.im } > max_r * max_r {
            break;
        }
        z = z * z + c;
        i = iteration;
    }
    return i;
}