use std::io::net::ip::SocketAddr;
use std::io::net::addrinfo::get_host_addresses;
use std::io::net::tcp::TcpStream;

fn main() {
  let server_address = match get_host_addresses("irc.freenode.net") {
    Some(ips) => SocketAddr { ip: ips[0], port: 6667 },
    None => fail!("cannot get ip!")
  };

  let mut stream = match TcpStream::connect(server_address) {
    Some(x) => x,
    None => fail!("cannot connect to server!")
  };

  let mut buffer = [0u8, ..512];

  loop {
    match stream.read(buffer) {
      Some(_) => {
        match std::str::from_utf8_opt(buffer) {
          // TODO i'm seeing occasional race conditions with printing lines
          Some(line) => print(line),
          None => println("not utf8!")
        }
      },
      None => {
        println("server terminated the connection!");
        return;
      }
    }
  }
}
