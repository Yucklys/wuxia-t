use std::{
    fs::{self, DirEntry},
    path::Path,
};

use chrono::{DateTime, Utc};
use tui::{
    backend::Backend,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};

use super::{StatefulTable, TableData};

#[derive(Default)]
pub struct SaveData {
    entry: Vec<DirEntry>,
}

impl TableData for SaveData {
    fn header(&self) -> Row {
        Row::new(vec!["名称", "日期"])
    }

    fn data(&self) -> Vec<Row> {
        let mut rows = Vec::new();
        for save in self.entry.iter() {
            let name = save
                .file_name()
                .to_str()
                .unwrap()
                .to_string()
                .split('-')
                .nth(0)
                .unwrap()
                .to_string();
            let date: DateTime<Utc> = save
                .metadata()
                .expect("Read save file metadata failed")
                .modified()
                .expect("Read save file modified time failed")
                .into();

            rows.push(Row::new(vec![
                name,
                format!("{}", date.format("%d/%m/%Y %T")),
            ]));
        }

        rows
    }

    fn len(&self) -> usize {
        self.entry.len()
    }
}

pub struct SaveMenu {
    list: StatefulTable<SaveData>,
}

impl Default for SaveMenu {
    fn default() -> Self {
        Self {
            list: StatefulTable::new(SaveData::default()),
        }
    }
}

impl SaveMenu {
    pub fn new(save_dir: &Path) -> Self {
        let data: Vec<DirEntry> = fs::read_dir(save_dir)
            .expect("Cannot read save directory")
            .map(|entry| entry.expect("Cannot read save file"))
            .collect();

        Self {
            list: StatefulTable::new(SaveData { entry: data }),
        }
    }

    pub fn view<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let header = self.list.items.header();
        let data = self.list.items.data();

        let saves_table = Table::new(data)
            .header(header)
            .block(Block::default().borders(Borders::ALL).title("存档"))
            .widths(&[Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
            .highlight_style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        f.render_stateful_widget(saves_table, area, &mut self.list.state)
    }

    pub fn on_key(&mut self, code: char) {
        match code {
            'j' => self.list.next(),
            'k' => self.list.previous(),
            ' ' => self.list.unselect(),
            _ => {}
        }
    }
}
