use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::sync::Arc;
use tonic::{Status, Code};

#[derive(Debug)]
pub enum BackendGameError2 {
    Internal(Cause),
    NotFound(&'static str),
    GameAlreadyMatched,
    InvalidPlay(Reason),
}

impl Error for BackendGameError2 {}

impl Display for BackendGameError2 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            BackendGameError2::NotFound(entity) => f.write_str(&format!("{} not found!", entity)),
            BackendGameError2::Internal(cause) => f.write_str(&format!("Unexpected error: {:?}", cause)),
            BackendGameError2::GameAlreadyMatched => f.write_str("No room for u."),
            BackendGameError2::InvalidPlay(reason) => f.write_str(&format!("You cannot make that play: {:?}", reason)),
        }
    }
}

impl From<BackendGameError2> for Status {
    fn from(game_error: BackendGameError2) -> Self {
        match game_error {
            BackendGameError2::NotFound(resource) => Status::new(Code::NotFound, format!("Resource {} not found.", resource)),
            BackendGameError2::GameAlreadyMatched => Status::new(Code::AlreadyExists, format!("The game you attempted to join is full.")),
            BackendGameError2::InvalidPlay(reason) => Status::new(Code::InvalidArgument, format!("Can't play card. {}", reason)),
            BackendGameError2::Internal(cause) => {
                println!("ERROR: Internal failure caused by '{:?}'", cause);
                Status::new(Code::Internal, format!("Internal server failure"))
            },
        }
    }
}

/// Causes of `GameError::Internal` errors.
#[derive(Debug)]
pub enum Cause {

    /// Error caused by unknown internal behavior. This is the default case.
    Internal(&'static str),

    /// Error caused by internal/dependency storage layer
    Storage(&'static str, Arc<dyn Error + Send + Sync>),

    /// Error caused by some impossible circumstance, but an error is needed for rust code to compile.
    ///
    /// Example:
    /// ```
    /// use types::{GameError, Cause};
    /// let mut v = vec![1, 2, 3];
    /// let first = v.pop().ok_or(GameError::Internal(Cause::Impossible));
    /// ```
    ///
    /// I truly expect this to never happen. #FamousLastWords
    Impossible,
}

/// This is basically the "rules" enum. For each rule dictating allowed plays, there will be an entry here.
#[derive(Debug)]
pub enum Reason {
    NotYourTurn,
    CardNotInHand,
    CantPlayDecreasingCardValue,
    NeutralDrawPileEmpty,
    CantRedrawCardJustPlayed,
}

/// User-facing message to educate the user how to play.
impl Display for Reason {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Reason::NotYourTurn => write!(f, "It is not your turn."),
            Reason::CardNotInHand => write!(f, "The card is not in your hand."),
            Reason::CantPlayDecreasingCardValue => write!(f, "For a specific color, you must play cards of the same or higher value."),
            Reason::NeutralDrawPileEmpty => write!(f, "You can't draw from the neutral discard pile for that color because it is empty."),
            Reason::CantRedrawCardJustPlayed => write!(f, "You are not allowed to redraw the same card you just discarded."),
        }
    }
}
