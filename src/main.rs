mod logic;

use std::str::FromStr;
use clap::Parser;
use simplelog::{SimpleLogger, Config};
use log::LevelFilter;
use socha_client_2024::client::{GameClient, DebugMode};

use logic::OwnLogic;

/// Software Challenge 2024 client.
#[derive(Parser, Debug)]
struct Args {
    /// The game server's host address.
    #[clap(long, default_value = "localhost")]
    host: String,
    /// The game server's port.
    #[clap(short, long, default_value_t = 13050)]
    port: u16,
    /// A game reservation.
    #[clap(short, long)]
    reservation: Option<String>,
    /// The level to log at.
    #[clap(short, long, default_value = "Info")]
    level: String,
    /// Reads incoming XML messages from the console for debugging.
    #[clap(short = 'd', long)]
    debug_reader: bool,
    /// Prints outgoing XML messages to the console for debugging.
    #[clap(short = 'D', long)]
    debug_writer: bool,
}

fn main() {
    // Parse command line arguments
    let args = Args::parse();
    
    // Set up logging
    SimpleLogger::init(LevelFilter::from_str(&args.level).expect("Invalid log level."), Config::default()).expect("Could not initialize logger.");
    
    // Setup the client and the delegate
    let debug_mode = DebugMode {
        debug_reader: args.debug_reader,
        debug_writer: args.debug_writer,
    };

    let client = GameClient::new(OwnLogic, debug_mode, args.reservation);
    let _result = client.connect(&args.host, args.port).expect("Error while running client.");
}
