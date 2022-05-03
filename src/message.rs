use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct GameMessage<'a> {
    messages: Vec<Msg<'a>>,
}

impl<'a> Default for GameMessage<'a> {
    fn default() -> Self {
        Self { messages: vec![] }
    }
}

impl<'a> GameMessage<'a> {
    pub fn add_sentence(&mut self, msg_type: MsgType, sentence: Vec<Span<'a>>) {
        self.messages.push(Msg::new(msg_type, sentence));
    }

    pub fn add_sentences(&mut self, bunch: Vec<(MsgType, Vec<Span<'a>>)>) {
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

pub struct Msg<'a> {
    msg_type: MsgType,
    contents: Vec<Span<'a>>,
}

impl<'a> Msg<'a> {
    pub fn new(msg_type: MsgType, contents: Vec<Span<'a>>) -> Self {
        Self { msg_type, contents }
    }

    // pub fn contents(mut self, contents: Vec<Span<'a>>) -> Self {
    //     self.contents = contents;
    //     self
    // }

    pub fn to_spans(&self) -> Spans {
        let mut output = vec![Span::styled(
            format!("{}: ", self.msg_type.get_from()),
            self.msg_type.get_style(),
        )];
        output.extend(self.contents.iter().map(|c| c.clone()));
        Spans::from(output)
    }
}

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
