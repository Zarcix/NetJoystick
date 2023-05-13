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
        
        let chosen_device = 0;
        
        let mut device_list = crawl_devices();
        
        let device = device_list.remove(0);
        drop(device_list);
        
        let userCli = CliController::new(device.1);
        
        
        
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
        
        return Ok(())
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

fn crawl_devices() -> Vec<(std::path::PathBuf, evdev::Device)> {
    let allDevs = evdev::enumerate();
    
    let mut available_devs = Vec::new();
    
    for dev in allDevs {
        let curDev = dev.1.supported_absolute_axes().map(|ins| ins.iter().collect::<Vec<_>>().contains(&evdev::AbsoluteAxisType::ABS_X));
        if curDev.is_some() && curDev.unwrap() == true {
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