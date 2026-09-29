use image::DynamicImage;

pub fn load_png(path: &str) -> image::ImageResult<DynamicImage> {
    image::ImageReader::open(path)?.decode()
}