/// TODO I'd like to separate this module into it's own crate, too.

use game_api::api::GameApi2;

mod frontend_impl;
pub mod frontend_error;

/// Does this mean every call from main to API will incur the cost of a v-lookup table query?
/// Consider removing this interface.
///
/// See:
/// * https://stackoverflow.com/a/27570064
/// * https://stackoverflow.com/questions/28621980/what-are-the-actual-runtime-performance-costs-of-dynamic-dispatch
pub fn new_frontend_game_api() -> Box<dyn GameApi2<frontend_error::ClientGameError>> {
    match frontend_impl::BackendClient::new_sync() {
        Ok(client) => Box::new(client),
        Err(e) => panic!(format!("Failed to connect to backend. AAAAAA! {:?}", e)),
    }
}
