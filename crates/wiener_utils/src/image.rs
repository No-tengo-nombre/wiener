use image::io::Reader;
use std::path::Path;

pub fn load(path: &str) -> (image::DynamicImage, i32, i32) {
    let img_data = Reader::open(&Path::new(path)).unwrap().decode().unwrap();

    let width = img_data.width().try_into().unwrap();
    let height = img_data.height().try_into().unwrap();

    return (img_data, width, height);
}
