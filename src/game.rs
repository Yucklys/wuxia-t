use std::collections::HashMap;

use assets_manager::{loader, Asset, AssetCache};
use crossterm::event::{KeyCode, KeyEvent};
use serde::{Deserialize, Serialize};
use tui::{backend::Backend, Frame};

use crate::{
    character::Player,
    components::Direction,
    events::*,
    message::MessageSystem,
    ui::Dashboard,
    world::{Maps, Tiles, World},
};

pub struct Game<'a> {
    pub ui: GameUI<'a>,
    pub state: GameState<'a>,
    pub cache: AssetCache,
}

impl<'a> Game<'a> {
    pub fn on_key(&mut self, key: KeyEvent) {
        match &self.state.game_mode {
            Some(mode) => match mode {
                GameMode::Edit => {}
                GameMode::Story => match key.code {
                    KeyCode::Char(c) => self.state.on_key(c),
                    _ => {}
                },
            },
            None => match key.code {
                KeyCode::Char('q') => self.state.should_quit = true,
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
                KeyCode::Char(c) => self.ui.dashboard.on_key(c),
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
        self.state.curr_map = Some(Maps::HuanHuaCun("tiles"));
        self.state.load(&self.cache);
    }

    pub fn on_tick(&mut self) {
        let cache = &self.cache;

        // call on_tick() on UI and state
        self.state.on_tick(cache);
    }
}

#[derive(Default)]
pub struct GameUI<'a> {
    pub dashboard: Dashboard<'a>,
}

impl<'a> GameUI<'a> {
    pub fn draw<'s, B: Backend>(&mut self, f: &mut Frame<B>, state: &mut GameState<'s>) {
        self.dashboard.draw(f, state);
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct GameState<'a> {
    #[serde(borrow)]
    pub curr_map: Option<Maps<'a>>,
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

impl GameState<'_> {
    pub fn new() -> Self {
        Self {
            visible_range: 4,
            world_grid: World::default(),
            ..GameState::default()
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

    pub fn on_key(&mut self, code: char) {
        match code {
            'q' => self.should_quit = true,
            'h' => self
                .world_grid
                .player_move(&mut self.player, Direction::Left),
            'l' => self
                .world_grid
                .player_move(&mut self.player, Direction::Right),
            'j' => self
                .world_grid
                .player_move(&mut self.player, Direction::Down),
            'k' => self.world_grid.player_move(&mut self.player, Direction::Up),
            _ => {}
        }
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
