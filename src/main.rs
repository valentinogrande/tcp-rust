use etherparse::Ethernet2Slice;
use mac_parser::MACAddress;
use std::process::Command;
use tun_tap::Iface;

fn main() -> Result<(), std::io::Error> {
    let interface = Iface::new("Iface0", tun_tap::Mode::Tap)?;

    //setting ip for interface Iface0

    let ip_address = "10.0.0.1"; // private ip
    let mask = "24";

    let ip = format!("{}/{}", ip_address, mask);
    let name = interface.name();

    let child = Command::new("sudo")
        .arg("ip")
        .arg("addr")
        .arg("add")
        .arg(ip)
        .arg("dev")
        .arg(name)
        .spawn()
        .expect("Failed to set ip")
        .wait();

    let child = Command::new("sudo")
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
    let mut buffer: [u8; 1526] = [0u8; 1526]; //this is Maximun Transmission Unit(MTU)

    loop {
        let p = interface.recv(&mut buffer);
        if let Ok(len) = p {
            let packet = Ethernet2Slice::from_slice_without_fcs(&buffer[..len]);
            let pac = packet.unwrap();
            let source = MACAddress::new(pac.source());
            let destination = MACAddress::new(pac.destination());
            println!("Source MAC: {}, Destination MAC: {}", source, destination);
        } else {
            println!("Error");
        }
    }
}
