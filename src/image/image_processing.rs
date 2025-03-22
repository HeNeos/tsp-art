use image::{DynamicImage, GenericImageView, GrayImage, imageops::FilterType};

const MAX_HEIGHT: u32 = 1080;

/// Loads an image from a file and converts it to grayscale.
pub fn load_and_grayscale(path: &str) -> (DynamicImage, GrayImage) {
    let img: DynamicImage = image::open(path).expect("Failed to open image");
    let (width, height) = img.dimensions();

    let (new_width, new_height) = if height > MAX_HEIGHT {
        (MAX_HEIGHT * width / height, MAX_HEIGHT)
    } else {
        (width, height)
    };

    let resized_img = if new_width != width || new_height != height {
        img.resize(new_width, new_height, FilterType::Lanczos3)
    } else {
        img
    };

    let gray_img = resized_img.to_luma8();
    (resized_img, gray_img)
}
