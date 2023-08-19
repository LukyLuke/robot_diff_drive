extern crate librobotcontrol_sys;

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
pub fn run_motor(motor: Motor, duty: f64) {
	unsafe { librobotcontrol_sys::rc_motor_set(motor as i32, duty); }
}
pub fn brake_motor(motor: Motor) {
	unsafe { librobotcontrol_sys::rc_motor_brake(motor as i32); }
}
