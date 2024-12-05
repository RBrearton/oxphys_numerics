/// # Logistic function
/// The logistic function is a simple model for a smoothed-out step function. The value is 0.5
/// (exactly half way between its minimum of 0 and maximum of 1) at the midpoint_coord. The rate
/// at which the function transitions from 0 to 1 is controlled by the steepness parameter.
pub fn logistic(x_coord: f64, midpoint_coord: f64, steepness: f64) -> f64 {
    // Evaluate the logistic function.
    let exponential_argument = -steepness * (x_coord - midpoint_coord);
    1.0 / (1.0 + exponential_argument.exp())
}
