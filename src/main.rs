mod hal;
mod motor;
mod wheel;
mod position;
mod diff_drive;

use std::thread::sleep;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use wheel::Wheel as Wheel;

fn main() {
	let terminate = Arc::new(AtomicBool::new(false));
	signal_hook::flag::register(signal_hook::consts::SIGINT,  Arc::clone(&terminate)).unwrap();
	signal_hook::flag::register(signal_hook::consts::SIGQUIT, Arc::clone(&terminate)).unwrap();

	// Pause and Start buttons for controlling
	hal::init_button(hal::Button::Pause);
	hal::init_button(hal::Button::Mode);
	hal::register_button_callback(hal::Button::Pause, pause_pressed);
	hal::register_button_callback(hal::Button::Mode, mode_pressed);

	// Initialize Encoders and Motors
	hal::init_encoders();
	hal::init_motors();

	// Robot Values
	let wheel_distance = 155.0;
	let caster_wheel_distance = 163.0;

	let wheel_left_raduis = 40.0;
	let wheel_left_hal = (hal::Encoder::ENCODER3, hal::Motor::MOTOR3);
	let wheel_left_gearbox = 3441.0 / 104.0;
	let wheel_left_resolution = 32.0;
	let wheel_left_reversed = true;

	let wheel_right_raduis = 40.0;
	let wheel_right_hal = (hal::Encoder::ENCODER2, hal::Motor::MOTOR2);
	let wheel_right_gearbox = 3441.0 / 104.0;
	let wheel_right_resolution = 32.0;
	let wheel_right_reversed = false;

	// Initialize the robot
	let mut robot = diff_drive::new(wheel_distance, caster_wheel_distance);
	robot.add_wheel(Wheel::left(
		wheel_left_raduis,
		wheel_left_hal.0,
		wheel_left_hal.1,
		wheel_left_gearbox,
		wheel_left_resolution
	), wheel_left_reversed);
	robot.add_wheel(Wheel::right(
		wheel_right_raduis,
		wheel_right_hal.0,
		wheel_right_hal.1,
		wheel_right_gearbox,
		wheel_right_resolution
	), wheel_right_reversed);

	robot.set_goal(500.0, 500.0);
	robot.start();

	while !terminate.load(Ordering::Relaxed) {
		sleep(Duration::from_millis(1));
		robot.step();
	}

	robot.halt();
	hal::cleanup();
}

extern "C" fn pause_pressed() {
	println!("Pause Pressed!");
}

extern "C" fn mode_pressed() {
	println!("Mode Pressed!");
}
