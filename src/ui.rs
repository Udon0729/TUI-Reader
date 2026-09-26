use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::AppState;

/// アプリの状態を用いて文書欄とチャット欄を描画
pub fn draw(frame: &mut Frame<'_>, app: &AppState) {
    let [document_area, chat_area] =
        Layout::horizontal([Constraint::Percentage(65), Constraint::Percentage(35)])
            .areas(frame.area());

    let document_panel =
        Paragraph::new(app.message()).block(Block::default().title("文書").borders(Borders::ALL));
    frame.render_widget(document_panel, document_area);

    let chat_panel = Paragraph::new("チャット")
        .block(Block::default().title("チャット欄").borders(Borders::ALL));
    frame.render_widget(chat_panel, chat_area);
}
