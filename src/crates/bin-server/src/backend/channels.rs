use std::sync::{Arc, Mutex};
use game_api::api::GameApi2;
use crate::backend::backend_error::BackendGameError2;
use crate::backend::backend_impl;

pub fn new_backend_game_api() -> Arc<Mutex<dyn GameApi2<BackendGameError2> + Send>> {
    Arc::new(Mutex::new(backend_impl::StorageBackedGameApi::new()))
}
