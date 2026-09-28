use image::DynamicImage;

// パスのPNGファイルを読み込んで、画素をもつDynamicImage型のデータを返す
pub fn load_png(path: &str) -> image::ImageResult<DynamicImage> {
    // decode（）で画像データを読み込んでDynamicImage型のデータに変換する
    image::ImageReader::open(path)?.decode()
}