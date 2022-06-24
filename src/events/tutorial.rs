use tui::style::{Color, Style};

use crate::message::{GameMessage, MsgType};

pub fn tutorial(messages: &mut GameMessage) {
    let raw = |raw| (raw, Style::default());
    messages.add_sentences(vec![
        (MsgType::Input, vec![raw("总算是回到村子里了，今天还是没有找到那头野猪，不过幸好还有两只兔子送上门，晚饭有着落了。")]),
        (MsgType::Input, vec![raw("先回家再说吧，往"), ("东南", Style::default().fg(Color::Magenta)), raw("方向走一段路就到了。")]),
        (MsgType::System, vec![("使用<h/j/k/l>向左/下/上/右方向移动。", MsgType::System.get_style())]),
    ]);
}
