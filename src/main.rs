use std::net::{SocketAddr, TcpStream, IpAddr, Ipv4Addr, Ipv6Addr};
use std::time::Duration;

struct Host {
    ip: IpAddr,
    port: u16,
}

impl Host {
    fn new(ip: String, port: u16) -> Host {
        Host {
            ip: ip.parse().unwrap(), 
            port: port
        }
    }
}

fn main() {
    //let ip4 = IpAddr::V4(8, 8, 8, 8);
    let ip4 = String::from("8.8.8.8");
    let ip6 = String::from("fe80::91aa:2ff:cfe2:ae5e");
    let timeout = Duration::new(1, 0); // sec, nanosec
    let host = Host::new(ip4, 80);
    let socket = SocketAddr::new(host.ip, host.port);
    if let Ok(_stream) = TcpStream::connect_timeout(&socket, timeout) {
        println!("Yes {}", socket)
    } else {
        println!("No {}", socket)
    }
    println!("Hello, world!");
}
fn _test() {

} 