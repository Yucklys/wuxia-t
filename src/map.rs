use crate::components::{Direction, GameState, Player};
use assets_manager::{loader, Asset, AssetCache};
use serde::Deserialize;
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
    const EXTENSION: &'static str = "toml";

    type Loader = loader::TomlLoader;
}

#[derive(Default, Deserialize)]
pub struct Tile {
    pub name: String,
    pub label: String,
    pub passing: Passing,
}

#[derive(Default, Deserialize)]
pub struct Passing {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub down: bool,
}

#[derive(Default, Clone, Deserialize)]
pub struct WorldGrid {
    pub blocks: Vec<Vec<usize>>,
    pub tiles: String,
    pub name: String,
    pub region: String,
}

impl Asset for WorldGrid {
    const EXTENSION: &'static str = "toml";

    type Loader = loader::TomlLoader;
}

impl WorldGrid {
    pub fn load_map(cache: &AssetCache, file: &str) -> WorldGrid {
        let handle = cache.load_expect::<WorldGrid>(file);
        handle.read().to_owned()
    }

    pub fn draw_info<B: Backend>(&self, f: &mut Frame<B>, state: &GameState, area: Rect) {
        let style = Style::default().fg(Color::Cyan);
        let name = Paragraph::new(Spans::from(vec![
            Span::styled(format!("{}{} {}", self.region, DOT, self.name), style),
            Span::raw(format!("({}, {})", state.player.pos.0, state.player.pos.1)),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(style),
        );
        f.render_widget(name, area);
    }

    pub fn draw_tiles<B: Backend>(
        &self,
        f: &mut Frame<B>,
        cache: &AssetCache,
        setting: &GameState,
        area: Rect,
    ) {
        // destruct fields of arguments
        let Self { tiles, blocks, .. } = self;
        let GameState {
            visible_range: range,
            player,
            ..
        } = setting;
        let range = *range;
        let height = blocks.len();
        let width = blocks[0].len();
        let display_range = (3 * range) as f64;
        let pos = player.pos;

        // read tileset from file
        let handle = cache.load_expect::<Tiles>(&tiles);
        let tiles = &handle.read().data;

        // find visible tiles relative to current pos
        let (x_start, x_end, y_start, y_end) = {
            let r = display_range as usize;
            (
                match pos.0.checked_sub(r) {
                    Some(start) => start,
                    None => 0,
                },
                if pos.0 + r > width {
                    width - 1
                } else {
                    pos.0 + r
                },
                match pos.1.checked_sub(r) {
                    Some(start) => start,
                    None => 0,
                },
                if pos.1 + r > height {
                    height - 1
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

    pub fn player_move(&self, cache: &AssetCache, player: &mut Player, direction: Direction) {
        let (x, y) = player.pos;
        let Self { blocks, tiles, .. } = self;
        let width = blocks[0].len();
        let height = blocks.len();
        let handle = cache.load_expect::<Tiles>(&tiles);
        let tiles = &handle.read().data;

        match direction {
            Direction::Left => {
                if x > 0
                    && tiles[blocks[y][x]].passing.left
                    && tiles[blocks[y][x - 1]].passing.right
                {
                    player.pos.0 = x - 1;
                }
            }
            Direction::Right => {
                if x < width - 1
                    && tiles[blocks[y][x]].passing.right
                    && tiles[blocks[y][x + 1]].passing.left
                {
                    player.pos.0 = x + 1;
                }
            }
            Direction::Up => {
                if y > 0 && tiles[blocks[y][x]].passing.top && tiles[blocks[y - 1][x]].passing.down
                {
                    player.pos.1 = y - 1;
                }
            }
            Direction::Down => {
                if y < height - 1
                    && tiles[blocks[y][x]].passing.down
                    && tiles[blocks[y + 1][x]].passing.top
                {
                    player.pos.1 = y + 1;
                }
            }
        }
    }
}
