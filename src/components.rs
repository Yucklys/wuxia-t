use core::fmt;
use std::collections::HashSet;

use assets_manager::AssetCache;
use serde::{Deserialize, Serialize};
use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    events::*,
    message::GameMessage,
    ui::Dashboard,
    world::{Maps, Tiles, World},
};

#[derive(Default, Serialize, Deserialize)]
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
    pub switches: HashSet<GameSwitch>,
    pub game_mode: Option<GameMode>,
    pub messages: GameMessage,
    pub player: Player,
    #[serde(skip)]
    pub should_quit: bool,
    pub visible_range: usize,
    #[serde(skip)]
    pub world_grid: World,
}

impl GameState<'_> {
    pub fn new() -> Self {
        let mut switches = HashSet::new();
        switches.insert(GameSwitch::Tutorial);
        Self {
            visible_range: 4,
            world_grid: World::default(),
            switches,
            ..GameState::default()
        }
    }

    pub fn load(&mut self, cache: &AssetCache) {
        self.load_map(cache);
    }

    /// Load current map from assets if curr_map is not None.
    pub fn load_map(&mut self, cache: &AssetCache) {
        if let Some(map) = &self.curr_map {
            self.world_grid = World::load(cache, map);
        }
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

        self.update();
    }

    pub fn update(&mut self) {
        // check active event
        let events = &mut self.world_grid.events;
        for e in events {
            if e.is_active(self.player.pos, &self.switches) {
                if e.repeat || e.num_execute == 0 {
                    match e.name {
                        Event::Tutorial => tutorial::tutorial(&mut self.messages),
                    }

                    e.num_execute += 1;
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
    }
}

#[derive(Serialize, Deserialize)]
pub enum GameMode {
    Story,
    Edit,
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
