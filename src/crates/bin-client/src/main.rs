use std::error::Error;
use game_api::types::{Play, GameState, GameStatus, GameResult};
use game_api::api::GameApi2;
use client_engine::client_game_api::provider;
use client_engine::client_game_api::error::ClientGameError;
use bin_client::cli::smart_cli;
use bin_client::state_machine::Alternator;
use bin_client::move_selector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut game_api = provider::new_frontend_game_api().await;

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
        let (card, card_target, draw_pile) = move_selector::get_next_play_from_cli(&game_state);

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

    let game_state = game_api.get_game_state(game_id.clone(), p1_id.to_owned()).await?;
    if let GameStatus::InProgress(my_turn) = game_state.status() {
        if !my_turn {
            // Tick player turn order forward if player 2 is supposed to start.
            player_turns.next();
        }
    } else {
        // I have a ways to go to make the client resilient. Accepting a fragile "panic" here.
        panic!("Game was matched, but is somehow not in-progress!");
    }

    Ok(player_turns)
}

fn check_is_game_over_and_print_outcome(game_state: &GameState) -> bool {
    match game_state.status() {
        GameStatus::InProgress(_) => false,
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
