use crate::dot_data::{set_color, Color};
use crate::math::Vec2;
use std::ops::Deref;
use web_sys::ImageData;

/// 指定のピクセル値と、ImageData内の指定の座標のピクセル値が一致するか判定する
///
/// * `image_width` - image_dataの幅
/// * `pos` - image_data内の座標
/// * `color` - 比較対象の色
/// * `image_data` - 比較したいピクセルを含むImageData
pub fn detect_pixel_diff(
    canvas_width: f64,
    pos: Vec2,
    color: Color,
    image_data: ImageData,
) -> bool {
    let image_data = image_data.data();
    let rgba_map = image_data.deref();
    // 比較対象の色のrgba
    let rgba = set_color(color);
    // 指定座標のピクセル値と比較対象のピクセル値を比較して判定
    let index = (pos.y * canvas_width + pos.x) as usize * 4;

    rgba_map[index] == rgba[0]
        && rgba_map[index + 1] == rgba[1]
        && rgba_map[index + 2] == rgba[2]
        && rgba_map[index + 3] == rgba[3]
}
