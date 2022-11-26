use crate::dot_data::dot_data;
use wasm_bindgen::Clamped;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{window, ImageBitmap, ImageData};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

mod dot_data;

struct Vec2 {
    x: f64,
    y: f64,
}
impl Vec2 {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

pub enum Msg {
    OneImageOk(Vec<ImageData>, Vec<Vec<u8>>),
    Register(ImageBitmap, Vec<ImageData>, Vec<Vec<u8>>),
    Render,
}

struct Enemy {
    width: f64,  // 描画サイズの幅 [pixel]
    height: f64, // 描画サイズの高さ [pixel]
    pos: Vec2,   // 中心位置
    image: ImageBitmap,
}

impl Enemy {
    fn render(&self, ctx: &CanvasRenderingContext2d) {
        ctx.draw_image_with_image_bitmap_and_dw_and_dh(
            &self.image,
            self.pos.x - self.width / 2.,
            self.pos.y - self.height / 2.,
            self.width,
            self.height,
        )
        .unwrap();
    }
}

struct AnimationCanvas {
    canvas: NodeRef,
    enemys_list: Vec<Enemy>,
    callback: Closure<dyn FnMut()>,
}

impl Component for AnimationCanvas {
    type Properties = ();
    type Message = Msg;
    fn create(ctx: &Context<Self>) -> Self {
        let mut image_data_list = Vec::new();
        let mut image_rgb_list = Vec::new();

        let image_dot = dot_data("crab_banzai");
        let image_rgba = image_dot.create_color_dot_map("TURQUOISE");
        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&image_rgba),
            image_dot.width,
            image_dot.height,
        )
        .unwrap();
        image_data_list.push(image_data);
        image_rgb_list.push(image_rgba);

        let image_dot = dot_data("octopus_open");
        let image_rgba = image_dot.create_color_dot_map("PURPLE");
        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&image_rgba),
            image_dot.width,
            image_dot.height,
        )
        .unwrap();
        image_data_list.push(image_data);
        image_rgb_list.push(image_rgba);

        let image_dot = dot_data("squid_open");
        let image_rgba = image_dot.create_color_dot_map("GREEN");
        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&image_rgba),
            image_dot.width,
            image_dot.height,
        )
        .unwrap();
        image_data_list.push(image_data);
        image_rgb_list.push(image_rgba);

        ctx.link()
            .send_future(async { Msg::OneImageOk(image_data_list, image_rgb_list) });

        let comp_ctx = ctx.link().clone();
        let callback =
            Closure::wrap(Box::new(move || comp_ctx.send_message(Msg::Render)) as Box<dyn FnMut()>);
        Self {
            canvas: NodeRef::default(),
            enemys_list: vec![],
            callback,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // 1枚のビットマップ画像取得完了
            Msg::OneImageOk(mut image_data_list, _image_rgb) => {
                if let Some(image_data) = image_data_list.pop() {
                    ctx.link().send_future(async {
                        let image_bitmap = imagedata2bitmap(image_data).await.unwrap();
                        Msg::Register(image_bitmap, image_data_list, _image_rgb)
                    });
                    true
                } else {
                    ctx.link().send_message(Msg::Render);
                    true
                }
            }
            Msg::Register(image_bitmap, image_data_list, image_rgb) => {
                self.enemys_list.push(Enemy {
                    width: image_bitmap.width() as f64 * 3.,
                    height: image_bitmap.height() as f64 * 3.,
                    pos: Vec2::new(200. + 50. * image_data_list.len() as f64, 50.),
                    image: image_bitmap,
                });
                ctx.link()
                    .send_message(Msg::OneImageOk(image_data_list, image_rgb));
                true
            }
            // 描画処理
            Msg::Render => {
                self.render();
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
            // キャンバスのサイズはここで指定
                <canvas
                    id="canvas"
                    width="600" height="400"
                    ref={self.canvas.clone()}
                    />
            </div>
        }
    }
}

impl AnimationCanvas {
    fn render(&mut self) {
        let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
        let ctx: CanvasRenderingContext2d =
            canvas.get_context("2d").unwrap().unwrap().unchecked_into();
        // 画面全体クリア
        ctx.set_global_alpha(1.);
        // 描画確認のため背景はグレーにしておく
        ctx.set_fill_style(&JsValue::from("rgb(100,100,100)"));
        ctx.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
        // 画像のぼやけを防ぐ
        ctx.set_image_smoothing_enabled(false);
        self.enemys_list.iter_mut().for_each(|enemy| {
            enemy.render(&ctx);
        });

        log::info!(
            "canvas width() = {}, canvas height() = {}",
            canvas.width(),
            canvas.height()
        );

        window()
            .unwrap()
            .request_animation_frame(self.callback.as_ref().unchecked_ref())
            .unwrap();
    }
}

#[function_component(App)]
fn app_body() -> Html {
    html! {
        <>
            <AnimationCanvas />
        </>
    }
}

// ImageData画像をImageBitmap画像に変換
async fn imagedata2bitmap(image_data: ImageData) -> Result<ImageBitmap, JsValue> {
    let promise = window()
        .unwrap()
        .create_image_bitmap_with_image_data(&image_data);
    let result = wasm_bindgen_futures::JsFuture::from(promise.unwrap())
        .await
        .unwrap();

    Ok(result.dyn_into::<web_sys::ImageBitmap>()?)
}

fn main() {
    // デバッグ出力用
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
