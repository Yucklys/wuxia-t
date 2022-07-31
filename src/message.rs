use serde::{Deserialize, Serialize};
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[derive(Serialize, Deserialize)]
pub struct MessageSystem {
    messages: Vec<Msg>,
}

impl Default for MessageSystem {
    fn default() -> Self {
        Self { messages: vec![] }
    }
}

impl MessageSystem {
    pub fn add_sentence(&mut self, msg: Msg) {
        self.messages.push(msg);
    }

    pub fn add_sentences(&mut self, bunch: Vec<Msg>) {
        for msg in bunch {
            self.add_sentence(msg);
        }
    }

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        // find all messages in the tubes of comb_id
        let msgs: Vec<Spans> = self.messages.iter().map(|m| m.to_spans()).collect();

        let msgs_list = Paragraph::new(msgs)
            .block(Block::default().borders(Borders::ALL))
            .wrap(tui::widgets::Wrap { trim: true });

        f.render_widget(msgs_list, area);
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum MsgStyle {
    Default,
    Hint,
    Target,
}

impl MsgStyle {
    pub fn to_style(&self) -> Style {
        match *self {
            MsgStyle::Default => Style::default(),
            MsgStyle::Hint => Style::default().fg(Color::Yellow),
            MsgStyle::Target => Style::default().fg(Color::Blue),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Msg {
    msg_type: MsgType,
    contents: Vec<(String, MsgStyle)>,
}

impl Msg {
    // pub fn contents(mut self, contents: Vec<Span<'a>>) -> Self {
    //     self.contents = contents;
    //     self
    // }

    pub fn to_spans(&self) -> Spans {
        let mut output = vec![Span::styled(
            format!("{}:", self.msg_type.get_from()),
            self.msg_type.get_style(),
        )];
        output.extend(
            self.contents
                .iter()
                .map(|(raw, style)| Span::styled(raw, style.to_style())),
        );
        Spans::from(output)
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub enum MsgType {
    System,
    Input,
    Battle,
}

impl MsgType {
    pub fn get_style(&self) -> Style {
        match *self {
            MsgType::System => Style::default().fg(Color::Yellow),
            MsgType::Input => Style::default().fg(Color::Cyan),
            MsgType::Battle => Style::default().fg(Color::Red),
        }
    }

    pub fn get_from(&self) -> &str {
        match *self {
            MsgType::System => "系统",
            MsgType::Input => "我",
            MsgType::Battle => "战斗",
        }
    }
}
