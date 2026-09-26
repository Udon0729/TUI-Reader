use ratatui::{
    Frame,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::AppState;

/// アプリの状態をターミナル画面へ描画する
pub fn draw(frame: &mut Frame<'_>, app: &AppState) {
    // メッセージを枠付きの文章領域として描画
    let panel = Paragraph::new(app.message())
        .block(Block::default().title("TUI Reader").borders(Borders::ALL));
    frame.render_widget(panel, frame.area());
}
