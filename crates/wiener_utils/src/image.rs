use image;
use image::io::Reader;
use log;
use std::path::Path;

pub fn load(path: &str) -> (image::DynamicImage, i32, i32) {
    let img_data = Reader::open(&Path::new(path)).unwrap().decode().unwrap();

    let width = img_data.width().try_into().unwrap();
    let height = img_data.height().try_into().unwrap();

    return (img_data, width, height);
}

pub fn save_image(path: &str, buffer: *const u8, width: i32, height: i32) {
    log::info!(
        "save_image :: Saving {:?}x{:?} image to file {:?}",
        width,
        height,
        path
    );

    log::debug!("save_image :: Loading image from data");
    let data;
    unsafe {
        data = std::slice::from_raw_parts(buffer, (3 * width * height) as usize);
    }
    let result =
        image::ImageBuffer::<image::Rgb<u8>, &[u8]>::from_raw(width as u32, height as u32, data)
            .expect("Error reading image data");

    log::debug!("save_image :: Saving image to file");
    match result.save(path) {
        Ok(_) => {}
        _ => log::warn!("save_image :: Could not save image"),
    }
}
