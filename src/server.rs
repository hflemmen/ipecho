use std::net::UdpSocket;
use std::str;

extern crate pnet;
use pnet::datalink;

fn main() -> std::io::Result<()> {
    {
        while true {
            
            for iface in datalink::interfaces() {
                println!("{:?}", iface.ips);
            }

            let mut socket = UdpSocket::bind("0.0.0.0:34254")?;
            socket.set_broadcast(true);

            // Receives a single datagram message on the socket. If `buf` is too small to hold
            // the message, it will be cut off.
            let mut buf = [0; 10];
            let (amt, src) = socket.recv_from(&mut buf)?;

            std::println!("We received something. Local addr:{}", socket.local_addr().unwrap());

            let s = match str::from_utf8(&buf) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };


            let b = "sending".as_bytes();
            std::println!("We received {} from {}", s, src);


            // Redeclare `buf` as slice of the received data and send reverse data back to origin.
            let buf = &mut buf[..amt];
            buf.reverse();
            println!("Sent result: {}",socket.send_to(b, &src)?);
            
        }
    } // the socket is closed here
    Ok(())
}