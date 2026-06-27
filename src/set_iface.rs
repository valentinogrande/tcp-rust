use std::process::Command;
use tun_tap::Iface;

pub fn set_interface() -> Result<Iface, std::io::Error> {
    let interface = Iface::new("tun0", tun_tap::Mode::Tun)?;

    //setting ip for interface Iface0

    let ip_address = "10.0.0.1"; // private ip
    let mask = "24";
    let ip = format!("{}/{}", ip_address, mask);

    let _child = Command::new("sudo")
        .arg("ip")
        .arg("addr")
        .arg("add")
        .arg(ip)
        .arg("dev")
        .arg(interface.name())
        .spawn()
        .expect("Failed to set ip")
        .wait();

    let _child = Command::new("sudo")
        .arg("ip")
        .arg("link")
        .arg("set")
        .arg("dev")
        .arg(interface.name())
        .arg("up")
        .spawn()
        .expect("Error setting interface up")
        .wait();

    Ok(interface)
}
