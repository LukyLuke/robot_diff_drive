
use std::thread::sleep;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod librobotcontrol;
mod diff_drive;

fn main() {
	// Pause and Start buttons for controlling
	librobotcontrol::init_button(librobotcontrol::Button::Pause);
	librobotcontrol::init_button(librobotcontrol::Button::Mode);
	librobotcontrol::register_button_callback(librobotcontrol::Button::Pause, pause_pressed);
	librobotcontrol::register_button_callback(librobotcontrol::Button::Mode, mode_pressed);

	// Initialize Encoders and Motors
	librobotcontrol::init_encoders();

	// Initialize the robot
	let mut robot = diff_drive::new(155, 163);
	robot.add_wheel(diff_drive::Wheel::left(40.0, 3, 3, 3441.0 / 104.0, 32.0), true);
	robot.add_wheel(diff_drive::Wheel::right(40.0, 2, 2, 3441.0 / 104.0, 32.0), false);
	robot.start();

	let terminate = Arc::new(AtomicBool::new(false));
	signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&terminate)).unwrap();
	while !terminate.load(Ordering::Relaxed) {
		sleep(Duration::from_millis(10));
		robot.step();
	}
	librobotcontrol::cleanup();
}

extern "C" fn pause_pressed() {
	println!("Pause Pressed!");
}

extern "C" fn mode_pressed() {
	println!("Mode Pressed!");
}
