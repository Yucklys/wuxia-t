use std::collections::HashMap;

use crossterm::event::{Event, KeyCode, KeyEvent};
use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Widget},
};

use crate::components::{dashboard::Dashboard, saves::SaveMenu, Direction, Id};

use super::{GameState, Message, Value};

pub struct GameUI {
    pub focus: Id,
    pub dashboard: Dashboard,
    pub save_menu: SaveMenu,
}

impl Default for GameUI {
    fn default() -> Self {
        // TODO read save path from game configuration
        let save_path = dirs::data_dir().unwrap().join("wuxia").join("saves");

        Self {
            focus: Id::Dashboard,
            dashboard: Dashboard::default(),
            save_menu: SaveMenu::new(&save_path),
        }
    }
}

impl GameUI {
    // TODO Replace char with KeyEvent. Add Msg callback
    pub fn on_key(&mut self, code: KeyEvent, state: &mut GameState) {
        match code {
            KeyEvent { code, .. } => match code {
                KeyCode::Char(c) => match self.focus {
                    Id::Dashboard => match c {
                        'q' => state.should_quit = true,
                        _ => self.dashboard.on_key(c),
                    },
                    Id::SaveMenu => match c {
                        'q' => self.focus(Id::Dashboard),
                        _ => self.save_menu.on_key(c),
                    },
                    Id::Map => match c {
                        'h' => state
                            .world_grid
                            .player_move(&mut state.player, Direction::Left),
                        'l' => state
                            .world_grid
                            .player_move(&mut state.player, Direction::Right),
                        'j' => state
                            .world_grid
                            .player_move(&mut state.player, Direction::Down),
                        'k' => state
                            .world_grid
                            .player_move(&mut state.player, Direction::Up),
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            },
        }
    }

    pub fn focus(&mut self, id: Id) {
        self.focus = id;
    }
}
