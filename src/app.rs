use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use crate::ui;

// アプリの操作と表示に必要な状態を保持するための構造体
pub struct AppState {
    quit_requested: bool, // 終了要求
    message: String,      // 表示するメッセージ
}

// 終了状態とメッセージのみをもつ
impl AppState {
    // 起動直後の状態を保持
    // いまはrun関数内でのみ使用するためprivateにする
    fn new() -> Self {
        Self {
            quit_requested: false,
            message: "q キーで終了する".to_string(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    // ターミナルのイベントをアプリの状態変更へ結び付ける。
    fn handle_event(&mut self, event: Event) {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                self.quit_requested = true;
            }
        }
    }
}

/// 画面を更新しながら、ターミナル上のイベントを処理する。
pub fn run(terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
    let mut app = AppState::new();

    while !app.quit_requested {
        // 現在のアプリの状態を画面に描画する
        terminal.draw(|frame| ui::draw(frame, &app))?;

        // 入力が届いているかを最大100ミリ秒待ち、届いていれば読む
        if event::poll(Duration::from_millis(100))? {
            let event = event::read()?;
            app.handle_event(event);
        }
    }

    Ok(())
}
