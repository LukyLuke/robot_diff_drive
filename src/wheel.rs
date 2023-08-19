use super::hal;

// To differentiate between LEFT and RIGHT
#[derive(Default)]
pub(crate) enum Orientation {
	#[default]
	UNDEFINED,
	LEFT,
	RIGHT,
}

// A motorized wheel with an encoder
#[derive(Default)]
pub struct Wheel {
	pub(crate) orientation: Orientation,
	pub(crate) radius: f32,
	pub(crate) encoder: hal::Encoder,
	pub(crate) motor: hal::Motor,
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
	pub fn left(radius: f32, encoder: hal::Encoder, motor: hal::Motor, gear_ratio: f32, encoder_resolution: f32) -> Self {
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
	pub fn right(radius: f32, encoder: hal::Encoder, motor: hal::Motor, gear_ratio: f32, encoder_resolution: f32) -> Self {
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
