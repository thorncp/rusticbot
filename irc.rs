use std::io::net::ip::SocketAddr;
use std::io::net::addrinfo::get_host_addresses;

fn main() {
  let ips = get_host_addresses("irc.freenode.net");

  let ip = match ips {
    Some(vector) => vector[0],
    None => fail!("cannot get ips!")
  };

  let server_address = SocketAddr { ip: ip, port: 6667 };

  println(server_address.to_str());
}
