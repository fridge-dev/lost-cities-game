/// Simple I/O
pub mod raw_cli {
    use std::{io, process};

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

/// CLI module which has logic to parse the CLI input.
pub mod smart_cli {
    use super::raw_cli::prompt_for_input;
    use std::borrow::Cow;
    use game_api::types::{CardTarget, Card, DrawPile, CardColor, DecoratedCard};

    pub type PromptResult<T> = Result<T, Cow<'static, str>>;

    pub fn prompt_for_player_id() -> PromptResult<String> {
        Ok(prompt_for_input("Please enter your name: "))
    }

    pub fn prompt_for_card(hand: &Vec<DecoratedCard>) -> PromptResult<&DecoratedCard> {
        let cli_hand_index = prompt_for_input("[1/3] Which card would you like to play? (press 0-7 to select card)");

        let hand_index: usize = cli_hand_index.parse().unwrap_or(100);
        if hand_index > hand.len() - 1 {
            return Err(Cow::from(format!("Please enter a number between 0 and {}.", hand.len() - 1)));
        }

        hand.get(hand_index)
            .ok_or_else(|| Cow::from(format!("Couldn't find card number '{:?}' in your hand. This is likely a bug.", hand_index)))
    }

    pub fn prompt_for_card_target(card: &Card) -> PromptResult<CardTarget> {
        let cli_card_target = prompt_for_input(&format!("[2/3] Where would you like to play '{}'? (press: [B]oard [N]eutral)", card));

        match cli_card_target.to_lowercase().as_str() {
            "b" => Ok(CardTarget::Player),
            "n" => Ok(CardTarget::Neutral),
            _ => Err(Cow::from("Please press either 'b' for your board or 'n' for neutral board.")),
        }
    }

    pub fn prompt_draw_pile() -> PromptResult<DrawPile> {
        let cli_draw_pile = prompt_for_input("[3/3] Where would you like to draw your new card from? (press: [M]ain [R]ed [G]reen [W]hite [B]lue [Y]ellow)");

        match cli_draw_pile.to_lowercase().as_str() {
            "m" => Ok(DrawPile::Main),
            "r" => Ok(DrawPile::Neutral(CardColor::Red)),
            "g" => Ok(DrawPile::Neutral(CardColor::Green)),
            "w" => Ok(DrawPile::Neutral(CardColor::White)),
            "b" => Ok(DrawPile::Neutral(CardColor::Blue)),
            "y" => Ok(DrawPile::Neutral(CardColor::Yellow)),
            _ => Err(Cow::from("Please press either 'm' for main draw pile or 'rgwby' for one of the colored discard piles.")),
        }
    }

    pub fn prompt_confirm_play(card: &Card, target: &CardTarget, draw_pile: &DrawPile) -> PromptResult<bool> {
        let cli_y_n = prompt_for_input(&format!("Confirm: Play '{}' on '{:?}', then draw from '{:?}'. [y/n]", card, target, draw_pile));
        println!();

        match cli_y_n.to_lowercase().as_str() {
            "y" => Ok(true),
            "n" => Ok(false),
            _ => Err(Cow::from("Please enter either 'y' to confirm your play or 'n' to reselect your play.")),
        }
    }
}
