
/// Create a path planner based on points
///
/// # Arguments
///
/// * `init` - A tuple do place the robot somewhere: (X in mm, Y in mm, ORIENTATION in rad)
/// * `points` - A List of tuples which the robot should drive through: (X in mm, Y in mm)
///
/// # Result
///
/// A prepared Planner instance
pub fn from_points(init: (f64, f64, f32), points: &[(f64, f64)]) -> Planner {
	let mut planner = Planner::default();
	planner.start = init;
	for point in points {
		planner.push(point.0, point.1);
	}
	planner
}

/// Struct which identifies a path planner
#[derive(Default)]
pub struct Planner {
	pub(crate) start: (f64, f64, f32),
	points: Vec<(f64, f64)>,
	pos: usize,
}

impl Planner {
	/// Add a point to the end of list
	///
	/// # Arguments
	///
	/// * `x` - X-Coordinate in mm
	/// * `y` - Y-Coordinate in mm
	pub fn push(&mut self, x: f64, y: f64) {
		self.points.push((x, y));
	}

	/// Get the start point and orientation
	///
	/// # Result
	///
	/// A tuple ith the X- and Y-Coordinate and the Orientation
	pub fn start(&self) -> (f64, f64, f32) {
		(self.start.0, self.start.1, self.start.2)
	}

	/// Restart the planner, this sets the first point as the goal to reach
	pub fn restart(&mut self) {
		self.pos = 0;
	}

	/// Returns the next goal to reach
	///
	/// # Result
	///
	/// A Result with a tuple which has the X- and Y-Coordinate
	/// If no more points are reachable, an Error is returnes
	pub fn next_goal(&mut self) -> Result<(f64, f64), &str> {
		if self.points.len() > self.pos {
			let index = self.pos;
			self.pos = self.pos + 1;
			Ok(self.points[index])
		} else {
			Err("No more goals")
		}
	}

}
