mod net;
use std;

use crate::net::{start, client};
fn main() {
    println!("Server or client?");
    println!("1 or 2 : ");
    let mut ip = String::new();
    let mut num = String::new();
    for i in std::io::stdin().lines(){ num = i.unwrap(); break;}
    let rr = num.as_str();
    if rr == "1"{
        println!("ip addr to set server : ip:port");
        std::io::stdin().read_line(&mut ip).unwrap();
        let v: Vec<&str> = ip.split(':').collect();
        println!("ip : {}\nport : {}", v[0], v[1]);
        println!("bind the server");
        start(ip.as_str());
    }
    else if rr == "2" {
        println!("ip addr to connect : ip:port");
        std::io::stdin().read_line(&mut ip).unwrap();
        let v: Vec<&str> = ip.split(':').collect();
        println!("ip : {}\nport : {}", v[0], v[1]);
        client(ip.as_str());
    }
    else {
        return;
    }
    
    
    
}
