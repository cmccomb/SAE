mod utils;

const V_CAR: f64 = 26.8;
const W_E: f64 = 3600 * 2 * pi / 60;
const RHO_AIR: f64 = 1.225;
const R_TRACK: f64 = 9.0;
const P_BRK: f64 = 10 * *7;
const C_DC: f64 = 0.04;
const GRAVITY: f64 = 9.81;
const Y_SUSPENSION: f64 = 0.05;
const DYDT_SUSPENSION: f64 = 0.025;

// Weights
const EVEN_WEIGHTS: [f64; 11] = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];

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
        let x = utils::random_unit_draw();
        Car {}
    }

    pub fn new_from_parameters(p: Vec<f64>) -> Self {
        Car {}
    }

    pub fn objective(&self, weights: Vec<f64>) -> f64 {
        self.mass()
            + self.center_of_gravity()
            + self.drag_force()
            + self.downward_force()
            + self.acceleration()
            + self.crash_force()
            + self.impact_attenuator_volume()
            + self.corner_velocity()
            + self.breaking_distance()
            + self.suspension_acceleration()
            + self.pitch_moment()
    }

    pub fn objectives(&self) -> f64 {
        todo!()
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
        t1 = (self.mrw() * self.yrw
            + self.mfw() * self.yfw
            + self.me * self.ye
            + self.mc() * self.yc
            + self.mia() * self.yia)
            / self.mass();
        t2 = 2
            * (self.msw() * self.ysw
                + self.mrt * self.rrt
                + self.mft * self.rft
                + self.mbrk() * self.rft
                + self.mrsp * self.yrsp
                + self.mfsp * self.yfsp)
            / self.mass();
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
    pub fn breaking_distance(&self) -> f64 {
        todo!()
    }
    pub fn suspension_acceleration(&self) -> f64 {
        todo!()
    }
    pub fn pitch_moment(&self) -> f64 {
        todo!()
    }

    pub fn mass(&self) -> f64 {
        self.mass_rw()
            + self.mass_fw()
            + 2 * self.mass_sw()
            + 2 * self.mrt
            + 2 * self.mft
            + self.me
            + self.mass_c()
            + self.mass_ia()
            + 4 * self.mass_brk()
            + 2 * self.mrsp
            + 2 * self.mfsp
    }

    fn mass_rw(&self) -> f64 {
        self.lrw * self.wrw * self.hrw * self.qrw
    }

    fn mass_fw(&self) -> f64 {
        self.lfw * self.wfw * self.hfw * self.qfw
    }

    fn mass_sw(&self) -> f64 {
        self.lsw * self.wsw * self.hsw * self.qsw
    }
    fn mass_ia(&self) -> f64 {
        self.lia * self.wia * self.hia * self.qia
    }
    fn mass_c(&self) -> f64 {
        2 * (self.hc * self.lc * self.tc
            + self.hc * self.wc * self.tc
            + self.lc * self.hc * self.tc)
            * self.qc
    }
    fn mass_brk(&self) -> f64 {
        self.lbrk * self.wbrk * self.hbrk * self.qbrk
    }
}

#[cfg(feature = "optimize")]
impl argmin::prelude::ArgminOp for Car {
    type Param = Vec<f64>;
    type Output = f64;
    type Hessian = ();
    type Jacobian = ();
    type Float = ();

    fn apply(&self, _param: &Self::Param) -> Result<Self::Output, Error> {
        Ok(Car::new_from_parameters(p).objective(EVEN_WEIGHTS))
    }
}

impl Default for Car {
    fn default() -> Self {
        Car::new()
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

    #[cfg(feature = "optimize")]
    #[test]
    fn internal() {
        let cost = Car {};
        let init_param: Vec<f64> = vec![0.0, 39];
        let line_search = argmin::solver::linesearch::MoreThuenteLineSearch::new();
        let solver = argmin::solver::gradientdescent::SteepestDescent::new(line_search);
        let res = argmin::prelude::Executor::new(cost, solver, init_param)
            .add_observer(
                argmin::prelude::ArgminSlogLogger::term(),
                argmin::prelude::ObserverMode::Always,
            )
            .max_iters(10)
            .run()?;
    }
}
