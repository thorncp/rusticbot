use std::io::net::ip::SocketAddr;
use std::io::net::addrinfo::get_host_addresses;

fn main() {
  let ips = get_host_addresses("irc.freenode.net");
  let mut ip: ~str;

  match ips {
    Some(vector) => ip = vector[0].to_str(),
    None => fail!("cannot get ips!")
  }

  let wat: Option<SocketAddr> = FromStr::from_str(format!("{:s}:6667", ip));

  match wat {
    Some(addr) => println(addr.to_str()),
    None => println("no addr :(")
  }
}
