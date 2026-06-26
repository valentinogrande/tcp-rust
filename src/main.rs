use tun_tap::Iface;

fn main() -> Result<(), std::io::Error> {
    let interface = Iface::new("Iface0", tun_tap::Mode::Tap)?;
    loop {}
}
