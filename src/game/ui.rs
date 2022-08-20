use crate::components::{dashboard::Dashboard, saves::SaveMenu, Direction, Id};

use super::GameState;

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
    // TODO Replace code with KeyEvent. Add Msg callback
    pub fn on_key(&mut self, code: char, state: &mut GameState) {
        match self.focus {
            Id::Dashboard => match code {
                'q' => state.should_quit = true,
                _ => self.dashboard.on_key(code),
            },
            Id::SaveMenu => match code {
                'q' => self.focus(Id::Dashboard),
                _ => self.save_menu.on_key(code),
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
