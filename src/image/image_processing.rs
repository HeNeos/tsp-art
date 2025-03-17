use image::{DynamicImage, GrayImage};

/// Loads an image from a file and converts it to grayscale.
pub fn load_and_grayscale(path: &str) -> (DynamicImage, GrayImage) {
    let img: DynamicImage = image::open(path).expect("Failed to open image");
    let gray_img = img.to_luma8();
    (img, gray_img)
}
