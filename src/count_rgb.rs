use crate::encrypt_image::EncryptedImage;
use tfhe::shortint::prelude::*;

/// Internal: run connected component labeling on a boolean image.
fn ccl(width: u32, height: u32, map: &[bool]) -> u32 {
    let mut visited = vec![false; map.len()];
    let mut count = 0u32;
    let dirs = [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)];
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            if map[idx] && !visited[idx] {
                count += 1;
                let mut stack = vec![idx];
                while let Some(cur) = stack.pop() {
                    if visited[cur] { continue; }
                    visited[cur] = true;
                    let cx = (cur as u32) % width;
                    let cy = (cur as u32) / width;
                    for (dx, dy) in dirs.iter() {
                        let nx = cx as i32 + dx;
                        let ny = cy as i32 + dy;
                        if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                            let nidx = (ny as u32 * width + nx as u32) as usize;
                            if map[nidx] && !visited[nidx] {
                                stack.push(nidx);
                            }
                        }
                    }
                }
            }
        }
    }
    count
}

/// Count how many objects (connected components) in the encrypted image match
/// the reference RGB value.
/// RGB comparison is performed homomorphically and then a plain CCL step
/// groups adjacent matching pixels.
pub fn count_rgb_objects(
    enc_img: &EncryptedImage,
    ref_rgb: [Ciphertext; 3],
    client_key: &ClientKey,
    server_key: &ServerKey,
) -> u32 {
    // 1. equality check for each pixel in the encrypted domain
    let mut eq_pixels = Vec::with_capacity((enc_img.width * enc_img.height) as usize);
    for px in enc_img.data.chunks(3) {
        let r = server_key.equal(&px[0], &ref_rgb[0]);
        let g = server_key.equal(&px[1], &ref_rgb[1]);
        let b = server_key.equal(&px[2], &ref_rgb[2]);
        let rg = server_key.bitand(&r, &g);
        let rgb = server_key.bitand(&rg, &b);
        eq_pixels.push(rgb);
    }

    // 2. decrypt boolean map
    let bool_map: Vec<bool> = eq_pixels
        .iter()
        .map(|c| client_key.decrypt(c) != 0)
        .collect();

    // 3. connected component labeling on plaintext boolean map
    ccl(enc_img.width, enc_img.height, &bool_map)
}
