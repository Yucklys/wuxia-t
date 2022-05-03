use tui::{
    style::{Color, Style},
    text::Span,
};

use crate::{components::GameState, message::MsgType};

pub fn tutorial(state: &mut GameState) {
    state.messages.add_sentences(vec![
        (MsgType::Input, vec![Span::raw("总算是回到村子里了，今天还是没有找到那头野猪，不过幸好还有两只兔子送上门，晚饭有着落了。")]),
        (MsgType::Input, vec![Span::raw("先回家再说吧，往"), Span::styled("东南", Style::default().fg(Color::Magenta)), Span::raw("方向走一段路就到了。")]),
        (MsgType::System, vec![Span::styled("使用 <h/j/k/l> 向左/下/上/右方向移动。", MsgType::System.get_style())]),
    ]);
}
