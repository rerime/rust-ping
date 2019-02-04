use std::net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs};
use std::thread::sleep;
use std::time::{Duration, Instant};

struct Host {
    ip: IpAddr,
    port: u16,
}

impl Host {
    fn new(ip: String, port: u16) -> Host {
        Host {
            ip: ip.parse().unwrap(),
            port: port,
        }
    }
}

fn pinger(host: &str) {
    let ip4 = String::from(host);
    let _ip6 = String::from("2001:4860:4860::8888");
    let timeout = Duration::new(1, 0); // sec, nanosec
    let host = Host::new(ip4, 53);
    let socket = SocketAddr::new(host.ip, host.port);
    ///let socket = SocketAddr::from("ya.ru");
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
    use clap::{App, Arg, SubCommand, load_yaml};
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    //let host = matches.value_of("host").to_socket_addrs().unwrap();
    let mut host = "google.com:443".to_socket_addrs().unwrap();

    loop {
        //pinger(host);
        println!("hostname {:?}", host);
        sleep(Duration::new(1, 0));
    }
}
