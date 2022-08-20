use tui::{backend::Backend, layout::Rect, Frame};

use super::Menu;

pub struct Dashboard {
    menu: Menu,
}

impl Default for Dashboard {
    fn default() -> Dashboard {
        Self {
            menu: Menu::with_items(vec![
                "再入江湖",
                "初出茅庐",
                "天工造物",
                "游戏设置",
                "退隐山林",
            ]),
        }
    }
}

impl Dashboard {
    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        f.render_widget(self.menu.clone(), area);
    }

    pub fn on_key(&mut self, code: char) {
        match code {
            'h' => self.menu.previous(),
            'l' => self.menu.next(),
            ' ' => self.menu.unselect(),
            _ => {}
        }
    }

    pub fn selected(&self) -> Option<usize> {
        self.menu.selected()
    }
}
