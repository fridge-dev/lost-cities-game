#![allow(unused_variables)]
#![allow(unused_mut)]
use game_api::types::{GameState, Card, CardTarget, DrawPile, GameStatus, GameResult, Play};
use crate::cli::smart_cli;
use client_engine::client_game_api::error::ClientGameError;
use game_api::api::GameApi2;
use std::error::Error;
use std::time::Duration;
use std::thread;

/// Return Ok when game is filled
pub async fn wait_for_game_to_fill(
    game_api: &mut Box<dyn GameApi2<ClientGameError>>,
    game_id: String
) -> Result<(), Box<dyn Error>> {
    loop {
        let game_metadata = game_api.describe_game(game_id.clone()).await?;
        if let Some((guest_player_id, status)) = game_metadata.matched_data() {
            println!("Player '{}' joined. Let's play!", guest_player_id);
            return Ok(());
        } else {
            // Sleep 5 seconds
            thread::sleep(Duration::new(5, 0));
            continue;
        }
    }
}

/// Return Ok(()) - doesn't necessarily mean success, just that we've handled
///                 the outcome appropriately.
///
/// Return Err(_) - an unhandled error
pub async fn execute_game_loop(
    mut game_api: &mut Box<dyn GameApi2<ClientGameError>>,
    game_id: String,
    my_player_id: String
) -> Result<(), Box<dyn Error>> {

    // Check status of game before starting
    let game_metadata = game_api.describe_game(game_id.clone()).await?;
    let op_player_id = match game_metadata.matched_data() {
        None => {
            println!("Can't execute game loop of unmatched game!");
            return Ok(());
        }
        Some((guest_player_id, status)) => {
            match status {
                GameStatus::Complete(_) => {
                    println!("Can't execute game loop of completed game!");
                    return Ok(());
                },
                GameStatus::InProgress(_) => {
                    let p1 = game_metadata.host_player_id().to_owned();
                    let p2 = guest_player_id.to_owned();
                    if my_player_id == p1 {
                        p2
                    } else if my_player_id == p2 {
                        p1
                    } else {
                        println!("You, '{}', are not in this game of '{}' vs '{}'.", my_player_id, p1, p2);
                        return Ok(())
                    }
                },
            }
        }
    };

    println!();
    println!("Welcome. This game will feature '{}' vs '{}'.", my_player_id, op_player_id);
    println!();

    // This is kind of lame for flow control and printing. Oh well. :P
    let mut first_loop = true;
    let game_state = game_api.get_game_state(game_id.clone(), my_player_id.clone()).await?;
    if will_way_on_first_loop(&game_state) {
        println!("{}", game_state);
        println!("-- {}'s turn --", op_player_id);
        println!();
        println!("Waiting...");
    }

    // Until end of game:
    // 1. Wait for op turn
    // 2. End game check
    // 3. My turn
    // 4. End game check
    loop {
        // 1. Wait for op turn
        if !first_loop {
            println!("-- {}'s turn --", op_player_id);
            println!();
            println!("Waiting...");
        }
        let game_state = wait_for_my_turn(
            &mut game_api,
            game_id.clone(),
            my_player_id.clone(),
        ).await?;

        println!();
        println!("{}", game_state);

        // 2. Check for end game
        if check_is_game_over_and_print_outcome(&game_state) {
            break;
        }

        // 3. My turn
        println!("-- {}'s turn --", my_player_id);
        let (card, card_target, draw_pile) = get_next_play_from_cli(&game_state);

        game_api.play_card(Play::new(
            game_id.clone(),
            my_player_id.clone(),
            card.clone(),
            card_target,
            draw_pile,
        )).await?;

        // 4. End game check
        let game_state = game_api.get_game_state(game_id.clone(), my_player_id.clone()).await?;
        println!();
        println!("{}", game_state);
        if check_is_game_over_and_print_outcome(&game_state) {
            break;
        }

        if first_loop {
            first_loop = false;
        }
    }

    println!("Thanks for playing! Goodbye.");
    Ok(())
}

async fn wait_for_my_turn(
    game_api: &mut Box<dyn GameApi2<ClientGameError>>,
    game_id: String,
    my_player_id: String,
) -> Result<GameState, Box<dyn Error>> {
    loop {
        let game_state = game_api.get_game_state(game_id.clone(), my_player_id.clone()).await?;

        if let GameStatus::InProgress(is_my_turn) = game_state.status() {
            if !is_my_turn {
                // Sleep 2 seconds. This makes game playable, but not a great experience.
                // TODO Implement server->client push.
                thread::sleep(Duration::new(2, 0));
                continue;
            }
        }

        return Ok(game_state);
    }
}

fn will_way_on_first_loop(game_state: &GameState) -> bool {
    // If it's not our turn, we will wait on the first loop
    if let GameStatus::InProgress(is_my_turn) = game_state.status() {
        !*is_my_turn
    } else {
        // This should never happen.
        panic!("BUG: Game just started, but isn't in progress.");
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

fn check_is_game_over_and_print_outcome(game_state: &GameState) -> bool {
    match game_state.status() {
        GameStatus::InProgress(_) => false,
        GameStatus::Complete(result) => {
            match result {
                GameResult::Win => print!("Congratulations, you win! "),
                GameResult::Lose => print!("Sorry, you lost. "),
                GameResult::Draw => print!("It was a draw! How rare! "),
            }

            println!(
                "Score: {} to {}",
                game_state.game_board().my_score(),
                game_state.game_board().op_score()
            );
            return true;
        }
    }
}
