use std::collections::HashMap;
use wasm_bindgen::{Clamped, JsCast, JsValue};
use web_sys::{window, ImageBitmap, ImageData};

use crate::dot_data;

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum ImageType {
    Player,
    OctopusOpen,
    OctopusClose,
    CrabBanzai,
    CrabDown,
    SquidOpen,
    SquidClose,
}
// すべての画像タイプをまとめて返す
impl ImageType {
    pub fn ret_all_types() -> Vec<ImageType> {
        vec![
            ImageType::Player,
            ImageType::CrabBanzai,
            ImageType::CrabDown,
            ImageType::OctopusOpen,
            ImageType::OctopusClose,
            ImageType::SquidOpen,
            ImageType::SquidClose,
        ]
    }
}

pub fn image_data_collect() -> (HashMap<ImageType, ImageData>, Vec<Vec<u8>>) {
    let mut image_data_list: HashMap<ImageType, ImageData> = HashMap::new();
    // ダングリング防止のため、対応するImageDataがある間は保存する
    let mut image_rgb_list = Vec::new();

    let image_dot = dot_data::ret_dot_data("player");
    let image_rgba = image_dot.create_color_dot_map("TURQUOISE");
    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&image_rgba),
        image_dot.width,
        image_dot.height,
    )
    .unwrap();
    image_data_list.insert(ImageType::Player, image_data);
    image_rgb_list.push(image_rgba);

    let image_dot = dot_data::ret_dot_data("crab_banzai");
    let image_rgba = image_dot.create_color_dot_map("TURQUOISE");
    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&image_rgba),
        image_dot.width,
        image_dot.height,
    )
    .unwrap();
    image_data_list.insert(ImageType::CrabBanzai, image_data);
    image_rgb_list.push(image_rgba);

    let image_dot = dot_data::ret_dot_data("crab_down");
    let image_rgba = image_dot.create_color_dot_map("TURQUOISE");
    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&image_rgba),
        image_dot.width,
        image_dot.height,
    )
    .unwrap();
    image_data_list.insert(ImageType::CrabDown, image_data);
    image_rgb_list.push(image_rgba);

    let image_dot = dot_data::ret_dot_data("octopus_open");
    let image_rgba = image_dot.create_color_dot_map("PURPLE");
    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&image_rgba),
        image_dot.width,
        image_dot.height,
    )
    .unwrap();
    image_data_list.insert(ImageType::OctopusOpen, image_data);
    image_rgb_list.push(image_rgba);

    let image_dot = dot_data::ret_dot_data("octopus_close");
    let image_rgba = image_dot.create_color_dot_map("PURPLE");
    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&image_rgba),
        image_dot.width,
        image_dot.height,
    )
    .unwrap();
    image_data_list.insert(ImageType::OctopusClose, image_data);
    image_rgb_list.push(image_rgba);

    let image_dot = dot_data::ret_dot_data("squid_open");
    let image_rgba = image_dot.create_color_dot_map("GREEN");
    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&image_rgba),
        image_dot.width,
        image_dot.height,
    )
    .unwrap();
    image_data_list.insert(ImageType::SquidOpen, image_data);
    image_rgb_list.push(image_rgba);

    let image_dot = dot_data::ret_dot_data("squid_close");
    let image_rgba = image_dot.create_color_dot_map("GREEN");
    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&image_rgba),
        image_dot.width,
        image_dot.height,
    )
    .unwrap();
    image_data_list.insert(ImageType::SquidClose, image_data);
    image_rgb_list.push(image_rgba);

    (image_data_list, image_rgb_list)
}

// ImageData画像をImageBitmap画像に変換
pub async fn imagedata2bitmap(image_data: ImageData) -> Result<ImageBitmap, JsValue> {
    let promise = window()
        .unwrap()
        .create_image_bitmap_with_image_data(&image_data);
    let result = wasm_bindgen_futures::JsFuture::from(promise.unwrap())
        .await
        .unwrap();

    Ok(result.dyn_into::<ImageBitmap>()?)
}
