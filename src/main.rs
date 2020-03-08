use std::error::Error;
use std::{io, process};
use types::{Play, Card, CardColor, CardTarget, DrawPile, GameState, GameError};
use api::GameApi;

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

    let mut player_turns = create_alternator(&game_api, &game_id, &p1_id, &p2_id)?;

    // Game loop
    loop {
        let current_player_id: &str = player_turns.next();
        let game_state = game_api.get_game_state(&game_id, &current_player_id)?;
        println!("Frontend Game state: {:?}", game_state);

        // End game check
        // TODO move this to lib, as it's a rule of the game, not the CLI's responsibility.
        if 0 == *game_state.game_board().draw_pile_cards_remaining() {
            break;
        }

        // Player turn
        println!("-- {}'s turn --", current_player_id);
        let (card, card_target, draw_pile) = get_next_play_from_cli(&game_state);

        game_api.play_card(Play::new(
            &game_id,
            &current_player_id,
            card,
            card_target,
            draw_pile,
        ))?;
    }

    println!("Game over. Goodbye.");
    Ok(())
}

fn create_alternator<'a>(
    game_api: &Box<dyn GameApi>,
    game_id: &String,
    p1_id: &'a String,
    p2_id: &'a String
) -> Result<Alternator<'a, String>, GameError> {
    let mut player_turns = Alternator::new(p1_id, p2_id);

    // Tick player turn order forward if player 2 is supposed to start.
    if !game_api.get_game_state(&game_id, &p1_id)?.is_my_turn() {
        assert!(game_api.get_game_state(&game_id, &p2_id)?.is_my_turn());
        player_turns.next();
    }

    Ok(player_turns)
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

fn get_next_play_from_cli(game_state: &GameState) -> (&Card, CardTarget, DrawPile) {
    loop {
        println!();

        // Card
        let cli_hand_index = Cli::prompt_for_input(&format!("[1/3] Which card would you like to play? (press 0-7 to select card)"));

        let hand_index: usize = cli_hand_index.parse().unwrap_or(100);
        if hand_index > 7 {
            println!("Please enter a number between 0 and 7.");
            continue;
        }

        let card_opt = game_state.my_hand().get(hand_index);
        if card_opt.is_none() {
            println!("Couldn't find card number '{:?}' in your hand. This is likely a bug.", hand_index);
            continue;
        }
        let decorated_card = card_opt.unwrap();
        // TODO this is a bug, fix it
        if !decorated_card.is_playable() {
            // This is also validated in backend, but to short-circuit well-behaving clients, we check here first.
            println!("You're not allowed to play card '{:?}'.", decorated_card.card());
            continue;
        }
        let card = decorated_card.card();

        // CardTarget
        let cli_card_target = Cli::prompt_for_input(&format!("[2/3] Where would you like to play {:?}? (press b=board, n=neutral)", card));
        let card_target = match cli_card_target.as_str() {
            "b" => CardTarget::Player,
            "n" => CardTarget::Neutral,
            _ => {
                // Note:
                // Ideally, each step of input should be repeated independently, allowing for a 'r' to reset.
                // But I'm not focusing on frontend beautification right now. :D
                println!("Please press either 'b' for your board or 'n' for neutral board.");
                continue;
            }
        };

        // DrawPile
        let cli_draw_pile = Cli::prompt_for_input(&format!("[3/3] Where would you like to draw your new card from? (press m=main, n=neutral)"));
        let draw_pile = match cli_draw_pile.as_str() {
            "m" => DrawPile::Main,
            "n" => {
                let cli_neutral_draw_color = Cli::prompt_for_input(&format!("...And which color would you like to draw from? (enter the first letter of the color)"));
                let draw_color = match cli_neutral_draw_color.as_str() {
                    "b" => CardColor::Blue,
                    "g" => CardColor::Green,
                    "r" => CardColor::Red,
                    "w" => CardColor::White,
                    "y" => CardColor::Yellow,
                    _ => {
                        println!("Please select draw color by entering one of the following starting letters: (b)lue, (g)reen, (r)ed, (w)hite, (y)ellow");
                        continue;
                    }
                };
                DrawPile::Neutral(draw_color)
            },
            _ => {
                println!("Please press either 'm' for main draw pile or 'n' for neutral draw pile.");
                continue;
            }
        };

        return (card, card_target, draw_pile);
    }
}