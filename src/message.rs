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
pub struct GameMessage {
    messages: Vec<Msg>,
}

impl Default for GameMessage {
    fn default() -> Self {
        Self { messages: vec![] }
    }
}

impl GameMessage {
    pub fn add_sentence(&mut self, msg_type: MsgType, sentence: Vec<(&str, Style)>) {
        self.messages.push(Msg::new(msg_type, sentence));
    }

    pub fn add_sentences(&mut self, bunch: Vec<(MsgType, Vec<(&str, Style)>)>) {
        for (msg_type, sentence) in bunch {
            self.add_sentence(msg_type, sentence);
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

#[derive(Serialize, Deserialize)]
pub struct Msg {
    msg_type: MsgType,
    contents: Vec<(String, Style)>,
}

impl Msg {
    pub fn new(msg_type: MsgType, contents: Vec<(&str, Style)>) -> Self {
        Self {
            msg_type,
            contents: contents
                .iter()
                .map(|(raw, style)| (raw.to_string(), *style))
                .collect(),
        }
    }

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
                .map(|(raw, style)| Span::styled(raw, *style)),
        );
        Spans::from(output)
    }
}

#[derive(Deserialize, Serialize)]
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
