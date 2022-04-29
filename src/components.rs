use core::fmt;

use assets_manager::AssetCache;
use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{map::WorldGrid, message::GameMessage};

#[derive(Default)]
pub struct Player {
    pub name: String,
    pub pos: (usize, usize),
}

impl Player {
    pub fn init() -> Self {
        Self {
            name: "少年".to_string(),
            pos: (3, 0),
        }
    }

    pub fn draw_basic_info<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let info = Paragraph::new(self.name.as_str()).block(Block::default().borders(Borders::ALL));

        f.render_widget(info, area);
    }
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Default)]
pub struct GameState {
    pub visible_range: usize,
    pub world_grid: WorldGrid,
    pub messages: GameMessage,
    pub should_quit: bool,
    pub player: Player,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            visible_range: 4,
            world_grid: WorldGrid::default(),
            ..GameState::default()
        }
    }

    pub fn load_map(&mut self, cache: &AssetCache, map: &str) {
        self.world_grid = WorldGrid::load_map(cache, map);
    }

    pub fn on_key(&mut self, cache: &AssetCache, code: char) {
        match code {
            'q' => self.should_quit = true,
            'h' => self
                .world_grid
                .player_move(&cache, &mut self.player, Direction::Left),
            'l' => self
                .world_grid
                .player_move(&cache, &mut self.player, Direction::Right),
            'j' => self
                .world_grid
                .player_move(&cache, &mut self.player, Direction::Down),
            'k' => self
                .world_grid
                .player_move(&cache, &mut self.player, Direction::Up),
            _ => {}
        }
    }
}

#[derive(Default)]
pub struct WorldState {
    pub clock: GameClock,
}

pub struct GameClock {
    pub hour: Hour,
    pub subs: u16,
}

impl Default for GameClock {
    fn default() -> Self {
        Self {
            hour: Hour::Zi,
            subs: 0,
        }
    }
}

impl fmt::Display for GameClock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.hour,
            match self.subs {
                0 => "初",
                1 => "正",
                _ => "",
            }
        )
    }
}

// TODO apply time system
#[allow(dead_code)]
pub enum Hour {
    Zi,
    Chou,
    Yin,
    Mao,
    Chen,
    Si,
    Wu,
    Mo,
    Shen,
    You,
    Xu,
    Hai,
}

impl fmt::Display for Hour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Hour::Zi => "子",
                Hour::Chou => "丑",
                Hour::Yin => "寅",
                Hour::Mao => "卯",
                Hour::Chen => "辰",
                Hour::Si => "巳",
                Hour::Wu => "午",
                Hour::Mo => "未",
                Hour::Shen => "申",
                Hour::You => "酉",
                Hour::Xu => "戌",
                Hour::Hai => "亥",
            }
        )
    }
}
