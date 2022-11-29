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
    let mut all_image_list = ImageDataList {
        image_data_list: HashMap::new(),
        image_rgba_list: Vec::new(),
    };
    all_image_list.ret_image_data("player", ImageType::Player, "TURQUOISE");
    all_image_list.ret_image_data("crab_banzai", ImageType::CrabBanzai, "TURQUOISE");
    all_image_list.ret_image_data("crab_down", ImageType::CrabDown, "TURQUOISE");
    all_image_list.ret_image_data("octopus_open", ImageType::OctopusOpen, "PURPLE");
    all_image_list.ret_image_data("octopus_close", ImageType::OctopusClose, "PURPLE");
    all_image_list.ret_image_data("squid_open", ImageType::SquidOpen, "GREEN");
    all_image_list.ret_image_data("squid_close", ImageType::SquidClose, "GREEN");
    (
        all_image_list.image_data_list,
        all_image_list.image_rgba_list,
    )
}

struct ImageDataList {
    image_data_list: HashMap<ImageType, ImageData>,
    // ダングリング防止のため、対応するImageDataがある間は保存する
    image_rgba_list: Vec<Vec<u8>>,
}

impl ImageDataList {
    fn ret_image_data(&mut self, name: &str, image_type: ImageType, color: &str) {
        let image_dot = dot_data::ret_dot_data(name);
        let image_rgba = image_dot.create_color_dot_map(color);
        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&image_rgba),
            image_dot.width,
            image_dot.height,
        )
        .unwrap();

        self.image_data_list.insert(image_type, image_data);
        self.image_rgba_list.push(image_rgba);
    }
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
