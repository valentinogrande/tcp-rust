use conn::ConnectionId;
use etherparse::{Ipv4HeaderSlice, TcpHeaderSlice};
use std::collections::HashMap;

mod conn;
mod set_iface;

struct State {}

fn main() -> Result<(), std::io::Error> {
    let interface = set_iface::set_interface()?;

    let mut conns: HashMap<ConnectionId, State> = HashMap::new();

    // when interface calls recv copies the packege into this buffer.
    let mut buffer: [u8; 1504] = [0u8; 1504]; //this is Maximun Transmission Unit(MTU)

    loop {
        let p = interface.recv(&mut buffer);
        let flags = [buffer[0], buffer[1]]; // first 2 bytes
        let proto = [buffer[2], buffer[3]]; // 3 and 4th bytes

        let _flags = u16::from_be_bytes(flags);
        let proto = u16::from_be_bytes(proto);
        let mut _packet_len = 0usize;

        if let Ok(len) = p {
            _packet_len = len;
        }

        // 0x800 means IpV4 packet
        if proto != 0x800 {
            continue;
        }

        let packet = Ipv4HeaderSlice::from_slice(&buffer[4..]).unwrap();

        // 6 is tcp
        if packet.protocol().0 == 6 {
            let tcp_packet =
                TcpHeaderSlice::from_slice(&buffer[4 + packet.slice().len()..]).unwrap();

            let conn = ConnectionId::new(
                packet.source(),
                tcp_packet.source_port(),
                packet.destination(),
                tcp_packet.destination_port(),
            );

            println!("{conn}");
        }
    }
}
