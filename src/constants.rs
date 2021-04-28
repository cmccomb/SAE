pub(crate) const CONST_BOUNDS: [[f64; 2]; 19] = [
    [0.025, 0.7],
    [0.05, 0.25],
    [0.0, std::f64::consts::FRAC_PI_4],
    [0.025, 0.2],
    [0.05, 0.5],
    [0.3, 3.0],
    [0.0, std::f64::consts::FRAC_PI_4],
    [0.025, 0.2],
    [0.05, 0.55],
    [0.05, 0.4],
    [0.0, std::f64::consts::FRAC_PI_4],
    [0.758, 1.03],
    [0.758, 1.03],
    [0.5, 1.17],
    [1.525, 3.0],
    [0.5, 2.0],
    [0.0001, 0.01],
    [0.1, 0.5],
    [0.2, 0.5],
];

pub(crate) fn get_tires() -> std::collections::HashMap<usize, std::collections::HashMap<String, f64>>
{
    let mut tires: std::collections::HashMap<usize, std::collections::HashMap<String, f64>> =
        std::collections::HashMap::new();
    tires.insert(0, build_tire(0.2286, 3.636));
    tires.insert(1, build_tire(0.22987, 4.091));
    tires.insert(2, build_tire(0.23241, 4.545));
    tires.insert(3, build_tire(0.24638, 4.545));
    tires.insert(4, build_tire(0.24765, 5.0));
    tires.insert(5, build_tire(0.26162, 5.455));
    tires.insert(6, build_tire(0.2667, 5.0));
    tires
}

fn build_tire(radius: f64, mass: f64) -> std::collections::HashMap<String, f64> {
    [("radius".to_owned(), radius), ("mass".to_owned(), mass)]
        .iter()
        .cloned()
        .collect()
}

pub(crate) fn get_brakes(
) -> std::collections::HashMap<usize, std::collections::HashMap<String, f64>> {
    let mut brakes: std::collections::HashMap<usize, std::collections::HashMap<String, f64>> =
        std::collections::HashMap::new();

    brakes.insert(0, build_brake(0.5, 0.104, 0.0325, 0.016, 0.32, 0.03));
    brakes.insert(1, build_brake(0.4, 0.1, 0.0425, 0.02, 0.35, 0.025));
    brakes.insert(2, build_brake(0.5, 0.1135, 0.04, 0.016, 0.35, 0.025));
    brakes.insert(3, build_brake(0.5, 0.1, 0.038, 0.016, 0.35, 0.03));
    brakes.insert(4, build_brake(0.5, 0.104, 0.0375, 0.016, 0.355, 0.03));
    brakes.insert(5, build_brake(0.4, 0.14, 0.049, 0.0175, 0.35, 0.025));
    brakes.insert(6, build_brake(0.52, 0.1, 0.0425, 0.02, 0.35, 0.028));
    brakes.insert(7, build_brake(0.5, 0.11, 0.052, 0.014, 0.35, 0.032));
    brakes.insert(8, build_brake(0.5, 0.1, 0.0425, 0.02, 0.355, 0.032));
    brakes.insert(9, build_brake(0.5, 0.132, 0.042, 0.018, 0.355, 0.028));
    brakes.insert(10, build_brake(0.52, 0.14, 0.042, 0.0175, 0.35, 0.028));
    brakes.insert(11, build_brake(0.63, 0.1, 0.0425, 0.02, 0.32, 0.028));
    brakes.insert(12, build_brake(0.52, 0.14, 0.049, 0.0175, 0.33, 0.025));
    brakes.insert(13, build_brake(0.63, 0.14, 0.042, 0.0175, 0.35, 0.025));
    brakes.insert(14, build_brake(0.5, 0.132, 0.052, 0.017, 0.355, 0.028));
    brakes.insert(15, build_brake(0.5, 0.132, 0.049, 0.016, 0.32, 0.032));
    brakes.insert(16, build_brake(0.4, 0.14, 0.0615, 0.0175, 0.35, 0.028));
    brakes.insert(17, build_brake(0.5, 0.14, 0.049, 0.0175, 0.35, 0.03));
    brakes.insert(18, build_brake(0.5, 0.14, 0.0615, 0.0175, 0.32, 0.028));
    brakes.insert(19, build_brake(0.5, 0.164, 0.049, 0.018, 0.355, 0.03));
    brakes.insert(20, build_brake(0.52, 0.132, 0.049, 0.022, 0.35, 0.03));
    brakes.insert(21, build_brake(0.63, 0.14, 0.049, 0.0175, 0.355, 0.03));
    brakes.insert(22, build_brake(0.63, 0.14, 0.0615, 0.0175, 0.35, 0.025));
    brakes.insert(23, build_brake(0.52, 0.14, 0.0615, 0.0175, 0.355, 0.032));
    brakes.insert(24, build_brake(0.63, 0.132, 0.049, 0.025, 0.35, 0.025));
    brakes.insert(25, build_brake(0.5, 0.152, 0.054, 0.025, 0.35, 0.028));
    brakes.insert(26, build_brake(0.5, 0.164, 0.051, 0.025, 0.355, 0.028));
    brakes.insert(27, build_brake(0.52, 0.164, 0.0645, 0.025, 0.35, 0.025));
    brakes.insert(28, build_brake(0.63, 0.164, 0.051, 0.022, 0.33, 0.03));
    brakes.insert(29, build_brake(0.52, 0.164, 0.051, 0.025, 0.35, 0.032));
    brakes.insert(30, build_brake(0.52, 0.152, 0.054, 0.028, 0.355, 0.032));
    brakes.insert(31, build_brake(0.63, 0.152, 0.054, 0.025, 0.32, 0.03));
    brakes.insert(32, build_brake(0.52, 0.164, 0.0645, 0.029, 0.355, 0.028));
    brakes.insert(33, build_brake(0.63, 0.164, 0.0645, 0.025, 0.355, 0.032));
    brakes
}

fn build_brake(
    density: f64,
    length: f64,
    height: f64,
    width: f64,
    thickness: f64,
    radius: f64,
) -> std::collections::HashMap<String, f64> {
    [
        ("density".to_owned(), density),
        ("length".to_owned(), length),
        ("height".to_owned(), height),
        ("width".to_owned(), width),
        ("thickness".to_owned(), thickness),
        ("radius".to_owned(), radius),
    ]
    .iter()
    .cloned()
    .collect()
}

pub(crate) fn get_materials(
) -> std::collections::HashMap<usize, std::collections::HashMap<String, f64>> {
    let mut materials: std::collections::HashMap<usize, std::collections::HashMap<String, f64>> =
        std::collections::HashMap::new();
    materials
}

pub(crate) fn get_motors(
) -> std::collections::HashMap<usize, std::collections::HashMap<String, f64>> {
    let mut motors: std::collections::HashMap<usize, std::collections::HashMap<String, f64>> =
        std::collections::HashMap::new();
    motors
}

pub(crate) fn get_suspensions(
) -> std::collections::HashMap<usize, std::collections::HashMap<String, f64>> {
    let mut suspensions: std::collections::HashMap<usize, std::collections::HashMap<String, f64>> =
        std::collections::HashMap::new();
    suspensions
}
