use std::net::{UdpSocket, Ipv4Addr};
use std::str;


fn main() {
    println!("Hello, world!");

    // let broadcast : Ipv4Addr = ("255.255.255.255:34254").parse().unwrap();
    let mut socket = UdpSocket::bind("0.0.0.0:34254").unwrap();
    socket.set_broadcast(true);
    let b = "Hei!".as_bytes();
    println!("Send result: {}",socket.send_to(b,"255.255.255.255:34254").unwrap());

    loop {
        let mut buf = [0; 100];
        let (amt, src) = socket.recv_from(&mut buf).unwrap();

        let s = match str::from_utf8(&buf) {
            Ok(v) => v,
            Err(e) => "This was not utf8.",
        };
        println!("We received: {}", s);
    }

}
