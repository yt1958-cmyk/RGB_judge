<<<<<<< HEAD
use std::fs::File;
use std::io::Read;
use std::process::Command;

use image::GenericImageView;

mod encrypt_image;
use serde_json;
mod count_rgb;
mod count_shape;
use encrypt_image::{create_keys, encrypt_image, merge_encrypted_blocks};
use count_rgb::count_rgb_objects;
use count_shape::count_same_shape_fhe;

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

    let img = image::open(img_path).expect("cannot open image");
    let (client_key, server_key) = create_keys();

    // Encrypt image in blocks
    let blocks = encrypt_image(&img, block_size, &client_key);
    // Merge blocks back into full encrypted image for analysis
    let enc_img = merge_encrypted_blocks(&blocks, img.width(), img.height(), &client_key);

    // Reference color (center pixel of selected region)
    let cx = x + w / 2;
    let cy = y + h / 2;
    let ref_pixel = img.get_pixel(cx, cy);
    let ref_rgb = [
        client_key.encrypt(u64::from(ref_pixel[0])),
        client_key.encrypt(u64::from(ref_pixel[1])),
        client_key.encrypt(u64::from(ref_pixel[2])),
    ];

    let rgb_count = count_rgb_objects(&enc_img, ref_rgb, &client_key, &server_key);
    let shape_count = count_same_shape_fhe(&img, (x, y, w, h), &client_key, &server_key);

    println!(
        "画像の中に、ユーザが選択した物体と同じRGB値の物体は{}個含まれています",
        rgb_count
    );
    println!(
        "この画像の中に、ユーザが指定した物体と同じ形のものは{}個含まれています",
        shape_count
    );
}
=======
use std::fs::File;
use std::io::Read;
use std::process::Command;

use image::GenericImageView;

mod encrypt_image;
use serde_json;
mod count_rgb;
mod count_shape;
use encrypt_image::{create_keys, encrypt_image, merge_encrypted_blocks};
use count_rgb::count_rgb_objects;
use count_shape::{count_same_shape, count_same_shape_fhe};

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

    let img = image::open(img_path).expect("cannot open image");
    let (client_key, server_key) = create_keys();

    // Encrypt image in blocks
    let blocks = encrypt_image(&img, block_size, &client_key);
    // Merge blocks back into full encrypted image for analysis
    let enc_img = merge_encrypted_blocks(&blocks, img.width(), img.height(), &client_key);

    // Reference color (center pixel of selected region)
    let cx = x + w / 2;
    let cy = y + h / 2;
    let ref_pixel = img.get_pixel(cx, cy);
    let ref_rgb = [
        client_key.encrypt(u64::from(ref_pixel[0])),
        client_key.encrypt(u64::from(ref_pixel[1])),
        client_key.encrypt(u64::from(ref_pixel[2])),
    ];

    let rgb_count = count_rgb_objects(&enc_img, ref_rgb, &client_key, &server_key);
    let shape_count = count_same_shape_fhe(&img, (x, y, w, h), &client_key, &server_key);

    println!(
        "画像の中に、ユーザが選択した物体と同じRGB値の物体は{}個含まれています",
        rgb_count
    );
    println!(
        "この画像の中に、ユーザが指定した物体と同じ形のものは{}個含まれています",
        shape_count
    );
}
>>>>>>> 2988ef4 (Save local changes before rebase)
