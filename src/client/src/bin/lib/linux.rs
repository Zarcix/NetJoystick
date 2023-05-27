use evdev::EventStream;


pub struct CliController {
	device: EventStream, // ID of current device
	calibration: Vec<Vec<i32>> // [[axis_min, axis_max], [axis_min, axis_max], [axis_min, axis_max]]
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
		Self {
			device,
			calibration: Vec::new(),
		}
	}
	
	pub fn calibrate(&mut self) {
		println!("Calibration Called");
		use std::time::{Duration, Instant};
		
		let runtime = tokio::runtime::Runtime::new().unwrap();
		// L Joystick Horizontal
		let mut l_h_min = i32::max_value();
		let mut l_h_max = i32::min_value();
		// L Joystick Vertical
		let mut  l_v_min = i32::max_value();
		let mut l_v_max = i32::min_value();
		
		// R Joystick Horizontal
		let mut r_h_min = i32::max_value();
		let mut  r_h_max = i32::min_value();
		// R Joystick Vertical
		let mut  r_v_min = i32::max_value();
		let mut  r_v_max = i32::min_value();
		
		// L Trigger
		let mut l_t_min = i32::max_value();
		let mut l_t_max = i32::min_value();
		
		// R Trigger
		let mut r_t_min = i32::max_value();
		let mut r_t_max = i32::min_value();
		
		let mut start = Instant::now();
		let end = start + Duration::from_secs(5);
		
		loop {
			if start >= end {
				break;
			} else {
				start = Instant::now();
			}
			
			let event = runtime.block_on(async {
				let ev = self.device.next_event().await;
				ev
			});
			
			if event.is_err() {
				println!("No next event in calibrate()");
				return
			}
			
			let event = event.unwrap();
			
			//// If Joystick Horizontal
			
			// Left
			if event.kind() == evdev::InputEventKind::AbsAxis(evdev::AbsoluteAxisType::ABS_X) {
				if l_h_min > event.value() {
					println!("a");
					l_h_min = event.value();
				} else if l_h_max < event.value() {
					l_h_max = event.value();
				}
			}
			
			// Right
			if event.kind() == evdev::InputEventKind::AbsAxis(evdev::AbsoluteAxisType::ABS_RX) {
				if r_h_min > event.value() {
					r_h_min = event.value();
				} else if r_h_max < event.value() {
					r_h_max = event.value();
				}
			}
			
			//// If Joystick Vertical
			
			// Left
			if event.kind() == evdev::InputEventKind::AbsAxis(evdev::AbsoluteAxisType::ABS_Y) {
				if l_v_min > event.value() {
					l_v_min = event.value();
				} else if l_v_max < event.value() {
					l_v_max = event.value();
				}
			}
			
			// Right
			if event.kind() == evdev::InputEventKind::AbsAxis(evdev::AbsoluteAxisType::ABS_RY) {
				if r_v_min > event.value() {
					r_v_min = event.value();
				} else if r_v_max < event.value() {
					r_v_max = event.value();
				}
			}
			
			//// If Trigger
			
			// Left
			if event.kind() == evdev::InputEventKind::AbsAxis(evdev::AbsoluteAxisType::ABS_Z) {
				if l_t_min > event.value() {
					l_t_min = event.value();
				} else if l_t_max < event.value() {
					l_t_max = event.value();
				}
			}
			
			// Right
			if event.kind() == evdev::InputEventKind::AbsAxis(evdev::AbsoluteAxisType::ABS_RZ) {
				if r_t_min > event.value() {
					r_t_min = event.value();
				} else if r_t_max < event.value() {
					r_t_max = event.value();
				}
			}
			
		}
		
		//// Push into calibration
		// Left Joystick
		let l_j_calibration = Vec::from([Vec::from([l_h_min, l_h_max]), Vec::from([l_v_min, l_v_max])]);
		
		// Right Joystick
		let r_j_calibration = Vec::from([Vec::from([r_h_min, r_h_max]), Vec::from([r_v_min, r_v_max])]);
		
		// Triggers
		let t_calibration = Vec::from([Vec::from([l_t_min, l_t_max]), Vec::from([r_t_min, r_t_max])]);
		
		self.calibration = [l_j_calibration.as_slice(), r_j_calibration.as_slice(), t_calibration.as_slice()].concat();
		
		println!("{:?}", self.calibration);
	}
	
	pub async fn next_event(&mut self) -> Result<evdev::InputEvent, std::io::Error> {
		return self.device.next_event().await
	}
}