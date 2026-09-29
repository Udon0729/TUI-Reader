use image::DynamicImage;
use mupdf::{Colorspace, Document, Matrix};

// テキストを含むPDFの先頭ページを、画面へ渡せる画素データにして返す。
// MuPDFのDocumentはここで開き、関数を抜けるときに捨てる。
pub fn render_first_page(path: &str) -> color_eyre::Result<DynamicImage> {
    let document = Document::open(path)?;
    // ページ番号は0から始まる。0が先頭ページ。
    let page = document.load_page(0)?;
    // 2.0は縦横2倍の画素数で描く指定。公式例と同じ倍率。
    // device_rgb は赤・緑・青。第3引数の false は透明度のバイトを付けない。
    // 第4引数の true は、mupdf 0.8.0 の公式例と同じ値。
    let pixmap = page.to_pixmap(
        &Matrix::new_scale(2.0, 2.0),
        &Colorspace::device_rgb(),
        false,
        true,
    )?;

    // samples() は pixmap が持っているバイトの借りもの。
    // to_vec() でコピーし、pixmap が関数の終わりで捨てられても画素が残るようにする。
    let bytes = pixmap.samples().to_vec();
    let rgb = image::RgbImage::from_raw(pixmap.width(), pixmap.height(), bytes)
        .ok_or_else(|| color_eyre::eyre::eyre!("PDFページの画素数がRGB画像の大きさと合いません"))?;

    Ok(DynamicImage::ImageRgb8(rgb))
}