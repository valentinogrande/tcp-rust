use std::fmt;

pub struct ConnectionId {
    pub source_address: [u8; 4],
    pub source_port: u16,
    pub destination_address: [u8; 4],
    pub destination_port: u16,
}

impl ConnectionId {
    pub fn new(
        source_address: [u8; 4],
        source_port: u16,
        destination_address: [u8; 4],
        destination_port: u16,
    ) -> Self {
        Self {
            source_address,
            source_port,
            destination_address,
            destination_port,
        }
    }
}

impl fmt::Display for ConnectionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let i = self.source_address;
        let s = format!("{}.{}.{}.{}", i[0], i[1], i[2], i[3]);
        let i = self.destination_address;
        let d = format!("{}.{}.{}.{}", i[0], i[1], i[2], i[3]);
        write!(
            f,
            "{s}:{} -> {d}:{}",
            self.source_port, self.destination_port
        )
    }
}
