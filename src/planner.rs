
pub fn from_points(points: &[(f64, f64)]) -> Planner {
	let mut planner = Planner::default();
	for point in points {
		planner.push(point.0, point.1);
	}
	planner
}

#[derive(Default)]
pub struct Planner {
	points: Vec<(f64, f64)>,
	pos: usize,
}

impl Planner {
	pub fn push(&mut self, x: f64, y: f64) {
		self.points.push((x, y));
	}

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
