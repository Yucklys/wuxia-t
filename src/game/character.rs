use tui::{backend::Backend, layout::Rect, Frame};

pub trait Character {
    fn draw_long_desc<B: Backend>(&self, f: &mut Frame<B>, area: Rect);
    fn draw_short_desc<B: Backend>(&self, f: &mut Frame<B>, area: Rect);

    fn symbol(&self) -> &str;

    fn move_left(&mut self);
    fn move_right(&mut self);
    fn move_up(&mut self);
    fn move_down(&mut self);

    fn get_pos(&self) -> (usize, usize);
    fn get_x(&self) -> usize;
    fn get_y(&self) -> usize;
}
