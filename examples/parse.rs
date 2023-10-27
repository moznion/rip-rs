use rip::parser;

fn main() {
    let result = parser::parse(
        vec![
            2, 2, 0, 0, 0, 2, 1, 2, 192, 0, 2, 100, 255, 255, 255, 0, 192, 0, 2, 111, 4, 3, 2, 1,
        ]
        .as_slice(),
    );

    let packet = match result.unwrap() {
        parser::ParsedPacket::V1(_) => {
            panic!("the packet version must not be 1 because the second byte is 2");
        }
        parser::ParsedPacket::V2(p) => p,
    };

    println!("{:?}", packet);
    // =>
    //   Packet {
    //     header: Header {
    //       command: Response,
    //       version: Version2
    //     },
    //     entries: [
    //       Entry {
    //         address_family_identifier: IP,
    //         route_tag: 258,
    //         ip_address: 192.0.2.100,
    //         subnet_mask: 255.255.255.0,
    //         next_hop: 192.0.2.111,
    //         metric: 67305985
    //       }
    //     ]
    //   }
}
