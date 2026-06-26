use std::process::Command;
use tun_tap::Iface;

//TODO
struct Ipv4Packege {
    version: u8,
    ihl: u8,
    tos: u8,
    total_lenght: u16,
}

// TODO
impl Ipv4Packege {
    pub fn new() -> Self {
        unimplemented!();
    }
}

fn main() -> Result<(), std::io::Error> {
    let interface = Iface::new("Iface0", tun_tap::Mode::Tun)?;

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
    let mut buffer: [u8; 1504] = [0u8; 1504]; //this is Maximun Transmission Unit(MTU)

    loop {
        let p = interface.recv(&mut buffer);

        if p.is_ok() {
            let flags = u16::from_be_bytes([buffer[0], buffer[1]]);
            let protocol = u16::from_be_bytes([buffer[2], buffer[3]]);

            let mut data = [0u8; 1500];
            data.copy_from_slice(&buffer[4..1504]);

            todo!();
            let Ipv4Packege = Ipv4Packege::new(data);

            println!("flags: {:x}, protocol: {:x}", flags, protocol);
        }
    }
}
