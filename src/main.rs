use std::{process::Command, thread::spawn};
use tun_tap::Iface;

fn main() -> Result<(), std::io::Error> {
    let interface = Iface::new("Iface0", tun_tap::Mode::Tap)?;

    //setting ip for interface Iface0

    let ip_address = "10.0.0.1";
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
    let mut buffer: [u8; 1504] = [0u8; 1504]; //this is Maximun Transmission Unit(MTU)

    loop {
        let p = interface.recv(&mut buffer);
        eprintln!("Packege: {:?}", p);
    }
}
