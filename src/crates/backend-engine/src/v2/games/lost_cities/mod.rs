use crate::v2::framework::ClientOut;
use std::collections::HashMap;

pub enum LostCitiesEvent {
    Play
}

pub struct LostCitiesInstanceManager {
    players: HashMap<String, ClientOut>,
    mock_state: HashMap<String, u8>,
}

impl LostCitiesInstanceManager {
    pub fn new() -> Self {
        LostCitiesInstanceManager {
            players: HashMap::new(),
            mock_state: HashMap::new(),
        }
    }

    pub async fn handle_event(&mut self, event: LostCitiesEvent) -> Result<(), ()> {
        unimplemented!()
    }
}
