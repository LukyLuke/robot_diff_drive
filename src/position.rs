/// +/- number if mm around the goal meand the goal is reached
const GOAL_AREA: f64 = 10.0;

/// Position of the Robot in the World and it's orientation
#[derive(Default)]
pub struct Position {
	pub(crate) x: f64,
	pub(crate) y: f64,
	pub(crate) goal_x: f64,
	pub(crate) goal_y: f64,
	pub(crate) phi: f32,
}

impl Position {
	/// Set the goal which should be reached
	///
	/// # Arguments
	///
	/// * `x` - X-Position to reach
	/// * `y` - Y-Position to reach
	pub fn set_goal(&mut self, x: f64, y: f64) {
		self.goal_x = x;
		self.goal_y = y;
	}

	/// Given the distance the left and right wheel travelled, the new Position of the robot is calculated and set
	///
	/// # Arguments
	///
	/// * `left` - Number of mm the left Wheel was driven
	/// * `right` - Number of mm the right Wheel was driven
	/// * `wheel_distance` - Number of mm the left and right wheels are apart from each other
	pub fn calculate_position(&mut self, left: f32, right: f32, wheel_distance: f32) {
		// Distance the center of the robot travelled
		let distance = (left + right) / 2.0;

		// Delta values compared to the last position
		let delta_x = self.phi.cos() * distance;
		let delta_y = self.phi.sin() * distance;
		let delta_angle = (right - left) / wheel_distance;

		self.x = self.x + delta_x as f64;
		self.y = self.y + delta_y as f64;
		self.phi = self.phi + delta_angle;
	}

	/// Calculates the velocities for a left and right wheel to the goal
	///
	/// # Arguments
	///
	/// * `wheel_distance` - Number of mm the left and right wheels are apart from each other
	///
	/// # Result
	///
	/// A tuple with the (left, right) velocity to the end [ -1.0 - 1.0]
	pub fn get_goal_velocities(&self, wheel_distance: f32) -> (f64, f64) {
		let delta_x = self.goal_x - self.x;
		let delta_y = self.goal_y - self.y;
		let distance = ( delta_x.powi(2) + delta_y.powi(2) ).sqrt();
		let delta_phi = delta_y.atan2(delta_x) - self.phi as f64;

		// Factor based on delta-phi to define how faster/slower the right wheel should drive
		let fact = 1.0 + (delta_phi / std::f64::consts::FRAC_PI_2);
		//println!("Diff:\tx={}\ty={}\tphi={}\tdist={}\tfact={}", delta_x, delta_y, Self::degree(delta_phi), distance, fact);

		(0.1, 0.1 * fact)
	}

	/// Check if the goal is reached
	/// The goal is reached in a area of GOAL_AREA around the real goal
	pub fn goal_reached(&self) -> bool {
		((self.x + GOAL_AREA) > self.goal_x && (self.x - GOAL_AREA) < self.goal_x)
		&& ((self.y + GOAL_AREA) > self.goal_y && (self.y - GOAL_AREA) < self.goal_y)
	}

	pub fn degree(rad: f64) -> f64 {
		(rad * 180.0) / std::f64::consts::PI
	}

	pub fn radian(deg: f64) -> f64 {
		(deg * std::f64::consts::PI) / 180.0
	}

}
