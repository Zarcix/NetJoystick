use evdev::EventStream;


pub struct CliController {
	device: EventStream, // ID of current device
}

// Setter Getter
impl CliController {
	pub fn get_device(&self) -> &EventStream {
		return &self.device
	}
	
	pub fn change_device(&mut self, new_id: EventStream) {
		self.device = new_id
	}
}

impl CliController {
	pub fn new(device: EventStream) -> Self {
		Self {device}
	}
	
	pub async fn next_event(&mut self) -> Result<evdev::InputEvent, std::io::Error> {
		return self.device.next_event().await
	}
}