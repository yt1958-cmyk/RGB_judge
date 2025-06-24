use image::{DynamicImage, GenericImageView};
//並行処理の神ライブラリrayon
use rayon::prelude::*;
//TFHEの主要機能とshortint(整数暗号)用の型を使う準備(Ciphertext暗号化されたデータ、Cliantkey暗号化,復号に使う秘密鍵、Serverkey演算に使う公開鍵)
use tfhe::prelude::*;
use tfhe::shortint::{Ciphertext, ClientKey, ConfigBuilder, ServerKey};

///暗号化されたブロックをまとめる構造体
pub struct EncryptedBlock {
    pub data: Vec<Ciphertext>,
}

/// Encrypt the image using TFHE and return encrypted blocks.
/// The image is divided into blocks of `block_size` pixels.
/// The last blocks on the edges may be smaller if the image size is not
/// a multiple of `block_size`.
pub fn encrypt_image(
    img: &DynamicImage, //入力画像（平文）
    block_size: u32, //分割するブロックのサイズ
    client_key: &ClientKey, //暗号化用の鍵
) -> Vec<EncryptedBlock> { //上の画像、ブロックサイズ、鍵の３つを受け取って暗号化されたEncreptedBlockをベクターで返す
    let (width, height) = img.dimensions(); //画像の横幅と高さを取得

    // ブロック分割。縦・横方向に block_size ごとに分割し、「(x,y)＝ブロックの左上座標」のリストを作ってる。
    let blocks: Vec<EncryptedBlock> = (0..height)
        .step_by(block_size as usize)
        .flat_map(|y| { //flat_map で全体のブロック座標を列挙
            (0..width)
                .step_by(block_size as usize)
                .map(move |x| (x, y))
        })

        //並列で各ブロックを暗号化
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|(x, y)| {
            let mut block_pixels = Vec::new();

            //各ブロック内のピクセル暗号化
            for j in y..(y + block_size).min(height) {
                for i in x..(x + block_size).min(width) {
                    let pixel = img.get_pixel(i, j);
                    for c in pixel.0 {
                        block_pixels.push(client_key.encrypt(u64::from(c)));
                    }
                }
            }
            //ブロックごとの暗号化ピクセルをEncryptedBlockにしてベクターで返す
            EncryptedBlock { data: block_pixels }
        })
        .collect();

    //暗号化済のブロックを返す
    blocks
}

///クライアント鍵とサーバ鍵を作る関数
pub fn create_keys() -> (ClientKey, ServerKey) {
    //暗号化の設定をカスタマイズ（今回は uint4型を使う設定）
    let config = ConfigBuilder::all_disabled().enable_default_uint4().build();
    //クライアント鍵とそれに対応したサーバ鍵を作成
    let client_key = ClientKey::generate(config);
    let server_key = ServerKey::generate(&client_key);
    (client_key, server_key)
}
