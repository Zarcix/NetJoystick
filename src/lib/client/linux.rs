use evdev::Device;

pub struct CliController {
	id: Device, // ID of current device
}

// Setter Getter
impl CliController {
	pub fn get_id(&self) -> &Device {
		return &self.id
	}
	
	pub fn set_id(&mut self, new_id: Device) {
		self.id = new_id
	}
}

impl CliController {
	pub fn new(id: Device) -> Self {
		Self {id}
	}
}