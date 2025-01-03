use std::{f64::consts, ptr};

use oxphys_numerics::{
    functions::{constant, sqrt, variable},
    traits::expression::Expression,
};

#[inline(never)]
fn native_expression(x: f64, y: f64) -> f64 {
    ((consts::PI / y * x).sqrt()) * y
}

fn main() {
    // Set up the expression.
    let x = variable(0);
    let y = variable(1);
    let pi = constant(consts::PI);
    let expr = sqrt(pi / y.clone() * x) * y;

    // The values that we'll give to x and y.
    let variables = vec![1.0, 2.0];

    // Evaluate the expression using .evaluate().
    let start = std::time::Instant::now();
    let num_iterations = 100_000_000;
    for _ in 0..num_iterations {
        let mut result = expr.evaluate(&variables);
        unsafe {
            ptr::write_volatile(&mut result, result);
        }
    }
    let duration = start.elapsed();

    // Print the duration.
    println!("Duration: {:?}", duration);

    // Now jit compile the expression and evaluate it.
    let f = expr.compile_nd().unwrap();
    let variables_ptr = variables.as_ptr();
    let variables_len = variables.len();
    let start = std::time::Instant::now();
    for _ in 0..num_iterations {
        let mut result = f(variables_ptr, variables_len);
        unsafe {
            ptr::write_volatile(&mut result, result);
        }
    }

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);

    // Now jit compiler to specifically a 2d function.
    let f = expr.compile_2d().unwrap();
    let start = std::time::Instant::now();
    for _ in 0..num_iterations {
        let mut result = f(variables[0], variables[1]);
        unsafe {
            ptr::write_volatile(&mut result, result);
        }
    }

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);

    // Now run the native function.
    let start = std::time::Instant::now();
    for _ in 0..num_iterations {
        let mut result = native_expression(variables[0], variables[1]);
        unsafe {
            ptr::write_volatile(&mut result, result);
        }
    }
    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
