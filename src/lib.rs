use argmin::prelude::*;

const EVEN_WEIGHTS: Vec<f64> = vec![1.0 / 11.0; 11];

pub struct Car {}

impl Car {
    pub fn new() -> Self {
        Car {}
    }

    pub fn new_from_parameters(p: Vec<f64>) -> Self {}

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

    pub fn objectives(&self) -> f64 {}

    pub fn check_bounds(&self) -> Vec<f64> {}

    pub fn check_nonlinear_constraints(&self) -> Vec<f64> {}

    pub fn check_linear_constraints(&self) -> Vec<f64> {}

    pub fn mass(&self) -> f64 {}

    pub fn center_of_gravity(&self) -> f64 {}

    pub fn drag_force(&self) -> f64 {}

    pub fn downward_force(&self) -> f64 {}
}

impl ArgminOp for Car {
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

    #[test]
    fn internal() {
        let cost = Car {};
        let init_param: Vec<f64> = vec![0.0, 39];
        let line_search = argmin::solver::linesearch::MoreThuenteLineSearch::new();
        let solver = argmin::solver::gradientdescent::SteepestDescent::new(line_search);
        let res = Executor::new(cost, solver, init_param)
            .add_observer(ArgminSlogLogger::term(), ObserverMode::Always)
            .max_iters(10)
            .run()?;
    }
}
