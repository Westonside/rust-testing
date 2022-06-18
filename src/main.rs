use server::Server;

use http::{Request,Method};

mod http;
mod server;

fn main() {


    //server is a struct
    //a string literal size is known at compile time so it is a string slice which is immutable
    let string = String::from("127.0.0.1:8080");
    let server_port = &string[10..]; //10 to the last byte to get the port get everything after 10thy
    //cannot pass a string slice
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}



