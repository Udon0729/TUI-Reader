use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph},
};
use ratatui_image::Image;

use crate::app::AppState;

/// アプリの状態を用いて文書欄とチャット欄を描画
pub fn draw(frame: &mut Frame<'_>, app: &AppState) {
    let [document_area, chat_area] =
        Layout::horizontal([Constraint::Percentage(65), Constraint::Percentage(35)])
            .areas(frame.area()); // この時点で使える画面領域を[document_area, chat_area]として返す

    let document_block = Block::default()
        .title(format!("文書 ({})", app.protocol_label()))
        .borders(Borders::ALL);

    if let Some(image) = app.page_image() {
        // 枠線に隠れないように画像は枠の内側に置く
        let inner = document_block.inner(document_area);
        frame.render_widget(Image::new(image), inner);
        // 画像の後に枠を描画して、周囲の枠線を残す
        frame.render_widget(document_block, document_area);
    } else {
        // Kitty方式でないときはテキストを表示
        let message = format!(
            "画像方式は {} です。 Kitty方式のときは画像が表示されます。", 
            app.protocol_label()
        );

        let paragraph = Paragraph::new(message).block(document_block);
        frame.render_widget(paragraph, document_area);
    }

    let chat_panel = Paragraph::new("チャット").block(Block::default().title("チャット欄").borders(Borders::ALL));
    frame.render_widget(chat_panel, chat_area);
}
