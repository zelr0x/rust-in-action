use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use clap::{App, Arg};
use rand;
use trust_dns_client::op::{Message, MessageType, OpCode, Query};
use trust_dns_client::rr::domain::Name;
use trust_dns_client::rr::record_type::RecordType;
use trust_dns_client::serialize::binary::*;

fn main() {
    let app = App::new("resolve")
        .about("A simple to use DNS resolver")
        .arg(
            Arg::with_name("dns-server")
                .short("s")
                .default_value("1.1.1.1"),
        )
        .arg(Arg::with_name("domain-name").required(true))
        .get_matches();
    let domain_name = app.value_of("domain-name").unwrap();
    let domain_name = Name::from_ascii(&domain_name).unwrap();
    let dns_server = app.value_of("dns-server").unwrap();
    let dns_server: SocketAddr = format!("{}:53", dns_server)
        .parse()
        .expect("invalid address");

    let mut req_b: Vec<u8> = Vec::with_capacity(512);
    let mut resp_b: Vec<u8> = vec![0; 512];
    let mut msg = Message::new();
    msg.set_id(rand::random::<u16>())
        .set_message_type(MessageType::Query) // Same repr as answer in bytes; used only by code.
        .add_query(Query::query(domain_name, RecordType::A))
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true);
    let mut encoder = BinEncoder::new(&mut req_b);
    msg.emit(&mut encoder).unwrap();

    let localhost = UdpSocket::bind("0.0.0.0:0") // Listen all addresses on a random port.
        .expect("cannot bind to local socket");
    let timeout = Duration::from_secs(3);
    localhost.set_read_timeout(Some(timeout)).unwrap();
    localhost.set_nonblocking(false).unwrap();
    let _amt = localhost
        .send_to(&req_b, dns_server)
        .expect("socket misconfigured");
    let (_amt, _remote) = localhost.recv_from(&mut resp_b).expect("timeout_reached");
    let dns_message = Message::from_vec(&resp_b).expect("unable to parse response");

    for ans in dns_message.answers() {
        if ans.rr_type() == RecordType::A {
            // TODO: improve error message
            let resource = ans.data().expect("No resource data in received record");
            let ip = resource.to_ip_addr().expect("invalid IP address received");
            println!("{}", ip.to_string());
        }
    }
}
