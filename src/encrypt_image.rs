use image::{DynamicImage, GenericImageView};
use rayon::prelude::*;
use tfhe::prelude::*;
use tfhe::shortint::{Ciphertext, ClientKey, ConfigBuilder, ServerKey};

/// Structure holding the encrypted blocks
pub struct EncryptedBlock {
    pub data: Vec<Ciphertext>,
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
            for j in y..(y + block_size).min(height) {
                for i in x..(x + block_size).min(width) {
                    let pixel = img.get_pixel(i, j);
                    for c in pixel.0 {
                        block_pixels.push(client_key.encrypt(u64::from(c)));
                    }
                }
            }
            EncryptedBlock { data: block_pixels }
        })
        .collect();
    blocks
}

/// Simple helper to create TFHE keys
pub fn create_keys() -> (ClientKey, ServerKey) {
    let config = ConfigBuilder::all_disabled().enable_default_uint4().build();
    let client_key = ClientKey::generate(config);
    let server_key = ServerKey::generate(&client_key);
    (client_key, server_key)
}
