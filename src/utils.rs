//! This contains some functions that are useful

use rand::thread_rng;
use rand_distr::{Distribution, Uniform, WeightedIndex};

/// Random number between 0 and 1
pub(crate) fn random_uniform(low: f64, high: f64) -> f64 {
    // Make a distribution to upll from
    let mut rng = thread_rng();
    let uniform = Uniform::new_inclusive(low, high);
    uniform.sample(&mut rng)
}

/// This make a multinomial draw from a set of weights - think a loaded die
pub(crate) fn multinomial_draw(weights: Vec<f64>) -> usize {
    let mut rng = thread_rng();
    let weighted = WeightedIndex::new(weights).unwrap();
    weighted.sample(&mut rng)
}
