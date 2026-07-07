use std::fmt;

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct ConnectionId {
    pub source_address: [u8; 4],
    pub source_port: u16,
    pub destination_address: [u8; 4],
    pub destination_port: u16,
}

#[allow(unused)]
pub struct SendSequence {
    pub una: usize,
    pub nxt: usize,
    pub wnd: usize,
    pub up: bool,
    pub wl1: usize,
    pub wl2: usize,
    pub iss: usize,
}

#[allow(unused)]
pub struct ReceiveSequence {
    pub nxt: usize,
    pub wnd: usize,
    pub up: bool,
    pub irs: usize,
}

#[allow(unused)]
pub struct Conn {
    pub state: ConnState,
    pub send: SendSequence,
    pub receive: ReceiveSequence,
}

impl Conn {
    pub fn new() -> Self {
        Self {
            state: ConnState::Syn,
            send: SendSequence {
                una: (0),
                nxt: (1),
                wnd: (10),
                up: (false),
                wl1: (10),
                wl2: (10),
                iss: (0),
            },
            receive: ReceiveSequence {
                nxt: (0),
                wnd: (10),
                up: (false),
                irs: (0),
            },
        }
    }
}

#[allow(unused)]
pub enum ConnState {
    Syn,
    SynAck,
    Established,
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
