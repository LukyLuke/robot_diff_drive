use super::motor::Motor as Motor;
use super::wheel::Wheel as Wheel;
use super::wheel::Orientation as Orientation;
use super::position::Position as Position;

use std::time::Instant;
use std::f32::consts::PI;

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
		running: false,
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

	running: bool,
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
	pub fn start(&mut self) {
		self.running = true;
		self.last_step = Instant::now();
	}

	/// Stop the robot
	pub fn halt(&mut self) {
		self.running = false;
	}

	/// Called on each step, calculates the new position and how to get to the wanted one, etc.
	pub fn step(&mut self) {
		if self.running {
			let now = Instant::now();
			let duration = now.duration_since(self.last_step).as_nanos();

			let (dist_l, angle_l) = self.left.step(duration);
			let (dist_r, angle_r) = self.right.step(duration);

			println!("Step[{:?}ns]: left: {}mm, {}rad, {}mm total | right: {}mm, {}rad, {}mm total", duration, dist_l, angle_l, self.left.total_distance(), dist_r, angle_r, self.right.total_distance());
			self.last_step = now;
		}
	}

}

