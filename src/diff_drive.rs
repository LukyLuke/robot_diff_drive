use super::planner::Planner as Planner;
use super::motor::Motor as Motor;
use super::wheel::Wheel as Wheel;
use super::wheel::Orientation as Orientation;
use super::position::Position as Position;

use std::time::Instant;


// Create a new Differential-Drive Robbot
//
// #Arguments
//
// * `wheel_distance` - Distance between the wheels (middle of the wheel)
// * `caster_distance` - Distance from the amin Axle to the caster wheel mounting point
pub fn new(wheel_distance: f32, caster_distance: f32) -> DifferentialDrive {
	DifferentialDrive {
		wheel_distance,
		caster_distance,
		left: Motor::default(),
		right: Motor::default(),
		position: Position::default(),
		planner: None,
		running: false,
		loop_run: false,
		last_step: Instant::now(),
	}
}

/// This is the main Robot
pub struct DifferentialDrive {
	wheel_distance: f32,
	caster_distance: f32,
	left: Motor,
	right: Motor,
	position: Position,
	planner: Option<Planner>,
	running: bool,
	loop_run: bool,
	last_step: Instant,
}
impl DifferentialDrive {
	/// Add a motorized wheel
	///
	/// # Arguments
	///
	/// * `wheel` - A Wheel-Instance
	/// * `reversed` - If the motor and encoder is reversed (rotated 180°, even the left or right is)
	pub fn add_wheel(&mut self, wheel: Wheel, reversed: bool) {
		match wheel.orientation {
			Orientation::LEFT => self.left = Motor {
				wheel,
				angle: 0.0,
				rotations: 0,
				reversed,
				last_encoder: 0,
			},
			Orientation::RIGHT => self.right = Motor {
				wheel,
				angle: 0.0,
				rotations: 0,
				reversed,
				last_encoder: 0,
			},
			_ => {},
		}
	}

	/// Start the robot
	pub fn start(&mut self, restart_on_end: bool) {
		self.running = true;
		self.loop_run = restart_on_end;
		self.last_step = Instant::now();
	}

	/// Stop the robot
	pub fn halt(&mut self) {
		self.running = false;
		self.left.stop();
		self.right.stop();
	}

	/// Set the Coordinates the robot should reach
	///
	/// # Arguments
	///
	/// * `x` - X-Coordinates in mm
	/// * `y` - Y-Coordinates in mm
	pub fn set_goal(&mut self, x: f64, y: f64) {
		self.position.set_goal(x, y);
	}

	/// Set a path-planner
	///
	/// # Arguments
	///
	/// * `planner` - The Pathplanner to fetch points
	pub fn path_planner(&mut self, planner: Planner) {
		self.planner = Some(planner);
		if let Some(plan) = &self.planner {
			let start = plan.start();
			self.position.set_position(start.0, start.1, start.2);
		}
		self.next_goal();
	}

	/// sets the next goal for the Robot based on the PathPlanner
	fn next_goal(&mut self) {
		if let Some(planner) = self.planner.as_mut() {
			if let Ok(goal) = planner.next_goal() {
				self.set_goal(goal.0, goal.1);
			} else if self.loop_run {
				planner.restart();
				let start = planner.start();
				self.set_goal(start.0, start.1);
			}
		}
	}

	/// Called on each step, calculates the new position and how to get to the wanted one, etc.
	pub fn step(&mut self) {
		if self.running {
			let now = Instant::now();
			let duration = now.duration_since(self.last_step).as_nanos();

			// Get the travelling distance
			let (dist_l, _angle_l) = self.left.step(duration);
			let (dist_r, _angle_r) = self.right.step(duration);

			// Update the new position of of the robot
			self.position.calculate_position(dist_l, dist_r, self.wheel_distance);
			self.position.debug();

			// If we reached the goal and have a path planner, set the next goal
			if self.planner.is_some() && self.position.goal_reached() {
				self.next_goal();
			}

			// Stop if the goal is reached, otherwise get the velocities for the wheels
			if !self.position.goal_reached() {
				let (left, right) = self.position.get_goal_velocities(self.wheel_distance);
				self.left.set_speed(left);
				self.right.set_speed(right);
			} else {
				self.left.stop();
				self.right.stop();
			}

			self.last_step = now;
		}
	}

}

