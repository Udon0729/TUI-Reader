# TUI-Reader

PDFやEPUBをターミナルで読み、選択した本文・図・数式についてLLMに質問するためのアプリ。Rustで開発している。Linuxでの実行とSSH接続先での利用を想定している。

## 現在できること

- 画面を文書欄とチャット欄に分ける。
- ターミナルの画像表示方式を調べ、Kitty方式の場合にPDFの先頭ページを表示する。
- 先頭ページの本文と単語の矩形座標を取得し、本文を右側に表示する。
- リサイズ時に画像を表示領域に合わせて変換し直す。
- `q`キーで終了する。

右側のチャット欄は現在、抽出した本文の確認に使っている。LLM接続、ページ移動、範囲選択、コピー、EPUB対応は未実装。SSH・tmuxでの画像表示は検証が必要。

## 起動

Rustのstableツールチェーンを用意する。MuPDFをビルドするため、C/C++コンパイラとlibclangも必要。Ubuntuでは次のパッケージを使う。

```sh
sudo apt-get update
sudo apt-get install -y build-essential clang libclang-dev pkg-config libfontconfig1-dev
```

表示したいPDFを`assets/sample.pdf`に置き、リポジトリのルートで実行する。PDFはリポジトリに含めていない。

```sh
mkdir -p assets
# 手元のPDFを assets/sample.pdf にコピーする
cargo run --locked
```

現在はファイル名と先頭ページがコードに固定されている。画像表示にはKitty方式を使えるターミナルが必要で、それ以外のターミナルでは画像方式の案内を表示する。

## 開発

```sh
cargo fmt --all -- --check
cargo check --locked
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

GitHub Actionsでも同じチェックを実行する。現在、自動テストはないため、`cargo test`の成功だけではPDF表示や操作の正しさを確認できない。
