use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let game_api = api::new_game_api();

    let game_meta = game_api.create_game()?;
    let _ = game_api.get_game_state(game_meta.game_id())?;

    Ok(())
}
