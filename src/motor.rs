use std::f32::consts::PI;

use super::hal;
use super::wheel::Wheel as Wheel;
use super::wheel::Orientation as Orientation;

/// Implementation of a Motor with an attached Wheel
#[derive(Default)]
pub(crate) struct Motor {
	pub(crate) wheel: Wheel,
	pub(crate) reversed: bool,
	pub(crate) rotations: i32,
	pub(crate) angle: f32,
	pub(crate) last_encoder: i32,
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
	pub(crate) fn step(&mut self, _duration: u128) -> (f32, f32) {
		if let Orientation::UNDEFINED = self.wheel.orientation {
			println!("ERROR: Undefined Wheel-Orientation for Encoder {} and Motor {}", self.wheel.encoder as i32, self.wheel.motor as i32);
			return (0.0, 0.0);
		}

		let enc = hal::get_encoder_value(self.wheel.encoder);
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

	/// Set the speed of the motor
	///
	/// The value can go from -1.0 up to 1.0
	/// where minus values will drive backward
	///
	/// # Arguments
	///
	/// * `speed` - The speed for this motor
	pub(crate) fn set_speed(&self, speed: f64) {
		let mut duty = if speed > 1.0 { 1.0 } else { speed };
		duty = if speed < -1.0 { -1.0 } else { duty };
		hal::run_motor(self.wheel.motor, match self.reversed {
			true => -1.0 * duty,
			false => duty,
		});
	}

	/// Stop the motor and brake
	pub(crate) fn stop(&self) {
		hal::brake_motor(self.wheel.motor);
	}

}
