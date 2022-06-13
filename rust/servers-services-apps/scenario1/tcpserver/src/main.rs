use std::io::{ Read, Write };
use std::net::TcpListener;

fn main() {
    // Binding to the port
    let connection_listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000");

    // Iterating over incoming TcpStream objects
    for stream in connection_listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established");
        let mut buffer = [0; 1024];

        // Here we 'echo' back whatever is written in the TcpStream
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
