use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct GameMessage<'a> {
    comb_id: Vec<usize>,
    tubes: Vec<Tube<'a>>,
}

impl<'a> Default for GameMessage<'a> {
    fn default() -> Self {
        Self {
            comb_id: vec![0, 1],
            tubes: vec![
                Tube::new(TubeType::System, "系统"),
                Tube::new(TubeType::Input, "我"),
                Tube::new(TubeType::Battle, "战斗"),
            ],
        }
    }
}

impl<'a> GameMessage<'a> {
    pub fn add_sentence(&mut self, id: usize, sentence: Span<'a>) {
        if id < self.tubes.len() {
            self.tubes[id].contents.push(sentence);
        }
    }

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        // find all messages in the tubes of comb_id
        let msgs: Vec<Spans> = self.get_comb().iter().map(|m| m.to_spans()).collect();

        let msgs_list = Paragraph::new(msgs).block(Block::default().borders(Borders::ALL));

        f.render_widget(msgs_list, area);
    }

    pub fn get_comb(&self) -> Vec<&Tube<'a>> {
        self.comb_id.iter().fold(vec![], |mut comb, &id| {
            comb.push(&self.tubes[id]);
            comb
        })
    }

    pub fn new_tube(&mut self, tube: Tube<'a>) {
        self.tubes.push(tube);
    }
}

pub struct Tube<'a> {
    tube_type: TubeType,
    from: String,
    contents: Vec<Span<'a>>,
}

impl<'a> Tube<'a> {
    pub fn new(msg_type: TubeType, from: &str) -> Self {
        Self {
            tube_type: msg_type,
            from: from.to_string(),
            contents: Vec::new(),
        }
    }

    // pub fn contents(mut self, contents: Vec<Span<'a>>) -> Self {
    //     self.contents = contents;
    //     self
    // }

    pub fn to_spans(&self) -> Spans {
        let mut output = vec![Span::styled(
            format!("{}: ", self.from),
            self.tube_type.get_style(),
        )];
        output.extend(self.contents.iter().map(|c| c.clone()));
        Spans::from(output)
    }
}

pub enum TubeType {
    System,
    Input,
    Battle,
}

impl TubeType {
    pub fn get_style(&self) -> Style {
        match *self {
            TubeType::System => Style::default().fg(Color::Yellow),
            TubeType::Input => Style::default().fg(Color::Cyan),
            TubeType::Battle => Style::default().fg(Color::Red),
        }
    }
}
