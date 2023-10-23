use std::io::{Write, Read};
use std::net::{TcpListener, TcpStream};
use std::thread;
 
pub fn start(addr : &str) {
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
 
        thread::spawn(|| {
            handle_client(stream);
        });
    }
}
 
fn handle_client(mut stream: TcpStream) {
    println!("getit");
    let mut buffer = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
 
        let response = format!("Hello, {}", stream.peer_addr().unwrap().to_string());
        stream.write(response.as_bytes()).unwrap();
    }
}
 
pub fn client(addr: &str) {
    let mut stream = TcpStream::connect(addr).unwrap();
    println!("successful connect");
    let message = "Hello, server!";
    loop{
        println!("print message");
        std::io::stdin().read_line(&mut message.to_string()).unwrap();
        stream.write(message.as_bytes()).unwrap();
        
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        println!("{}", String::from_utf8_lossy(&buffer));
    }
}