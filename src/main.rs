use conn::{Conn, ConnectionId};
use etherparse::{Ipv4Header, Ipv4HeaderSlice, TcpHeader, TcpHeaderSlice};
use std::collections::HashMap;

mod conn;
mod set_iface;

#[allow(dead_code)]
enum TcpState {
    Listen,
    Closed,
}

fn main() -> Result<(), std::io::Error> {
    let interface = set_iface::set_interface()?;

    let mut conns: HashMap<ConnectionId, Conn> = HashMap::new();

    let state = TcpState::Listen;

    #[allow(unused_mut)]
    let mut ports: Vec<u16> = Vec::from([80, 443]);

    // when interface calls recv copies the packege into this buffer.
    let mut buffer: [u8; 1504] = [0u8; 1504]; //this is Maximun Transmission Unit(MTU)

    #[allow(unused)]
    let payload = [0u8; 10];

    loop {
        if let TcpState::Closed = state {
            continue;
        }

        let p = interface.recv(&mut buffer);
        let flags = [buffer[0], buffer[1]]; // first 2 bytes
        let proto = [buffer[2], buffer[3]]; // 3 and 4th bytes

        let _flags = u16::from_be_bytes(flags);
        let proto = u16::from_be_bytes(proto);
        let mut packet_len = 0usize;

        if let Ok(len) = p {
            packet_len = len;
        }

        // 0x800 means IpV4 packet
        if proto != 0x800 {
            continue;
        }

        let packet = Ipv4HeaderSlice::from_slice(&buffer[4..packet_len]).unwrap();

        // protocol 6 is tcp
        if packet.protocol().0 == 6 {
            let tcp_packet =
                TcpHeaderSlice::from_slice(&buffer[4 + packet.slice().len()..packet_len]).unwrap();

            let conn = ConnectionId::new(
                packet.source(),
                tcp_packet.source_port(),
                packet.destination(),
                tcp_packet.destination_port(),
            );

            if !ports.contains(&conn.destination_port) {
                continue;
            }

            if tcp_packet.syn() {
                let mut syn_ack = TcpHeader::new(conn.destination_port, conn.source_port, 0, 0);

                syn_ack.syn = true; // we also want to comunicate with this conn
                syn_ack.ack = true; // we accept him to talk us

                let ipv4 = Ipv4Header::new(
                    syn_ack.header_len_u16(),
                    64,
                    etherparse::IpNumber::TCP,
                    conn.destination_address,
                    conn.source_address,
                );

                if let Err(e) = ipv4 {
                    println!("{}", e);
                    panic!()
                }

                let ipv4 = ipv4.unwrap();

                let connection = Conn::new();
                conns.insert(conn, connection);

                let s = {
                    let mut s = &mut buffer[..];

                    ipv4.write(&mut s)?;
                    syn_ack.write(&mut s)?;
                    s.len()
                };

                interface.send(&buffer[..s])?;
            } else if tcp_packet.ack() {
                if !conns.contains_key(&conn) {
                    continue;
                }
            } else {
                continue;
            }
        }
    }
}
