use client_engine::client_game_api::provider;
use bin_client::cli::smart_cli;
use bin_client::screens::main_menu;
use std::{env, process};

const DEFAULT_HOSTNAME: &str = "localhost";

#[tokio::main]
async fn main() {
    let (program_name, hostname) = get_cli_args();

    // Connect to server and run game
    let mut game_api = provider::new_frontend_game_api(hostname)
        .await
        .unwrap_or_else(|e| {
            eprintln!("ERROR: {:?}", e);
            eprintln!();
            eprintln!("Failed to connect to the server. Are you sure you entered the right hostname? Is the server up?");
            print_usage_exit(&program_name);
        });

    // Run game
    let player_id = smart_cli::prompt_for_player_id().expect("This should never fail.");
    loop {
        let result = main_menu::handle_menu(&mut game_api, player_id.clone()).await;
        if let Err(error) = result {
            println!("UNHANDLED ERROR: Debug='{:?}', Display='{}'", error, error);
            println!();
            println!("I haven't implemented robust error handling yet, so your game is probably lost. Sorry.");
        }
    }
}

fn get_cli_args() -> (String, String) {
    let mut cli_args = env::args();

    // Arg 0
    let program_name = cli_args.next().unwrap_or_else(|| {
        eprintln!("Program name is somehow missing? You should never see this.");
        process::exit(1);
    });

    // Arg 1
    let hostname = cli_args.next()
        .unwrap_or_else(|| {
            println!("Using default hostname '{}'", DEFAULT_HOSTNAME);
            DEFAULT_HOSTNAME.to_owned()
        });

    (program_name, hostname)
}

fn print_usage_exit(program_name: &str) -> ! {
    eprintln!();
    eprintln!("Usage:  \t{} <server hostname>", program_name);
    eprintln!("Example:\t{} example-hostname.com", program_name);
    eprintln!();
    process::exit(1);
}
