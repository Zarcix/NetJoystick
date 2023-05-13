mod lib;

use lib::client::linux::CliController;

use std::net::UdpSocket;

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
        
        let userCli = std::sync::Arc::new(std::sync::Mutex::new(CliController::new(device.1.into_event_stream().unwrap())));
        
        // Create Client Channels
            
        let (dev_socket_send, dev_socket_recv) = std::sync::mpsc::channel::<(std::sync::Arc<std::sync::Mutex<CliController>>, std::sync::Arc<UdpSocket>)>();
        
		// No mutexes are needed since only one thread is being created. No overwriting of data concurrently will happen like ever...hopefully
		
        std::thread::spawn(move || {
			loop {
				// Tokio Blocking Runtime
				
				
				// Read input from channel
				let pot_data = dev_socket_recv.recv();
				if pot_data.is_err() {
					panic!("Sender Channel was closed. Thread crashing...");
				}
				let (device, socket) = pot_data.unwrap();
				let mut device = device.lock().unwrap();
				
				// Read Input from Device
				let runtime = tokio::runtime::Runtime::new().unwrap();
				let device_event = runtime.block_on(async {
					let result = device.next_event().await;
					result
				});
				if device_event.is_err() {
					break;
				}
				let device_event = device_event.unwrap();
				
				// TODO Convert event to packet
				
				// TODO Send packet over Socket
				
			}
			
		});
        
		loop {
			// Send Device and Socket to sender thread
			let send_stat = dev_socket_send.send((userCli.clone(), socket.clone()));
			if send_stat.is_err() {
				panic!("Receiver Channel was closed. Thread has died. Main Crashing...")
			}
			// TODO Check if user wishes to change device
			
			// TODO Check if user wishes to change server
		}
		
        //let device = Device {id: wanted_id.unwrap()};
//         loop {
//             let bytes = device.get_send_devEvent(&mut gilrs);
//             if bytes.is_some() {
//                 let mut amount = u16::from_be_bytes([bytes.unwrap()[3], bytes.unwrap()[4]]) as f32;
//                 if bytes.unwrap()[2] == 0 {
//                     amount = amount * -1.0;
//                 }
//                 println!("Sent Value: {}", amount);
//                 
//                 socket.send_to(&bytes.unwrap(), "192.168.4.122:999")?;
//             }
//         }
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

fn search_for_devices() -> Vec<(std::path::PathBuf, evdev::Device)> {
    let allDevs = evdev::enumerate();
    
    let mut available_devs = Vec::new();
    
    for dev in allDevs {
        let cur_dev = dev.1.supported_absolute_axes().map(|ins| ins.iter().collect::<Vec<_>>().contains(&evdev::AbsoluteAxisType::ABS_X));
        if cur_dev.is_some() && cur_dev.unwrap() == true {
            available_devs.push(dev);
        }
    }
    
    return available_devs;
}

#[cfg(target_os = "windows")]
fn main() {
    println!("TODO windows")
}

#[cfg(target_os = "macos")]
fn main() {
    println!("TODO macos")
}