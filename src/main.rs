use clap::Parser;

/// A private Focused Peer-to-Peer cli Chat.
/// 
/// Allowing users to chat privatelly in a local chat server.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli{
    /// Defines the port on which the server is set to be hosted at
    #[arg(short = 'H', long = "host-at-port", default_value_t = 55555)]
    port: u16,
}

fn main() {
    let cli = Cli::parse();

    println!("Port: {}", cli.port);
}
