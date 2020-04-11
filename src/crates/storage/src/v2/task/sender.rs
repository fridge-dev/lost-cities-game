use crate::v2::db_api::GameDatabase;
use crate::v2::db_types::{DbGameSummary, DbError, DbGameData, DbErrorCause};
use crate::v2::task::events::{DbTaskEvent, WriteTargetTable};
use std::sync::mpsc::Sender;
use tokio::sync::{oneshot, oneshot::Receiver, oneshot::error::RecvError};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct DatabaseClient {
    sender: Arc<Mutex<Sender<DbTaskEvent>>>,
}

impl DatabaseClient {

    pub(crate) fn new(sender: Sender<DbTaskEvent>) -> Self {
        DatabaseClient {
            sender: Arc::new(Mutex::new(sender))
        }
    }

    async fn send_and_wait<T>(
        &self,
        event: DbTaskEvent,
        response_callback: Receiver<Result<T, DbError>>
    ) -> Result<T, DbError> {
        self.sender.lock().unwrap().send(event)
            .map_err(|_| DbError::Internal(DbErrorCause::Internal(
                "The DatabaseBackendTask event loop has stopped. This should never happen."
            )))?;

        // Type is annotated to remind future-me how this nested error handling works
        let receive_result: Result<Result<T, DbError>, RecvError> = response_callback.await;

        receive_result
            .map_err(|_| DbError::Internal(DbErrorCause::Internal(
                "The DatabaseBackendTask event loop dropped our callback sender without sending a \
                message. This should never happen."
            )))?
    }
}

#[async_trait::async_trait]
impl GameDatabase for DatabaseClient {

    async fn create_game_summary(&self, game_summary: DbGameSummary) -> Result<(), DbError> {
        let (tx, rx) = oneshot::channel::<Result<(), DbError>>();
        let event = DbTaskEvent::Create(
            WriteTargetTable::GameSummary(game_summary),
            tx
        );

        self.send_and_wait(event, rx).await
    }

    async fn create_game_data(&self, game_data: DbGameData) -> Result<(), DbError> {
        let (tx, rx) = oneshot::channel::<Result<(), DbError>>();
        let event = DbTaskEvent::Create(
            WriteTargetTable::GameData(game_data),
            tx
        );

        self.send_and_wait(event, rx).await
    }

    async fn update_game_summary(&self, game_summary: DbGameSummary) -> Result<(), DbError> {
        let (tx, rx) = oneshot::channel::<Result<(), DbError>>();
        let event = DbTaskEvent::Update(
            WriteTargetTable::GameSummary(game_summary),
            tx
        );

        self.send_and_wait(event, rx).await
    }

    async fn update_game_data(&self, game_data: DbGameData) -> Result<(), DbError> {
        let (tx, rx) = oneshot::channel::<Result<(), DbError>>();
        let event = DbTaskEvent::Update(
            WriteTargetTable::GameData(game_data),
            tx
        );

        self.send_and_wait(event, rx).await
    }

    async fn load_game_summary(&self, game_id: String) -> Result<DbGameSummary, DbError> {
        let (tx, rx) = oneshot::channel::<Result<DbGameSummary, DbError>>();
        let event = DbTaskEvent::GetGameSummary(game_id, tx);

        self.send_and_wait(event, rx).await
    }

    async fn load_game_data(&self, game_id: String) -> Result<DbGameData, DbError> {
        let (tx, rx) = oneshot::channel::<Result<DbGameData, DbError>>();
        let event = DbTaskEvent::GetGameData(game_id, tx);

        self.send_and_wait(event, rx).await
    }
}