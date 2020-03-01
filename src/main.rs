fn main() {
    println!("Hello, world!");

    let game_meta = api::create_game();
    let _ = api::get_game_state(game_meta.game_id());
}
