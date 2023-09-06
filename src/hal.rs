extern crate librobotcontrol_sys;

use std::thread::sleep;
use std::time::{Duration, Instant};

pub const BUTTON_DEBOUNCE: ::std::os::raw::c_int = 2000; // 2ms
pub const MOTOR_PWM: ::std::os::raw::c_int = 25000; // 25kHz

pub fn cleanup() {
	unsafe {
		librobotcontrol_sys::rc_button_cleanup();
		librobotcontrol_sys::rc_encoder_eqep_cleanup();
		librobotcontrol_sys::rc_encoder_pru_cleanup();
		librobotcontrol_sys::rc_led_cleanup();
		librobotcontrol_sys::rc_motor_cleanup();
		librobotcontrol_sys::rc_servo_cleanup();

		librobotcontrol_sys::rc_bmp_power_off();
		librobotcontrol_sys::rc_mpu_power_off();
	}
}

/// Buttons
#[derive(Default, Copy, Clone)]
pub enum Button {
	#[default]
	Pause = 5,
	Mode = 4,
}
pub fn init_button(button: Button) {
	unsafe { librobotcontrol_sys::rc_button_init(2, button as i32, 1, BUTTON_DEBOUNCE); }
}
pub fn register_button_callback(button: Button, callback: unsafe extern "C" fn()) {
	unsafe { librobotcontrol_sys::rc_button_set_callbacks(2, button as i32, Some(callback), None); }
}


/// Encoders
#[derive(Default, Copy, Clone)]
pub enum Encoder {
	#[default]
	ENCODER1 = 1,
	ENCODER2 = 2,
	ENCODER3 = 3,
	ENCODER4 = 4,
}
pub fn init_encoders() {
	unsafe {
		librobotcontrol_sys::rc_encoder_eqep_init();
		librobotcontrol_sys::rc_encoder_pru_init();
	}
}
pub fn get_encoder_value(encoder: Encoder) -> i32 {
	match encoder {
		Encoder::ENCODER4 => unsafe { librobotcontrol_sys::rc_encoder_pru_read() }
		_ =>  unsafe { librobotcontrol_sys::rc_encoder_eqep_read(encoder as i32) }
	}
}


/// Motors
#[derive(Default, Copy, Clone)]
pub enum Motor {
	#[default]
	MOTOR1 = 1,
	MOTOR2 = 2,
	MOTOR3 = 3,
	MOTOR4 = 4,
}
pub fn init_motors() {
	unsafe {
		librobotcontrol_sys::rc_motor_init_freq(MOTOR_PWM);
		librobotcontrol_sys::rc_motor_brake(Motor::MOTOR1 as i32);
		librobotcontrol_sys::rc_motor_brake(Motor::MOTOR2 as i32);
		librobotcontrol_sys::rc_motor_brake(Motor::MOTOR3 as i32);
		librobotcontrol_sys::rc_motor_brake(Motor::MOTOR4 as i32);
	}
}
pub fn run_motor(motor: Motor, speed: f64) -> i32 {
	let mut duty = if speed > 1.0 { 1.0 } else { speed };
	duty = if speed < -1.0 { -1.0 } else { duty };
	unsafe { librobotcontrol_sys::rc_motor_set(motor as i32, duty) }
}
pub fn brake_motor(motor: Motor) -> i32 {
	unsafe { librobotcontrol_sys::rc_motor_brake(motor as i32) }
}


/// GPIO
#[derive(Clone, Copy)]
pub enum GpioChip {
	GPIO0 = 0,
	GPIO1 = 1,
	GPIO2 = 2,
	GPIO3 = 3,
}
#[derive(Clone, Copy)]
pub enum GpioHandle {
	INPUT = 0,
	OUTPUT = 2,
	ACTIVE_LOW = 4,
	OPEN_DRAIN = 8,
	OPEN_SOURCE = 16,
}
pub enum GpioTrigger {
	LOW,
	HIGH,
}
impl GpioTrigger {
	pub(crate) fn get(&self) -> i32 {
		match self {
			GpioTrigger::LOW => 0,
			GpioTrigger::HIGH => 1,
		}
	}
	pub(crate) fn inv(&self) -> i32 {
		match self {
			GpioTrigger::LOW => 1,
			GpioTrigger::HIGH => 0,
		}
	}
}
pub fn gpio_init(chip: GpioChip, pin: i32, direction: GpioHandle) -> Result<(), String> {
	if unsafe { librobotcontrol_sys::rc_gpio_init(chip as i32, pin, direction as i32) } != 0 {
		Err(format!("Unable to open GPIO{}_{}", chip as i32, pin))
	} else {
		Ok(())
	}
}
pub fn gpio_cleanup(chip: GpioChip, pin: i32) {
	unsafe { librobotcontrol_sys::rc_gpio_cleanup(chip as i32, pin); }
}
pub fn gpio_send_pulse(chip: GpioChip, pin: i32, value: GpioTrigger, time: Duration) -> Result<(), String> {
	if unsafe { librobotcontrol_sys::rc_gpio_set_value(chip as i32, pin, value.get()) } != 0 {
		return Err(format!("Unable to initialize pulse to GPIO{}_{} for {}us", chip as i32, pin, time.as_micros()));
	}
	sleep(time);
	if unsafe { librobotcontrol_sys::rc_gpio_set_value(chip as i32, pin, value.inv()) } != 0 {
		return Err(format!("Unable to tear down pulse to GPIO{}_{} after {}us", chip as i32, pin, time.as_micros()));
	}
	Ok(())
}
pub fn gpio_read_pulse(chip: GpioChip, pin: i32, value: GpioTrigger) -> Result<Duration, String> {
	let signal_err = -1;
	let signal_check = value.get();
	let max_loop = 10240;
	let mut num_loop = 0;

	let mut signal = unsafe { librobotcontrol_sys::rc_gpio_get_value(chip as i32, pin) };
	if signal == signal_err { return Err(format!("Unable to get a signal from GPIO{}_{}", chip as i32, pin)); }

	// Wait for the signal to start
	while signal != signal_check {
		num_loop = num_loop + 1;
		if num_loop > max_loop { return Err(String::from("Timeout while reading GPIO")); }
		signal = unsafe { librobotcontrol_sys::rc_gpio_get_value(chip as i32, pin) };
	}

	// Count the duration the signal is in the given state
	let start = Instant::now();
	while signal == signal_check {
		num_loop = num_loop + 1;
		if num_loop > max_loop { return Err(String::from("Timeout while reading GPIO")); }
		signal = unsafe { librobotcontrol_sys::rc_gpio_get_value(chip as i32, pin) };
	}
	let duration = start.elapsed();
	Ok(duration)
}
