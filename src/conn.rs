use std::fmt;

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct ConnectionId {
    pub source_address: [u8; 4],
    pub source_port: u16,
    pub destination_address: [u8; 4],
    pub destination_port: u16,
}

/*

   1         2          3          4
----------|----------|----------|----------
     SND.UNA    SND.NXT    SND.UNA
                          +SND.WND

1 - anteriores números de secuencia de los que ya se ha
    recibido acuse de recibo
2 - número de secuencia de datos sin acuse de recibo
    recibido
3 - número de secuencia permitido en la siguiente
    transmisión de datos
4 - futuros números de secuencia no permitidos todavía en
    la siguiente transmisión
*/

#[allow(unused)]
pub struct SendSequence {
    pub una: usize, // what we have sent but it have not been acknowledged
    pub nxt: usize, // what we are going to send, the next time we sent something
    pub wnd: usize, // how much we are allowed to send. (As a reciever, we can limit how much the sender maximun byte len)
    pub up: bool,
    pub wl1: usize,
    pub wl2: usize,
    pub iss: usize, // initial sequence number. It dont need to be 0.
}

/*
   1          2          3
----------|----------|----------
      RCV.NXT    RCV.NXT
                +RCV.WND

1 - anteriores números de secuencia de los que ya se han
    envíado el acuse de recibo
2 - números de secuencia permitidos para una nueva
    recepción
3 - futuros números de secuencia que todavía no están
        permitidos
*/

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
