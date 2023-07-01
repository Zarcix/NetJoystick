// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod lib;
use lib::linux::CliController;

use std::net::UdpSocket;
use std::sync::{Arc, Mutex, mpsc::*};

struct Client {
  dev_list: Mutex<Vec<(std::path::PathBuf, evdev::Device)>>,
  client_socket: Arc<UdpSocket>,
  kill_old_sender: Mutex<Option<SyncSender<()>>>
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

fn init_device(device: &Arc<Mutex<CliController>>) -> Result<(), String> {
  // Calibrate
  let mut guard = device.lock().unwrap();
  println!("{:?}", guard.get_device().device().input_id());
  if guard.calibrate().is_err() {
    return Err("Calibrate: No next_event() for provided device".into())
  }
  drop(guard);



  // If nothing broke until now, we're good to go
  Ok(())
}

#[tauri::command]
fn startController(selected_device: String, selected_server: String, cli_state: tauri::State<Client>) {
  let runtime = tokio::runtime::Runtime::new().unwrap();
  // Kill Old Threads if they exist
  {
    let previous_worker_exist = &cli_state.kill_old_sender.lock().unwrap();
    if previous_worker_exist.is_some() {
      if previous_worker_exist.as_ref().unwrap().send(()).is_err() {
        println!("--\nFor some reason, old sender died. Old thread self killed so we're fine, continuing\n--")
      }
    }
  }

  // Gets the device
  let mut dev_list = cli_state.dev_list.lock().unwrap();
  let device = {
    let dev = dev_list.remove(selected_device.parse::<usize>().unwrap()).1;
    runtime.block_on(async {Arc::new(Mutex::new(CliController::new(dev.into_event_stream().unwrap())))})
  };

  println!("Device: {}", device.lock().unwrap().get_device().device().name().unwrap());

  // Gets the server
  let ip;
  let port;
  
  let server_parts = selected_server.split(":").collect::<Vec<&str>>();
  ip = {
    if server_parts.get(0).unwrap_or(&"localhost") == &"" {
      "localhost"
    } else {
      server_parts.get(0).unwrap_or(&"localhost")
    }
  };
  port = server_parts.get(1).unwrap_or(&"69420");
  
  { // Connect to Server
    if cli_state.client_socket.connect(format!("{}:{}", ip, port)).is_err() {
      println!("Could not connect to provided server, quitting here");
      return;
    }
  }

  { // Init Device
    let dev_init_status = init_device(&device);
    if dev_init_status.is_err() {
      println!("{}, quitting here", dev_init_status.err().unwrap());
      return;
    }
  }

  let (send, recv) = sync_channel::<()>(1);
  let device_threaded = Arc::clone(&device);

  {
    std::thread::spawn(move || { // Server Comms Thread
      let kill_thread = std::sync::Arc::new(std::sync::Mutex::new(false));

      let kill_thread_spawner = Arc::clone(&kill_thread);
      std::thread::spawn(move || { // Device Worker Thread
        let mut es = device_threaded.lock().unwrap();
        
        // Lock device 
        es.get_device().device_mut().grab().unwrap();

        loop {
          if kill_thread_spawner.lock().unwrap().clone() {
            println!("Device Thread has died, no longer valid");
            es.get_device().device_mut().ungrab().unwrap();
            break;
          }
          runtime.block_on (async {
            let input_ev = es.get_device().next_event().await.unwrap();
            if input_ev.event_type() != evdev::EventType::SYNCHRONIZATION {
              println!("Device Thread: Input Events:\n{:?} || {:?} || {:?} || {:?}", input_ev.timestamp(), input_ev.event_type(), input_ev.code(), input_ev.value())
            }
            });
        }
      });

      let _ = recv.recv();

      // Once this is received, kill no matter the bool
      println!("Kill Signal Received. Killing thread");

      let mut kt_lock = kill_thread.lock().unwrap();
      *kt_lock = true;
      drop(kt_lock);
    });
  }

  { // Update Client Struct
    let mut lock = cli_state.kill_old_sender.lock().unwrap();
    *lock = Some(send);
    drop(lock);
  }
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
    .manage(Client {
      dev_list: Vec::new().into(), 
      client_socket: std::sync::Arc::new(UdpSocket::bind("0.0.0.0:1025").expect("could not bind to address")),
      kill_old_sender: Mutex::new(None)
    })
    .invoke_handler(tauri::generate_handler![getDevices, startController])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
