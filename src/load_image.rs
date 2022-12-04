use std::collections::HashMap;
use wasm_bindgen::{Clamped, JsCast, JsValue};
use web_sys::{window, ImageBitmap, ImageData};

use crate::dot_data;

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum ImageType {
    PlayerFront,
    OctopusOpenFront,
    OctopusCloseFront,
    CrabBanzaiFront,
    CrabDownFront,
    SquidOpenFront,
    SquidCloseFront,
    PlayerBulletFront,

    ExplosionTurquoise,
    ExplosionPurple,
    ExpolsionGreen,
    ExplosionShadow,
    LandPlayerBulletFront,
    LandPlayerBulletShadow,
    Torchika,
}
// すべての画像タイプをまとめて返す
impl ImageType {
    pub fn ret_all_types() -> Vec<ImageType> {
        vec![
            ImageType::PlayerFront,
            ImageType::CrabBanzaiFront,
            ImageType::CrabDownFront,
            ImageType::OctopusOpenFront,
            ImageType::OctopusCloseFront,
            ImageType::SquidOpenFront,
            ImageType::SquidCloseFront,
            ImageType::PlayerBulletFront,
            ImageType::ExplosionTurquoise,
            ImageType::ExplosionPurple,
            ImageType::ExpolsionGreen,
            ImageType::ExplosionShadow,
            ImageType::LandPlayerBulletFront,
            ImageType::LandPlayerBulletShadow,
            ImageType::Torchika,
        ]
    }
}
// すべての画像のImageDataをまとめて返す
pub fn image_data_collect() -> (HashMap<ImageType, ImageData>, Vec<Vec<u8>>) {
    let mut all_image_list = ImageDataList {
        image_data_list: HashMap::new(),
        image_rgba_list: Vec::new(),
    };
    all_image_list.ret_image_data("player", ImageType::PlayerFront, "TURQUOISE");
    all_image_list.ret_image_data("crab_banzai", ImageType::CrabBanzaiFront, "TURQUOISE");
    all_image_list.ret_image_data("crab_down", ImageType::CrabDownFront, "TURQUOISE");
    all_image_list.ret_image_data("octopus_open", ImageType::OctopusOpenFront, "PURPLE");
    all_image_list.ret_image_data("octopus_close", ImageType::OctopusCloseFront, "PURPLE");
    all_image_list.ret_image_data("squid_open", ImageType::SquidOpenFront, "GREEN");
    all_image_list.ret_image_data("squid_close", ImageType::SquidCloseFront, "GREEN");
    all_image_list.ret_image_data("player_bullet", ImageType::PlayerBulletFront, "TURQUOISE");

    all_image_list.ret_image_data("explosion", ImageType::ExplosionTurquoise, "TURQUOISE");
    all_image_list.ret_image_data("explosion", ImageType::ExplosionPurple, "PURPLE");
    all_image_list.ret_image_data("explosion", ImageType::ExpolsionGreen, "GREEN");
    all_image_list.ret_image_data("explosion", ImageType::ExplosionShadow, "BACKGROUND");

    all_image_list.ret_image_data(
        "land_player_bullet",
        ImageType::LandPlayerBulletFront,
        "RED",
    );
    all_image_list.ret_image_data(
        "land_player_bullet",
        ImageType::LandPlayerBulletShadow,
        "BACKGROUND",
    );

    all_image_list.ret_image_data("torchika", ImageType::Torchika, "RED");

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
