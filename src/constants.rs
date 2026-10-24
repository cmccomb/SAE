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

use uom::si::{
    f64::{Length, Mass, MassDensity, Power, Pressure, Torque},
    length::meter,
    mass::kilogram,
    mass_density::kilogram_per_cubic_meter,
    power::watt,
    pressure::pascal,
    torque::newton_meter,
};

/// Inferred model convention: `q` is density in kg/m³ and `E` is modulus in Pa.
/// The CSV headers do not label these units.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Material {
    pub(crate) density: MassDensity,
    pub(crate) modulus: Pressure,
}

/// Inferred model convention: Length and Height are meters, Mass is kilograms,
/// Power is watts, and Torque is newton-meters. The CSV omits unit labels.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Motor {
    pub(crate) length: Length,
    pub(crate) height: Length,
    pub(crate) mass: Mass,
    pub(crate) power: Power,
    pub(crate) torque: Torque,
}

/// Inferred model convention: `krsp`/`kfsp` are N/m, `crsp`/`cfsp` are N·s/m,
/// and `mrsp`/`mfsp` are kg. The CSV omits unit labels; rear and front values
/// stay separate.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Suspension {
    pub(crate) rear_spring_constant_newtons_per_meter: f64,
    pub(crate) rear_damping_coefficient_newton_seconds_per_meter: f64,
    pub(crate) rear_mass: Mass,
    pub(crate) front_spring_constant_newtons_per_meter: f64,
    pub(crate) front_damping_coefficient_newton_seconds_per_meter: f64,
    pub(crate) front_mass: Mass,
}

// The first CSV column is the public zero-based component index. Store rows in
// that order: Car::get_parameter_vector exposes these indices to callers.
fn parse_catalog<T>(
    source: &str,
    name: &str,
    parse_row: impl Fn(&csv::StringRecord, &csv::StringRecord, usize, &str) -> T,
) -> Vec<T> {
    let mut reader = csv::ReaderBuilder::new().from_reader(source.as_bytes());
    let headers = reader
        .headers()
        .unwrap_or_else(|error| panic!("{} catalog has an invalid header: {}", name, error))
        .clone();
    let mut catalog = Vec::new();
    for (row_number, result) in reader.records().enumerate() {
        let record = result.unwrap_or_else(|error| {
            panic!(
                "{} catalog row {} is invalid: {}",
                name,
                row_number + 2,
                error
            )
        });
        let index = record
            .get(0)
            .unwrap_or("")
            .parse::<usize>()
            .unwrap_or_else(|error| {
                panic!(
                    "{} catalog row {} has an invalid index: {}",
                    name,
                    row_number + 2,
                    error
                )
            });
        assert_eq!(
            index,
            row_number,
            "{} catalog row {} must have index {}",
            name,
            row_number + 2,
            row_number
        );

        catalog.push(parse_row(&headers, &record, row_number + 2, name));
    }
    assert!(!catalog.is_empty(), "{} catalog is empty", name);
    catalog
}

fn parse_number(
    headers: &csv::StringRecord,
    record: &csv::StringRecord,
    row_number: usize,
    name: &str,
    column: &str,
) -> f64 {
    let position = headers
        .iter()
        .position(|header| header == column)
        .unwrap_or_else(|| panic!("{} catalog is missing column {:?}", name, column));
    let value = record
        .get(position)
        .unwrap_or("")
        .parse::<f64>()
        .unwrap_or_else(|error| {
            panic!(
                "{} catalog row {} has invalid {}: {}",
                name, row_number, column, error
            )
        });
    assert!(
        value.is_finite(),
        "{} catalog row {} has non-finite {}",
        name,
        row_number,
        column
    );
    value
}

pub(crate) fn get_materials() -> Vec<Material> {
    parse_catalog(
        include_str!("data/materials.csv"),
        "materials",
        |headers, row, row_number, name| Material {
            density: MassDensity::new::<kilogram_per_cubic_meter>(parse_number(
                headers, row, row_number, name, "q",
            )),
            modulus: Pressure::new::<pascal>(parse_number(headers, row, row_number, name, "E")),
        },
    )
}

pub(crate) fn get_motors() -> Vec<Motor> {
    parse_catalog(
        include_str!("data/motors.csv"),
        "motors",
        |headers, row, row_number, name| Motor {
            length: Length::new::<meter>(parse_number(headers, row, row_number, name, "Length")),
            height: Length::new::<meter>(parse_number(headers, row, row_number, name, "Height")),
            mass: Mass::new::<kilogram>(parse_number(headers, row, row_number, name, "Mass")),
            power: Power::new::<watt>(parse_number(headers, row, row_number, name, "Power")),
            torque: Torque::new::<newton_meter>(parse_number(
                headers, row, row_number, name, "Torque",
            )),
        },
    )
}

pub(crate) fn get_suspensions() -> Vec<Suspension> {
    parse_catalog(
        include_str!("data/suspension.csv"),
        "suspension",
        |headers, row, row_number, name| Suspension {
            rear_spring_constant_newtons_per_meter: parse_number(
                headers, row, row_number, name, "krsp",
            ),
            rear_damping_coefficient_newton_seconds_per_meter: parse_number(
                headers, row, row_number, name, "crsp",
            ),
            rear_mass: Mass::new::<kilogram>(parse_number(headers, row, row_number, name, "mrsp")),
            front_spring_constant_newtons_per_meter: parse_number(
                headers, row, row_number, name, "kfsp",
            ),
            front_damping_coefficient_newton_seconds_per_meter: parse_number(
                headers, row, row_number, name, "cfsp",
            ),
            front_mass: Mass::new::<kilogram>(parse_number(headers, row, row_number, name, "mfsp")),
        },
    )
}

#[cfg(test)]
mod catalog_tests {
    use super::*;

    #[test]
    fn bundled_catalogs_have_expected_rows_and_values() {
        let materials = get_materials();
        assert_eq!(materials.len(), 13);
        assert_eq!(
            materials[0].density.get::<kilogram_per_cubic_meter>(),
            900.0
        );
        assert_eq!(materials[12].modulus.get::<pascal>(), 7_700_000_000.0);

        let motors = get_motors();
        assert_eq!(motors.len(), 21);
        assert_eq!(motors[0].power.get::<watt>(), 4100.0);
        assert_eq!(motors[20].torque.get::<newton_meter>(), 44.13);

        let suspensions = get_suspensions();
        assert_eq!(suspensions.len(), 5);
        assert_eq!(
            suspensions[0].rear_spring_constant_newtons_per_meter,
            4000.0
        );
        assert_eq!(
            suspensions[4].front_damping_coefficient_newton_seconds_per_meter,
            2500.0
        );
        assert_eq!(suspensions[4].front_mass.get::<kilogram>(), 0.3);
    }

    #[test]
    #[should_panic(expected = "must have index 0")]
    fn malformed_catalog_index_has_a_clear_error() {
        parse_catalog(
            "index,q\n1,900\n",
            "fixture",
            |headers, row, row_number, name| parse_number(headers, row, row_number, name, "q"),
        );
    }
}
