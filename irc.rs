use std::io::net::ip::SocketAddr;
use std::io::net::addrinfo::get_host_addresses;

fn main() {
  let ips = get_host_addresses("irc.freenode.net");

  let server_address = match ips {
    Some(vector) => SocketAddr { ip: vector[0], port: 6667 },
    None => fail!("cannot get ip!")
  };

  println(server_address.to_str());
}
