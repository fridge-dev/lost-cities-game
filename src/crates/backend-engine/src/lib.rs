pub mod backend_error;
pub mod game_api;

mod cache_slots;
mod game_engine;
mod task;

/// Entry point of the lib
pub fn start_backend() -> Box<dyn game_api::GameApi2Immut + Send + Sync> {
    Box::new(cache_slots::slotted_backend::spawn_slotted_backend())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_api::GameApiResult;

    #[tokio::test]
    async fn hello() -> GameApiResult<()> {
        let client = start_backend();
        let game_id = "game";

        client.host_game(game_id.to_owned(), "mememe".to_owned()).await?;
        client.join_game(game_id.to_owned(), "youyou".to_owned()).await?;

        Ok(())
    }
}
