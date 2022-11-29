use web_sys::CanvasRenderingContext2d;
use web_sys::ImageBitmap;

use crate::input::KeyDown;
use crate::math::Vec2;
pub struct Player {
    width: f64,  // 描画サイズの幅 [pixel]
    height: f64, // 描画サイズの高さ [pixel]
    pos: Vec2,   // 中心位置
    pub image: Option<ImageBitmap>,
}

impl Player {
    pub fn default() -> Self {
        Player {
            width: 0.,
            height: 0.,
            pos: Vec2 { x: 0., y: 0. },
            image: None,
        }
    }
    pub fn new(pos: Vec2, image: ImageBitmap) -> Self {
        Player {
            width: image.width() as f64 * 3.,
            height: image.height() as f64 * 3.,
            pos,
            image: Some(image),
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
    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        ctx.draw_image_with_image_bitmap_and_dw_and_dh(
            &self.image.as_ref().unwrap(),
            self.pos.x - self.width / 2.,
            self.pos.y - self.height / 2.,
            self.width,
            self.height,
        )
        .unwrap();
    }
}
