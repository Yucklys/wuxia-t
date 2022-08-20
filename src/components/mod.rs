pub mod dashboard;
pub mod map;
pub mod message;
pub mod player;
pub mod saves;

use core::fmt;

use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Row, TableState, Widget},
};

pub enum Id {
    Dashboard,
    Map,
    SaveMenu,
    Dialogue,
    PlayerInfo,
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Default)]
pub struct WorldState {
    pub clock: Clock,
}

pub struct Clock {
    pub hour: Hour,
    pub subs: u16,
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            hour: Hour::Zi,
            subs: 0,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.hour,
            match self.subs {
                0 => "初",
                1 => "正",
                _ => "",
            }
        )
    }
} // TODO apply time system
#[allow(dead_code)]
pub enum Hour {
    Zi,
    Chou,
    Yin,
    Mao,
    Chen,
    Si,
    Wu,
    Mo,
    Shen,
    You,
    Xu,
    Hai,
}

impl fmt::Display for Hour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Hour::Zi => "子",
                Hour::Chou => "丑",
                Hour::Yin => "寅",
                Hour::Mao => "卯",
                Hour::Chen => "辰",
                Hour::Si => "巳",
                Hour::Wu => "午",
                Hour::Mo => "未",
                Hour::Shen => "申",
                Hour::You => "酉",
                Hour::Xu => "戌",
                Hour::Hai => "亥",
            }
        )
    }
}

#[derive(Clone)]
struct Menu {
    selected: Option<usize>,
    items: Vec<String>,
}

impl Widget for Menu {
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

impl Menu {
    fn with_items(items: Vec<&str>) -> Menu {
        Menu {
            selected: if items.is_empty() { None } else { Some(0) },
            items: items.iter().map(|i| i.to_string()).collect(),
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

#[derive(Debug)]
struct StatefulTable<T: TableData> {
    items: T,
    state: TableState,
}

impl<T: TableData> StatefulTable<T> {
    fn new(items: T) -> Self {
        Self {
            items,
            state: TableState::default(),
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

trait TableData {
    fn header(&self) -> Row;
    fn data(&self) -> Vec<Row>;
    fn len(&self) -> usize;
}
