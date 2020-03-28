use std::net::SocketAddr;
use tonic::transport::Server;
use bin_server::server_impl::LostCitiesBackendServer;
use bin_server::wire_api::proto_lost_cities::proto_lost_cities_server::ProtoLostCitiesServer;
use std::{env, process};

const DEFAULT_PORT: u16 = 8051;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_, port) = get_cli_args();

    let server_impl = LostCitiesBackendServer::new();

    let addr: SocketAddr = format!("[::]:{}", port).parse()
        .expect("This should never happen. It's a valid IP address, dammit.");
    println!("Going to listen on '{:?}'", addr);

    Server::builder()
        .add_service(ProtoLostCitiesServer::new(server_impl))
        .serve(addr)
        .await?;

    Ok(())
}

fn get_cli_args() -> (String, u16) {
    let mut cli_args = env::args();

    // Arg 0
    let program_name = cli_args.next().unwrap_or_else(|| {
        eprintln!("Program name is somehow missing? You should never see this.");
        process::exit(1);
    });

    // Arg 1
    let port = cli_args.next()
        .map(|port_str| port_str.parse().unwrap_or_else(|_| {
            print_usage_exit(&program_name);
        }))
        .unwrap_or_else(|| {
            println!("Using default port '{}'", DEFAULT_PORT);
            DEFAULT_PORT
        });

    (program_name, port)
}

fn print_usage_exit(program_name: &str) -> ! {
    eprintln!();
    eprintln!("Usage:  \t{} <server port>", program_name);
    eprintln!("Example:\t{} 3000", program_name);
    eprintln!();
    process::exit(1);
}
