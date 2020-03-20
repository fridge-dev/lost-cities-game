use game_api::api::GameApi2;
use game_api::types::{GameState, Play, Card, GameBoard, CardTarget, CardColor, CardValue, DrawPile, GameMetadata};
use rules::deck::DeckFactory;
use rules::{plays, scoring, endgame};
use std::collections::HashMap;
use storage::local_storage::LocalStore;
use storage::storage_api::GameStore;
use storage::storage_types::{StorageGameMetadata, GameStatus, StorageError, StorageGameState};
use crate::backend::backend_error::{BackendGameError2, Cause, Reason};
use std::sync::Arc;

pub struct StorageBackedGameApi {
    storage: Box<dyn GameStore + Send>,
    deck_factory: DeckFactory,
}

impl StorageBackedGameApi {
    pub fn new() -> Self {
        StorageBackedGameApi {
            storage: Box::new(LocalStore::new()),
            deck_factory: DeckFactory::new(),
        }
    }

    fn update_game_metadata(&mut self, game_id: &str, p2_id: String) -> Result<(), BackendGameError2> {
        let mut metadata = self.storage.load_game_metadata(game_id)
            .map_err(|e| match e {
                StorageError::NotFound => BackendGameError2::NotFound("Game metadata"),
                _ => BackendGameError2::Internal(Cause::Storage("Failed to load game metadata", Arc::new(e)))
            })?;
        metadata.set_p2_id(p2_id)
            .map_err(|e| match e {
                StorageError::IllegalModification => BackendGameError2::GameAlreadyMatched,
                _ => BackendGameError2::Internal(Cause::Storage("Failed to mutate game metadata", Arc::new(e))),
            })?;
        self.storage.update_game_metadata(metadata)
            .map_err(|e| match e {
                StorageError::NotFound => BackendGameError2::NotFound("Game metadata"),
                _ => BackendGameError2::Internal(Cause::Storage("Failed to save game metadata", Arc::new(e)))
            })
    }

    fn create_initial_game_state(&mut self, game_id: String) -> Result<(), BackendGameError2> {
        let mut deck = self.deck_factory.new_shuffled_deck();

        let mut p1_hand: Vec<Card> = Vec::with_capacity(8);
        let mut p2_hand: Vec<Card> = Vec::with_capacity(8);
        for _ in 0..8 {
            p1_hand.push(deck.pop().ok_or_else(|| BackendGameError2::Internal(Cause::Impossible))?);
            p2_hand.push(deck.pop().ok_or_else(|| BackendGameError2::Internal(Cause::Impossible))?);
        }

        let game_state = StorageGameState::new(
            game_id,
            p1_hand,
            p2_hand,
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            deck,
            is_first_turn_p1(),
        );

        self.storage.create_game_state(game_state)
            .map_err(|e| BackendGameError2::Internal(Cause::Storage("Failed to save initial game state", Arc::new(e))))
    }

    fn load_game(&self, game_id: &str, player_id: &str) -> Result<(StorageGameState, bool), BackendGameError2> {
        let metadata = self.storage.load_game_metadata(game_id)
            .map_err(|e| match e {
                StorageError::NotFound => BackendGameError2::NotFound("Game metadata"),
                _ => BackendGameError2::Internal(Cause::Storage("Failed to load game", Arc::new(e)))
            })?;

        let is_player_1 = if player_id == metadata.p1_id() {
            true
        } else if player_id == metadata.p2_id() {
            false
        } else {
            return Err(BackendGameError2::NotFound("Player in game"));
        };

        let storage_game_state = self.storage.load_game_state(game_id)
            .map_err(|e| match e {
                StorageError::NotFound => BackendGameError2::NotFound("Game state"),
                _ => BackendGameError2::Internal(Cause::Storage("Failed to load game state.", Arc::new(e))),
            })?;

        Ok((storage_game_state, is_player_1))
    }
}

#[async_trait::async_trait]
impl GameApi2<BackendGameError2> for StorageBackedGameApi {
    async fn host_game(&mut self, p1_id: String) -> Result<String, BackendGameError2> {
        let game_id = create_game_id();
        let storage_result = self.storage.create_game_metadata(StorageGameMetadata::new(
            game_id.clone(),
            p1_id,
            None,
            GameStatus::InProgress,
        ));

        match storage_result {
            Ok(_) => Ok(game_id),
            // This should never fail.
            Err(e) => Err(BackendGameError2::Internal(Cause::Storage("Failed to list game as hosted.", Arc::new(e))))
        }
    }

    async fn join_game(&mut self, game_id: String, p2_id: String) -> Result<(), BackendGameError2> {
        self.update_game_metadata(&game_id, p2_id)?;
        self.create_initial_game_state(game_id)
    }

    async fn describe_game(&mut self, game_id: String) -> Result<GameMetadata, BackendGameError2> {
        unimplemented!()
    }

    async fn query_unmatched_games(&mut self, player_id: String) -> Result<Vec<GameMetadata>, BackendGameError2> {
        unimplemented!()
    }

    async fn query_in_progress_games(&mut self, player_id: String) -> Result<Vec<GameMetadata>, BackendGameError2> {
        unimplemented!()
    }

    async fn query_completed_games(&mut self, player_id: String) -> Result<Vec<GameMetadata>, BackendGameError2> {
        unimplemented!()
    }

    async fn query_all_unmatched_games(&mut self) -> Result<Vec<GameMetadata>, BackendGameError2> {
        unimplemented!()
    }

    async fn get_game_state(&mut self, game_id: String, player_id: String) -> Result<GameState, BackendGameError2> {
        let (storage_game_state, is_player_1) = self.load_game(&game_id, &player_id)?;

        let game_state = convert_game_state(storage_game_state, is_player_1);

        return Ok(game_state);
    }

    async fn play_card(&mut self, play: Play) -> Result<(), BackendGameError2> {
        let (storage_game_state, is_player_1) = self.load_game(play.game_id(), play.player_id())?;

        let updated_game_state = apply_play_to_game_state(play, storage_game_state, is_player_1)?;

        self.storage.update_game_state(updated_game_state)
            .map_err(|e| BackendGameError2::Internal(Cause::Storage("Failed to save the updated game state", Arc::new(e))))
    }
}

// ================ private, static (stateless) methods related to StorageBackedGameApi =================

fn create_game_id() -> String {
    // random hex string
    format!("{:x}", rand::random::<u128>())
}

fn is_first_turn_p1() -> bool {
    rand::random()
}

// Expensive cloning incoming... :P
fn convert_game_state(storage_game_state: StorageGameState, is_player_1: bool) -> GameState {
    // Here is where we only show what the player is allowed to see.
    let mut concealed_neutral_draw_pile = HashMap::new();
    for (color, value_vec) in storage_game_state.neutral_draw_pile().iter() {
        if let Some(top_card) = value_vec.last() {
            concealed_neutral_draw_pile.insert(*color, (*top_card, value_vec.len()));
        }
    }

    let (my_plays, op_plays) = if is_player_1 {
        (storage_game_state.p1_plays(), storage_game_state.p2_plays())
    } else {
        (storage_game_state.p2_plays(), storage_game_state.p1_plays())
    };

    let game_board = GameBoard::new(
        my_plays.to_owned(),
        op_plays.to_owned(),
        scoring::compute_score(my_plays),
        scoring::compute_score(op_plays),
        concealed_neutral_draw_pile,
        storage_game_state.main_draw_pile().len(),
    );

    let (my_hand, my_previous_plays, is_my_turn) = get_players_info(&storage_game_state, is_player_1);
    let game_status = endgame::get_game_status(&game_board, is_my_turn);

    GameState::new(
        game_board,
        plays::decorate_hand(my_hand.to_owned(), my_previous_plays),
        game_status
    )
}

/// Extract the player-specific info from the storage state, based on the player making the backend request.
fn get_players_info(
    storage_game_state: &StorageGameState,
    is_player_1: bool
) -> (
    &Vec<Card>, /* player's hand */
    &HashMap<CardColor, Vec<CardValue>>, /* player's previous plays */
    bool, /* is player's turn */
) {
    if is_player_1 {
        (
            storage_game_state.p1_hand(),
            storage_game_state.p1_plays(),
            *storage_game_state.p1_turn()
        )
    } else {
        (
            storage_game_state.p2_hand(),
            storage_game_state.p2_plays(),
            !*storage_game_state.p1_turn()
        )
    }
}

fn apply_play_to_game_state(
    play: Play,
    storage_game_state: StorageGameState,
    is_player_1: bool
) -> Result<StorageGameState, BackendGameError2> {

    let pa_sgs = storage_game_state.convert_to_player_aware(is_player_1);

    let card_in_hand_index = validate_play(
        &play,
        pa_sgs.my_hand(),
        pa_sgs.my_plays(),
        pa_sgs.inner().neutral_draw_pile(),
        pa_sgs.is_my_turn()
    ).map_err(|e| BackendGameError2::InvalidPlay(e))?;

    let mut pa_sgs = pa_sgs;
    // Model a turn like in real life:

    // 1. Remove the card from hand
    let _removed_card = pa_sgs.my_hand_mut().remove(card_in_hand_index);

    // 2. Add card on top of target pile
    let target_pile = match play.target() {
        CardTarget::Player => pa_sgs.my_plays_mut(),
        CardTarget::Neutral => pa_sgs.neutral_draw_pile_mut(),
    };

    target_pile.entry(*play.card().card_color())
        .or_insert_with(|| Vec::new())
        .push(*play.card().card_value());

    // 3. Draw new card
    let new_card_opt = match play.draw_pile() {
        DrawPile::Main => pa_sgs.main_draw_pile_mut().pop(),
        DrawPile::Neutral(color) => {
            pa_sgs.neutral_draw_pile_mut()
                .get_mut(color)
                .and_then(|draw_pile| draw_pile.pop())
                .map(|drawn_value| Card::new(*color, drawn_value))
        }
    };
    let new_card = new_card_opt.ok_or_else(|| BackendGameError2::Internal(Cause::Impossible))?;
    pa_sgs.my_hand_mut().push(new_card);

    // 4. Flip the turn marker
    let mut sgs = pa_sgs.convert_to_inner();
    sgs.swap_turn();

    Ok(sgs)
}

fn validate_play(
    play: &Play,
    my_hand: &Vec<Card>,
    my_previous_plays: &HashMap<CardColor, Vec<CardValue>>,
    neutral_draw_pile: &HashMap<CardColor, Vec<CardValue>>,
    is_my_turn: bool
) -> Result<usize, Reason> {

    // RULE: You can only play on your turn.
    if !is_my_turn {
        return Err(Reason::NotYourTurn);
    }

    // RULE: You can only play cards that are in your hand.
    let card_in_hand_index = my_hand.iter()
        .position(|card| card == play.card())
        .ok_or_else(|| Reason::CardNotInHand)?;

    // RULE: You must play cards in increasing order.
    if *play.target() == CardTarget::Player && !plays::is_card_playable(play.card(), my_previous_plays) {
        return Err(Reason::CantPlayDecreasingCardValue);
    }

    if let DrawPile::Neutral(color_to_draw) = play.draw_pile() {
        // RULE: You can't redraw the same card you just played.
        if *play.target() == CardTarget::Neutral && color_to_draw == play.card().card_color() {
            return Err(Reason::CantRedrawCardJustPlayed);
        }

        // RULE: You can't draw from an empty pile.
        let neutral_draw_pile_size = neutral_draw_pile
            .get(color_to_draw)
            .map(|vec| vec.len())
            .unwrap_or(0);
        if neutral_draw_pile_size == 0 {
            return Err(Reason::NeutralDrawPileEmpty);
        }
    }

    return Ok(card_in_hand_index);
}
