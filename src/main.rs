use std::error::Error;

use clap::Parser;

use traceroute_rust::{TracerouteError, TracerouteTerminal};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(name = "traceroute")]
#[command(bin_name = "traceroute")]
pub struct TracerouteOptions {
    #[clap(required = true, index = 1)]
    host: String,

    #[clap(long, default_value_t = 30)]
    hops: u16,

    #[clap(short, long, default_value_t = 3)]
    queries_per_hop: u16,

    #[clap(short = 'p', long, default_value_t = 33434u16)]
    initial_destination_port: u16,

    #[clap(short, long, default_value_t = 3)]
    wait: u8
}

fn main() -> Result<(), Box<dyn Error>> {
    let traceroute_options = TracerouteOptions::parse();
    
    let mut traceroute_terminal = match TracerouteTerminal::new(traceroute_options) {
        Ok(traceroute) => traceroute,
        Err(traceroute_error) => match traceroute_error {
            TracerouteError::HostnameNotResolved(hostname) => {
                panic!("{hostname}: Temporary failure in name resolution");
            }
        }
    };
    
    traceroute_terminal.start();
    
    Ok(())
}