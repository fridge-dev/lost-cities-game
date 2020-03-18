use game_api::api::GameApi2;
use crate::client_game_api::error::ClientGameError;
use crate::client_game_api::game_client::GameClient;

/// Does this mean every call from main to API will incur the cost of a v-lookup table query?
/// Consider removing this interface.
///
/// See:
/// * https://stackoverflow.com/a/27570064
/// * https://stackoverflow.com/questions/28621980/what-are-the-actual-runtime-performance-costs-of-dynamic-dispatch
pub fn new_frontend_game_api() -> Box<dyn GameApi2<ClientGameError>> {
    match GameClient::new_sync() {
        Ok(client) => Box::new(client),
        Err(e) => panic!(format!("Failed to connect to backend. AAAAAA! {:?}", e)),
    }
}
