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

    // Arbitrarily load p1 view, then reload p2.
    let game_state = game_api.get_game_state(&game_id, &p1_id)?;
    let mut player_turns: Alternator<String>;
    if !game_state.is_my_turn() {
        let game_state = game_api.get_game_state(&game_id, &p2_id)?;
        assert!(game_state.is_my_turn());
        player_turns = Alternator::new(&p2_id, &p1_id);
    } else {
        player_turns = Alternator::new(&p1_id, &p2_id);
    }
    println!("Frontend Game state: {:?}", game_state);

    // Game loop
    loop {
        let current_player_id: &str = player_turns.next();

        let card = Cli::prompt_for_input(&format!("'{}', play a card: ", current_player_id));
        if &card == "q" {
            println!("Quitting, GG.");
            break;
        }

        game_api.play_card(Play::new(
            &game_id,
            &current_player_id,
            &Card::new(CardColor::White, CardValue::Seven),
            &CardTarget::Player
        ))?;
    }

    Ok(())
}

// TODO move this to lib, this doesn't belong in main
struct Alternator<'a, T> {
    pair: [&'a T; 2],
    next_index: usize,
}

impl<'a, T> Alternator<'a, T> {
    pub fn new(first: &'a T, second: &'a T) -> Self {
        Alternator {
            pair: [first, second],
            // first call to "next" will return index 0
            next_index: 1,
        }
    }

    pub fn next(&mut self) -> &T {
        self.next_index ^= 1;
        self.pair[self.next_index]
    }
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
