use std::error::Error;
use std::{io, process};
use types::{Play, Card, CardColor, CardValue, CardTarget};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let mut game_api = api::new_game_api();

    // Game setup
    let p1_id = Cli::prompt_for_input("Please enter Player 1's name: ");
    let game_id = game_api.host_game(&p1_id)?;
    println!("Created Game ID = {}", game_id);

    let p2_id = Cli::prompt_for_input("Please enter Player 2's name: ");
    game_api.join_game(&game_id, &p2_id)?;

    println!();
    println!("Welcome. This game will feature '{}' vs '{}'.", p1_id, p2_id);
    println!();

    let game_state = game_api.get_game_state(&game_id)?;

    // Game loop
    let players: [&str; 2] = [&p1_id, &p2_id];
    let mut next_player = 0;
    loop {
        println!("Game state: {:?}", game_state);
        let card = Cli::prompt_for_input("Play a card: ");
        if &card == "q" {
            println!("Quitting, GG.");
            break;
        }

        let current_player_id: &str = players[next_player];

        game_api.play_card(Play::new(
            &game_id,
            &current_player_id,
            &Card::new(CardColor::White, CardValue::Seven),
            &CardTarget::Player
        ))?;

        next_player ^= 1;
    }

    Ok(())
}

struct Cli;

impl Cli {

    pub fn prompt_for_input(prompt: &str) -> String {
        // print!() doesn't work, newline is needed, and it looks ugly.
        // Will make it pretty, later.
        println!("{}", prompt);
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input) {
            Ok(_num_bytes_read) => {},
            Err(e) => {
                eprintln!("Failed to read CLI input from user: {}", e);
                eprintln!("Literally crashing the entire program. Goodbye.");
                process::exit(1);
            },
        }
        user_input.trim().to_string()
    }

}
