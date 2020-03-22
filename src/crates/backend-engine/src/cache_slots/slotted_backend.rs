use crate::cache_slots::slots::Slots;
use crate::game_api::{GameApi2Immut, GameApiResult};
use crate::task::backend_task_client;
use crate::task::backend_task_client::BackendTaskClientAdapter;
use game_api::types::{GameMetadata, Play, GameState};
use std::cmp;

pub fn spawn_slotted_backend() -> SlottedGameApi2Immut {
    let num_tasks = get_num_backend_tasks_to_spawn();

    let mut task_clients = Vec::with_capacity(num_tasks);
    for _ in 0..num_tasks {
        task_clients.push(backend_task_client::spawn_backend_task());
    }

    SlottedGameApi2Immut {
        slots: Slots::new(task_clients)
    }
}

fn get_num_backend_tasks_to_spawn() -> usize {
    let num_cpus = num_cpus::get();
    let num_tasks = cmp::max(num_cpus, 3);

    println!("There are {} CPUs. Spawning {} backend tasks.", num_cpus, num_tasks);

    num_tasks
}

/// A GameId-consistent-hash-based cache over the backend DB. This will yield higher parallelism
/// (yes, parallelism, not just concurrency) than a single-tasked approach.
///
/// Is this a premature optimization? Definitely! But I'm in it for the engineering challenge.
pub struct SlottedGameApi2Immut {
    slots: Slots<BackendTaskClientAdapter>,
}

#[async_trait::async_trait]
impl GameApi2Immut for SlottedGameApi2Immut {
    async fn host_game(&self, game_id: String, p1_id: String) -> GameApiResult<()> {
        self.slots
            .get(&game_id)
            .host_game(game_id, p1_id)
            .await
    }

    async fn join_game(&self, game_id: String, p2_id: String) -> GameApiResult<()> {
        self.slots
            .get(&game_id)
            .join_game(game_id, p2_id)
            .await
    }

    async fn describe_game(&self, game_id: String) -> GameApiResult<GameMetadata> {
        self.slots
            .get(&game_id)
            .describe_game(game_id)
            .await
    }

    async fn get_game_state(&self, game_id: String, player_id: String) -> GameApiResult<GameState> {
        self.slots
            .get(&game_id)
            .get_game_state(game_id, player_id)
            .await
    }

    async fn play_card(&self, play: Play) -> GameApiResult<()> {
        self.slots
            .get(play.game_id())
            .play_card(play)
            .await
    }

    /// TODO: Non-GameId based APIs are currently unsupported (in this hash-based impl and in general)
    async fn query_unmatched_games(&self, _player_id: String) -> GameApiResult<Vec<GameMetadata>> { unimplemented!() }
    async fn query_in_progress_games(&self, _player_id: String) -> GameApiResult<Vec<GameMetadata>> { unimplemented!() }
    async fn query_completed_games(&self, _player_id: String) -> GameApiResult<Vec<GameMetadata>> { unimplemented!() }
    async fn query_all_unmatched_games(&self, _player_id: String) -> GameApiResult<Vec<GameMetadata>> { unimplemented!() }
}
