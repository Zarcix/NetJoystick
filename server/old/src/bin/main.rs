mod lib;

use std::net::UdpSocket;

fn main() {
    println!("Server");
        let mut data = [0; 5];
        
        let socket = UdpSocket::bind("0.0.0.0:999").unwrap();
        
        let mut server_object = lib::linux::Server::new();
        
        loop {
            let (_, src) = socket.recv_from(&mut data).unwrap();
            //println!("Server Received: {:?}\nSending to client", data);
            
            server_object.parse_data(src.to_string(), data);
        }
}
