use web_sys::CanvasRenderingContext2d;
use web_sys::ImageBitmap;

use crate::input::KeyDown;
use crate::math::Vec2;
pub struct Player {
    width: f64,                            // 描画サイズの幅 [pixel]
    height: f64,                           // 描画サイズの高さ [pixel]
    pos: Vec2,                             // 移動後の中心位置
    pre_pos: Vec2,                         // 前回描画時の中心位置
    pub image_front: Option<ImageBitmap>,  // 表画像
    pub image_shadow: Option<ImageBitmap>, // 影画像
}

impl Player {
    // 仮の値を返す
    pub fn empty() -> Self {
        Player {
            width: 0.,
            height: 0.,
            pos: Vec2 { x: 0., y: 0. },
            pre_pos: Vec2 { x: 0., y: 0. },
            image_front: None,
            image_shadow: None,
        }
    }
    pub fn new(pos: Vec2, image_front: ImageBitmap, image_shadow: ImageBitmap) -> Self {
        Player {
            width: image_front.width() as f64 * 3.,
            height: image_front.height() as f64 * 3.,
            pos,
            pre_pos: pos,
            image_front: Some(image_front),
            image_shadow: Some(image_shadow),
        }
    }
    pub fn update(&mut self, input_key: &KeyDown, canvas_width: f64) {
        // 一回(1フレーム)の移動距離
        let distance = 5.;
        if input_key.left && 0. < self.pos.x - self.width / 2. - distance {
            self.pos.x -= distance;
        }
        if input_key.right && self.pos.x + self.width / 2. + distance < canvas_width {
            self.pos.x += distance;
        }
    }
    pub fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        // 影画像(前回の部分を消す)
        ctx.draw_image_with_image_bitmap_and_dw_and_dh(
            &self.image_shadow.as_ref().unwrap(),
            self.pre_pos.x - self.width / 2.,
            self.pre_pos.y - self.height / 2.,
            self.width,
            self.height,
        )
        .unwrap();
        // 表画像
        ctx.draw_image_with_image_bitmap_and_dw_and_dh(
            &self.image_front.as_ref().unwrap(),
            self.pos.x - self.width / 2.,
            self.pos.y - self.height / 2.,
            self.width,
            self.height,
        )
        .unwrap();
        // 位置更新
        self.pre_pos = self.pos;
    }
}
