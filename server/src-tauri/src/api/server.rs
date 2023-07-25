use std::sync::mpsc::*;
use std::collections::{HashMap, HashSet};

use super::vjoystick::Joystick;

use log::*;

pub struct Server {
	connected_clients: HashMap<String, SyncSender<[u8; 5]>>,
	potential_clients: HashSet<String>
}

impl Server {
	pub fn new() -> Self {
		Self {
			connected_clients: std::collections::HashMap::new(),
			potential_clients: HashSet::new()
		}
	}
	
	// Potential Clients

	pub fn add_client(&mut self, client_information: String) {
		if !self.potential_clients.insert(client_information.clone()) {
			debug!("SERVER | {} | Failed to add client to potential clients", client_information)
		}
	}

	pub fn remove_client(&mut self, client_information: String) {
		if !self.potential_clients.remove(&client_information) {
			debug!("SERVER | {} | Failed to remove client from potential clients", client_information.to_string());
		}
	}

	pub fn get_clients(&self) -> HashSet<String> {
		return self.potential_clients.clone();
	}

	pub fn find_client(&self, client_information: String) -> Option<&String> {
		self.potential_clients.get(&client_information)
	}

	// Connected Clients

	pub fn connect_client(&mut self, client_information: String) {
		debug!("Adding Client: {}", client_information.clone());
		
		let (sender, receiver) = sync_channel::<[u8; 5]>(5);
		
		// Push the client's sender 
		let old_elem = self.connected_clients.insert(client_information.clone(), sender);
		if old_elem.is_some() {
			warn!("Ovewriting old ip's sender with a new sender for: {}", client_information.clone());
		}

		// Create receive request from current client's sender
		std::thread::spawn(move || {

			if let Ok(device) = Joystick::new(client_information.clone()) {
				start_device(&device, receiver, &client_information);
				warn!("SERVER | {} | Thread dying. Removing", &client_information);

				// Free up device
				if device.destroy_device().is_err() {
					error!("SERVER | {:?} | Failed to destroy device", device.get_path())
				}
			} else {
				warn!("SERVER | {} | Failed to create joystick...exiting", &client_information);
			}
			
			
		});
	}

	pub fn disconnect_client(&mut self, client_information: String) {
		let removal = self.connected_clients.remove(&client_information);
		debug!("SERVER | {client_information} | {} | Removed client with result", if removal.is_some() { "success" } else { "fail" } );
	}

	pub fn find_connected_client(&self, client_information: String) -> Option<&SyncSender<[u8; 5]>> {
		self.connected_clients.get(&client_information)
	}

	pub fn get_connected_clients(&self) -> &HashMap<String, SyncSender<[u8; 5]>> {
		return &self.connected_clients
	}
}

fn start_device(device: &Joystick, receiver: Receiver<[u8; 5]>, client_information: &String) {
	loop {
		// If sender is deallocated, then end thread
		let recv_result = receiver.recv();
		if recv_result.is_err() {
			break
		}
		let recv_result = recv_result.unwrap();

		write_data(device, &recv_result);
		
		
		let _ = device.synchronise();
		debug!("SERVER | {} | Received Value: {:?}", client_information, recv_result);
	}
}

fn write_data(device: &Joystick, data: &[u8; 5]) {
	// Format = [neg?, /100, abs/key, code, evtype]
	
	match data[2] {
		1 => {
			let parsed_data = parse_button(&data);
			if parsed_data.0.is_none() {
				error!("SERVER | {:?} | Button not found. Not continuing", data);
				return
			}

			let _ = device.button_press(parsed_data.0.unwrap(), parsed_data.1);
		}
		2 => {
			parse_joystick(&data);
		}
		_ => {
			warn!("SERVER | {} | Unrecognized input type", data[2])
		}
	}
}

fn parse_joystick(data: &[u8; 5]) {
	// Parse Move Amount
	let mut move_amt = data[1] as i32;
	if data[0] == 1 {
		move_amt = move_amt * -1;
	}

	
}

fn joystick_map(i: u8) -> Option<super::vjoystick::Axis> {
	use super::vjoystick::Axis::*;

	match i {
		0 => Some(LJSH),
		1 => Some(LJSV),
		2 => Some(TRGL),
		3 => Some(RJSH),
		4 => Some(RJSV),
		5 => Some(TRGR),
		6 => Some(HT0X),
		7 => Some(HT0Y),
		_ => None
	}
}

fn parse_button(data: &[u8; 5]) -> (Option<super::vjoystick::Button>, bool) {
	let is_pressed = data[1] == 1;
	let button = button_map(data[3]);

	(button, is_pressed)
}

fn button_map(i: u8) -> Option<super::vjoystick::Button> {
	use super::vjoystick::Button::*;

	match i {
		0 => Some(A),
		1 => Some(B),
		2 => Some(Y),
		3 => Some(X),
		4 => Some(Select),
		5 => Some(Start),
		6 => Some(ShoulderL),
		7 => Some(ShoulderR),
		8 => Some(ThumbL),
		9 => Some(ThumbR),
		10 => Some(Mode),
		_ => None
	}
}