use std::io::net::ip::SocketAddr;
use std::io::net::addrinfo::get_host_addresses;
use std::io::net::tcp::TcpStream;
use std::os;

fn main() {
  let args = os::args();

  if args.len() < 5 {
    fail!("usage: ./irc <server> <port> <nick> <channel>");
  }

  // TODO figure out better args usage
  let server = args[1].clone();
  let port = match from_str::<u16>(args[2].clone()) {
    Some(x) => x,
    None => fail!("invalid port")
  };
  let nick = args[3].clone();
  let channel = args[4].clone();

  let server_address = match get_host_addresses(server) {
    Some(ips) => SocketAddr { ip: ips[0], port: port },
    None => fail!("cannot get ip!")
  };

  let mut stream = match TcpStream::connect(server_address) {
    Some(x) => x,
    None => fail!("cannot connect to server!")
  };

  let mut buffer = [0u8, ..512];

  stream.write_line(format!("USER {:s} {:s} {:s} {:s}", nick, nick, nick, nick));
  stream.write_line(format!("NICK {:s}", nick));
  stream.write_line(format!("JOIN {:s}", channel));
  stream.write_line(format!("PRIVMSG {:s} :hello from rust!", channel));

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
