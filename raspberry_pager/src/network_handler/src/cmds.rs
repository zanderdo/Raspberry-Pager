pub const MAX_PAYLOAD_LEN: usize = 255;

pub struct Packet {
    pub cmd_code: u8,
    pub payload: [u8; MAX_PAYLOAD_LEN]
}

#[repr(u8)]
pub enum Commands {
    Unknown = 0,
    TestCmd = 1,
    UpdateScreen = 2,
    BlipBuzzer = 3,
    ClearScreen = 4,
}

pub fn print_commands() {
    println!("(0) - Unknown");
    println!("(1) - TestCmd");
    println!("(2) - UpdateScreen");
    println!("(3) - BlipBuzzer");
    println!("(4) - ClearScreen");
}

pub fn create_test_command_packet() -> Packet {
    let mut test_payload: [u8; 255] = [0; 255];
    for i in 0..255 {
        test_payload[i] = i as u8;
    }
    create_packet(Commands::TestCmd, test_payload)
}

pub fn create_update_screen_packet(text: [u8; 255]) -> Packet {
    Packet {
        cmd_code: Commands::UpdateScreen as u8,
        payload: text
    }
}

pub fn print_command_value(cmd: Commands) {
    println!("Command value: {}", cmd as u8)
}

pub fn print_packet_contents(packet: Packet) {
    println!("Packet with command {} and contents:\n", 
              packet.cmd_code as u8 
            );
    let mut iteration: usize = 1;
    for x in packet.payload {
        let x_hex = format!("{x:#X}");
        print!("{x_hex} ");
        if iteration % 8 == 0 {
            print!("\n");
        }
        iteration += 1;
    }
    print!("\n");
}

pub fn create_packet(cmd: Commands, payload: [u8; 255]) -> Packet {
    Packet {
        cmd_code: cmd as u8,
        payload: payload
    }
}