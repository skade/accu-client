#[macro_use]
extern crate structopt;

use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, Shutdown};
use std::net::TcpStream;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "client")]
struct Opt {
    #[structopt(name = "MESSAGE")]
    input: Option<String>
}

fn main() {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let mut stream = TcpStream::connect(socket).unwrap();

    let opt = Opt::from_args();

    if let Some(message) = opt.input {
        write!(stream, "{}", message).unwrap();
    }
    stream.shutdown(Shutdown::Write).unwrap();

    let mut buffer = String::new();
    stream.read_to_string(&mut buffer).unwrap();
    println!("Result: {}", buffer);
}
