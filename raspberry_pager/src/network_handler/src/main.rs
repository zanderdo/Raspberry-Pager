pub mod cmds;
use clap::{Parser};
use std::io;
use addresses::addresses::{TEST_SERVER_ADDR};
use std::net::{TcpStream};



#[repr(u8)]
enum CliOptions {
    Quit = 0,
    DisplayOptions = 1,
    CreateAndDisplayPacket = 2,
    ConnectToTestServer = 3,
}


fn get_update_screen_string_from_user() -> [u8; cmds::MAX_PAYLOAD_LEN] {
    println!("Enter the string to update the screen with. Max # of characters: {}", cmds::MAX_PAYLOAD_LEN);
    let mut update_string = String::new();
    io::stdin().read_line(&mut update_string).expect("Unable to read line from command line");
    let update_bytes: &[u8] = update_string.trim().as_bytes();
    let mut payload: [u8; cmds::MAX_PAYLOAD_LEN] = [0; cmds::MAX_PAYLOAD_LEN];
    let dest_slice = &mut payload[0..update_bytes.len()];
    dest_slice.copy_from_slice(&update_bytes[0..update_bytes.len()]);
    payload
}

fn create_and_display_packet() {
    println!("Enter command of packet to create.");
    println!("Command options are:");
    cmds::print_commands();
    let mut cmd_string = String::new();
    io::stdin().read_line(&mut cmd_string);
    let cmd_num: u8 = cmd_string.trim().parse().expect("Input not a valid u8 number");
    match cmd_num {
        val if val == cmds::Commands::Unknown as u8 => {println!("Unknown command selected, doing nothing!");},
        val if val == cmds::Commands::TestCmd as u8 => cmds::print_packet_contents(cmds::create_test_command_packet()),
        val if val == cmds::Commands::UpdateScreen as u8 => {
            let payload = get_update_screen_string_from_user();
            cmds::print_packet_contents(cmds::create_update_screen_packet(payload));
        },
        _ => println!("Unrecognized or unhandled command type {}", cmd_num)
    }
}

fn display_options() {
    println!("(0) - Quit");
    println!("(1) - DisplayOptions");
    println!("(2) - CreateAndDisplayPacket");
    println!("(3) - ConnectToTestServer");
}

fn prompt_user_for_menu_choice() -> u8 {
    println!("Enter CliOption to execute:");
    display_options();
    let mut option_string = String::new();
    io::stdin().read_line(&mut option_string).expect("Could not read line from command line");
    option_string.trim().parse().expect("Input not a valid u8 number")
}

fn connect_to_test_server() -> io::Result<TcpStream> {
    TcpStream::connect(TEST_SERVER_ADDR)
}

fn main() {

    println!("Network handler CLI, select option to continue:");

    loop {
        let user_choice: u8 = prompt_user_for_menu_choice();
        match user_choice {
            val if val == CliOptions::Quit as u8 => break,
            val if val == CliOptions::DisplayOptions as u8 => display_options(),
            val if val == CliOptions::CreateAndDisplayPacket as u8 => create_and_display_packet(),
            val if val == CliOptions::ConnectToTestServer as u8 => {
                match connect_to_test_server() {
                    Ok((stream)) => println!("Successfully connected to test server"),
                    Err(e) => println!("Error connecting to test server: {e:?}"),
                }
            },
            _ => println!("Unrecognized CLI option {}!", user_choice)
        }
    }
}
