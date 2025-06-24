use image::{DynamicImage, GenericImageView};
use rayon::prelude::*;
use tfhe::shortint::prelude::*;
use tfhe::shortint::parameters::PARAM_MESSAGE_1_CARRY_1_KS_PBS;
use tfhe::shortint::gen_keys;

/// Structure holding the encrypted blocks.
/// Each block remembers its position within the original image so
/// that the blocks can later be merged back into a full image.
#[derive(Clone)]
pub struct EncryptedBlock {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub data: Vec<Ciphertext>,
}

/// Encrypted image reconstructed from individual blocks.
pub struct EncryptedImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Ciphertext>, // RGB data flattened row major
}

/// Encrypt the image using TFHE and return encrypted blocks.
/// The image is divided into blocks of `block_size` pixels.
/// The last blocks on the edges may be smaller if the image size is not
/// a multiple of `block_size`.
pub fn encrypt_image(
    img: &DynamicImage,
    block_size: u32,
    client_key: &ClientKey,
) -> Vec<EncryptedBlock> {
    let (width, height) = img.dimensions();
    // Iterate over blocks in parallel
    let blocks: Vec<EncryptedBlock> = (0..height)
        .step_by(block_size as usize)
        .flat_map(|y| {
            (0..width)
                .step_by(block_size as usize)
                .map(move |x| (x, y))
        })
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|(x, y)| {
            let mut block_pixels = Vec::new();
            let h = (y + block_size).min(height) - y;
            let w = (x + block_size).min(width) - x;
            for j in 0..h {
                for i in 0..w {
                    let pixel = img.get_pixel(x + i, y + j);
                    for c in pixel.0 {
                        block_pixels.push(client_key.encrypt(u64::from(c)));
                    }
                }
            }
            EncryptedBlock {
                x,
                y,
                width: w,
                height: h,
                data: block_pixels,
            }
        })
        .collect();
    blocks
}

/// Merge encrypted blocks back into a single encrypted image so that
/// higher level algorithms can operate on the original 2D layout.
pub fn merge_encrypted_blocks(
    blocks: &[EncryptedBlock],
    width: u32,
    height: u32,
    client_key: &ClientKey,
) -> EncryptedImage {
    // Create a zero ciphertext as initial value for all pixels
    let zero = client_key.encrypt(0u64);
    let mut data = vec![zero; (width * height * 3) as usize];

    for block in blocks {
        for by in 0..block.height {
            for bx in 0..block.width {
                let img_x = block.x + bx;
                let img_y = block.y + by;
                let src_off = ((by * block.width + bx) * 3) as usize;
                let dst_off = ((img_y * width + img_x) * 3) as usize;
                data[dst_off..dst_off + 3]
                    .clone_from_slice(&block.data[src_off..src_off + 3]);
            }
        }
    }

    EncryptedImage { width, height, data }
}

/// Simple helper to create TFHE keys
pub fn create_keys() -> (ClientKey, ServerKey) {
    gen_keys(PARAM_MESSAGE_1_CARRY_1_KS_PBS)
}
