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

pub fn save_image_rgb_f32(path: &str, buffer: &[f32], width: i32, height: i32) {
    log::info!(
        "save_image :: Saving {:?}x{:?} f32 RGB image to file {:?}",
        width,
        height,
        path
    );

    log::debug!("save_image :: Loading image from data");
    let result = image::ImageBuffer::<image::Rgb<f32>, &[f32]>::from_raw(
        width as u32,
        height as u32,
        buffer,
    )
    .expect("Not enough memory was allocated");

    log::debug!("save_image :: Saving image to file");
    match result.save(path) {
        Ok(_) => {}
        Err(r) => log::warn!("save_image :: Could not save image, found error '{r}'"),
    }
}

pub fn save_image_rgba_f32(path: &str, buffer: &[f32], width: i32, height: i32) {
    log::info!(
        "save_image :: Saving {:?}x{:?} f32 RGB image to file {:?}",
        width,
        height,
        path
    );

    log::debug!("save_image :: Loading image from data");
    let result = image::ImageBuffer::<image::Rgba<f32>, &[f32]>::from_raw(
        width as u32,
        height as u32,
        buffer,
    )
    .expect("Not enough memory was allocated");

    log::debug!("save_image :: Saving image to file");
    match result.save(path) {
        Ok(_) => {}
        Err(r) => log::warn!("save_image :: Could not save image, found error '{r}'"),
    }
}

pub fn save_image_rgb_u8(path: &str, buffer: &[u8], width: i32, height: i32) {
    log::info!(
        "save_image :: Saving {:?}x{:?} u8 RGB image to file {:?}",
        width,
        height,
        path
    );

    log::debug!("save_image :: Loading image from data");
    let result =
        image::ImageBuffer::<image::Rgb<u8>, &[u8]>::from_raw(width as u32, height as u32, buffer)
            .expect("Not enough memory was allocated");

    log::debug!("save_image :: Saving image to file");
    match result.save(path) {
        Ok(_) => {}
        Err(r) => log::warn!("save_image :: Could not save image, found error '{r}'"),
    }
}

pub fn save_image_rgba_u8(path: &str, buffer: &[u8], width: i32, height: i32) {
    log::info!(
        "save_image :: Saving {:?}x{:?} u8 RGB image to file {:?}",
        width,
        height,
        path
    );

    log::debug!("save_image :: Loading image from data");
    let result =
        image::ImageBuffer::<image::Rgba<u8>, &[u8]>::from_raw(width as u32, height as u32, buffer)
            .expect("Not enough memory was allocated");

    log::debug!("save_image :: Saving image to file");
    match result.save(path) {
        Ok(_) => {}
        Err(r) => log::warn!("save_image :: Could not save image, found error '{r}'"),
    }
}

pub fn save_image_rgb_u16(path: &str, buffer: &[u16], width: i32, height: i32) {
    log::info!(
        "save_image :: Saving {:?}x{:?} u16 RGB image to file {:?}",
        width,
        height,
        path
    );

    log::debug!("save_image :: Loading image from data");
    let result = image::ImageBuffer::<image::Rgb<u16>, &[u16]>::from_raw(
        width as u32,
        height as u32,
        buffer,
    )
    .expect("Not enough memory was allocated");

    log::debug!("save_image :: Saving image to file");
    match result.save(path) {
        Ok(_) => {}
        Err(r) => log::warn!("save_image :: Could not save image, found error '{r}'"),
    }
}

pub fn save_image_rgba_u16(path: &str, buffer: &[u16], width: i32, height: i32) {
    log::info!(
        "save_image :: Saving {:?}x{:?} u16 RGB image to file {:?}",
        width,
        height,
        path
    );

    log::debug!("save_image :: Loading image from data");
    let result = image::ImageBuffer::<image::Rgba<u16>, &[u16]>::from_raw(
        width as u32,
        height as u32,
        buffer,
    )
    .expect("Not enough memory was allocated");

    log::debug!("save_image :: Saving image to file");
    match result.save(path) {
        Ok(_) => {}
        Err(r) => log::warn!("save_image :: Could not save image, found error '{r}'"),
    }
}
