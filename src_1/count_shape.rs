
//画像型 DynamicImage（カラー画像全般）と GrayImage（モノクロ画像）を使う
use image::{DynamicImage, GrayImage};
//imageproc ライブラリから「輪郭検出」に使う関数や型を読み込む(find_contours：輪郭を見つける関数
// BorderType::Outer：外側の輪郭だけを見る
// Contour：輪郭そのもの（点の集まり）)
use imageproc::contours::{find_contours, BorderType, Contour};

//ユーザが選んだ領域から輪郭を検出
//img はモノクロ画像、rect は (x, y, w, h) の矩形範囲（ユーザが選んだ領域）
fn detect_shape(img: &GrayImage, rect: (u32, u32, u32, u32)) -> usize {
    //四つの値（x: 左上x, y: 左上y, w: 横幅, h: 高さ）に分解
    let (x, y, w, h) = rect;
    //画像から指定された矩形だけを切り出して部分画像 sub_image にする
    let sub_image = image::imageops::crop_imm(img, x, y, w, h).to_image();
    //sub_image に対して輪郭を検出！u8：画像のピクセル値が8bit、Outer：外枠だけを検出
    let contours = find_contours::<u8>(&sub_image, BorderType::Outer);
    //各輪郭の「長さ（点の数）」を調べて、一番長い輪郭の長さを返す（該当する輪郭がなければ 0 を返す）
    contours
        .iter()
        .map(Contour::len)
        .max()
        .unwrap_or(0)
}

///全体画像から上で検出した輪郭を検出
//img：元のカラー画像、rect：選択範囲の四角 (x, y, w, h)
//count_same_shapeは選択範囲と「同じ形状（同じ長さの輪郭）」を持つ物体を数える関数
pub fn count_same_shape(img: &DynamicImage, rect: (u32, u32, u32, u32)) -> u32 {
    //画像をグレースケール（輝度画像）に変換。輪郭検出はモノクロでやる
    let gray = img.to_luma8();
    //選択範囲の中で一番大きい輪郭の「点の数（形状の特徴）」を取得。例：角が8つあれば「正八角形っぽい」とみなせる
    let ref_sides = detect_shape(&gray, rect);
    //全体画像に対して輪郭検出（モノクロ処理）
    let contours = find_contours::<u8>(&gray, BorderType::Outer);
    //同じ形の輪郭を数える！
    contours
        .iter()
        .filter(|c| c.len() == ref_sides)
        .count() as u32
}
