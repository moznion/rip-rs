use rip::header::Header;
use rip::packet::Packet;
use rip::serializer::serialize_v2_packet;
use rip::{address_family, command, v2, version};
use std::net::Ipv4Addr;

fn main() {
    let packet = Packet::make_v2_packet(
        Header::new(command::Kind::Response, version::Version::Version2),
        vec![v2::Entry::new(
            address_family::Identifier::IP,
            258,
            Ipv4Addr::new(192, 0, 2, 100),
            Ipv4Addr::new(255, 255, 255, 0),
            Ipv4Addr::new(192, 0, 2, 111),
            67305985,
        )],
    )
    .unwrap();

    let serialized = serialize_v2_packet(packet).unwrap();
    println!("{:?}", serialized);
    // =>
    //   [
    //     2, 2, 0, 0,
    //     0, 2, 1, 2,
    //     192, 0, 2, 100,
    //     255, 255, 255, 0,
    //     192, 0, 2, 111,
    //     4, 3, 2, 1
    //   ]
}
