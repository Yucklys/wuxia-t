mod dashboard;

pub use dashboard::Dashboard;
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Widget,
};

#[derive(Clone)]
struct Menu<'a> {
    selected: Option<usize>,
    items: Vec<&'a str>,
}

impl<'a> Widget for Menu<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let width = area.width;
        let menu_str_len = self.items.iter().fold(0, |sum, i| sum + i.chars().count()) as u16; // "< item >"
        let spacer = if width >= menu_str_len {
            (width - menu_str_len) / (self.items.len() as u16 + 1)
        } else {
            0
        };

        let normal_style = Style::default();
        let hl_style = Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED);

        let mut x = area.left();
        x += spacer;
        for (idx, item) in self.items.iter().enumerate() {
            let style = match self.selected {
                None => normal_style,
                Some(i) => {
                    if i == idx {
                        hl_style
                    } else {
                        normal_style
                    }
                }
            };
            buf.set_string(x, area.top(), format!("< {} >", item), style);
            x += spacer;
        }
    }
}

impl<'a> Menu<'a> {
    fn with_items(items: Vec<&'a str>) -> Menu<'a> {
        Menu {
            selected: if items.is_empty() { None } else { Some(0) },
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.selected {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selected = Some(i);
    }

    fn previous(&mut self) {
        let i = match self.selected {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selected = Some(i);
    }

    fn unselect(&mut self) {
        self.selected = None;
    }

    pub fn selected(&self) -> Option<usize> {
        self.selected
    }
}
