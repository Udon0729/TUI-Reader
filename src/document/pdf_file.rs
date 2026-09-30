use image::DynamicImage;
use mupdf::{Colorspace, Document, Matrix, TextExtractOptions};

// ページ内の1単語。座標はPDF上の矩形での絶対座標。
// 範囲選択で各フィールドを読むまで、ここでは保持するだけ。
#[allow(dead_code)]
pub struct PageWord {
    pub page_number: i32,
    pub text: String,
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

// 先頭ページの画素と、そのページから取った単語。
pub struct FirstPage {
    pub image: DynamicImage,
    pub text: String,
    pub words: Vec<PageWord>,
}

pub fn render_first_page(path: &str) -> color_eyre::Result<FirstPage> {
    let document = Document::open(path)?;
    let page = document.load_page(0)?;

    let pixmap = page.to_pixmap(
        &Matrix::new_scale(2.0, 2.0),
        &Colorspace::device_rgb(),
        false,
        true,
    )?;
    let bytes = pixmap.samples().to_vec();
    let rgb =
        image::RgbImage::from_raw(pixmap.width(), pixmap.height(), bytes).ok_or_else(|| {
            color_eyre::eyre::eyre!("PDFページの画素数がRGB画像の大きさと合いません！")
        })?;

    // text()は改行を残した本文を返し、words()は単語ごとの矩形を返す。
    // どちらも同じページから読み、Document はこの関数の終わりで捨てる。
    let text = page.text(TextExtractOptions::default())?;
    let words = page
        .words(TextExtractOptions::default())?
        .into_iter()
        .map(|word| PageWord {
            page_number: 0,
            text: word.text,
            x0: word.bounds.x0,
            y0: word.bounds.y0,
            x1: word.bounds.x1,
            y1: word.bounds.y1,
        })
        .collect();

    Ok(FirstPage {
        image: DynamicImage::ImageRgb8(rgb),
        text,
        words,
    })
}
