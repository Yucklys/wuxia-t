mod character;
mod events;
mod state;
mod ui;

use std::collections::HashMap;

use assets_manager::{loader, Asset, AssetCache};
use crossterm::event::{KeyCode, KeyEvent};
use serde::{Deserialize, Serialize};

pub use character::*;
pub use events::*;
pub use state::GameState;
pub use ui::GameUI;

use crate::components::{map::Maps, Id};

pub struct Game {
    pub ui: GameUI,
    pub state: GameState,
    pub cache: AssetCache,
}

impl Game {
    pub fn on_key(&mut self, key: KeyEvent) {
        match &self.state.game_mode {
            Some(mode) => match mode {
                GameMode::Edit => {}
                GameMode::Story => match key.code {
                    KeyCode::Char(c) => self.ui.on_key(c, &mut self.state),
                    _ => {}
                },
            },
            None => match key.code {
                KeyCode::Enter => {
                    if let Some(i) = self.ui.dashboard.selected() {
                        match i {
                            0 => self.load_save(),
                            1 => self.start_game(),
                            2 => self.state.game_mode = Some(GameMode::Edit),
                            _ => {}
                        }
                    }
                }
                KeyCode::Char(c) => self.ui.on_key(c, &mut self.state),
                _ => {}
            },
        }
    }

    pub fn load_save(&mut self) {
        // TODO: Add load from save
        self.state.game_mode = Some(GameMode::Story);
    }

    pub fn start_game(&mut self) {
        self.state.game_mode = Some(GameMode::Story);
        self.state.curr_map = Some(Maps::HuanHuaCun("tiles".to_string()));
        self.state.load(&self.cache);
        self.ui.focus(Id::Map);
    }

    pub fn on_tick(&mut self) {
        let cache = &self.cache;

        // call on_tick() on UI and state
        self.state.on_tick(cache);
    }
}

#[derive(Serialize, Deserialize)]
pub enum GameMode {
    Story,
    Edit,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct GameSwitch(HashMap<String, bool>);

impl Asset for GameSwitch {
    const EXTENSION: &'static str = "json";

    type Loader = loader::JsonLoader;

    const HOT_RELOADED: bool = true;
}

impl GameSwitch {
    pub fn load(cache: &AssetCache) -> GameSwitch {
        let switch_file = "switches";
        let handle = cache.load_expect::<GameSwitch>(switch_file);

        handle.read().to_owned()
    }

    /// Check if a switch is on. Return false if the switch does not
    /// exist.
    pub fn is_on(&self, other: &str) -> bool {
        match self.0.get(other) {
            Some(&state) => state,
            None => false,
        }
    }

    /// Check if a list of switches is on.
    ///
    /// Empty list will always return true. For non empty list,
    /// all switches must be on for the return to be true.
    pub fn is_all_on(&self, others: &Vec<String>) -> bool {
        let mut active = true;
        let mut index = 0;

        while active && index < others.len() {
            active = self.is_on(&others[index]);
            index += 1;
        }

        active
    }
}
