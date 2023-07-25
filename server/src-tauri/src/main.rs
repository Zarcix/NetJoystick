// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;

use tauri::{Manager, State};
use std::sync::{Arc, Mutex};

use crate::api::server::Server;

#[tauri::command]
async fn disconnect_client(app_handle: tauri::AppHandle, server: State<'_, Arc<Mutex<Server>>>, client: String) -> Result<(), ()> {
    debug!("Request to disconnect client: {}", client);

    let mut server_lock = server.lock().unwrap();
    server_lock.disconnect_client(client.clone());
    drop(server_lock);

    app_handle.emit_all("reload_clients", ()).unwrap();
    Ok(())
}

#[tauri::command]
async fn deny_client(app_handle: tauri::AppHandle, server: State<'_, Arc<Mutex<Server>>>, client: String) -> Result<(), ()> {
    debug!("Request to deny client: {}", client);

    let mut server_lock = server.lock().unwrap();
    server_lock.remove_client(client.clone());
    drop(server_lock);

    app_handle.emit_all("reload_clients", ()).unwrap();
    Ok(())
}

#[tauri::command]
async fn accept_client(app_handle: tauri::AppHandle, server: State<'_, Arc<Mutex<Server>>>, client: String) -> Result<(), ()> {
    debug!("Request to accept client: {}", client);

    let mut server_lock = server.lock().unwrap();
    server_lock.connect_client(client.clone());
    server_lock.remove_client(client.clone());
    drop(server_lock);

    app_handle.emit_all("reload_clients", ()).unwrap();
    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn reload_clients(app_handle: tauri::AppHandle) -> (Vec<String>, Vec<String>) {
    let state = app_handle.state::<Arc<Mutex<Server>>>();
    let server = state.lock().unwrap();

    // Pending Clients

    debug!("SERVER | {:?} | Pending Clients", server.get_clients());
    let pending_clients = server.get_clients().iter().map(|val| val.to_string()).collect();

    // Connected Clients
    debug!("SERVER | {:?} | Connected Clients", server.get_connected_clients());
    let connected_clients: Vec<String> = server.get_connected_clients().iter().map(|val| val.0.to_string()).collect();

    drop(server);
    return (connected_clients, pending_clients);

}

#[tauri::command]
fn init_server(app_handle: tauri::AppHandle) {
    let socket = std::net::UdpSocket::bind("0.0.0.0:1025").unwrap();
    let mut data = [0; 4];

    std::thread::spawn(move || {
        loop {
            let (_, src) = socket.recv_from(&mut data).unwrap();

            let state = app_handle.state::<Arc<Mutex<Server>>>();
            let mut server = state.lock().unwrap();

            let connected_cli = server.find_connected_client(src.to_string());
            let potential_cli = server.find_client(src.to_string());
            
            if potential_cli.is_none() && connected_cli.is_none() {
                // Add new users to potential clients
                server.add_client(src.to_string());
                app_handle.emit_all("reload_clients", ()).unwrap();

            } else if connected_cli.is_some() {
                let _ = connected_cli.unwrap().send(data);
            }

            data.fill(0);

            // Unlock mutex
            drop(server);
        }
        
    });
}

fn main() {
    env_logger::init();
    log::debug!("Starting logger");

    let server = std::sync::Arc::new(std::sync::Mutex::new(api::server::Server::new()));

    // For Tauri TODO FIGURE OUT AFTER I GET SERVER RUNNING
    tauri::Builder::default()
        .manage(server)
        .invoke_handler(tauri::generate_handler![init_server, reload_clients, accept_client, deny_client, disconnect_client])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
