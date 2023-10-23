mod net;
use std;

use crate::net::{start, client};
fn main() {
    println!("Server or client?");
    println!("1 or 2 : ");
    let mut ip: String = String::new();
    let mut num = String::new();
    for i in std::io::stdin().lines(){ num = i.unwrap(); break;}
    let rr = num.as_str();
    if rr == "1"{
        println!("ip addr to set server : ip:port");
        for i in std::io::stdin().lines(){ ip = i.unwrap(); break;}
        start(ip.as_str());
    }
    else if rr == "2" {
        println!("ip addr to connect : ip:port");
        for i in std::io::stdin().lines(){ ip = i.unwrap(); break;}
        client(ip.as_str());
    }
    else {
        return;
    }
    
    
    
}
