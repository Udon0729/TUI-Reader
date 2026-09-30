use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use image::DynamicImage;
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Layout, Rect, Size},
    widgets::{Block, Borders},
};

use ratatui_image::{
    Resize,
    picker::{Picker, ProtocolType},
    protocol::Protocol,
};

use crate::{document, ui};

// 一枚の固定画像についてアプリ側で保持する情報
struct PreparedImage {
    picker: Picker,       // 起動時にターミナルに問い合わせた結果
    source: DynamicImage, // 読み込んだ画像データ
    rendered: Protocol,   // ターミナルの画像方式に変換した結果
}

// アプリの操作と表示に必要な状態を保持するための構造体
pub struct AppState {
    quit_requested: bool,                // 終了要求
    protocol_label: String,              // ターミナルの画像方式を表すラベル
    page: Option<PreparedImage>,         // Kitty方式のときだけSomeとなる。それ以外はNone
    page_text: String,                   // 先頭ページから取った本文。チャット欄に出す
    page_words: Vec<document::PageWord>, // 単語ごとのページ内座標。範囲選択で使う
}

// 終了状態とメッセージのみをもつ
impl AppState {
    pub fn protocol_label(&self) -> &str {
        &self.protocol_label
    }

    // 描画では変換済み画像のみを渡す。元画像とPickerはアプリ側に残しておく。
    pub fn page_image(&self) -> Option<&Protocol> {
        self.page.as_ref().map(|page| &page.rendered)
    }

    pub fn page_text(&self) -> &str {
        &self.page_text
    }

    pub fn page_words(&self) -> &[document::PageWord] {
        &self.page_words
    }
}

/// 画面を更新しながら、ターミナル上のイベントを処理する。
pub fn run(terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
    // mainのratatui::init()で代替画面に入った後、最初のevent::read()より前に問い合わせる。
    let picker = Picker::from_query_stdio()?;
    let protocol_label = format!("{:?}", picker.protocol_type());
    let (page, page_text, page_words) = if picker.protocol_type() == ProtocolType::Kitty {
        let first_page = document::render_first_page("assets/sample.pdf")?;
        let rendered = encode(&picker, &first_page.image, current_size(terminal)?)?;
        let prepared = PreparedImage {
            picker,
            source: first_page.image,
            rendered,
        };
        (Some(prepared), first_page.text, first_page.words)
    } else {
        (None, String::new(), Vec::new())
    };

    let mut app = AppState {
        quit_requested: false,
        protocol_label,
        page,
        page_text,
        page_words,
    };

    while !app.quit_requested {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key)
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') =>
                {
                    app.quit_requested = true;
                }
                // 幅と高さは文字セル数。ここでProtocolを作り直して、次のdraw()で描画するようにする。
                Event::Resize(width, height) => {
                    if let Some(page) = &mut app.page {
                        let rendered =
                            encode(&page.picker, &page.source, Size::new(width, height))?;
                        page.rendered = rendered;
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}

// 描画コールバックの中ではサイズを読むだけにして、画像変換はコールバックの外で行う。
fn current_size(terminal: &mut DefaultTerminal) -> color_eyre::Result<Size> {
    let mut size = Size::default();
    terminal.draw(|frame| {
        size = frame.area().into();
    })?;
    Ok(size)
}

fn encode(
    picker: &Picker,
    source: &DynamicImage,
    terminal_size: Size,
) -> Result<Protocol, ratatui_image::errors::Errors> {
    let full = Rect::new(0, 0, terminal_size.width, terminal_size.height);
    // 65% / 35%に分割して、それぞれを描画する。
    let [document_area, _] =
        Layout::horizontal([Constraint::Percentage(65), Constraint::Percentage(35)]).areas(full);
    let image_area: Size = Block::default()
        .borders(Borders::ALL)
        .inner(document_area)
        .into();

    picker.new_protocol(source.clone(), image_area, Resize::Fit(None))
}
