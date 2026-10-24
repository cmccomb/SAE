mod constants;
mod utils;

use uom::si::{
    angle::radian,
    f64::{Angle, Length, Mass, MassDensity, Power, Pressure, Torque},
    length::meter,
    mass::kilogram,
    mass_density::kilogram_per_cubic_meter,
    pressure::bar,
};

// Car stores physical quantities with uom. The existing model equations take
// their SI base-unit f64 values at the calculation boundary (`.value`);
// the legacy tire-pressure equations explicitly take bar instead.
const V_CAR: f64 = 26.8;
const W_E: f64 = 3600.0 * 2.0 * std::f64::consts::PI / 60.0;
const RHO_AIR: f64 = 1.225;
const R_TRACK: f64 = 9.0;
const P_BRAKE: f64 = 10_000_000.0;
const C_DC: f64 = 0.04;
const GRAVITY: f64 = 9.81;
const Y_SUSPENSION: f64 = 0.05;
const DYDT_SUSPENSION: f64 = 0.025;

// Weights
const EVEN_WEIGHTS: [f64; 11] = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
const WEIGHTS1: [i32; 11] = [14, 1, 20, 30, 10, 1, 1, 10, 10, 2, 1]; // TODO: Make f64
const WEIGHTS2: [i32; 11] = [25, 1, 15, 20, 15, 1, 1, 15, 5, 1, 1]; // TODO: Make f64
const WEIGHTS3: [i32; 11] = [14, 1, 20, 15, 25, 1, 1, 10, 10, 2, 1]; // TODO: Make f64

#[derive(Default)]
pub struct Car {
    rear_wing_material_index: usize,
    front_wing_material_index: usize,
    side_wing_material_index: usize,
    cabin_material_index: usize,
    impact_attenuator_material_index: usize,
    rear_tire_index: usize,
    front_tire_index: usize,
    brake_index: usize,
    motor_index: usize,
    suspension_index: usize,
    rear_wing_height: Length,
    rear_wing_length: Length,
    rear_wing_angle_of_attack: Angle,
    front_wing_height: Length,
    front_wing_length: Length,
    front_wing_width: Length,
    front_wing_angle_of_attack: Angle,
    side_wings_height: Length,
    side_wings_length: Length,
    side_wings_width: Length,
    side_wings_angle_of_attack: Angle,
    rear_tire_pressure: Pressure,
    front_tire_pressure: Pressure,
    cabin_height: Length,
    cabin_length: Length,
    cabin_width: Length,
    cabin_thickness: Length,
    impact_attenuator_height: Length,
    impact_attenuator_width: Length,
    rear_wing_density: MassDensity,
    front_wing_density: MassDensity,
    side_wing_density: MassDensity,
    cabin_density: MassDensity,
    impact_attenuator_density: MassDensity,
    impact_attenuator_modulus: Pressure,
    rear_tire_radius: Length,
    rear_tire_mass: Mass,
    front_tire_radius: Length,
    front_tire_mass: Mass,
    engine_power: Power,
    engine_length: Length,
    engine_height: Length,
    engine_torque: Torque,
    engine_mass: Mass,
    brake_radius: Length,
    brake_density: MassDensity,
    brake_length: Length,
    brake_height: Length,
    brake_width: Length,
    brake_thickness: Length,
    rear_suspension_spring_constant: f64,
    rear_suspension_damping_coefficient: f64,
    rear_suspension_mass: Mass,
    front_suspension_spring_constant: f64,
    front_suspension_damping_coefficient: f64,
    front_suspension_mass: Mass,
    rear_wing_width: Length,
    rear_wing_y_position: Length,
    front_wing_y_position: Length,
    side_wing_y_position: Length,
    engine_y_position: Length,
    cabin_y_position: Length,
    impact_attenuator_length: Length,
    impact_attenuator_y_position: Length,
    rear_suspension_y_position: Length,
    front_suspension_y_position: Length,
}

impl Car {
    // The front-wing Y parameter is its center. Using half its height on both
    // sides keeps the wing inside the 0.03–0.25 m clearance envelope. The
    // source workbook used a full height on the lower side, which can invert
    // the range for otherwise valid front-wing heights.
    fn front_wing_center_y_bounds(height: Length) -> (f64, f64) {
        let half_height_m = height.get::<meter>() / 2.0;
        (0.03 + half_height_m, 0.25 - half_height_m)
    }

    pub fn new() -> Self {
        // Get libraries
        let tires = constants::get_tires();
        let brakes = constants::get_brakes();
        let motors = constants::get_motors();
        let materials = constants::get_materials();
        let suspensions = constants::get_suspensions();

        // Define indicates
        let rear_tire_index = utils::multinomial_draw(vec![1.0; tires.len()]);
        let front_tire_index = utils::multinomial_draw(vec![1.0; tires.len()]);
        let brake_index = utils::multinomial_draw(vec![1.0; brakes.len()]);
        let motor_index = utils::multinomial_draw(vec![1.0; motors.len()]);
        let suspension_index = utils::multinomial_draw(vec![1.0; suspensions.len()]);
        let rear_wing_material_index = utils::multinomial_draw(vec![1.0; materials.len()]);
        let front_wing_material_index = utils::multinomial_draw(vec![1.0; materials.len()]);
        let side_wing_material_index = utils::multinomial_draw(vec![1.0; materials.len()]);
        let cabin_material_index = utils::multinomial_draw(vec![1.0; materials.len()]);
        let impact_attenuator_material_index = utils::multinomial_draw(vec![1.0; materials.len()]);

        // Variables used in multiple places
        let impact_attenuator_height = Length::new::<meter>(utils::random_uniform(
            constants::CONST_BOUNDS[17][0],
            constants::CONST_BOUNDS[17][1],
        ));
        let front_wing_length = Length::new::<meter>(utils::random_uniform(
            constants::CONST_BOUNDS[4][0],
            constants::CONST_BOUNDS[4][1],
        ));
        let cabin_height = Length::new::<meter>(utils::random_uniform(
            constants::CONST_BOUNDS[13][0],
            constants::CONST_BOUNDS[13][1],
        ));
        let rear_wing_height = Length::new::<meter>(utils::random_uniform(
            constants::CONST_BOUNDS[0][0],
            constants::CONST_BOUNDS[0][1],
        ));
        let front_wing_height = Length::new::<meter>(utils::random_uniform(
            constants::CONST_BOUNDS[3][0],
            constants::CONST_BOUNDS[3][1],
        ));
        let side_wings_height = Length::new::<meter>(utils::random_uniform(
            constants::CONST_BOUNDS[7][0],
            constants::CONST_BOUNDS[7][1],
        ));
        let (front_wing_y_min, front_wing_y_max) =
            Self::front_wing_center_y_bounds(front_wing_height);

        Car {
            // Index variables
            rear_wing_material_index,
            front_wing_material_index,
            side_wing_material_index,
            cabin_material_index,
            impact_attenuator_material_index,
            rear_tire_index,
            front_tire_index,
            brake_index,
            motor_index,
            suspension_index,
            // Parameters with uniform bounds
            rear_wing_height,
            rear_wing_length: Length::new::<meter>(utils::random_uniform(
                constants::CONST_BOUNDS[1][0],
                constants::CONST_BOUNDS[1][1],
            )),
            rear_wing_angle_of_attack: Angle::new::<radian>(utils::random_uniform(
                constants::CONST_BOUNDS[2][0],
                constants::CONST_BOUNDS[2][1],
            )),
            front_wing_height,
            front_wing_length,
            front_wing_width: Length::new::<meter>(utils::random_uniform(
                constants::CONST_BOUNDS[5][0],
                constants::CONST_BOUNDS[5][1],
            )),
            front_wing_angle_of_attack: Angle::new::<radian>(utils::random_uniform(
                constants::CONST_BOUNDS[6][0],
                constants::CONST_BOUNDS[6][1],
            )),
            side_wings_height,
            side_wings_length: Length::new::<meter>(utils::random_uniform(
                constants::CONST_BOUNDS[8][0],
                constants::CONST_BOUNDS[8][1],
            )),
            side_wings_width: Length::new::<meter>(utils::random_uniform(
                constants::CONST_BOUNDS[9][0],
                constants::CONST_BOUNDS[9][1],
            )),
            side_wings_angle_of_attack: Angle::new::<radian>(utils::random_uniform(
                constants::CONST_BOUNDS[10][0],
                constants::CONST_BOUNDS[10][1],
            )),
            rear_tire_pressure: Pressure::new::<bar>(utils::random_uniform(
                constants::CONST_BOUNDS[11][0],
                constants::CONST_BOUNDS[11][1],
            )),
            front_tire_pressure: Pressure::new::<bar>(utils::random_uniform(
                constants::CONST_BOUNDS[12][0],
                constants::CONST_BOUNDS[12][1],
            )),
            cabin_height,
            cabin_length: Length::new::<meter>(utils::random_uniform(
                constants::CONST_BOUNDS[14][0],
                constants::CONST_BOUNDS[14][1],
            )),
            cabin_width: Length::new::<meter>(utils::random_uniform(
                constants::CONST_BOUNDS[15][0],
                constants::CONST_BOUNDS[15][1],
            )),
            cabin_thickness: Length::new::<meter>(utils::random_uniform(
                constants::CONST_BOUNDS[16][0],
                constants::CONST_BOUNDS[16][1],
            )),
            impact_attenuator_height,
            impact_attenuator_width: Length::new::<meter>(utils::random_uniform(
                constants::CONST_BOUNDS[18][0],
                constants::CONST_BOUNDS[18][1],
            )),

            // Parameters based on lookup from indices
            rear_wing_density: materials[rear_wing_material_index].density,
            front_wing_density: materials[front_wing_material_index].density,
            side_wing_density: materials[side_wing_material_index].density,
            cabin_density: materials[cabin_material_index].density,
            impact_attenuator_density: materials[impact_attenuator_material_index].density,
            impact_attenuator_modulus: materials[impact_attenuator_material_index].modulus,
            rear_tire_radius: Length::new::<meter>(tires[&rear_tire_index]["radius"]),
            rear_tire_mass: Mass::new::<kilogram>(tires[&rear_tire_index]["mass"]),
            front_tire_radius: Length::new::<meter>(tires[&front_tire_index]["radius"]),
            front_tire_mass: Mass::new::<kilogram>(tires[&front_tire_index]["mass"]),
            engine_power: motors[motor_index].power,
            engine_length: motors[motor_index].length,
            engine_height: motors[motor_index].height,
            engine_torque: motors[motor_index].torque,
            engine_mass: motors[motor_index].mass,
            brake_radius: Length::new::<meter>(brakes[&brake_index]["radius"]),
            brake_density: MassDensity::new::<kilogram_per_cubic_meter>(
                brakes[&brake_index]["density"],
            ),
            brake_length: Length::new::<meter>(brakes[&brake_index]["length"]),
            brake_height: Length::new::<meter>(brakes[&brake_index]["height"]),
            brake_width: Length::new::<meter>(brakes[&brake_index]["width"]),
            brake_thickness: Length::new::<meter>(brakes[&brake_index]["thickness"]),
            rear_suspension_spring_constant: suspensions[suspension_index]
                .rear_spring_constant_newtons_per_meter,
            rear_suspension_damping_coefficient: suspensions[suspension_index]
                .rear_damping_coefficient_newton_seconds_per_meter,
            rear_suspension_mass: suspensions[suspension_index].rear_mass,
            front_suspension_spring_constant: suspensions[suspension_index]
                .front_spring_constant_newtons_per_meter,
            front_suspension_damping_coefficient: suspensions[suspension_index]
                .front_damping_coefficient_newton_seconds_per_meter,
            front_suspension_mass: suspensions[suspension_index].front_mass,

            // Parameters with variables bounds
            rear_wing_width: Length::new::<meter>(utils::random_uniform(
                0.3,
                9.0 - 2.0 * tires[&rear_tire_index]["radius"],
            )),
            rear_wing_y_position: Length::new::<meter>(utils::random_uniform(
                0.5 + rear_wing_height.value / 2.0,
                1.2 - rear_wing_height.value / 2.0,
            )),
            front_wing_y_position: Length::new::<meter>(utils::random_uniform(
                front_wing_y_min,
                front_wing_y_max,
            )),
            side_wing_y_position: Length::new::<meter>(utils::random_uniform(
                0.03 + side_wings_height.value / 2.0,
                0.25 - side_wings_height.value / 2.0,
            )),
            engine_y_position: Length::new::<meter>(utils::random_uniform(
                0.03 + motors[motor_index].height.value / 2.0,
                0.5 - motors[motor_index].height.value / 2.0,
            )),
            cabin_y_position: Length::new::<meter>(utils::random_uniform(
                0.03 + cabin_height.value / 2.0,
                1.2 - cabin_height.value / 2.0,
            )),
            impact_attenuator_length: Length::new::<meter>(utils::random_uniform(
                0.2,
                0.7 - front_wing_length.value,
            )),
            impact_attenuator_y_position: Length::new::<meter>(utils::random_uniform(
                0.03 + impact_attenuator_height.value / 2.0,
                1.2 - impact_attenuator_height.value / 2.0,
            )),
            rear_suspension_y_position: Length::new::<meter>(utils::random_uniform(
                tires[&rear_tire_index]["radius"],
                2.0 * tires[&rear_tire_index]["radius"],
            )),
            front_suspension_y_position: Length::new::<meter>(utils::random_uniform(
                tires[&front_tire_index]["radius"],
                2.0 * tires[&front_tire_index]["radius"],
            )),
        }
    }

    /// Returns 39 optimizer parameters in this fixed order:
    ///
    /// - 0–2: rear-wing height (m), length (m), angle (rad)
    /// - 3–6: front-wing height (m), length (m), width (m), angle (rad)
    /// - 7–10: side-wing height (m), length (m), width (m), angle (rad)
    /// - 11–12: rear and front tire pressure (bar)
    /// - 13–16: cabin height, length, width, thickness (m)
    /// - 17–18: impact-attenuator height and width (m)
    /// - 19–23: rear-wing, front-wing, side-wing, cabin, impact material indices
    /// - 24–28: rear tire, front tire, brake, motor, suspension indices
    /// - 29–34: rear-wing width and rear-wing, front-wing, side-wing, engine,
    ///   cabin center Y positions (m)
    /// - 35–38: impact-attenuator length and center Y, rear and front
    ///   suspension center Y positions (m)
    pub fn get_parameter_vector(&self) -> Vec<f64> {
        vec![
            self.rear_wing_height.value,
            self.rear_wing_length.value,
            self.rear_wing_angle_of_attack.value,
            self.front_wing_height.value,
            self.front_wing_length.value,
            self.front_wing_width.value,
            self.front_wing_angle_of_attack.value,
            self.side_wings_height.value,
            self.side_wings_length.value,
            self.side_wings_width.value,
            self.side_wings_angle_of_attack.value,
            self.rear_tire_pressure.get::<bar>(),
            self.front_tire_pressure.get::<bar>(),
            self.cabin_height.value,
            self.cabin_length.value,
            self.cabin_width.value,
            self.cabin_thickness.value,
            self.impact_attenuator_height.value,
            self.impact_attenuator_width.value,
            self.rear_wing_material_index as f64,
            self.front_wing_material_index as f64,
            self.side_wing_material_index as f64,
            self.cabin_material_index as f64,
            self.impact_attenuator_material_index as f64,
            self.rear_tire_index as f64,
            self.front_tire_index as f64,
            self.brake_index as f64,
            self.motor_index as f64,
            self.suspension_index as f64,
            self.rear_wing_width.value,
            self.rear_wing_y_position.value,
            self.front_wing_y_position.value,
            self.side_wing_y_position.value,
            self.engine_y_position.value,
            self.cabin_y_position.value,
            self.impact_attenuator_length.value,
            self.impact_attenuator_y_position.value,
            self.rear_suspension_y_position.value,
            self.front_suspension_y_position.value,
        ]
    }

    /// Reconstructs a car from the same parameter order and units returned by
    /// `get_parameter_vector`.
    pub fn new_from_parameters(p: &Vec<f64>) -> Self {
        // Get libraries
        let tires = constants::get_tires();
        let brakes = constants::get_brakes();
        let motors = constants::get_motors();
        let materials = constants::get_materials();
        let suspensions = constants::get_suspensions();

        // Pull out indices
        let rear_wing_material_index = p[19] as usize;
        let front_wing_material_index = p[20] as usize;
        let side_wing_material_index = p[21] as usize;
        let cabin_material_index = p[22] as usize;
        let impact_attenuator_material_index = p[23] as usize;
        let rear_tire_index = p[24] as usize;
        let front_tire_index = p[25] as usize;
        let brake_index = p[26] as usize;
        let motor_index = p[27] as usize;
        let suspension_index = p[28] as usize;

        Car {
            rear_wing_material_index,
            front_wing_material_index,
            side_wing_material_index,
            cabin_material_index,
            impact_attenuator_material_index,
            rear_tire_index,
            front_tire_index,
            brake_index,
            motor_index,
            suspension_index,
            rear_wing_height: Length::new::<meter>(p[0]),
            rear_wing_length: Length::new::<meter>(p[1]),
            rear_wing_angle_of_attack: Angle::new::<radian>(p[2]),
            front_wing_height: Length::new::<meter>(p[3]),
            front_wing_length: Length::new::<meter>(p[4]),
            front_wing_width: Length::new::<meter>(p[5]),
            front_wing_angle_of_attack: Angle::new::<radian>(p[6]),
            side_wings_height: Length::new::<meter>(p[7]),
            side_wings_length: Length::new::<meter>(p[8]),
            side_wings_width: Length::new::<meter>(p[9]),
            side_wings_angle_of_attack: Angle::new::<radian>(p[10]),
            rear_tire_pressure: Pressure::new::<bar>(p[11]),
            front_tire_pressure: Pressure::new::<bar>(p[12]),
            cabin_height: Length::new::<meter>(p[13]),
            cabin_length: Length::new::<meter>(p[14]),
            cabin_width: Length::new::<meter>(p[15]),
            cabin_thickness: Length::new::<meter>(p[16]),
            impact_attenuator_height: Length::new::<meter>(p[17]),
            impact_attenuator_width: Length::new::<meter>(p[18]),
            rear_wing_density: materials[rear_wing_material_index].density,
            front_wing_density: materials[front_wing_material_index].density,
            side_wing_density: materials[side_wing_material_index].density,
            cabin_density: materials[cabin_material_index].density,
            impact_attenuator_density: materials[impact_attenuator_material_index].density,
            impact_attenuator_modulus: materials[impact_attenuator_material_index].modulus,
            rear_tire_radius: Length::new::<meter>(tires[&rear_tire_index]["radius"]),
            rear_tire_mass: Mass::new::<kilogram>(tires[&rear_tire_index]["mass"]),
            front_tire_radius: Length::new::<meter>(tires[&front_tire_index]["radius"]),
            front_tire_mass: Mass::new::<kilogram>(tires[&front_tire_index]["mass"]),
            engine_power: motors[motor_index].power,
            engine_length: motors[motor_index].length,
            engine_height: motors[motor_index].height,
            engine_torque: motors[motor_index].torque,
            engine_mass: motors[motor_index].mass,
            brake_radius: Length::new::<meter>(brakes[&brake_index]["radius"]),
            brake_density: MassDensity::new::<kilogram_per_cubic_meter>(
                brakes[&brake_index]["density"],
            ),
            brake_length: Length::new::<meter>(brakes[&brake_index]["length"]),
            brake_height: Length::new::<meter>(brakes[&brake_index]["height"]),
            brake_width: Length::new::<meter>(brakes[&brake_index]["width"]),
            brake_thickness: Length::new::<meter>(brakes[&brake_index]["thickness"]),
            rear_suspension_spring_constant: suspensions[suspension_index]
                .rear_spring_constant_newtons_per_meter,
            rear_suspension_damping_coefficient: suspensions[suspension_index]
                .rear_damping_coefficient_newton_seconds_per_meter,
            rear_suspension_mass: suspensions[suspension_index].rear_mass,
            front_suspension_spring_constant: suspensions[suspension_index]
                .front_spring_constant_newtons_per_meter,
            front_suspension_damping_coefficient: suspensions[suspension_index]
                .front_damping_coefficient_newton_seconds_per_meter,
            front_suspension_mass: suspensions[suspension_index].front_mass,
            rear_wing_width: Length::new::<meter>(p[29]),
            rear_wing_y_position: Length::new::<meter>(p[30]),
            front_wing_y_position: Length::new::<meter>(p[31]),
            side_wing_y_position: Length::new::<meter>(p[32]),
            engine_y_position: Length::new::<meter>(p[33]),
            cabin_y_position: Length::new::<meter>(p[34]),
            impact_attenuator_length: Length::new::<meter>(p[35]),
            impact_attenuator_y_position: Length::new::<meter>(p[36]),
            rear_suspension_y_position: Length::new::<meter>(p[37]),
            front_suspension_y_position: Length::new::<meter>(p[38]),
        }
    }

    pub fn objective(&self, weights: [f64; 11]) -> f64 {
        weights[0] * self.mass()
            + weights[1] * self.center_of_gravity()
            + weights[2] * self.total_drag_force()
            + weights[3] * self.total_downward_force()
            + weights[4] * self.acceleration()
            + weights[5] * self.crash_force()
            + weights[6] * self.impact_attenuator_volume()
            + weights[7] * self.corner_velocity()
            + weights[8] * self.brakeing_distance()
            + weights[9] * self.suspension_acceleration()
            + weights[10] * self.pitch_moment()
    }

    pub fn objectives(&self) -> [f64; 11] {
        [
            self.mass(),
            self.center_of_gravity(),
            self.total_drag_force(),
            self.total_downward_force(),
            self.acceleration(),
            self.crash_force(),
            self.impact_attenuator_volume(),
            self.corner_velocity(),
            self.brakeing_distance(),
            self.suspension_acceleration(),
            self.pitch_moment(),
        ]
    }

    pub fn check_bounds(&self) -> Vec<f64> {
        todo!()
    }

    pub fn check_nonlinear_constraints(&self) -> Vec<f64> {
        todo!()
    }

    pub fn check_linear_constraints(&self) -> Vec<f64> {
        todo!()
    }

    pub fn center_of_gravity(&self) -> f64 {
        let total_mass = self.mass();
        let t1 = (self.mass_rear_wing() * self.rear_wing_y_position.value
            + self.mass_front_wing() * self.front_wing_y_position.value
            + self.engine_mass.value * self.engine_y_position.value
            + self.mass_cabin() * self.cabin_y_position.value
            + self.mass_impact_attenuator() * self.impact_attenuator_y_position.value)
            / total_mass;
        let t2 = 2.0
            * (self.mass_side_wings() * self.side_wing_y_position.value
                + self.rear_tire_mass.value * self.rear_tire_radius.value
                + self.front_tire_mass.value * self.front_tire_radius.value
                + self.mass_brake() * self.front_tire_radius.value
                + self.rear_suspension_mass.value * self.rear_suspension_y_position.value
                + self.front_suspension_mass.value * self.front_suspension_y_position.value)
            / total_mass;
        return t1 + t2;
    }

    pub fn mass(&self) -> f64 {
        self.mass_rear_wing()
            + self.mass_front_wing()
            + 2.0 * self.mass_side_wings()
            + 2.0 * self.rear_tire_mass.value
            + 2.0 * self.front_tire_mass.value
            + self.engine_mass.value
            + self.mass_cabin()
            + self.mass_impact_attenuator()
            + 4.0 * self.mass_brake()
            + 2.0 * self.rear_suspension_mass.value
            + 2.0 * self.front_suspension_mass.value
    }

    fn mass_rear_wing(&self) -> f64 {
        self.rear_wing_length.value
            * self.rear_wing_width.value
            * self.rear_wing_height.value
            * self.rear_wing_density.value
    }

    fn mass_front_wing(&self) -> f64 {
        self.front_wing_length.value
            * self.front_wing_width.value
            * self.front_wing_height.value
            * self.front_wing_density.value
    }

    fn mass_side_wings(&self) -> f64 {
        self.side_wings_length.value
            * self.side_wings_width.value
            * self.side_wings_height.value
            * self.side_wing_density.value
    }
    fn mass_impact_attenuator(&self) -> f64 {
        self.impact_attenuator_length.value
            * self.impact_attenuator_width.value
            * self.impact_attenuator_height.value
            * self.impact_attenuator_density.value
    }
    fn mass_cabin(&self) -> f64 {
        2.0 * (self.cabin_height.value * self.cabin_length.value * self.cabin_thickness.value
            + self.cabin_height.value * self.cabin_width.value * self.cabin_thickness.value
            + self.cabin_length.value * self.cabin_height.value * self.cabin_thickness.value)
            * self.cabin_density.value
    }
    fn mass_brake(&self) -> f64 {
        self.brake_length.value
            * self.brake_width.value
            * self.brake_height.value
            * self.brake_density.value
    }

    // aspect ratio of wing
    fn aspect_ratio(w: f64, alpha: f64, l: f64) -> f64 {
        w * alpha.cos() / l
    }

    // lift co-effecient
    fn lift_coefficient(aspect_ratio: f64, alpha: f64) -> f64 {
        2.0 * std::f64::consts::PI * (aspect_ratio / (aspect_ratio + 2.0)) * alpha
    }

    // drag co-efficient
    fn drag_coefficient(lift_coefficient: f64, aspect_ratio: f64) -> f64 {
        return lift_coefficient.powi(2) / (std::f64::consts::PI * aspect_ratio);
    }

    // wing downforce
    fn wing_down_force(&self, w: f64, h: f64, l: f64, alpha: f64) -> f64 {
        let wing_aspect_ratio = Car::aspect_ratio(w, alpha, l);
        let c_lift = Car::lift_coefficient(wing_aspect_ratio, alpha);
        0.5 * alpha * h * w * RHO_AIR * (V_CAR.powi(2)) * c_lift
    }

    // wing drag
    fn wing_drag_force(&self, w: f64, h: f64, l: f64, alpha: f64) -> f64 {
        let wing_aspect_ratio = Car::aspect_ratio(w, alpha, l);
        let c_lift = Car::lift_coefficient(wing_aspect_ratio, alpha);
        let c_drag = Car::drag_coefficient(c_lift, wing_aspect_ratio);
        Car::drag_force(w, h, c_drag)
    }

    // drag
    fn drag_force(w: f64, h: f64, c_drag: f64) -> f64 {
        return 0.5 * w * h * RHO_AIR * V_CAR.powi(2) * c_drag;
    }
    // # objective 3 - total drag (minimize)
    fn total_drag_force(&self) -> f64 {
        let cabinDrag = Car::drag_force(self.cabin_width.value, self.cabin_height.value, C_DC);
        let rearWingDrag = self.wing_drag_force(
            self.rear_wing_width.value,
            self.rear_wing_height.value,
            self.rear_wing_length.value,
            self.rear_wing_angle_of_attack.value,
        );
        let frontWingDrag = self.wing_drag_force(
            self.front_wing_width.value,
            self.front_wing_height.value,
            self.front_wing_length.value,
            self.front_wing_angle_of_attack.value,
        );
        let sideWingDrag = self.wing_drag_force(
            self.side_wings_width.value,
            self.side_wings_height.value,
            self.side_wings_length.value,
            self.side_wings_angle_of_attack.value,
        );
        rearWingDrag + frontWingDrag + 2.0 * sideWingDrag + cabinDrag
    }

    // # objective 4 - total downforce (maximize)
    fn total_downward_force(&self) -> f64 {
        let downForceRearWing = self.wing_down_force(
            self.rear_wing_width.value,
            self.rear_wing_height.value,
            self.rear_wing_length.value,
            self.rear_wing_angle_of_attack.value,
        );
        let downForceFrontWing = self.wing_down_force(
            self.front_wing_width.value,
            self.front_wing_height.value,
            self.front_wing_length.value,
            self.front_wing_angle_of_attack.value,
        );
        let downForceSideWing = self.wing_down_force(
            self.side_wings_width.value,
            self.side_wings_height.value,
            self.side_wings_length.value,
            self.side_wings_angle_of_attack.value,
        );
        downForceRearWing + downForceFrontWing + 2.0 * downForceSideWing
    }

    // rolling resistance
    fn rolling_resistance(&self, tire_pressure_bar: f64) -> f64 {
        let c = 0.005 + 1.0 / tire_pressure_bar * (0.01 + 0.0095 * ((V_CAR * 3.6 / 100.0).powi(2)));
        return c * self.mass() * GRAVITY;
    }
    // # objective 5 - acceleration (maximize)
    fn acceleration(&self) -> f64 {
        let total_resistance =
            self.total_drag_force() + self.rolling_resistance(self.rear_tire_pressure.get::<bar>());

        let w_wheels = V_CAR / self.rear_tire_radius.value;
        let efficiency = total_resistance * V_CAR / self.engine_power.value;
        let torque = self.engine_torque.value;

        let f_wheels = torque * efficiency * W_E / (self.rear_tire_radius.value * w_wheels);

        (f_wheels - total_resistance) / self.mass()
    }

    // # objective 6 - crash force (minimize)
    fn crash_force(&self) -> f64 {
        (self.mass()
            * V_CAR.powi(2)
            * self.impact_attenuator_width.value
            * self.impact_attenuator_height.value
            * self.impact_attenuator_modulus.value
            / (2.0 * self.impact_attenuator_length.value))
            .sqrt()
    }

    // # objective 7 - impact attenuator volume (minimize)
    fn impact_attenuator_volume(&self) -> f64 {
        self.impact_attenuator_length.value
            * self.impact_attenuator_height.value
            * self.impact_attenuator_width.value
    }

    fn suspension_force(k: f64, c: f64) -> f64 {
        k * Y_SUSPENSION + c * DYDT_SUSPENSION
    }
    // # objective 8 - corner velocity in skid pad (maximize)
    fn corner_velocity(&self) -> f64 {
        let F_fsp = Car::suspension_force(
            self.front_suspension_spring_constant,
            self.front_suspension_damping_coefficient,
        );
        let F_rsp = Car::suspension_force(
            self.rear_suspension_spring_constant,
            self.rear_suspension_damping_coefficient,
        );
        let downforce = self.total_downward_force();
        let mTotal = self.mass();

        let Clat = 1.6;
        let forces = downforce + mTotal * GRAVITY - 2.0 * F_fsp - 2.0 * F_rsp;

        return if forces < 0.0 {
            0.0
        } else {
            (forces * Clat * R_TRACK / mTotal).sqrt()
        };
    }
    // # objective 9 - (minimize)
    fn brakeing_distance(&self) -> f64 {
        let mTotal = self.mass();
        let C = 0.005
            + 1.0 / self.rear_tire_pressure.get::<bar>()
                * (0.01 + 0.0095 * ((V_CAR * 3.6 / 100.0).powi(2)));

        let A_brk = self.brake_height.value * self.brake_width.value;
        let c_brk = 0.37;
        let Tbrk = 2.0 * c_brk * P_BRAKE * A_brk * self.brake_radius.value;

        let Ffsp = Car::suspension_force(
            self.front_suspension_spring_constant,
            self.front_suspension_damping_coefficient,
        );
        let Frsp = Car::suspension_force(
            self.rear_suspension_spring_constant,
            self.rear_suspension_damping_coefficient,
        );
        let mut Fy = mTotal * GRAVITY + self.total_downward_force() - 2.0 * Frsp - 2.0 * Ffsp;
        if Fy <= 0.0 {
            Fy = 1e-10;
        }
        let a_brk = Fy * C / mTotal + 4.0 * Tbrk / (self.rear_tire_radius.value * mTotal);
        return V_CAR.powi(2) / (2.0 * a_brk);
    }

    // # objective 10 - (minimize)
    fn suspension_acceleration(&self) -> f64 {
        let Ffsp = Car::suspension_force(
            self.front_suspension_spring_constant,
            self.front_suspension_damping_coefficient,
        );
        let Frsp = Car::suspension_force(
            self.rear_suspension_spring_constant,
            self.rear_suspension_damping_coefficient,
        );
        let mTotal = self.mass();
        let Fd = self.total_downward_force();
        return -(2.0 * Ffsp - 2.0 * Frsp - mTotal * GRAVITY - Fd) / mTotal;
    }

    // # objective 11 - (minimize)
    fn pitch_moment(&self) -> f64 {
        let Ffsp = Car::suspension_force(
            self.front_suspension_spring_constant,
            self.front_suspension_damping_coefficient,
        );
        let Frsp = Car::suspension_force(
            self.rear_suspension_spring_constant,
            self.rear_suspension_damping_coefficient,
        );
        let downForceRearWing = self.wing_down_force(
            self.rear_wing_width.value,
            self.rear_wing_height.value,
            self.rear_wing_length.value,
            self.rear_wing_angle_of_attack.value,
        );
        let downForceFrontWing = self.wing_down_force(
            self.front_wing_width.value,
            self.front_wing_height.value,
            self.front_wing_length.value,
            self.front_wing_angle_of_attack.value,
        );
        let downForceSideWing = self.wing_down_force(
            self.side_wings_width.value,
            self.side_wings_height.value,
            self.side_wings_length.value,
            self.side_wings_angle_of_attack.value,
        );
        let lcg = self.cabin_length.value;
        let lf = 0.5;
        return (2.0 * Ffsp * lf
            + 2.0 * Frsp * lf
            + downForceRearWing * (lcg - self.rear_wing_length.value)
            - downForceFrontWing * (lcg - self.front_wing_length.value)
            - 2.0 * downForceSideWing * (lcg - self.side_wings_length.value))
            .abs();
    }
}

impl argmin::prelude::ArgminOp for Car {
    type Param = Vec<f64>;
    type Output = f64;
    type Hessian = ();
    type Jacobian = ();
    type Float = f64;

    fn apply(&self, param: &Self::Param) -> Result<Self::Output, argmin::prelude::Error> {
        Ok(Car::new_from_parameters(param).objective(EVEN_WEIGHTS))
    }
}

impl std::fmt::Display for Car {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uom::si::pressure::pascal;

    #[test]
    fn tire_pressure_uses_bar_in_the_parameter_vector_and_legacy_equation() {
        let mut parameters = Car::new().get_parameter_vector();
        parameters[11] = 1.0;
        parameters[12] = 0.8;
        let car = Car::new_from_parameters(&parameters);

        assert_eq!(car.rear_tire_pressure.get::<pascal>(), 100_000.0);
        assert_eq!(car.get_parameter_vector()[11], 1.0);
        assert_eq!(car.get_parameter_vector()[12], 0.8);

        let rolling_coefficient = 0.005 + (0.01 + 0.0095 * ((V_CAR * 3.6 / 100.0).powi(2))) / 1.0;
        let resistance = car.total_drag_force() + rolling_coefficient * car.mass() * GRAVITY;
        let wheel_speed = V_CAR / car.rear_tire_radius.value;
        let efficiency = resistance * V_CAR / car.engine_power.value;
        let wheel_force =
            car.engine_torque.value * efficiency * W_E / (car.rear_tire_radius.value * wheel_speed);
        let expected_acceleration = (wheel_force - resistance) / car.mass();
        assert!((car.acceleration() - expected_acceleration).abs() < 1e-10);
    }

    #[test]
    fn front_wing_center_bounds_are_feasible_at_both_height_limits() {
        for height_m in constants::CONST_BOUNDS[3] {
            let (lower, upper) = Car::front_wing_center_y_bounds(Length::new::<meter>(height_m));
            assert!((lower - (0.03 + height_m / 2.0)).abs() < 1e-12);
            assert!((upper - (0.25 - height_m / 2.0)).abs() < 1e-12);
            assert!(lower <= upper, "height {} m inverted its Y range", height_m);
        }
    }

    #[test]
    fn reconstruction_refreshes_catalog_derived_fields() {
        let mut parameters = Car::new().get_parameter_vector();
        parameters[19..29].copy_from_slice(&[
            12.0, 1.0, 2.0, 3.0, 4.0, // material indices
            6.0, 0.0, 33.0, 20.0, 4.0, // tire, brake, motor, suspension indices
        ]);
        let car = Car::new_from_parameters(&parameters);
        let materials = constants::get_materials();
        let motors = constants::get_motors();
        let suspensions = constants::get_suspensions();
        let brakes = constants::get_brakes();
        let tires = constants::get_tires();

        assert_eq!(car.rear_wing_density, materials[12].density);
        assert_eq!(car.front_wing_density, materials[1].density);
        assert_eq!(car.side_wing_density, materials[2].density);
        assert_eq!(car.cabin_density, materials[3].density);
        assert_eq!(car.impact_attenuator_density, materials[4].density);
        assert_eq!(car.impact_attenuator_modulus, materials[4].modulus);
        assert_eq!(car.rear_tire_radius.get::<meter>(), tires[&6]["radius"]);
        assert_eq!(car.front_tire_mass.get::<kilogram>(), tires[&0]["mass"]);
        assert_eq!(car.engine_length, motors[20].length);
        assert_eq!(car.engine_height, motors[20].height);
        assert_eq!(car.engine_power, motors[20].power);
        assert_eq!(car.engine_torque, motors[20].torque);
        assert_eq!(car.engine_mass, motors[20].mass);
        assert_eq!(car.brake_thickness.get::<meter>(), brakes[&33]["thickness"]);
        assert_eq!(
            car.rear_suspension_spring_constant,
            suspensions[4].rear_spring_constant_newtons_per_meter
        );
        assert_eq!(
            car.front_suspension_damping_coefficient,
            suspensions[4].front_damping_coefficient_newton_seconds_per_meter
        );
        assert_eq!(car.rear_suspension_mass, suspensions[4].rear_mass);
        assert_eq!(car.front_suspension_mass, suspensions[4].front_mass);
    }

    #[test]
    fn internal() {
        let cost = Car::new();
        println!("{:?}", cost.objective(EVEN_WEIGHTS));
        let init_param: Vec<f64> = vec![0.0; 39];
        // let line_search = argmin::solver::linesearch::MoreThuenteLineSearch::new();
        // let solver = argmin::solver::gradientdescent::SteepestDescent::new(line_search);
        // let res = argmin::prelude::Executor::new(cost, solver, init_param)
        //     .add_observer(
        //         argmin::prelude::ArgminSlogLogger::term(),
        //         argmin::prelude::ObserverMode::Always,
        //     )
        //     .max_iters(10)
        //     .run()?;
    }
}
