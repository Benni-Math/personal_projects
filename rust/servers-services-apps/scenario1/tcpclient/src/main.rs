use std::io::{ Read, Write };
use std::net::TcpStream;
use std::str;

fn main() {
    // Here we only use a single TcpStream, which we send to the server
    // --> connect vs bind
    let mut stream = TcpStream::connect("localhost:3000").unwrap();

    // Write to our stream (which the tcpserver should receive)
    stream.write("Hello".as_bytes()).unwrap();

    // Read what is sent back
    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();
    println!(
        "Got response from server: {:?}",
        // Unsure if std::str is needed...
        str::from_utf8(&buffer).unwrap()
    );
}
