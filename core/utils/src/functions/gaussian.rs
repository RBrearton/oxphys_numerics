/// # Gaussian function
/// An implementation of a simple Gaussian function.
/// This function takes in the x-coordinate, the mean, the standard deviation, and an optional
/// prefactor. If the prefactor is not provided, the function will, by default, make sure that the
/// Gaussian is normalized.
pub fn gaussian(x_coord: f64, mean: f64, std_dev: f64, prefactor: Option<f64>) -> f64 {
    // Check to see if we were given the prefactor.
    let prefactor = match prefactor {
        Some(value) => value,
        None => 1.0 / (std_dev * (2.0 * std::f64::consts::PI).sqrt()),
    };

    // Evaluate the Gaussian, multiplying by the appropriate prefactor.
    let exponential_argument = -0.5 * ((x_coord - mean) / std_dev).powi(2);
    prefactor * exponential_argument.exp()
}
