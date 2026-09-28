mod image_file;

// 呼び出し側はファイルの分割を意識せず、document::load_png()として使えるようにする
pub use image_file::load_png;