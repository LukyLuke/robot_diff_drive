use super::librobotcontrol;

use std::time::Instant;
use std::f32::consts::PI;

// Create a new Differential-Drive Robbot
//
// #Arguments
//
// * `wheel_distance` - Distance between the wheels (middle of the wheel)
// * `caster_distance` - Distance from the amin Axle to the caster wheel mounting point
pub fn new(wheel_distance: i32, caster_distance: i32) -> DifferentialDrive {
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
	wheel_distance: i32,
	caster_distance: i32,
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

// A motorized wheel with an encoder
#[derive(Default)]
pub struct Wheel {
	pub(crate) orientation: Orientation,
	pub(crate) radius: f32,
	pub(crate) encoder: i8,
	pub(crate) motor: i8,
	pub(crate) gear_ratio: f32,
	pub(crate) encoder_resolution: f32,
}
impl Wheel {
	/// Creates a LEFT-Sided Wheel and returns it
	///
	/// # Arguments
	///
	/// * `radius` - The Wheel-Radius
	/// * `encoder` - Which Encoder-PIN
	/// * `motor` - Which Motor-PIN
	/// * `gear_ratio` - Ratio of the attached gearbox
	/// * `encoder_resolution` - Resolution of the encoder
	pub fn left(radius: f32, encoder: i8, motor: i8, gear_ratio: f32, encoder_resolution: f32) -> Self {
		Self {
			orientation: Orientation::LEFT,
			radius,
			encoder,
			motor,
			gear_ratio,
			encoder_resolution,
		}
	}

	/// Creates a RIGHT-Sided Wheel and returns it
	///
	/// # Arguments
	///
	/// * `radius` - The Wheel-Radius
	/// * `encoder` - Which Encoder-PIN
	/// * `motor` - Which Motor-PIN
	/// * `gear_ratio` - Ratio of the attached gearbox
	/// * `encoder_resolution` - Resolution of the encoder
	pub fn right(radius: f32, encoder: i8, motor: i8, gear_ratio: f32, encoder_resolution: f32) -> Self {
		Self {
			orientation: Orientation::RIGHT,
			radius,
			encoder,
			motor,
			gear_ratio,
			encoder_resolution,
		}
	}
}

// To differentiate between LEFT and RIGHT
#[derive(Default)]
enum Orientation {
	#[default]
	UNDEFINED,
	LEFT,
	RIGHT,
}

/// Position of the Robot in the World and it's orientation
#[derive(Default)]
struct Position {
	pub(crate) x: f64,
	pub(crate) y: f64,
	pub(crate) phi: f32,
}

/// Implementation of a Motor with an attached Wheel
#[derive(Default)]
struct Motor {
	wheel: Wheel,
	reversed: bool,
	rotations: i32,
	angle: f32,
	last_encoder: i32,
}
impl Motor {
	/// Called on every calculation step
	///
	/// # Arguments
	///
	/// * `duration` - Duration in NS since the last calculation step
	///
	/// # Returns
	///
	/// A tuple of ( Distance driven since last step, Angle changed since last step )
	pub(crate) fn step(&mut self, duration: u128) -> (f32, f32) {
		if let Orientation::UNDEFINED = self.wheel.orientation {
			println!("ERROR: Undefined Wheel-Orientation for Encoder {} and Motor {}", self.wheel.encoder, self.wheel.motor);
			return (0.0, 0.0);
		}

		let enc = self.encoder_value();
		let diff = enc - self.last_encoder;
		self.last_encoder = enc;

		let mut angle = diff as f32 / self.wheel.encoder_resolution / self.wheel.gear_ratio;
		angle = match self.reversed {
			true => angle * -1.0,
			false => angle,
		};

		self.angle = self.angle + angle;
		let dist = angle * self.wheel.radius * PI;

		(dist, self.angle)
	}

	/// Get the total distance this motor and wheel drove
	pub(crate) fn total_distance(&self) -> f32 {
		self.angle * self.wheel.radius * PI
	}

	/// Get's the current Encoder-Value
	fn encoder_value(&self) -> i32 {
		librobotcontrol::get_encoder_value(self.wheel.encoder as i32)
	}
}



