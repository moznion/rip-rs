# rip-rs [![Check](https://github.com/moznion/rip-rs/actions/workflows/check.yaml/badge.svg)](https://github.com/moznion/rip-rs/actions/workflows/check.yaml) [![codecov](https://codecov.io/gh/moznion/rip-rs/graph/badge.svg?token=sKZKBUAxJp)](https://codecov.io/gh/moznion/rip-rs)

RIP v1/v2 protocol parser and serializer for Rust.

## Synopsis

### Parse a packet

```rust
use rip::parser;

fn main() {
    let result = parser::parse(
        vec![
            2, 2, 0, 0,
            0, 2, 1, 2,
            192, 0, 2, 100,
            255, 255, 255, 0,
            192, 0, 2, 111,
            4, 3, 2, 1,
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
```

### Serialize a packet

```rust
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
    ).unwrap();

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
```

see also [examples](./examples).

## TODO

- [ ] RIPv2 cryptographic authentication support
- [ ] RIPng support

## References

- [RFC 1058 - Routing Information Protocol](https://datatracker.ietf.org/doc/html/rfc1058)
- [RFC 2453 - RIP Version 2](https://datatracker.ietf.org/doc/html/rfc2453)
- [RFC 4822 - RIPv2 Cryptographic Authentication](https://datatracker.ietf.org/doc/html/rfc4822)
- https://www.iana.org/assignments/rip-types/rip-types.xhtml

## Author

moznion (<moznion@mail.moznion.net>)

