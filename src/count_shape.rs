use image::{DynamicImage, GrayImage};
use imageproc::contours::{find_contours, BorderType, Contour};

/// Detects polygons using contours and returns number of sides for the largest
/// contour inside the given rectangle.
fn detect_shape(img: &GrayImage, rect: (u32, u32, u32, u32)) -> usize {
    let (x, y, w, h) = rect;
    let sub_image = image::imageops::crop_imm(img, x, y, w, h).to_image();
    let contours = find_contours::<u8>(&sub_image, BorderType::Outer);
    contours
        .iter()
        .map(Contour::len)
        .max()
        .unwrap_or(0)
}

/// Count shapes with the same number of contour points as the reference shape.
pub fn count_same_shape(img: &DynamicImage, rect: (u32, u32, u32, u32)) -> u32 {
    // Convert to grayscale
    let gray = img.to_luma8();
    let ref_sides = detect_shape(&gray, rect);
    // Find contours on full image
    let contours = find_contours::<u8>(&gray, BorderType::Outer);
    contours
        .iter()
        .filter(|c| c.len() == ref_sides)
        .count() as u32
}
