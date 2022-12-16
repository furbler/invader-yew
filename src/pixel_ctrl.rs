use crate::dot_data::{set_color, Color};
use crate::math::Vec2;
use std::ops::Deref;
use web_sys::ImageData;

/// posで指定した複数の指定座標の中に、colorで指定した色のどれか一つでも一致する箇所があれば真を返す
///
/// * `image_width` - image_dataの幅
/// * `pos_list` - 比較するimage_data内の座標
/// * `colors` - 比較対象の色
/// * `image_data` - 比較したいピクセルを含むImageData
pub fn detect_pixel_diff(
    canvas_width: f64,
    pos_list: Vec<Vec2>,
    colors: Vec<Color>,
    image_data: ImageData,
) -> bool {
    let image_data = image_data.data();
    let rgba_map = image_data.deref();
    let mut result = false;
    for c in colors {
        // 比較対象の色のrgba
        let rgba = set_color(c);
        for pos in &pos_list {
            // 指定座標のピクセル値と比較対象のピクセル値を比較して判定
            let index = (pos.y * canvas_width + pos.x) as usize * 4;
            let equal = rgba_map[index] == rgba[0]
                && rgba_map[index + 1] == rgba[1]
                && rgba_map[index + 2] == rgba[2]
                && rgba_map[index + 3] == rgba[3];
            result = result || equal;
        }
    }
    result
}
