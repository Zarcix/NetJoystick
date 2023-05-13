mod axis;
mod buttons;

pub use axis::Axis;
pub use buttons::Button;

use input_linux::{sys};
use std::{fs, path, io::Error};

pub struct Joystick {
	device: input_linux::UInputHandle<fs::File>,
}

impl Joystick {
	pub fn new(identifier: String) -> Result<Self, Error> {
		let device = create_device(identifier)?;
		
		Ok(Self {device})
	}
	
	pub fn get_path(&self) -> Result<path::PathBuf, Error> {
		Ok(self.device.evdev_path()?)
	}
	
	
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
		}).collect::<Vec<_>>() // i dunno what the type is too lazy to find lol
	)?;
	
	Ok(device)
}