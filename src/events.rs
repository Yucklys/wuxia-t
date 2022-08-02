use assets_manager::{loader, Asset, AssetCache};
use serde::{Deserialize, Serialize};

use crate::{
    game::GameSwitch,
    message::{MessageSystem, Msg},
};

#[derive(Deserialize, Clone, Hash, Eq, PartialEq)]
pub enum EventActivation {
    Touch,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub enum EventStage {
    Waiting,
    Ready,
    Running,
    Closing,
}

impl Default for EventStage {
    fn default() -> Self {
        EventStage::Waiting
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GameEvent {
    id: usize,
    stage: EventStage,
    switch: Vec<String>,
    messages: Vec<Msg>,
}

impl std::hash::Hash for GameEvent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for GameEvent {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for GameEvent {}

impl GameEvent {
    pub fn ready(&mut self, g_switch: &GameSwitch) {
        if g_switch.is_all_on(&self.switch) {
            self.stage = EventStage::Ready;
        }
    }

    pub fn run(&mut self, msg_system: &mut MessageSystem) {
        // Only run the event on ready stage.
        if self.stage == EventStage::Ready {
            self.stage = EventStage::Running;
            msg_system.add_sentences(self.messages.clone());
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct EventSystem {
    events: Vec<GameEvent>,
}

impl Asset for EventSystem {
    const EXTENSION: &'static str = "json";

    type Loader = loader::JsonLoader;
}

impl EventSystem {
    pub fn load(cache: &AssetCache) -> EventSystem {
        let event_file = "events";
        let handle = cache.load_expect::<EventSystem>(event_file);

        handle.read().to_owned()
    }

    pub fn get_ready(&mut self) -> Vec<&mut GameEvent> {
        self.events
            .iter_mut()
            .filter(|e| e.stage == EventStage::Ready)
            .collect()
    }

    pub fn get_waiting(&mut self) -> Vec<&mut GameEvent> {
        self.events
            .iter_mut()
            .filter(|e| e.stage == EventStage::Waiting)
            .collect()
    }
}
