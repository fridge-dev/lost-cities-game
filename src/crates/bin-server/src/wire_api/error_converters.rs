use backend_engine::backend_error::BackendGameError;
use tonic::{Status, Code};

pub trait IntoTonicStatus {
    fn into_status(self) -> Status;
}

impl IntoTonicStatus for BackendGameError {
    fn into_status(self) -> Status {
        match self {
            BackendGameError::NotFound(resource) => {
                println!("INFO: Resource {} not found.", resource);
                Status::new(
                    Code::NotFound,
                    format!("Resource {} not found.", resource)
                )
            },
            BackendGameError::GameAlreadyMatched(p2_id) => {
                println!("INFO: User attempted to join game, but it's already populated by {}.", p2_id);
                Status::new(
                    Code::AlreadyExists,
                    format!("The game you attempted to join is full. {} already joined the game.", p2_id)
                )
            },
            BackendGameError::InvalidPlay(reason) => {
                println!("INFO: User can't play card for reason {}", reason);
                Status::new(
                    Code::InvalidArgument,
                    format!("Can't play card. {}", reason)
                )
            },
            BackendGameError::Internal(cause) => {
                println!("ERROR: Internal failure caused by '{:?}'", cause);
                Status::new(Code::Internal, "Internal server failure")
            },
        }
    }
}
