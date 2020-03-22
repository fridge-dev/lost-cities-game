pub mod backend_error;
pub mod task;

mod game_engine;

#[cfg(test)]
mod tests {
    use crate::task::backend_task_client;

    #[tokio::test]
    async fn hello() -> Result<(), ()> {
        let mut client = backend_task_client::spawn_backend_task();

        let result = client.host_game("mememe".to_owned()).await;

        match result {
            Ok(game_id) => println!("Received game ID: {}", game_id),
            Err(e) => println!("Err: Debug={:?} Display={}", e, e),
        }

        Ok(())
    }
}
