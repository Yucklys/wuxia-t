use serde::{Deserialize, Serialize};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

use crate::game::{Attribute, Character, Pos, Property, PropertyType, PropertyValue};

#[derive(Default, Serialize, Deserialize)]
pub struct Player {
    name: String,
    pos: Pos,

    attr: Attribute,
    prop: Property,
    // TODO Add a buff field to show changes in attr and prop
}

impl Player {
    pub fn init() -> Self {
        Self {
            name: "方洵".to_string(),
            pos: Pos::new(3, 0),
            attr: Attribute::human(),
            prop: Property::from_attrs(Attribute::human()),
        }
    }
}

impl Character for Player {
    fn draw_long_desc<B: Backend>(&self, _f: &mut Frame<B>, _area: Rect) {}

    fn draw_short_desc<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let chunks = Layout::default()
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(2),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .direction(Direction::Vertical)
            .margin(1)
            .split(area);

        // draw border
        let block = Block::default().borders(Borders::ALL);
        f.render_widget(block, area);

        // draw name, title, and level
        {
            let info = Paragraph::new(self.name.as_str()).alignment(Alignment::Center);
            f.render_widget(info, chunks[0]);
        }

        // draw Jing, Qi, and Shen
        {
            let chunks = Layout::default()
                .constraints(
                    [
                        Constraint::Ratio(1, 3),
                        Constraint::Ratio(1, 3),
                        Constraint::Ratio(1, 3),
                    ]
                    .as_ref(),
                )
                .direction(Direction::Horizontal)
                .split(chunks[1]);

            let (cur_jing, cur_qi, cur_shen) = (
                self.prop
                    .get(&PropertyType::Jing)
                    .unwrap_or(&PropertyValue::Number(0.0)),
                self.prop
                    .get(&PropertyType::Qi)
                    .unwrap_or(&PropertyValue::Number(0.0)),
                self.prop
                    .get(&PropertyType::Shen)
                    .unwrap_or(&PropertyValue::Number(0.0)),
            );
            let (max_jing, max_qi, max_shen) = (
                self.prop
                    .get(&PropertyType::MaxJing)
                    .unwrap_or(&PropertyValue::Number(0.0)),
                self.prop
                    .get(&PropertyType::MaxQi)
                    .unwrap_or(&PropertyValue::Number(0.0)),
                self.prop
                    .get(&PropertyType::MaxShen)
                    .unwrap_or(&PropertyValue::Number(0.0)),
            );

            let (jing_gauge, qi_gauge, shen_gauge) = (
                Gauge::default()
                    .block(
                        Block::default()
                            .borders(Borders::TOP)
                            .title("精")
                            .title_alignment(Alignment::Center),
                    )
                    .label(format!("{cur}/{max}", cur = cur_jing, max = max_jing))
                    .gauge_style(Style::default().fg(Color::Red).bg(Color::Black))
                    .ratio(max_jing.unwrap_number() / cur_jing.unwrap_number()),
                Gauge::default()
                    .block(
                        Block::default()
                            .borders(Borders::TOP)
                            .title("气")
                            .title_alignment(Alignment::Center),
                    )
                    .label(format!("{cur}/{max}", cur = cur_qi, max = max_qi))
                    .gauge_style(Style::default().fg(Color::Blue).bg(Color::Black))
                    .ratio(max_qi.unwrap_number() / cur_qi.unwrap_number()),
                Gauge::default()
                    .block(
                        Block::default()
                            .borders(Borders::TOP)
                            .title("神")
                            .title_alignment(Alignment::Center),
                    )
                    .gauge_style(Style::default().fg(Color::Yellow).bg(Color::Black))
                    .label(format!("{cur}/{max}", cur = cur_shen, max = max_shen))
                    .ratio(max_shen.unwrap_number() / cur_shen.unwrap_number()),
            );

            f.render_widget(jing_gauge, chunks[0]);
            f.render_widget(qi_gauge, chunks[1]);
            f.render_widget(shen_gauge, chunks[2]);
        }
    }

    fn symbol(&self) -> &str {
        "@"
    }

    fn move_left(&mut self) {
        self.pos.move_left(1);
    }

    fn move_right(&mut self) {
        self.pos.move_right(1);
    }

    fn move_up(&mut self) {
        self.pos.move_up(1);
    }

    fn move_down(&mut self) {
        self.pos.move_down(1);
    }

    fn get_pos(&self) -> (usize, usize) {
        self.pos.here()
    }

    fn get_x(&self) -> usize {
        self.pos.x()
    }

    fn get_y(&self) -> usize {
        self.pos.y()
    }
}
