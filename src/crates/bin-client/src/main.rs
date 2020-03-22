use std::error::Error;
use client_engine::client_game_api::provider;
use bin_client::cli::smart_cli;
use bin_client::screens::game;
use bin_client::screens::main_menu::MainMenuAction;
use bin_client::screens::main_menu;

#[tokio::main]
async fn main() {

    let player_id = smart_cli::prompt_for_player_id().expect("This should never fail.");

    loop {
        let result = execute_single_game(player_id.clone()).await;
        if let Err(error) = result {
            println!("UNHANDLED ERROR: Debug='{:?}', Display='{}'", error, error);
            println!();
            println!("I haven't implemented robust error handling yet, so your game is probably lost. Sorry.");
        }
    }
}

/// Layer of indirection to handle errors (and so we can easily use `?` syntax).
///
/// This is called once-per-game instance.
async fn execute_single_game(player_id: String) -> Result<(), Box<dyn Error>> {

    let mut game_api = provider::new_frontend_game_api().await;
    let action = main_menu::handle_main_menu();

    let game_id = match action {
        MainMenuAction::HostGame => {
            // Create game
            let game_id = game_api.host_game(player_id.clone()).await?;
            println!("Created Game ID = '{}'", game_id);

            // Poll for guest joining game
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
            println!("Sorry, I have written this part yet.");
            return Ok(());
        },
    };

    game::execute_game_loop(game_api, game_id, player_id).await?;

    Ok(())
}