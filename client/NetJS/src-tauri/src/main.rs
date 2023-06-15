// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod lib;
use lib::linux::CliController;

use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

struct Client {
  dev_list: Mutex<Vec<(std::path::PathBuf, evdev::Device)>>,
}

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

fn run_server(device: Arc<Mutex<CliController>>, server: (&str, &str)) {
  /* PreReq Server Section */

  let socket = std::sync::Arc::new(UdpSocket::bind("0.0.0.0:420").expect("could not bind to address"));
  socket.connect(format!("{}:{}", server.0, server.1)).expect("could not connect to provided server");

  /* PreReq Device Section */

  // Calibrate
  let mut guard = device.lock().unwrap();
  guard.calibrate();
  drop(guard);

  /* Client Section */
  let (mut snd, mut rcv) = std::sync::mpsc::channel::<(Arc<Mutex<CliController>>, Arc<UdpSocket>)>();

  

}

#[tauri::command]
fn startController(selected_device: String, selected_server: String, cli_state: tauri::State<Client>) {
    // Gets the device
    let mut dev_list = cli_state.dev_list.lock().unwrap();
    let device = {
      let runtime = tokio::runtime::Runtime::new().unwrap();
      let dev = dev_list.remove(selected_device.parse::<usize>().unwrap()).1;
      runtime.block_on(async {Arc::new(Mutex::new(CliController::new(dev.into_event_stream().unwrap())))})
    };
    
    println!("Device: {}", device.lock().unwrap().get_device().device().name().unwrap());

    // Gets the server
    let mut ip = "";
    let mut port = "";
    
    let server_parts = selected_server.split(":").collect::<Vec<&str>>();
    ip = {
      if server_parts.get(0).unwrap_or(&"localhost") == &"" {
        "localhost"
      } else {
        server_parts.get(0).unwrap_or(&"localhost")
      }
    };
    port = server_parts.get(1).unwrap_or(&"69420");
    
    run_server(device, (ip, port));
}

#[tauri::command]
fn getDevices(cli_state: tauri::State<Client>) -> Vec<String> {
  println!("Get Devices Called!");
  
  let devices = search_for_devices();
  let dev_strings: Vec<String> = devices.iter().map(|x| x.1.name().unwrap().to_string()).collect();
  
  let mut client = cli_state.dev_list.lock().unwrap();
  *client = devices;
  dev_strings
}

fn main() {
  tauri::Builder::default()
    .manage(Client {dev_list: Vec::new().into()})
    .invoke_handler(tauri::generate_handler![getDevices, startController])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
