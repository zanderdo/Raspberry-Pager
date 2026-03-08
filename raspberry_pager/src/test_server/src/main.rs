use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io;
use network_handler::cmds::{Packet, Commands};
use addresses::addresses::{TEST_SERVER_ADDR};

fn create_tcp_listener() -> TcpListener {
    TcpListener::bind(TEST_SERVER_ADDR).unwrap()
}

fn wait_for_connection(listener: TcpListener) -> io::Result<(TcpStream, SocketAddr)> {
    println!("Waiting for connection!");
    listener.accept()
}

fn handle_stream(stream: TcpStream) {

}


fn main() {
    let listener: TcpListener = create_tcp_listener();
    match wait_for_connection(listener) {
        Ok((stream, addr)) => {
            println!("Successful connection from {}", addr.to_string());
            handle_stream(stream);
        },
        Err(e) => println!("Failed connection attempt from {e:?}"),
    }
}
