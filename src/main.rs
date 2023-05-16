mod lib;

use lib::client::linux::CliController;

use std::net::UdpSocket;

use std::sync::{Arc, Mutex};

/*** 
 * Linux Helper Functions 
 */

/**
 * Client Helper Functions
 */

fn search_for_devices() -> Vec<(std::path::PathBuf, evdev::Device)> {
    let all_devs = evdev::enumerate();
    
    let mut available_devs = Vec::new();
    
    for dev in all_devs {
        let cur_dev = dev.1.supported_absolute_axes().map(|ins| ins.iter().collect::<Vec<_>>().contains(&evdev::AbsoluteAxisType::ABS_X));
        if cur_dev.is_some() && cur_dev.unwrap() == true {
            available_devs.push(dev);
        }
    }
    
    return available_devs;
}

fn event_to_packet(event: evdev::InputEvent) {
    // Packet Structure: [Client Id, Event Kind, Event Kind's Type, percentage value]
    
    let event_value = event.value();
    
}

/**
 * Server Helper Functions
 */



/***
 * Linux Main
 */

#[cfg(target_os = "linux")]
fn main() -> std::io::Result<()> {
    let client = true;
    
    // UI Asks for Use Case
    
    // On Case of Client
    if client {
        println!("Client");
        
        let mut device_list = search_for_devices();
        
        // Get user input for device selection
        
        let chosen_device = 0;
        let mut device = device_list.remove(chosen_device);
        drop(device_list); 
        
        // Get server IP from user
        
        let mut chosen_server = "192.168.4.122";
        
        // Create Client Connection
        
        let socket = std::sync::Arc::new(UdpSocket::bind("0.0.0.0:420").expect("could not bind to address"));
        socket.connect(format!("{}:69", chosen_server)).expect("could not connect to provided server");
        
        // Create Device
        
        let user_cli = Arc::new(Mutex::new(CliController::new(device.1.into_event_stream().unwrap())));
        
        // Calibrate Device
        
        let mut guard = user_cli.lock().unwrap();
        guard.calibrate();
        drop(guard);
        
        // Create Client Channels
            
        let (dev_socket_send, dev_socket_recv) = std::sync::mpsc::channel::<(Arc<Mutex<CliController>>, Arc<UdpSocket>)>();
        
		// Start Controller Thread
        
        let runtime = tokio::runtime::Runtime::new().unwrap();
		
        std::thread::spawn(move || {
			loop {
				// Read input from channel
				let pot_data = dev_socket_recv.recv();
				if pot_data.is_err() {
                    // Recv failed, kill thread since send no longer exists
                    drop(dev_socket_recv);
					panic!("A Sender Channel was closed. Killing thread...");
				}
				let (device, socket) = pot_data.unwrap();
				let mut device = device.lock().unwrap();
				
				// Read Input from Device
				let device_event = runtime.block_on(async {
                    // Run async code from client device
					let result = device.next_event().await;
					result
				});
				if device_event.is_err() {
                    // Device Event is invalid now (allegedly). Kill recv and kill thread
                    drop(dev_socket_recv);
					break;
				}
				let device_event = device_event.unwrap();
				
				// TODO Convert event to packet
				
				// TODO Send packet over Socket
				
			}
			
		});
        
		loop {
			// Send Device and Socket to sender thread
			let send_stat = dev_socket_send.send((user_cli.clone(), socket.clone()));
			if send_stat.is_err() {
				panic!("Receive thread has died. Main Crashing...")
                // TODO Perform a reset since device read failed
			}
			// TODO Check if user wishes to change device
			
			// TODO Check if user wishes to change server
		}
    } else {
        // On Case of Server
        
        println!("Server");
        let mut data = [0; 5];
        
        let socket = UdpSocket::bind("0.0.0.0:999")?;
        
        let mut server_object = lib::server::linux::Server::new();
        
        loop {
            
            let (_, src) = socket.recv_from(&mut data)?;
            //println!("Server Received: {:?}\nSending to client", data);
            
            server_object.parse_data(src.to_string(), data);
        }
        
    }
    
    
    
}

/***
 * Windows Client Helper Functions
 */

/***
 * Windows Main
 */

#[cfg(target_os = "windows")]
fn main() {
    println!("TODO windows")
}

/***
 * Mac Client Helper Functions
 */

/***
 * Mac Main
 */

#[cfg(target_os = "macos")]
fn main() {
    println!("TODO macos")
}