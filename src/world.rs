use crate::{
    character::*,
    components::{Direction, GameState},
};
use assets_manager::{loader, Asset, AssetCache};
use serde::{Deserialize, Serialize};
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    symbols::DOT,
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

#[derive(Deserialize)]
pub struct Tiles {
    pub data: Vec<Tile>,
}

impl Asset for Tiles {
    const EXTENSION: &'static str = "json";

    type Loader = loader::JsonLoader;
}

#[derive(Default, Deserialize, Clone)]
pub struct Tile {
    pub name: String,
    pub label: String,
    pub passing: Passing,
}

#[derive(Default, Deserialize, Clone)]
pub struct Passing {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub down: bool,
}

#[derive(Default, Clone, Deserialize)]
pub struct World {
    pub blocks: Vec<Vec<usize>>,
    #[serde(skip)]
    pub tiles: Vec<Tile>,
    pub name: String,
    pub region: String,
}

impl Asset for World {
    const EXTENSION: &'static str = "json";

    type Loader = loader::JsonLoader;
}

impl World {
    pub fn load(cache: &AssetCache, map: &Maps) -> World {
        // get map_file and tile_file name
        let map_file = map.map_file();
        let tile_file = map.tile_file();

        // load grid and tiles
        let handle = cache.load_expect::<World>(map_file);
        let mut world = handle.read().to_owned();
        let handle = cache.load_expect::<Tiles>(tile_file);
        let tiles = &handle.read().data;
        world.change_tiles(tiles);
        world
    }

    pub fn change_tiles(&mut self, new: &Vec<Tile>) {
        self.tiles = new.to_owned();
    }

    pub fn draw_info<B: Backend>(&self, f: &mut Frame<B>, state: &GameState, area: Rect) {
        let style = Style::default().fg(Color::Cyan);
        let name = Paragraph::new(Spans::from(vec![
            Span::styled(format!("{}{} {}", self.region, DOT, self.name), style),
            Span::raw(format!(
                "({}, {})",
                state.player.get_x(),
                state.player.get_y()
            )),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(style),
        );
        f.render_widget(name, area);
    }

    pub fn draw_tiles<B: Backend>(&self, f: &mut Frame<B>, setting: &GameState, area: Rect) {
        // destruct fields of arguments
        let Self { tiles, blocks, .. } = self;

        // do not render if blocks is empty
        if !blocks.is_empty() {
            let GameState {
                visible_range: range,
                player,
                ..
            } = setting;
            let range = *range;
            let height = blocks.len();
            let width = blocks[0].len();
            let display_range = (3 * range) as f64;
            let pos = player.get_pos();

            // find visible tiles relative to current pos
            let (x_start, x_end, y_start, y_end) = {
                let r = display_range as usize;
                (
                    match pos.0.checked_sub(r) {
                        Some(start) => start,
                        None => 0,
                    },
                    if pos.0 + r > width { width } else { pos.0 + r },
                    match pos.1.checked_sub(r) {
                        Some(start) => start,
                        None => 0,
                    },
                    if pos.1 + r > height {
                        height
                    } else {
                        pos.1 + r
                    },
                )
            };

            // generate Tiles informations
            let mut grid = vec![];
            for y in y_start..y_end {
                let mut row = vec![];
                for x in x_start..x_end {
                    let distance = ((x as f64 - pos.0 as f64).powf(2.0)
                        + (y as f64 - pos.1 as f64).powf(2.0))
                    .sqrt();
                    // ignore block if it is not within the display range
                    // default tile is the one with index 0
                    let tile = &tiles[blocks[y][x]];
                    let label = if pos == (x, y) { "@" } else { &tile.label };
                    // check if tile is in visible range
                    let style = if pos == (x, y) {
                        Style::default().fg(Color::Cyan)
                    } else if distance <= range as f64 {
                        Style::default().fg(Color::White)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    };
                    row.push(Span::styled(format!("{} ", label), style));
                }
                grid.push(Spans::from(row));
            }

            let map = Paragraph::new(grid).alignment(Alignment::Center);

            f.render_widget(map, area);
        }
    }

    pub fn player_move(&self, player: &mut Player, direction: Direction) {
        let (x, y) = player.get_pos();
        let Self { blocks, tiles, .. } = self;
        if !blocks.is_empty() {
            let width = blocks[0].len();
            let height = blocks.len();

            match direction {
                Direction::Left => {
                    if x > 0
                        && tiles[blocks[y][x]].passing.left
                        && tiles[blocks[y][x - 1]].passing.right
                    {
                        player.move_left();
                    }
                }
                Direction::Right => {
                    if x < width - 1
                        && tiles[blocks[y][x]].passing.right
                        && tiles[blocks[y][x + 1]].passing.left
                    {
                        player.move_right();
                    }
                }
                Direction::Up => {
                    if y > 0
                        && tiles[blocks[y][x]].passing.top
                        && tiles[blocks[y - 1][x]].passing.down
                    {
                        player.move_up();
                    }
                }
                Direction::Down => {
                    if y < height - 1
                        && tiles[blocks[y][x]].passing.down
                        && tiles[blocks[y + 1][x]].passing.top
                    {
                        player.move_down();
                    }
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Maps<'a> {
    HuanHuaCun(&'a str),
}

impl<'a> Maps<'a> {
    pub fn map_file(&self) -> &str {
        match &self {
            Maps::HuanHuaCun(_) => "浣花村",
        }
    }

    pub fn tile_file(&self) -> &str {
        match &self {
            Maps::HuanHuaCun(t) => t,
        }
    }
}
