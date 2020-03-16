use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug)]
pub enum BackendGameError {
    Internal(Cause),
    NotFound(&'static str),
    GameAlreadyMatched,
    InvalidPlay(Reason),
}

impl Error for BackendGameError {}

impl Display for BackendGameError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            BackendGameError::NotFound(entity) => f.write_str(&format!("{} not found!", entity)),
            BackendGameError::Internal(cause) => f.write_str(&format!("Unexpected error: {:?}", cause)),
            BackendGameError::GameAlreadyMatched => f.write_str("No room for u."),
            BackendGameError::InvalidPlay(reason) => f.write_str(&format!("You cannot make that play: {:?}", reason)),
        }
    }
}

/// Causes of `GameError::Internal` errors.
#[derive(Debug)]
pub enum Cause {

    /// Error caused by unknown internal behavior. This is the default case.
    Internal(&'static str),

    /// Error caused by internal/dependency storage layer
    Storage(&'static str, Box<dyn Error>),

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
