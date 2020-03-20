use game_api::types::{GameState, Card, CardTarget, DrawPile};
use crate::cli::smart_cli;

pub fn get_next_play_from_cli(game_state: &GameState) -> (&Card, CardTarget, DrawPile) {
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
