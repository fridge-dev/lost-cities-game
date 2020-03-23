use game_api::api::GameApi2;
use crate::client_game_api::error::ClientGameError;
use crate::client_game_api::game_client::GameClient;
use std::error::Error;

/// Does this mean every call from main to API will incur the cost of a v-lookup table query?
/// Consider removing this interface.
///
/// See:
/// * https://stackoverflow.com/a/27570064
/// * https://stackoverflow.com/questions/28621980/what-are-the-actual-runtime-performance-costs-of-dynamic-dispatch
pub async fn new_frontend_game_api(
    hostname: String
) -> Result<Box<dyn GameApi2<ClientGameError>>, Box<dyn Error>> {
    let client = GameClient::new(hostname).await?;
    Ok(Box::new(client))
}
