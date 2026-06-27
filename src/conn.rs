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
