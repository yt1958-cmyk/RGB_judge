//他ファイル encrypt_image.rs にある EncryptedBlock（暗号化されたブロック構造体）を使えるようにする
use crate::encrypt_image::EncryptedBlock;
//TFHEの主要な型を使う。（Ciphertext:暗号化された数値、ClientKey:復号に使う鍵（秘密鍵）、ServerKey:今回は未使用だが一応引数に入ってる）
use tfhe::shortint::{Ciphertext, ClientKey, ServerKey};

/// Count how many pixels in the encrypted image match the reference RGB value.
/// For simplicity we decrypt each pixel. Real applications would perform
/// homomorphic comparison instead.
//「暗号化された画像の中に、指定されたRGBのピクセルが何個あるか」をカウントする
//※簡単のため、いったん復号して平文で比較してる（本来はFHE上で比較したほうが良い）
pub fn count_rgb(
    blocks: &[EncryptedBlock],//暗号化された画像ブロックたち（配列）
    ref_rgb: [Ciphertext; 3],//参照するRGB値（暗号化されたR, G, B の3つ）
    client_key: &ClientKey,//復号するための秘密鍵
    _server_key: &ServerKey,//今は使ってないけど、将来的にFHE演算で使えるよう保持
) -> u32 {

    //ref_rgb（暗号化されたR/G/B）を復号して平文の [R, G, B] 数値ベクターに変換する
    let ref_vals: Vec<u64> = ref_rgb
        .iter()
        .map(|c| client_key.decrypt(c))
        .collect();
    
    // RGB一致ピクセル数のカウント用変数（最初は0）カウント初期化
    let mut count = 0u32;

    //各ブロックに対して、ピクセルデータ（暗号ビット列）のイテレーターを作る
    for block in blocks {
        let mut iter = block.data.iter();

        //暗号化データが [R, G, B, R, G, B, ...] の順なので、3個ずつ取り出す→ それぞれ rv, gv, bv に復号された数値を格納する
        while let (Some(r), Some(g), Some(b)) = (iter.next(), iter.next(), iter.next()) {
            let rv = client_key.decrypt(r);
            let gv = client_key.decrypt(g);
            let bv = client_key.decrypt(b);
            // もし復号したRGB値が、参照RGB値と完全一致してたら count += 1
            if rv == ref_vals[0] && gv == ref_vals[1] && bv == ref_vals[2] {
                count += 1;
            }
        }
    }
    count
}
