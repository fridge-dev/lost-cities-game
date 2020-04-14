mod types;
mod impl1;
mod impl2;
mod impl2b;

use crate::v2::framework::{ClientOut, Holder, Players, PlayersMut};
use crate::v2::games::love_letter::types::{GameData, Card, GameInstanceState};
use crate::v2::games::love_letter::impl2::LoveLetterStateMachine;
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
    // deprecated: delete
    players: HashMap<String, ClientOut>,
    state_machine: LoveLetterStateMachine,
    // Current state (in state machine) that the game is in
    state2: Holder<GameInstanceState>,
}

impl LoveLetterInstanceManager {
    pub fn new() -> Self {
        // TODO Phase::Setup refactoring
        let players = PlayersMut::new().into_immut();

        LoveLetterInstanceManager {
            players: HashMap::new(),
            state_machine: LoveLetterStateMachine::new(players),
            state2: Holder::new(GameInstanceState::WaitingForStart)
        }
    }
}
