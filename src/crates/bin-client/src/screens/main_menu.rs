use crate::cli::smart_cli;
use crate::cli::raw_cli;
use std::borrow::Cow;
use crate::screens::game;
use std::error::Error;
use client_engine::client_game_api::error::ClientGameError;
use game_api::api::GameApi2;

/// Layer of indirection to handle errors (and so we can easily use `?` syntax).
///
/// This is called once-per-game instance. Or rather, continuously in a `loop` in main.
pub async fn handle_menu(
    mut game_api: &mut Box<dyn GameApi2<ClientGameError>>,
    player_id: String
) -> Result<(), Box<dyn Error>> {

    let game_id = match prompt_loop() {
        MainMenuAction::HostGame => {
            // Create game
            let game_id = create_game_id();
            game_api.host_game(game_id.clone(), player_id.clone()).await?;
            println!("Created Game ID = '{}'", game_id);

            // Poll for guest joining game
            println!();
            println!("Waiting for player to join...");
            game::wait_for_game_to_fill(&mut game_api, game_id.clone()).await?;

            game_id
        },
        MainMenuAction::JoinGame(game_id) => {
            // Get game status
            let game_metadata = game_api.describe_game(game_id.clone()).await?;
            if let Some((player2_id, _status)) = game_metadata.matched_data() {
                println!("Game is full: Host='{}', Guest='{}'", game_metadata.host_player_id(), player2_id);
                return Ok(());
            }
            println!("This game is hosted by '{}'. Joining game...", game_metadata.host_player_id());

            // Join game
            game_api.join_game(game_id.clone(), player_id.clone()).await?;

            game_id
        },
        MainMenuAction::ReadRules => {
            println!("I haven't added this to the game yet. For now, go read https://github.com/fridge-dev/lost-cities-game/blob/master/rules.md");
            return Ok(());
        },
    };

    game::execute_game_loop(&mut game_api, game_id, player_id).await?;

    Ok(())
}

fn create_game_id() -> String {
    // random hex string
    format!("{:x}", rand::random::<u128>())
}

enum MainMenuAction {
    HostGame,
    JoinGame(/* GameId */ String),
    ReadRules,
}

const MAIN_MENU_PROMPT: &str = "\n\
What would you like to do? (press one of the following keys)\n\
h => [h]ost new game\n\
j => [j]oin existing game\n\
r => [r]ead the rules\n\
";

fn prompt_loop() -> MainMenuAction {
    println!();
    println!("Welcome to the Lost Cities game!");

    loop {
        match prompt_for_main_menu_action() {
            Ok(action) => return action,
            Err(msg) => println!("{}", msg),
        }
    }
}

fn prompt_for_main_menu_action() -> smart_cli::PromptResult<MainMenuAction> {
    let cli_host_or_join = raw_cli::prompt_for_input(MAIN_MENU_PROMPT);
    match cli_host_or_join.to_lowercase().as_str() {
        "h" => Ok(MainMenuAction::HostGame),
        "j" => Ok(MainMenuAction::JoinGame(raw_cli::prompt_for_input("Please enter the Game ID you'd like to join: "))),
        "r" => Ok(MainMenuAction::ReadRules),
        _ => Err(Cow::from("Please press either 'h', 'j', or 'r'.")),
    }
}

#[cfg(test)]
mod tests {
    use crate::screens::main_menu::MAIN_MENU_PROMPT;

    #[test]
    fn multiline_print() {
        println!("{}", MAIN_MENU_PROMPT);
    }
}
