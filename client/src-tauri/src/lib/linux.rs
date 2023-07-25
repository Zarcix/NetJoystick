use evdev::EventStream;
use log::*;

pub struct CliController {
	device: EventStream, // ID of current device
	calibration: Vec<Vec<i32>> // [[LH], [LV], [RH], [RV], [TL], [TR]]
}

// Setter Getter

impl CliController {
	pub fn get_device(&mut self) -> &mut EventStream {
		return &mut self.device
	}
}

impl CliController {
	pub fn new(device: EventStream) -> Self {
		Self {
			device,
			calibration: Vec::new(),
		}
	}
	
	pub fn calibrate(&mut self) -> Result<(), i32> {
		info!("Calibration Called");
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
				return Err(1)
			}
			
			let event = event.unwrap();
			
			//// If Joystick Horizontal
			
			// Left
			if event.kind() == evdev::InputEventKind::AbsAxis(evdev::AbsoluteAxisType::ABS_X) {
				if l_h_min > event.value() {
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
		
		debug!("--\nCalibration for device: {}\n{:?}\n--", self.device.device().name().unwrap(), self.calibration);

		Ok(())
	}
	
	pub fn parse_input(&self, event: &evdev::InputEvent) -> (bool, u8) {
		let max_calibration: Vec<_> = self.calibration.iter().map(|calib| { calib.get(1).unwrap() }).collect();

		let mut ret: (bool, u8) = (false, 0); // (neg bit, percentage * 100 as i32)

		let ljs = self.calibration.get(0..2).unwrap();
		let ljs_h_offset = (ljs.get(0).unwrap().get(0).unwrap() + ljs.get(0).unwrap().get(1).unwrap()) / 2;
		let ljs_v_offset = (ljs.get(1).unwrap().get(0).unwrap() + ljs.get(1).unwrap().get(1).unwrap()) / 2;

		let rjs = self.calibration.get(2..4).unwrap();
		let rjs_h_offset = (rjs.get(0).unwrap().get(0).unwrap() + rjs.get(0).unwrap().get(1).unwrap()) / 2;
		let rjs_v_offset = (rjs.get(1).unwrap().get(0).unwrap() + rjs.get(1).unwrap().get(1).unwrap()) / 2;
		
		match &event.kind() {
			// TODO There is a better way to do this....this has to be i'm just dumb
			// Left
			evdev::InputEventKind::AbsAxis(evdev::AbsoluteAxisType::ABS_X) => {
				let new_min = self.calibration.get(0).unwrap().get(0).unwrap() - ljs_h_offset;
				let new_max = self.calibration.get(0).unwrap().get(1).unwrap() - ljs_h_offset;
				
				let offset_val = event.value() - ljs_h_offset;
				if offset_val.is_negative() {
					ret.0 = true;
					ret.1 = ((offset_val as f32 / new_min as f32) * 100 as f32).round() as u8;
				} else {
					ret.0 = false;
					ret.1 = ((offset_val as f32 / new_max as f32) * 100 as f32).round() as u8;
				}
			},
			evdev::InputEventKind::AbsAxis(evdev::AbsoluteAxisType::ABS_Y) => {
				let new_min = self.calibration.get(1).unwrap().get(0).unwrap() - ljs_v_offset;
				let new_max = self.calibration.get(1).unwrap().get(1).unwrap() - ljs_v_offset;
				
				let offset_val = event.value() - ljs_v_offset;
				if offset_val.is_negative() {
					ret.0 = true;
					ret.1 = ((offset_val as f32 / new_min as f32) * 100 as f32).round() as u8;
				} else {
					ret.0 = false;
					ret.1 = ((offset_val as f32 / new_max as f32) * 100 as f32).round() as u8;
				}
			},

			// Right
			evdev::InputEventKind::AbsAxis(evdev::AbsoluteAxisType::ABS_RX) => {
				let new_min = self.calibration.get(2).unwrap().get(0).unwrap() - rjs_h_offset;
				let new_max = self.calibration.get(2).unwrap().get(1).unwrap() - rjs_h_offset;
				
				let offset_val = event.value() - rjs_h_offset;
				if offset_val.is_negative() {
					ret.0 = true;
					ret.1 = ((offset_val as f32 / new_min as f32) * 100 as f32).round() as u8;
				} else {
					ret.0 = false;
					ret.1 = ((offset_val as f32 / new_max as f32) * 100 as f32).round() as u8;
				}
			},
			evdev::InputEventKind::AbsAxis(evdev::AbsoluteAxisType::ABS_RY) => {
				let new_min = self.calibration.get(3).unwrap().get(0).unwrap() - rjs_v_offset;
				let new_max = self.calibration.get(3).unwrap().get(1).unwrap() - rjs_v_offset;
				
				let offset_val = event.value() - rjs_v_offset;
				if offset_val.is_negative() {
					ret.0 = true;
					ret.1 = ((offset_val as f32 / new_min as f32) * 100 as f32).round() as u8;
				} else {
					ret.0 = false;
					ret.1 = ((offset_val as f32 / new_max as f32) * 100 as f32).round() as u8;
				}
			},

			// Triger
			evdev::InputEventKind::AbsAxis(evdev::AbsoluteAxisType::ABS_Z) => {
				let max_value = max_calibration.get(4).unwrap().clone().to_owned();
				ret.1 = ((event.value() as f32 / max_value as f32) * 100 as f32).round() as u8;
			},
			evdev::InputEventKind::AbsAxis(evdev::AbsoluteAxisType::ABS_RZ) => {
				let max_value = max_calibration.get(5).unwrap().clone().to_owned();
				ret.1 = ((event.value() as f32 / max_value as f32) * 100 as f32).round() as u8;
			},

			// DPAD
			evdev::InputEventKind::AbsAxis(evdev::AbsoluteAxisType::ABS_HAT0X) | evdev::InputEventKind::AbsAxis(evdev::AbsoluteAxisType::ABS_HAT0Y) => {
				ret.0 = event.value().is_negative();
				ret.1 = event.value().abs() as u8;
			}
			_ => ()
		}
		ret
	}

	pub async fn next_event(&mut self) -> Result<evdev::InputEvent, std::io::Error> {
		return self.device.next_event().await
	}
}