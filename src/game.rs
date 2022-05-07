use assets_manager::AssetCache;
use crossterm::event::{KeyCode, KeyEvent};

use crate::components::{GameMode, GameState, GameUI};

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
                    KeyCode::Char(c) => self.state.on_key(&self.cache, c),
                    _ => {}
                },
            },
            None => match key.code {
                KeyCode::Char('q') => self.state.should_quit = true,
                KeyCode::Enter => {
                    if let Some(i) = self.ui.dashboard.selected() {
                        match i {
                            0 | 1 => self.state.game_mode = Some(GameMode::Story),
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
}
