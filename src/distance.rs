use super::hal;

use std::time::Duration;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub fn new(trigger: (hal::GpioChip, i32), echo: (hal::GpioChip, i32)) -> Ultrasonic {
	Ultrasonic {
		trigger_chip: trigger.0,
		trigger_pin: trigger.1,
		echo_chip: echo.0,
		echo_pin: echo.1,
		stop: Arc::new(AtomicBool::new(false)),
	}
}

#[derive(Clone)]
pub struct Ultrasonic {
	trigger_chip: hal::GpioChip,
	trigger_pin: i32,
	echo_chip: hal::GpioChip,
	echo_pin: i32,
	stop: Arc<AtomicBool>,
}

impl Ultrasonic {
	/// Start the distance measure process in a thread
	pub fn start(&mut self) -> Result<(),()> {
		match hal::gpio_init(self.trigger_chip, self.trigger_pin, hal::GpioHandle::OUTPUT) {
			Err(_) => return Err(()),
			_ => {},
		}
		match hal::gpio_init(self.echo_chip, self.echo_pin, hal::GpioHandle::INPUT) {
			Err(_) => return Err(()),
			_ => {},
		}

		// Spawn an unhandled thread
		thread::spawn({
			let stop = self.stop.clone();
			let trigger = (self.trigger_chip.clone(), self.trigger_pin.clone());
			let echo = (self.echo_chip.clone(), self.echo_pin.clone());

			// Distance = ((speed of sound in the air) * time) / 2
			// Speed of sound is: 343m/s = 0.0343 cm/uS
			let speed_constant = 0.0343 / 2.0;

			move || {
				loop {
					let inner_stop = stop.clone();
					if inner_stop.load(Ordering::Relaxed) { break; }

					// Initialize the Sensor by sending a 10 ms pulse
					let result = match hal::gpio_send_pulse(trigger.0, trigger.1, hal::GpioTrigger::LOW, Duration::from_micros(10)) {
						Ok(_) => {
							hal::gpio_read_pulse(echo.0, echo.1, hal::GpioTrigger::HIGH)
						},
						Err(err) => Err(err),
					};

					let duration = result.unwrap_or_default();
					let distance = speed_constant * duration.as_micros() as f64;
					println!("Distance ({:?}): {}cm", duration, distance);

					thread::sleep(Duration::from_millis(10));
				}

				println!("Distance Thread stopped");
				hal::gpio_cleanup(trigger.0, trigger.1);
				hal::gpio_cleanup(echo.0, echo.1);
			}
		});
		Ok(())
	}

	/// Stop the distance measurement
	pub fn stop(&mut self) {
		self.stop.store(true, Ordering::Relaxed);
	}

}
