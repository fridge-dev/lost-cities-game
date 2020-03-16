use std::error::Error;
use game_api::types::{Play, Card, CardColor, CardTarget, DrawPile, GameState, GameStatus, GameResult};
use client::{cli, frontend};
use client::state_machine::Alternator;
use client::frontend::frontend_error::ClientGameError;
use game_api::api::GameApi2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut game_api = frontend::new_frontend_game_api();

    // Game setup
    let p1_id = cli::prompt_for_input("Please enter Player 1's name: ");
    let game_id = game_api.host_game(p1_id.clone()).await?;
    println!("Created Game ID = {}", game_id);

    let p2_id = cli::prompt_for_input("Please enter Player 2's name: ");
    game_api.join_game(game_id.clone(), p2_id.clone()).await?;

    println!();
    println!("Welcome. This game will feature '{}' vs '{}'.", p1_id, p2_id);
    println!();

    let mut player_turns = create_alternator(&mut game_api, game_id.clone(), &p1_id, &p2_id).await?;

    // Game loop
    loop {
        let current_player_id: &str = player_turns.next();
        let game_state = game_api.get_game_state(game_id.clone(), current_player_id.to_owned()).await?;
        println!("{}", game_state);

        // End game check
        if check_is_game_over_and_print_outcome(&game_state) {
            break;
        }

        // Player turn
        println!("-- {}'s turn --", current_player_id);
        let (card, card_target, draw_pile) = get_next_play_from_cli(&game_state);

        game_api.play_card(Play::new(
            game_id.clone(),
            current_player_id.to_owned(),
            card.clone(),
            card_target,
            draw_pile,
        )).await?;
    }

    println!("Thanks for playing! Goodbye.");
    Ok(())
}

async fn create_alternator<'a>(
    game_api: &mut Box<dyn GameApi2<ClientGameError>>,
    game_id: String,
    p1_id: &'a String,
    p2_id: &'a String
) -> Result<Alternator<'a, String>, ClientGameError> {
    let mut player_turns = Alternator::new(p1_id, p2_id);

    // Tick player turn order forward if player 2 is supposed to start.
    if !game_api.get_game_state(game_id.clone(), p1_id.to_owned()).await?.is_my_turn() {
        assert!(game_api.get_game_state(game_id, p2_id.to_owned()).await?.is_my_turn());
        player_turns.next();
    }

    Ok(player_turns)
}

fn check_is_game_over_and_print_outcome(game_state: &GameState) -> bool {
    match game_state.status() {
        GameStatus::InProgress => false,
        GameStatus::Complete(result) => {
            match result {
                GameResult::Win => print!("Congratulations, you win! "),
                GameResult::Lose => print!("Sorry, you lost. "),
                GameResult::Draw => print!("It was a draw! How rare! "),
            }

            println!("Score: {} to {}", game_state.game_board().my_score(), game_state.game_board().op_score());
            return true;
        }
    }
}

fn get_next_play_from_cli(game_state: &GameState) -> (&Card, CardTarget, DrawPile) {
    loop {
        println!();

        // Card
        let cli_hand_index = cli::prompt_for_input(&format!("[1/3] Which card would you like to play? (press 0-7 to select card)"));

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
        let card = decorated_card.card();

        // CardTarget
        let cli_card_target = cli::prompt_for_input(&format!("[2/3] Where would you like to play '{}'? (press b=board, n=neutral)", card));
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

        if card_target == CardTarget::Player && !*decorated_card.is_playable() {
            // This is also validated in backend, but to short-circuit well-behaving clients, we check here first.
            println!("You're not allowed to play card '{:?}'.", decorated_card.card());
            continue;
        }

        // DrawPile
        let cli_draw_pile = cli::prompt_for_input(&format!("[3/3] Where would you like to draw your new card from? (press m=main, n=neutral)"));
        let draw_pile = match cli_draw_pile.as_str() {
            "m" => DrawPile::Main,
            "n" => {
                let cli_neutral_draw_color = cli::prompt_for_input(&format!("...And which color would you like to draw from? (enter the first letter of the color)"));
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