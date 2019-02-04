#![allow(unused)]
use std::net::{SocketAddr, TcpStream, IpAddr};
use std::time::{SystemTime, Duration, Instant};
use std::thread::sleep;

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

fn pinger() {
    let ip4 = String::from("8.8.8.8");
    let _ip6 = String::from("2001:4860:4860::8888");
    let timeout = Duration::new(1, 0); // sec, nanosec
    let host = Host::new(ip4, 53);
    let socket = SocketAddr::new(host.ip, host.port);
    let now = Instant::now();
    println!("{}", now.elapsed().as_nanos());
    if let Ok(_stream) = TcpStream::connect_timeout(&socket, timeout) {
        println!("{}", now.elapsed().as_nanos());
        println!("Yes {}", socket);
        
    } else {
        println!("No {}", socket);
    }
}

fn main() {
    //let ip4 = IpAddr::V4(8, 8, 8, 8);
    loop {
        pinger();
        sleep(Duration::new(1, 0));
    }

}