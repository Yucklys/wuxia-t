use crate::components::{
    map::{Maps, Tiles, World},
    message::MessageSystem,
    player::Player,
};
use assets_manager::AssetCache;
use serde::{Deserialize, Serialize};

use super::{EventSystem, GameMode, GameSwitch};

#[derive(Default, Serialize, Deserialize)]
pub struct GameState {
    pub curr_map: Option<Maps>,
    pub event_system: EventSystem,
    pub game_mode: Option<GameMode>,
    pub messages: MessageSystem,
    #[serde(skip)]
    pub need_update: bool,
    pub player: Player,
    #[serde(skip)]
    pub should_quit: bool,
    pub switches: GameSwitch,
    pub visible_range: usize,
    #[serde(skip)]
    pub world_grid: World,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            visible_range: 4,
            world_grid: World::default(),
            ..GameState::default()
        }
    }

    pub fn from_json(cache: &AssetCache, json: &str) -> Self {
        match serde_json::from_str::<GameState>(json) {
            Ok(mut state) => {
                state.load(cache);
                state
            }
            Err(_) => panic!("Failed to load from save"),
        }
    }

    pub fn load(&mut self, cache: &AssetCache) {
        self.load_map(cache);
        self.load_events(cache);
        self.load_switch(cache);

        self.update();
    }

    /// Load current map from assets if curr_map is not None.
    fn load_map(&mut self, cache: &AssetCache) {
        if let Some(map) = &self.curr_map {
            self.world_grid = World::load(cache, map);
        }
    }

    fn load_switch(&mut self, cache: &AssetCache) {
        self.switches = GameSwitch::load(cache);
    }

    // Load all events from assets
    pub fn load_events(&mut self, cache: &AssetCache) {
        self.event_system = EventSystem::load(cache);
    }

    pub fn update(&mut self) {
        // update events status
        self.update_events();

        self.need_update = false;
    }

    fn update_events(&mut self) {
        {
            let waiting_events = self.event_system.get_waiting();
            if !waiting_events.is_empty() {
                for e in waiting_events {
                    e.ready(&self.switches);
                }
            }
        }

        {
            let ready_events = self.event_system.get_ready();
            if !ready_events.is_empty() {
                for e in ready_events {
                    e.run(&mut self.messages);
                }
            }
        }
    }

    pub fn on_tick(&mut self, cache: &AssetCache) {
        // check file watchers
        if let Some(map) = &self.curr_map {
            let (mut map_watcher, mut tile_watcher) = (
                cache.load_expect::<World>(map.map_file()).reload_watcher(),
                cache.load_expect::<Tiles>(map.tile_file()).reload_watcher(),
            );

            cache.hot_reload();

            if map_watcher.reloaded() || tile_watcher.reloaded() {
                self.load_map(cache);
            }
        }

        // Check whether the game needs to update
        if self.need_update {
            self.update();
        }
    }
}
