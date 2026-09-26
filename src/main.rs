mod app;
mod ui;

// アプリを起動して終了時にターミナルを元の状態に戻す処理
fn main() -> color_eyre::Result<()> {
    // エラーやpanicを適切に処理するための設定
    color_eyre::install()?;

    let mut terminal = ratatui::init();

    // 実行結果を返す前に一旦保存しておき、app::runのエラー時、先にrestore()を呼べるように
    // ここでapp::run(&mut terminal)?;としてしまうとここでmain()が終了し、restore()まで進まない
    let result = app::run(&mut terminal);

    // アプリの実行結果を返す前に終了時にターミナルを元の状態に戻す
    ratatui::restore();

    result
}
