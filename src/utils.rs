//! This contains some functions that are useful

use rand::thread_rng;
use rand_distr::{Distribution, Uniform};

/// Random number between 0 and 1
pub(crate) fn random_uniform(low: f64, high: f64) -> f64 {
    // Make a distribution to upll from
    let mut rng = thread_rng();
    let uniform = Uniform::new_inclusive(low, high);
    uniform.sample(&mut rng)
}
