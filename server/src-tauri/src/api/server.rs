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

		// Format = [neg?, percentage out of 100, abs/key, code, evtype]
		
		
		let _ = device.synchronise();
		debug!("SERVER | {} | Received Value: {:?}", client_information, recv_result);
	}
}

fn parse_data(data: [u8; 5]) {
	let mut value = data[1] as isize;
	if data[0] == 1 {
		value = value * -1;
	}
}