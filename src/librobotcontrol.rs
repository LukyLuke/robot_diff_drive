extern crate librobotcontrol_sys;

pub enum Button {
	Pause = 5,
	Mode = 4
}
pub const BUTTON_DEBOUNCE: ::std::os::raw::c_int = 2000;

pub fn init_button(button: Button) {
	unsafe { librobotcontrol_sys::rc_button_init(2, button as i32, 1, BUTTON_DEBOUNCE); }
}

pub fn register_button_callback(button: Button, callback: unsafe extern "C" fn()) {
	unsafe { librobotcontrol_sys::rc_button_set_callbacks(2, button as i32, Some(callback), None); }
}

pub fn init_encoders() {
	unsafe {
		librobotcontrol_sys::rc_encoder_eqep_init();
		librobotcontrol_sys::rc_encoder_pru_init();
	}
}

pub fn get_encoder_value(encoder: i32) -> i32 {
	match encoder {
		4 => unsafe { librobotcontrol_sys::rc_encoder_pru_read() }
		_ =>  unsafe { librobotcontrol_sys::rc_encoder_eqep_read(encoder) }
	}
}

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
	};

}
