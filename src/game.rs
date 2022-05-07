use assets_manager::AssetCache;
use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    components::{GameMode, GameState, GameUI},
    map::Maps,
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
