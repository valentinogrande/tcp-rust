use etherparse::{Ipv4HeaderSlice, TcpHeaderSlice};
use std::process::Command;
use tun_tap::Iface;

fn main() -> Result<(), std::io::Error> {
    let interface = Iface::new("tun0", tun_tap::Mode::Tun)?;

    //setting ip for interface Iface0

    let ip_address = "10.0.0.1"; // private ip
    let mask = "24";

    let ip = format!("{}/{}", ip_address, mask);
    let name = interface.name();

    let _child = Command::new("sudo")
        .arg("ip")
        .arg("addr")
        .arg("add")
        .arg(ip)
        .arg("dev")
        .arg(name)
        .spawn()
        .expect("Failed to set ip")
        .wait();

    let _child = Command::new("sudo")
        .arg("ip")
        .arg("link")
        .arg("set")
        .arg("dev")
        .arg(name)
        .arg("up")
        .spawn()
        .expect("Error setting interface up")
        .wait();

    // when interface calls recv copies the packege into this buffer.
    let mut buffer: [u8; 1504] = [0u8; 1504]; //this is Maximun Transmission Unit(MTU)

    loop {
        let p = interface.recv(&mut buffer);
        let flags = [buffer[0], buffer[1]]; // first 2 bytes
        let proto = [buffer[2], buffer[3]]; // 3 and 4th bytes

        let _flags = u16::from_be_bytes(flags);
        let proto = u16::from_be_bytes(proto);

        if let Ok(_len) = p {
            if proto == 0x800 {
                // 0x800 means IpV4 packet
                let packet = Ipv4HeaderSlice::from_slice(&buffer[4..]).unwrap();

                let proto = packet.protocol().0;

                let source_address = packet.source();
                let destination_address = packet.destination();
                let len = packet.slice().len();

                // 6 is tcp
                if proto != 6 {
                } else {
                    let tcp_packet = TcpHeaderSlice::from_slice(&buffer[4 + len..]).unwrap();
                    let source_port = tcp_packet.source_port();
                    let destination_port = tcp_packet.destination_port();

                    println!("{source_port} -> {destination_port}");
                }
            }
        } else {
            println!("Error");
        }
    }
}
