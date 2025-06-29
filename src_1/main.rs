use std::fs::File;
use std::io::Read;
use std::process::Command;

use image::GenericImageView;

mod encrypt_image;
use serde_json;
mod count_rgb;
mod count_shape;
use encrypt_image::{create_keys, encrypt_image};
use count_rgb::count_rgb;
use count_shape::count_same_shape;

fn main() {
    // Parse arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <image_path> [block_size]");
        return;
    }
    let img_path = &args[1];
    let block_size: u32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(10);

    // Call Python script for region selection
    let _ = Command::new("python3")
        .arg("select_image.py")
        .arg(img_path)
        .arg("selection.json")
        .status()
        .expect("failed to run python script");

    // Read selection
    let mut file = File::open("selection.json").expect("selection.json missing");
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let sel: serde_json::Value = serde_json::from_str(&buf).unwrap();
    let x = sel["x"].as_u64().unwrap() as u32;
    let y = sel["y"].as_u64().unwrap() as u32;
    let w = sel["w"].as_u64().unwrap() as u32;
    let h = sel["h"].as_u64().unwrap() as u32;

    //画像を読み込む
    let img = image::open(img_path).expect("cannot open image");
    //暗号化に必要なTFHEのクライアント鍵とサーバ鍵を生成
    let (client_key, server_key) = create_keys();

    //画像をブロック単位で分割し、ブロックごとに暗号化
    let blocks = encrypt_image(&img, block_size, &client_key);

    //ユーザが指定した左上ピクセルを取り出す
    let ref_pixel = img.get_pixel(x, y);

    //ユーザが選択したピクセルのRGB値をそれぞれTFHEで暗号化して保存
    let ref_rgb = [
        client_key.encrypt(u64::from(ref_pixel[0])),
        client_key.encrypt(u64::from(ref_pixel[1])),
        client_key.encrypt(u64::from(ref_pixel[2])),
    ];

    //暗号化された画像を使って「この色と同じ物体はいくつ？」を計算
    let rgb_count = count_rgb(&blocks, ref_rgb, &client_key, &server_key);


    //元画像を使って形が同じ物体を数える
    let shape_count = count_same_shape(&img, (x, y, w, h));

    println!(
        "画像の中に、ユーザが選択した物体と同じRGB値の物体は{}個含まれています",
        rgb_count
    );
    println!(
        "この画像の中に、ユーザが指定した物体と同じ形のものは{}個含まれています",
        shape_count
    );
}
