use clap::Parser;
use std::net::*;

/// A private Focused Peer-to-Peer cli Chat.
/// 
/// Allowing users to chat privatelly in a local chat server.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli{
    /// Defines the port on which the server is set to be hosted at
    #[arg(short = 'p', long = "port", default_value_t = 55555)]
    port: u16,
    /// Defines the ip address on which the server will be hosted at
    #[arg(short = 'i', long = "ip-adrress")]
    ipa: Option<String>,
    /// If set you are going to start hosting a chat
    #[arg(short = 'H', long = "host-set", default_value_t = false)]
    host: bool
}

enum _AppMode{
    TypingMessage,
    EnteringChat, // Consider differ Singular Chat from Group
    CreatingChat,
    CreatingGroup,
}

fn main() {
    let cli = Cli::parse();
    if cli.host{
        let bind_address;
        if let Some(bip) = cli.ipa{
            bind_address = format!("{}:{}", bip, cli.port);
        }else {
            bind_address = format!("0.0.0.0:{}", cli.port);
        }
        if let Ok(listener) = TcpListener::bind(&bind_address){
            println!("Connected! Waiting for connection");
            if let Ok((_stream, peer_addr)) = listener.accept(){
                println!("Connected to peer: {}", peer_addr);
            }
        }else {
            println!("Failed to bind to port: {}", cli.port);
        }
    }else {
        if let Some(tip) = cli.ipa{
            let target_address = format!("{}:{}", tip, cli.port);
            println!("Attempting to connect to {}...", target_address);

            if let Ok(mut _stream) = TcpStream::connect(&target_address){
                println!("Successfully connected to host!");
            }else {
                println!("Failed to connect to server at {}", target_address);
            }

        }else{
            eprintln!("Error: You must provide an ip with -i if not hosting!");
            std::process::exit(1);
        }
    }
    


}
