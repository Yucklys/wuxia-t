use tui::{backend::Backend, Frame};

use crate::components::{dashboard::Dashboard, Direction, Id};

use super::GameState;

pub struct GameUI {
    pub focus: Id,
    pub dashboard: Dashboard,
}

impl Default for GameUI {
    fn default() -> Self {
        Self {
            focus: Id::Dashboard,
            dashboard: Default::default(),
        }
    }
}

impl GameUI {
    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, state: &mut GameState) {
        self.dashboard.draw(f, state);
    }

    pub fn on_key(&mut self, code: char, state: &mut GameState) {
        match self.focus {
            Id::Dashboard => match code {
                'q' => state.should_quit = true,
                _ => self.dashboard.on_key(code),
            },
            Id::Map => match code {
                'q' => state.should_quit = true,
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
        }
    }

    pub fn focus(&mut self, id: Id) {
        self.focus = id;
    }
}
