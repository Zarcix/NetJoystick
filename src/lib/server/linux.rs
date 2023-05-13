use std::sync::mpsc::*;

use super::vjoystick::Joystick;

pub struct Server {
	clients: std::collections::HashMap<String, Sender<[u8; 5]>>
}

impl Server {
	pub fn new() -> Self {
		Self {
			clients: std::collections::HashMap::new()
		}
	}
	
	pub fn parse_data(&mut self, client_information: String, data: [u8; 5]) {
		let potential_client = self.clients.get(&client_information);
		
		let client;
		
		if potential_client.is_none() {
			self.add_client(client_information.clone());
			client = self.clients.get(&client_information).unwrap();
		} else {
			client = potential_client.unwrap();
		}
		
		if client.send(data).is_err() {
			println!("
--Debug-- | lib::server::linux::Server::parse_data
	Attempting to send data to a deallocated receiver
	Arguments: {}", 
			client_information);
		}
		
	}
	
	pub fn remove_client(&mut self, client_information: String) {
		let potential_client = self.clients.get(&client_information);
		if potential_client.is_none() {
			println!("
--Debug-- | lib::server::linux::Server::remove_client
	Attempting to remove client that doesn't exist
	Arguments: {}",
			&client_information);
			return
		}
		
		// Remove and resize to save memory
		let sender = self.clients.remove(&client_information).unwrap();
		sender.send([6; 5]).unwrap();
		drop(sender);
		
		self.clients.shrink_to_fit();
	}
	
	fn add_client(&mut self, client_information: String) {
		println!("Adding Client: {}", client_information.clone());
		let (sender, receiver) = channel::<[u8; 5]>();
		
		// Push the client's sender 
		if self.clients.insert(client_information.clone(), sender).is_some() {
			println!("
--Debug-- | lib::server::linux::Server::add_client
	Ovewrite of old client information occured:
	Arguments: \nClient Information: {}\n", 
			&client_information);
		}
		
		// Create receive request from current client's sender
		std::thread::spawn(move || {
			// Create Device here
			
			
			loop {
				// If sender is deallocated, then end thread
				let recv_result = receiver.recv();
				if recv_result.is_err() {
					break
				}
				let recv_result = recv_result.unwrap();
				
				if recv_result[0] == 6 {
					// Exit Case
					break
				}
				
				let mut amount = u16::from_be_bytes([recv_result[3], recv_result[4]]) as f32;
				if recv_result[2] == 0 {
					amount = amount * -1.0;
				}
				println!("Received Value: {} from {}", amount, client_information.clone());
			}
			println!("
--Debug-- | lib::server::linux::Server::add_client
	Thread Died, Deleting Device since Send must have been deleted
	Arguments: {}", &client_information);
			// Delete Device here
			
			
		});
	}
}