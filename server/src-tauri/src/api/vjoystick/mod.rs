mod axis;
mod buttons;

pub use axis::Axis;
pub use buttons::Button;

use input_linux::sys;
use std::{fs, path, io::Error};

pub struct Joystick {
	device: input_linux::UInputHandle<fs::File>,
}

impl Joystick {
	// Device Initialization

	pub fn new(identifier: String) -> Result<Self, Error> {
		let device = create_device(identifier)?;
		
		Ok(Self {device})
	}
	
	pub fn get_path(&self) -> Result<path::PathBuf, Error> {
		Ok(self.device.evdev_path()?)
	}
	
	pub fn destroy_device(&self) -> Result<(), Error> {
		self.device.dev_destroy()
	}

	// Device Movement

	pub fn move_axis(&self, axis: Axis, position: i32) -> Result<(), std::io::Error> {
		if position < 0 || position > 100 {
			// TODO Turn into proper error
			return Err(std::io::ErrorKind::Other.into());
		}

		self.write_event(input_linux::AbsoluteEvent::new(
			empty_event_time(),
			axis.to_evdev(),
			position
		))
	}

	pub fn button_press(&self, button: Button, is_pressed: bool) -> Result<(), Error> {
		let value = if is_pressed {
            input_linux::KeyState::PRESSED
        } else {
            input_linux::KeyState::RELEASED
        };

		self.write_event(input_linux::KeyEvent::new(
            empty_event_time(),
            button.to_evdev(),
            value,
        ))
	}
	
	pub fn synchronise(&self) -> Result<(), Error> {
        self.write_event(input_linux::SynchronizeEvent::report(empty_event_time()))
    }

	fn write_event(&self, event: impl std::convert::AsRef<sys::input_event>) -> Result<(), Error> {
        self.device.write(&[*event.as_ref()])?;

        Ok(())
    }
}

fn empty_event_time() -> input_linux::EventTime {
    input_linux::EventTime::default()
}

fn create_device(identifier: String) -> Result<input_linux::UInputHandle<fs::File>, Error> {
	let uinput_file = fs::File::create("/dev/uinput")?;
	let device = input_linux::UInputHandle::new(uinput_file);
	
	let input_id = input_linux::InputId {
		bustype: sys::BUS_VIRTUAL,
		vendor: 69, // Heh
		product: 420, // What you smokin
		version: 7, // WYSI
	};
	
	let standard_info = input_linux::AbsoluteInfo {
		value: 0,
		minimum: -100, // Percentage Extremes
		maximum: 100, // Percentage Extremes
		fuzz: 0,
		flat: 0,
		resolution: 50,
	};
	
	device.set_evbit(input_linux::EventKind::Absolute)?;
	device.set_evbit(input_linux::EventKind::Key)?;
	device.set_keybit(input_linux::Key::ButtonTrigger)?; // Used to force joystick to being a joystick.
	
	for button in Button::all_buttons() {
        device.set_keybit(button.to_evdev())?;
    }
	
	device.create(
		&input_id,
		format!("VJoystick for {}", identifier).as_bytes(),
		0, // No FF today rip
		&Axis::all_axis().map(|current_axis| input_linux::AbsoluteInfoSetup {
			axis: current_axis.to_evdev(),
			info: standard_info,
		}).collect::<Vec<_>>()
	)?;
	
	Ok(device)
}