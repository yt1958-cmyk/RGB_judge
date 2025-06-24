use crate::encrypt_image::EncryptedBlock;
use tfhe::shortint::{Ciphertext, ClientKey, ServerKey};

/// Count how many pixels in the encrypted image match the reference RGB value.
/// For simplicity we decrypt each pixel. Real applications would perform
/// homomorphic comparison instead.
pub fn count_rgb(
    blocks: &[EncryptedBlock],
    ref_rgb: [Ciphertext; 3],
    client_key: &ClientKey,
    _server_key: &ServerKey,
) -> u32 {
    let ref_vals: Vec<u64> = ref_rgb
        .iter()
        .map(|c| client_key.decrypt(c))
        .collect();
    let mut count = 0u32;
    for block in blocks {
        let mut iter = block.data.iter();
        while let (Some(r), Some(g), Some(b)) = (iter.next(), iter.next(), iter.next()) {
            let rv = client_key.decrypt(r);
            let gv = client_key.decrypt(g);
            let bv = client_key.decrypt(b);
            if rv == ref_vals[0] && gv == ref_vals[1] && bv == ref_vals[2] {
                count += 1;
            }
        }
    }
    count
}
