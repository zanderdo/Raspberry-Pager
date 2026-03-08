mod cmds;
use clap::{Parser};
use std::io;



#[repr(u8)]
enum CliOptions {
    DisplayOptions = 0,
    CreateAndDisplayPacket = 1,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    #[arg(short, long)]
    command: u8

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
    println!("(0) - DisplayOptions");
    println!("(1) - CreateAndDisplayPacket");
}

fn main() {

    println!("Network handler CLI, select option to continue:");
    display_options();

    let args = Args::parse();

    match args.command {
        val if val == CliOptions::DisplayOptions as u8 => display_options(),
        val if val == CliOptions::CreateAndDisplayPacket as u8 => create_and_display_packet(),
        _ => println!("Unrecognized CLI option {}!", args.command)
    }
}
