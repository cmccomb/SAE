mod constants;
mod utils;

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
    rear_wing_height: f64,
    rear_wing_length: f64,
    rear_wing_angle_of_attack: f64,
    front_wing_height: f64,
    front_wing_length: f64,
    front_wing_width: f64,
    front_wing_angle_of_attack: f64,
    side_wings_height: f64,
    side_wings_length: f64,
    side_wings_width: f64,
    side_wings_angle_of_attack: f64,
    rear_tire_pressure: f64,
    front_tire_pressure: f64,
    cabin_height: f64,
    cabin_length: f64,
    cabin_width: f64,
    cabin_thickness: f64,
    impact_attenuator_height: f64,
    impact_attenuator_width: f64,
    rear_wing_density: f64,
    front_wing_density: f64,
    side_wing_density: f64,
    cabin_density: f64,
    impact_attenuator_density: f64,
    impact_attenuator_modulus: f64,
    rear_tire_radius: f64,
    rear_tire_mass: f64,
    front_tire_radius: f64,
    front_tire_mass: f64,
    engine_power: f64,
    engine_length: f64,
    engine_height: f64,
    engine_torque: f64,
    engine_mass: f64,
    brake_radius: f64,
    brake_density: f64,
    brake_length: f64,
    brake_height: f64,
    brake_width: f64,
    brake_thickness: f64,
    rear_suspension_spring_constant: f64,
    rear_suspension_damping_coefficient: f64,
    rear_suspension_mass: f64,
    front_suspension_spring_constant: f64,
    front_suspension_damping_coefficient: f64,
    front_suspension_mass: f64,
    rear_wing_width: f64,
    rear_wing_y_position: f64,
    front_wing_y_position: f64,
    side_wing_y_position: f64,
    engine_y_position: f64,
    cabin_y_position: f64,
    impact_attenuator_length: f64,
    impact_attenuator_y_position: f64,
    rear_suspension_y_position: f64,
    front_suspension_y_position: f64,
}

impl Car {
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
        let impact_attenuator_height = utils::random_uniform(
            constants::CONST_BOUNDS[17][0],
            constants::CONST_BOUNDS[17][1],
        );
        let front_wing_length =
            utils::random_uniform(constants::CONST_BOUNDS[4][0], constants::CONST_BOUNDS[4][1]);
        let cabin_height = utils::random_uniform(
            constants::CONST_BOUNDS[13][0],
            constants::CONST_BOUNDS[13][1],
        );
        let rear_wing_height =
            utils::random_uniform(constants::CONST_BOUNDS[0][0], constants::CONST_BOUNDS[0][1]);
        let front_wing_height =
            utils::random_uniform(constants::CONST_BOUNDS[3][0], constants::CONST_BOUNDS[3][1]);
        let side_wings_height =
            utils::random_uniform(constants::CONST_BOUNDS[7][0], constants::CONST_BOUNDS[7][1]);

        Car {
            // Parameters with uniform bounds
            rear_wing_height,
            rear_wing_length: utils::random_uniform(
                constants::CONST_BOUNDS[1][0],
                constants::CONST_BOUNDS[1][1],
            ),
            rear_wing_angle_of_attack: utils::random_uniform(
                constants::CONST_BOUNDS[2][0],
                constants::CONST_BOUNDS[2][1],
            ),
            front_wing_height,
            front_wing_length,
            front_wing_width: utils::random_uniform(
                constants::CONST_BOUNDS[5][0],
                constants::CONST_BOUNDS[5][1],
            ),
            front_wing_angle_of_attack: utils::random_uniform(
                constants::CONST_BOUNDS[6][0],
                constants::CONST_BOUNDS[6][1],
            ),
            side_wings_height,
            side_wings_length: utils::random_uniform(
                constants::CONST_BOUNDS[8][0],
                constants::CONST_BOUNDS[8][1],
            ),
            side_wings_width: utils::random_uniform(
                constants::CONST_BOUNDS[9][0],
                constants::CONST_BOUNDS[9][1],
            ),
            side_wings_angle_of_attack: utils::random_uniform(
                constants::CONST_BOUNDS[10][0],
                constants::CONST_BOUNDS[10][1],
            ),
            rear_tire_pressure: utils::random_uniform(
                constants::CONST_BOUNDS[11][0],
                constants::CONST_BOUNDS[11][1],
            ),
            front_tire_pressure: utils::random_uniform(
                constants::CONST_BOUNDS[12][0],
                constants::CONST_BOUNDS[12][1],
            ),
            cabin_height,
            cabin_length: utils::random_uniform(
                constants::CONST_BOUNDS[14][0],
                constants::CONST_BOUNDS[14][1],
            ),
            cabin_width: utils::random_uniform(
                constants::CONST_BOUNDS[15][0],
                constants::CONST_BOUNDS[15][1],
            ),
            cabin_thickness: utils::random_uniform(
                constants::CONST_BOUNDS[16][0],
                constants::CONST_BOUNDS[16][1],
            ),
            impact_attenuator_height,
            impact_attenuator_width: utils::random_uniform(
                constants::CONST_BOUNDS[18][0],
                constants::CONST_BOUNDS[18][1],
            ),

            // Parameters based on lookup from indices
            rear_wing_density: materials[&rear_wing_material_index]["density"],
            front_wing_density: materials[&front_wing_material_index]["density"],
            side_wing_density: materials[&side_wing_material_index]["density"],
            cabin_density: materials[&cabin_material_index]["density"],
            impact_attenuator_density: materials[&impact_attenuator_material_index]["density"],
            impact_attenuator_modulus: materials[&impact_attenuator_material_index]["modulus"],
            rear_tire_radius: tires[&rear_tire_index]["radius"],
            rear_tire_mass: tires[&rear_tire_index]["mass"],
            front_tire_radius: tires[&front_tire_index]["radius"],
            front_tire_mass: tires[&front_tire_index]["mass"],
            engine_power: motors[&motor_index]["power"],
            engine_length: motors[&motor_index]["length"],
            engine_height: motors[&motor_index]["height"],
            engine_torque: motors[&motor_index]["torque"],
            engine_mass: motors[&motor_index]["mass"],
            brake_radius: brakes[&brake_index]["radius"],
            brake_density: brakes[&brake_index]["density"],
            brake_length: brakes[&brake_index]["length"],
            brake_height: brakes[&brake_index]["height"],
            brake_width: brakes[&brake_index]["width"],
            brake_thickness: brakes[&brake_index]["thickness"],
            rear_suspension_spring_constant: suspensions[&suspension_index]["spring_constant"],
            rear_suspension_damping_coefficient: suspensions[&suspension_index]
                ["damping_coefficient"],
            rear_suspension_mass: suspensions[&suspension_index]["mass"],
            front_suspension_spring_constant: suspensions[&suspension_index]["spring_constant"],
            front_suspension_damping_coefficient: suspensions[&suspension_index]
                ["damping_coefficient"],
            front_suspension_mass: suspensions[&suspension_index]["mass"],

            // Parameters with variables bounds
            rear_wing_width: utils::random_uniform(
                0.3,
                9.0 - 2.0 * tires[&rear_tire_index]["radius"],
            ),
            rear_wing_y_position: utils::random_uniform(
                0.5 + rear_wing_height / 2.0,
                1.2 - rear_wing_height / 2.0,
            ),
            front_wing_y_position: utils::random_uniform(
                0.03 + front_wing_height,
                0.25 - rear_wing_height / 2.0,
            ),
            side_wing_y_position: utils::random_uniform(
                0.03 + side_wings_height / 2.0,
                0.25 - side_wings_height / 2.0,
            ),
            engine_y_position: utils::random_uniform(
                0.03 + motors[&motor_index]["height"] / 2.0,
                0.5 - motors[&motor_index]["height"] / 2.0,
            ),
            cabin_y_position: utils::random_uniform(
                0.03 + cabin_height / 2.0,
                1.2 - cabin_height / 2.0,
            ),
            impact_attenuator_length: utils::random_uniform(0.2, 0.7 - front_wing_length),
            impact_attenuator_y_position: utils::random_uniform(
                0.03 + impact_attenuator_height / 2.0,
                1.2 - impact_attenuator_height / 2.0,
            ),
            rear_suspension_y_position: utils::random_uniform(
                tires[&rear_tire_index]["radius"],
                2.0 * tires[&rear_tire_index]["radius"],
            ),
            front_suspension_y_position: utils::random_uniform(
                tires[&front_tire_index]["radius"],
                2.0 * tires[&front_tire_index]["radius"],
            ),
        }
    }

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
            rear_wing_height: p[0],
            rear_wing_length: p[1],
            rear_wing_angle_of_attack: p[2],
            front_wing_height: p[3],
            front_wing_length: p[4],
            front_wing_width: p[5],
            front_wing_angle_of_attack: p[6],
            side_wings_height: p[7],
            side_wings_length: p[8],
            side_wings_width: p[9],
            side_wings_angle_of_attack: p[10],
            rear_tire_pressure: p[11],
            front_tire_pressure: p[12],
            cabin_height: p[13],
            cabin_length: p[14],
            cabin_width: p[15],
            cabin_thickness: p[16],
            impact_attenuator_height: p[17],
            impact_attenuator_width: p[18],
            rear_wing_density: materials[&rear_wing_material_index]["density"],
            front_wing_density: materials[&front_wing_material_index]["density"],
            side_wing_density: materials[&side_wing_material_index]["density"],
            cabin_density: materials[&cabin_material_index]["density"],
            impact_attenuator_density: materials[&impact_attenuator_material_index]["density"],
            impact_attenuator_modulus: materials[&impact_attenuator_material_index]["modulus"],
            rear_tire_radius: tires[&rear_tire_index]["radius"],
            rear_tire_mass: tires[&rear_tire_index]["mass"],
            front_tire_radius: tires[&front_tire_index]["radius"],
            front_tire_mass: tires[&front_tire_index]["mass"],
            engine_power: motors[&motor_index]["power"],
            engine_length: motors[&motor_index]["length"],
            engine_height: motors[&motor_index]["height"],
            engine_torque: motors[&motor_index]["torque"],
            engine_mass: motors[&motor_index]["mass"],
            brake_radius: brakes[&brake_index]["radius"],
            brake_density: brakes[&brake_index]["density"],
            brake_length: brakes[&brake_index]["length"],
            brake_height: brakes[&brake_index]["height"],
            brake_width: brakes[&brake_index]["width"],
            brake_thickness: brakes[&brake_index]["thickness"],
            rear_suspension_spring_constant: suspensions[&suspension_index]["spring_constant"],
            rear_suspension_damping_coefficient: suspensions[&suspension_index]
                ["damping_coefficient"],
            rear_suspension_mass: suspensions[&suspension_index]["mass"],
            front_suspension_spring_constant: suspensions[&suspension_index]["spring_constant"],
            front_suspension_damping_coefficient: suspensions[&suspension_index]
                ["damping_coefficient"],
            front_suspension_mass: suspensions[&suspension_index]["mass"],
            rear_wing_width: p[29],
            rear_wing_y_position: p[30],
            front_wing_y_position: p[31],
            side_wing_y_position: p[32],
            engine_y_position: p[33],
            cabin_y_position: p[34],
            impact_attenuator_length: p[35],
            impact_attenuator_y_position: p[36],
            rear_suspension_y_position: p[37],
            front_suspension_y_position: p[38],
        }
    }

    pub fn objective(&self, weights: [f64; 11]) -> f64 {
        weights[0] * self.mass()
            + weights[1] * self.center_of_gravity()
            + weights[2] * self.drag_force()
            + weights[3] * self.downward_force()
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
            self.drag_force(),
            self.downward_force(),
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
        let t1 = (self.mass_rear_wing() * self.rear_wing_y_position
            + self.mass_front_wing() * self.front_wing_y_position
            + self.engine_mass * self.engine_y_position
            + self.mass_cabin() * self.cabin_y_position
            + self.mass_impact_attenuator() * self.impact_attenuator_y_position)
            / total_mass;
        let t2 = 2.0
            * (self.mass_side_wings() * self.side_wing_y_position
                + self.rear_tire_mass * self.rear_tire_radius
                + self.front_tire_mass * self.front_tire_radius
                + self.mass_brake() * self.front_tire_radius
                + self.rear_suspension_mass * self.rear_suspension_y_position
                + self.front_suspension_mass * self.front_suspension_y_position)
            / total_mass;
        return t1 + t2;
    }

    pub fn drag_force(&self) -> f64 {
        todo!()
    }

    pub fn downward_force(&self) -> f64 {
        todo!()
    }

    pub fn acceleration(&self) -> f64 {
        todo!()
    }
    pub fn crash_force(&self) -> f64 {
        todo!()
    }
    pub fn impact_attenuator_volume(&self) -> f64 {
        todo!()
    }
    pub fn corner_velocity(&self) -> f64 {
        todo!()
    }
    pub fn brakeing_distance(&self) -> f64 {
        todo!()
    }
    pub fn suspension_acceleration(&self) -> f64 {
        todo!()
    }
    pub fn pitch_moment(&self) -> f64 {
        todo!()
    }

    pub fn mass(&self) -> f64 {
        self.mass_rear_wing()
            + self.mass_front_wing()
            + 2.0 * self.mass_side_wings()
            + 2.0 * self.rear_tire_mass
            + 2.0 * self.front_tire_mass
            + self.engine_mass
            + self.mass_cabin()
            + self.mass_impact_attenuator()
            + 4.0 * self.mass_brake()
            + 2.0 * self.rear_suspension_mass
            + 2.0 * self.front_suspension_mass
    }

    fn mass_rear_wing(&self) -> f64 {
        self.rear_wing_length
            * self.rear_wing_width
            * self.rear_wing_height
            * self.rear_wing_density
    }

    fn mass_front_wing(&self) -> f64 {
        self.front_wing_length
            * self.front_wing_width
            * self.front_wing_height
            * self.front_wing_density
    }

    fn mass_side_wings(&self) -> f64 {
        self.side_wings_length
            * self.side_wings_width
            * self.side_wings_height
            * self.side_wing_density
    }
    fn mass_impact_attenuator(&self) -> f64 {
        self.impact_attenuator_length
            * self.impact_attenuator_width
            * self.impact_attenuator_height
            * self.impact_attenuator_density
    }
    fn mass_cabin(&self) -> f64 {
        2.0 * (self.cabin_height * self.cabin_length * self.cabin_thickness
            + self.cabin_height * self.cabin_width * self.cabin_thickness
            + self.cabin_length * self.cabin_height * self.cabin_thickness)
            * self.cabin_density
    }
    fn mass_brake(&self) -> f64 {
        self.brake_length * self.brake_width * self.brake_height * self.brake_density
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

    #[test]
    fn internal() {
        let cost = Car::new();
        println!("{:?}", cost.objective(EVEN_WEIGHTS));
        // let init_param: Vec<f64> = vec![0.0; 39];
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
