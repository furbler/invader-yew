use crate::math::Vec2;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

pub struct Title {
    pos: Vec2,
}

impl Title {
    pub fn new(canvas_width: f64, canvas_height: f64) -> Self {
        Title {
            // 大体キャンパスの中心上に設定
            pos: Vec2::new(canvas_width / 2., canvas_height / 4.),
        }
    }
    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_font("90px monospace");
        ctx.set_fill_style(&JsValue::from("rgba(200, 10, 10)"));
        ctx.fill_text("Invader", self.pos.x - 170., self.pos.y)
            .unwrap();

        ctx.set_font("40px monospace");
        ctx.fill_text("Press Enter", self.pos.x - 120., self.pos.y + 80.)
            .unwrap();
    }
}
