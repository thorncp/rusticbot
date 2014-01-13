use std::io::net::ip::SocketAddr;
use std::io::net::addrinfo::get_host_addresses;

fn main() {
  let server_address = match get_host_addresses("irc.freenode.net") {
    Some(ips) => SocketAddr { ip: ips[0], port: 6667 },
    None => fail!("cannot get ip!")
  };

  println(server_address.to_str());
}
