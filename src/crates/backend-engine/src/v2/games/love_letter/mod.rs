mod types;
mod impl1;

use crate::v2::framework::{ClientOut, Holder};
use crate::v2::games::love_letter::types::{GameData, Card, GameInstanceState};
use std::collections::HashMap;

pub enum LoveLetterEvent {
    // Common(?)
    Join(String, ClientOut),
    StartGame,
    GetGameState(String),

    // Game-specific
    PlayCardStaged(String, PlayCardSource),
    SelectTargetPlayer(String, String),
    SelectTargetCard(String, Card),
    PlayCardCommit(String),
}

pub enum PlayCardSource {
    Hand,
    TopDeck,
}

pub struct LoveLetterInstanceManager {
    players: HashMap<String, ClientOut>,
    state2: Holder<GameInstanceState>,
}

impl LoveLetterInstanceManager {
    pub fn new() -> Self {
        LoveLetterInstanceManager {
            players: HashMap::new(),
            state2: Holder::new(GameInstanceState::WaitingForStart)
        }
    }
}
