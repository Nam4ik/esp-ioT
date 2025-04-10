#![no_std]
use esp_idf_sys::lwip::*;
use core::fmt;

fn main() - Option<>{
let server_socket = socket(AF_INET, SOCK_STREAM, 0);
bind(server_socket, &sockaddr_in { sin_port: htons(8080), ..Default::default() });

loop {
    listen(server_socket, 5);
    let (client_socket, _) = accept(server_socket);
    let mut buffer = [0u8; 1024];
    let len = recv(client_socket, &mut buffer, 0);
}
}