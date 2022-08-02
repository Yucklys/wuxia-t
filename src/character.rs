use serde::{Deserialize, Serialize};
use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub trait Character {
    fn draw_long_desc<B: Backend>(&self, f: &mut Frame<B>, area: Rect);
    fn draw_short_desc<B: Backend>(&self, f: &mut Frame<B>, area: Rect);

    fn symbol(&self) -> &str;

    fn move_left(&mut self);
    fn move_right(&mut self);
    fn move_up(&mut self);
    fn move_down(&mut self);

    fn get_pos(&self) -> (usize, usize);
    fn get_x(&self) -> usize;
    fn get_y(&self) -> usize;
}

#[derive(Default, Serialize, Deserialize)]
pub struct Player {
    name: String,
    pos: (usize, usize),
}

impl Player {
    pub fn init() -> Self {
        Self {
            name: "少年".to_string(),
            pos: (3, 0),
        }
    }
}

impl Character for Player {
    fn draw_short_desc<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let info = Paragraph::new(self.name.as_str()).block(Block::default().borders(Borders::ALL));

        f.render_widget(info, area);
    }

    fn draw_long_desc<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {}

    fn symbol(&self) -> &str {
        ""
    }

    fn move_left(&mut self) {
        self.pos.0 -= 1
    }

    fn move_right(&mut self) {
        self.pos.0 += 1
    }

    fn move_up(&mut self) {
        self.pos.1 -= 1
    }

    fn move_down(&mut self) {
        self.pos.1 += 1
    }

    fn get_pos(&self) -> (usize, usize) {
        self.pos
    }

    fn get_x(&self) -> usize {
        self.pos.0
    }

    fn get_y(&self) -> usize {
        self.pos.1
    }
}
