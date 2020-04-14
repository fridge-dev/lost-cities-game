pub mod framework;
pub mod games;

use std::collections::HashMap;
use crate::v2::games::love_letter::{LoveLetterEvent, LoveLetterInstanceManager};
use crate::v2::games::lost_cities::{LostCitiesEvent, LostCitiesInstanceManager};

pub struct GameManager {
    // TODO in future, I should use generic w static dispatch (GameMgr signature will become ridiculous)
    love_letter_instances: HashMap<String, LoveLetterInstanceManager>,
    lost_cities_instances: HashMap<String, LostCitiesInstanceManager>,
}

impl GameManager {
    pub fn new() -> Self {
        GameManager {
            love_letter_instances: HashMap::new(),
            lost_cities_instances: HashMap::new(),
        }
    }

    pub async fn handle(&mut self, game_id: String, event: GameEvent) -> Result<(), ()> {
        match event {
            GameEvent::LoveLetter(inner) => {
                self.love_letter_instances
                    .entry(game_id)
                    .or_insert_with(|| LoveLetterInstanceManager::new())
                    .handle_event(inner)
                    .await
            },
            GameEvent::LostCities(inner) => {
                self.lost_cities_instances
                    .entry(game_id)
                    .or_insert_with(|| LostCitiesInstanceManager::new())
                    .handle_event(inner)
                    .await
            }
        }
    }
}

pub enum GameEvent {
    LoveLetter(LoveLetterEvent),
    LostCities(LostCitiesEvent),
}


///// One instance of this struct will exist for each single in-progress game.
//#[async_trait::async_trait]
//pub trait GameInstanceManager {
//
//    /// TODO
//    type Event;
//
//    /// TODO
//    async fn handle_event(&mut self, event: Self::Event);
//
//}
