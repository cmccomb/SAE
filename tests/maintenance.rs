use sae::Car;

const PARAMETER_COUNT: usize = 39;
const EQUAL_WEIGHTS: [f64; 11] = [1.0; 11];

fn assert_close(actual: f64, expected: f64, context: &str) {
    let tolerance = 1e-10 * expected.abs().max(1.0);
    assert!(
        (actual - expected).abs() <= tolerance,
        "{}: expected {}, got {}",
        context,
        expected,
        actual
    );
}

fn assert_vector_matches(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), PARAMETER_COUNT);
    assert_eq!(expected.len(), PARAMETER_COUNT);
    for (index, (&actual, &expected)) in actual.iter().zip(expected).enumerate() {
        assert_close(actual, expected, &format!("parameter {index}"));
    }
}

#[test]
fn generated_cars_have_finite_parameters_and_objectives() {
    for _ in 0..128 {
        let car = Car::new();
        let parameters = car.get_parameter_vector();
        assert_eq!(parameters.len(), PARAMETER_COUNT);
        assert!(parameters.iter().all(|value| value.is_finite()));
        assert!(car.objectives().iter().all(|value| value.is_finite()));
        assert!(car.objective(EQUAL_WEIGHTS).is_finite());
    }
}

#[test]
fn parameter_vector_round_trips_with_distinct_rear_wing_dimensions() {
    let car = Car::new();
    let original = car.get_parameter_vector();
    let reconstructed = Car::new_from_parameters(&original);
    assert_vector_matches(&reconstructed.get_parameter_vector(), &original);
    for (index, (&actual, &expected)) in reconstructed
        .objectives()
        .iter()
        .zip(car.objectives().iter())
        .enumerate()
    {
        assert_close(actual, expected, &format!("objective {index}"));
    }

    // Both values are within their respective bounds and expose an index swap.
    let mut distinct = original;
    distinct[0] = 0.11; // rear-wing height, m
    distinct[1] = 0.22; // rear-wing length, m
    let reconstructed = Car::new_from_parameters(&distinct);
    assert_vector_matches(&reconstructed.get_parameter_vector(), &distinct);
}

#[test]
fn generated_front_wings_stay_inside_the_clearance_envelope() {
    for _ in 0..128 {
        let parameters = Car::new().get_parameter_vector();
        let height = parameters[3];
        let center_y = parameters[31];
        let lowest_center = 0.03 + height / 2.0;
        let highest_center = 0.25 - height / 2.0;
        assert!(lowest_center <= highest_center);
        assert!(
            center_y >= lowest_center - 1e-12 && center_y <= highest_center + 1e-12,
            "front wing height {} m has center {} m outside [{}, {}] m",
            height,
            center_y,
            lowest_center,
            highest_center
        );
    }
}
