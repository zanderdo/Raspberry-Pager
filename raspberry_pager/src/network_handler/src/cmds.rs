

pub struct Packet {
    pub cmd_code: u8,
    pub payload: [u8; 255]
}

#[repr(u8)]
pub enum Commands {
    Unknown = 0,
    TestCmd = 1
}