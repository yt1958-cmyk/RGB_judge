<<<<<<< HEAD
use image::{DynamicImage, GrayImage};
use imageproc::contours::find_contours;
use tfhe::shortint::prelude::*;

/// Detects polygons using contours and returns number of sides for the largest
/// contour inside the given rectangle.
fn detect_shape(img: &GrayImage, rect: (u32, u32, u32, u32)) -> usize {
    let (x, y, w, h) = rect;
    let sub_image = image::imageops::crop_imm(img, x, y, w, h).to_image();
    let contours = find_contours::<u8>(&sub_image);
    contours
        .iter()
        .map(|c| c.points.len())
        .max()
        .unwrap_or(0)
}

/// Count shapes with the same number of contour points as the reference shape.
pub fn count_same_shape(img: &DynamicImage, rect: (u32, u32, u32, u32)) -> u32 {
    // Plaintext version kept for comparison
    let gray = img.to_luma8();
    let ref_sides = detect_shape(&gray, rect);
    let contours = find_contours::<u8>(&gray);
    contours
        .iter()
        .filter(|c| c.points.len() == ref_sides)
        .count() as u32
}

/// Attempt to count shapes homomorphically by comparing contour length.
/// Only the comparison and accumulation are done under FHE to keep the
/// example simple.
pub fn count_same_shape_fhe(
    img: &DynamicImage,
    rect: (u32, u32, u32, u32),
    client_key: &ClientKey,
    server_key: &ServerKey,
) -> u32 {
    let gray = img.to_luma8();
    let ref_sides = detect_shape(&gray, rect) as u64;
    let ref_ct = client_key.encrypt(ref_sides);
    let mut count_ct = client_key.encrypt(0u64);
    let contours = find_contours::<u8>(&gray);
    for c in contours {
        let len_ct = client_key.encrypt(c.points.len() as u64);
        let eq = server_key.equal(&len_ct, &ref_ct);
        count_ct = server_key.add(&count_ct, &eq);
    }
    client_key.decrypt(&count_ct) as u32
}
=======
use image::{DynamicImage, GrayImage};
use imageproc::contours::{find_contours, BorderType, Contour};
use crate::encrypt_image::EncryptedImage;
use tfhe::shortint::{Ciphertext, ClientKey, ServerKey};

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
    // Plaintext version kept for comparison
    let gray = img.to_luma8();
    let ref_sides = detect_shape(&gray, rect);
    let contours = find_contours::<u8>(&gray, BorderType::Outer);
    contours.iter().filter(|c| c.len() == ref_sides).count() as u32
}

/// Attempt to count shapes homomorphically by comparing contour length.
/// Only the comparison and accumulation are done under FHE to keep the
/// example simple.
pub fn count_same_shape_fhe(
    img: &DynamicImage,
    rect: (u32, u32, u32, u32),
    client_key: &ClientKey,
    server_key: &ServerKey,
) -> u32 {
    let gray = img.to_luma8();
    let ref_sides = detect_shape(&gray, rect) as u64;
    let ref_ct = client_key.encrypt(ref_sides);
    let mut count_ct = client_key.encrypt(0u64);
    let contours = find_contours::<u8>(&gray, BorderType::Outer);
    for c in contours {
        let len_ct = client_key.encrypt(c.len() as u64);
        let eq = server_key.eq(&len_ct, &ref_ct);
        count_ct = server_key.add(&count_ct, &eq);
    }
    client_key.decrypt(&count_ct) as u32
}

>>>>>>> 2988ef4 (Save local changes before rebase)
