use std::error::Error;
use game_api::types::{Play, Card, CardTarget, DrawPile, GameState, GameStatus, GameResult};
use game_api::api::GameApi2;
use client_engine::client_game_api::provider;
use client_engine::client_game_api::error::ClientGameError;
use bin_client::cli::smart_cli;
use bin_client::state_machine::Alternator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut game_api = provider::new_frontend_game_api();

    let player_id = smart_cli::prompt_for_player_id().expect("This should never fail.");

    let game_id = game_api.host_game(player_id.clone()).await?;
    println!("Created Game ID = {}", game_id);

    let p2_id = smart_cli::prompt_for_player_id()?;
    game_api.join_game(game_id.clone(), p2_id.clone()).await?;

    println!();
    println!("Welcome. This game will feature '{}' vs '{}'.", player_id, p2_id);
    println!();

    let mut player_turns = create_alternator(&mut game_api, game_id.clone(), &player_id, &p2_id).await?;

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
        let decorated_card = match smart_cli::prompt_for_card(game_state.my_hand()) {
            Ok(v) => v,
            Err(msg) => {
                println!("{}", msg);
                continue;
            }
        };
        let card_to_play = decorated_card.card();

        // CardTarget
        let card_target = match smart_cli::prompt_for_card_target(card_to_play) {
            Ok(v) => v,
            Err(msg) => {
                println!("{}", msg);
                continue;
            }
        };
        // This is also validated in backend, but to short-circuit well-behaving clients, we check here first.
        if card_target == CardTarget::Player && !*decorated_card.is_playable() {
            // TODO better explanation of rules.
            println!("You're not allowed to play card '{:?}'.", decorated_card.card());
            continue;
        }

        // DrawPile
        let draw_pile = match smart_cli::prompt_draw_pile() {
            Ok(v) => v,
            Err(msg) => {
                println!("{}", msg);
                continue;
            }
        };
        // This is also validated in backend, but to short-circuit well-behaving clients, we check here first.
        if card_target == CardTarget::Neutral {
            if let DrawPile::Neutral(draw_color) = draw_pile {
                if draw_color == *card_to_play.card_color() {
                    println!("You're not allowed to re-draw a card from the neutral board on the same turn that you discard it.");
                    continue;
                }
            }
        }

        // Confirm
        loop {
            match smart_cli::prompt_confirm_play(card_to_play, &card_target, &draw_pile) {
                Ok(confirmed) => {
                    if confirmed {
                        return (card_to_play, card_target, draw_pile);
                    } else {
                        // break inner loop, will continue in outer loop
                        break;
                    }
                }
                Err(msg) => {
                    println!("{}", msg);
                    // continue inner loop
                    continue;
                }
            }
        }

        // Play was aborted during confirmation prompt.
        continue;
    }
}