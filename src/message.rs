use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[derive(Default)]
pub struct GameMessage {
    msgs: Vec<Msg>,
}

impl GameMessage {
    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let msgs: Vec<Spans> = self.msgs.iter().map(|m| m.to_spans()).collect();

        let msgs_list = Paragraph::new(msgs).block(Block::default().borders(Borders::ALL));

        f.render_widget(msgs_list, area);
    }

    pub fn push_msg(&mut self, msg: Msg) {
        self.msgs.push(msg);
    }
}

pub struct Msg {
    msg_type: MsgType,
    from: String,
    content: String,
}

impl Msg {
    pub fn new(msg_type: MsgType, from: &str, content: &str) -> Self {
        Self {
            msg_type,
            from: from.to_string(),
            content: content.to_string(),
        }
    }

    pub fn get_style(&self) -> Style {
        match self.msg_type {
            MsgType::System => Style::default().fg(Color::Yellow),
        }
    }

    pub fn to_spans(&self) -> Spans {
        Spans::from(Span::styled(
            format!("{}: {}", self.from, self.content),
            self.get_style(),
        ))
    }
}

pub enum MsgType {
    System,
}
