use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{window, ImageBitmap, ImageData};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

use enemy::*;
use load_image::ImageType;
use math::Vec2;
use player::Player;
use ufo::Ufo;

mod dot_data;
mod enemy;
mod input;
mod load_image;
mod math;
mod player;
mod ufo;

pub enum Msg {
    // ビットマップ画像を取得
    RetBitmapImage(Vec<ImageType>, HashMap<ImageType, ImageData>, Vec<Vec<u8>>),
    // ビットマップ画像を保存
    RegisterImage(
        Vec<ImageType>,
        HashMap<ImageType, ImageData>,
        Vec<Vec<u8>>,
        (ImageType, ImageBitmap),
    ),
    Initialize,
    MainLoop,
}

struct AnimationCanvas {
    canvas: NodeRef,
    player: Player,
    enemy_manage: EnemyManage,
    torchika: Option<ImageBitmap>,
    ufo: Ufo,
    callback: Closure<dyn FnMut()>,
    input_key_down: Rc<RefCell<input::KeyDown>>,
    need_to_screen_init: bool, // 真ならば画面全体の初期化が必要
}

impl Component for AnimationCanvas {
    type Properties = ();
    type Message = Msg;
    fn create(ctx: &Context<Self>) -> Self {
        // 使用する画像のImageDataとその参照元の配列を取得
        let (image_data_list, image_rgb_list) = load_image::image_data_collect();
        // ビットマップ形式に変換
        ctx.link().send_future(async {
            Msg::RetBitmapImage(ImageType::ret_all_types(), image_data_list, image_rgb_list)
        });

        let comp_ctx = ctx.link().clone();
        let callback = Closure::wrap(
            Box::new(move || comp_ctx.send_message(Msg::MainLoop)) as Box<dyn FnMut()>
        );
        Self {
            canvas: NodeRef::default(),
            // まだ画像が未取得なので、仮の値を入れる
            player: Player::empty(),
            enemy_manage: EnemyManage::default(),
            torchika: None,
            callback,
            ufo: Ufo::empty(),
            input_key_down: Rc::new(RefCell::new(input::KeyDown {
                left: false,
                right: false,
                shot: false,
            })),
            need_to_screen_init: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // すべてのImageDataをビットマップ画像に変換するまで繰り返す
            Msg::RetBitmapImage(mut remain_image_type_list, mut image_data_list, _image_rgb) => {
                // ビットマップ画像へ未変換のImageDataが残っていた場合
                if let Some(image_type) = remain_image_type_list.pop() {
                    let image_data = image_data_list
                        .remove(&image_type)
                        .expect("要求する画像に対応するImageDataがありません。");
                    ctx.link().send_future(async {
                        let image_bitmap = load_image::imagedata2bitmap(image_data).await.unwrap();
                        Msg::RegisterImage(
                            remain_image_type_list,
                            image_data_list,
                            _image_rgb,
                            (image_type, image_bitmap),
                        )
                    });
                    true
                } else {
                    // すべての種類のキャラクター画像取得完了
                    ctx.link().send_message(Msg::Initialize);
                    true
                }
            }
            // 生成したビットマップ画像を登録
            Msg::RegisterImage(
                remain_image_type_list,
                image_data_list,
                _image_rgb,
                (image_type, image_bitmap),
            ) => {
                match image_type {
                    ImageType::Player => self.player.image_front = Some(image_bitmap),
                    ImageType::PlayerBullet => self.player.bullet.image_front = Some(image_bitmap),
                    ImageType::LandPlayerBulletFront => {
                        self.player.bullet.image_land_front = Some(image_bitmap)
                    }
                    ImageType::LandPlayerBulletShadow => {
                        self.player.bullet.image_land_shadow = Some(image_bitmap)
                    }
                    ImageType::Torchika => self.torchika = Some(image_bitmap),
                    ImageType::Ufo => self.ufo = Ufo::new(image_bitmap),
                    _ => {
                        self.enemy_manage
                            .images_list
                            .insert(image_type, image_bitmap);
                    }
                }
                ctx.link().send_message(Msg::RetBitmapImage(
                    remain_image_type_list,
                    image_data_list,
                    _image_rgb,
                ));
                true
            }
            // 初期化
            Msg::Initialize => {
                let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
                let (canvas_width, canvas_height) = (canvas.width() as f64, canvas.height() as f64);
                // 敵インベーダーの初期化
                self.enemy_manage.register_enemys(canvas_height);
                // プレイヤーの初期化
                self.player = Player::new(
                    Vec2::new(canvas_width / 2., canvas_height - 90.),
                    self.player.image_front.clone().unwrap(),
                    self.player.bullet.image_front.clone().unwrap(),
                    self.player.bullet.image_land_front.clone().unwrap(),
                    self.player.bullet.image_land_shadow.clone().unwrap(),
                );
                // キー入力情報初期化
                input::input_setup(&self.input_key_down);

                ctx.link().send_message(Msg::MainLoop);
                true
            }
            // ループ
            Msg::MainLoop => {
                self.main_loop();
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
                    width="800" height="600"
                    ref={self.canvas.clone()}
                    />
            </div>
        }
    }
}

impl AnimationCanvas {
    fn main_loop(&mut self) {
        let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
        let ctx: CanvasRenderingContext2d =
            canvas.get_context("2d").unwrap().unwrap().unchecked_into();
        let (canvas_width, canvas_height) = (canvas.width() as f64, canvas.height() as f64);
        ctx.set_global_alpha(1.);
        // 画面全体の初期化
        if self.need_to_screen_init {
            ctx.set_fill_style(&JsValue::from("rgb(0,0,0)"));
            ctx.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
            // プレイヤーの下に赤線を描く
            ctx.set_stroke_style(&JsValue::from("rgb(180,0,0)"));
            ctx.set_line_width(2.);
            ctx.begin_path();
            ctx.move_to(10., canvas_height - 40.);
            ctx.line_to(canvas_width - 10., canvas_height - 40.);
            ctx.stroke();
            // トーチカの描画サイズ
            let (torchika_width, torchika_height) = (
                self.torchika.as_ref().unwrap().width() as f64 * 3.,
                self.torchika.as_ref().unwrap().height() as f64 * 3.,
            );
            let torchika_start = canvas_width / 2. - 195.;
            // トーチカ描画
            for i in 0..4 {
                ctx.draw_image_with_image_bitmap_and_dw_and_dh(
                    &self.torchika.as_ref().unwrap(),
                    torchika_start + 130. * i as f64 - torchika_width / 2.,
                    canvas_height - 180.,
                    torchika_width,
                    torchika_height,
                )
                .unwrap();
            }
            // 初期化は最初のみ
            self.need_to_screen_init = false;
        }
        // 画像のぼやけを防ぐ
        ctx.set_image_smoothing_enabled(false);

        // プレイヤーの処理
        self.player
            .update(&ctx, &self.input_key_down.borrow(), canvas_width);
        // 敵インベーダーの処理
        self.enemy_manage.update(&ctx, &mut self.player.bullet);
        // UFOの処理
        self.ufo.update(&ctx, canvas_width);

        self.player.render(&ctx);
        self.enemy_manage.render(&ctx);
        self.ufo.render(&ctx);

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

fn main() {
    // デバッグ出力用
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}

// 指定した範囲を背景色で塗りつぶす
fn draw_background_rect(ctx: &CanvasRenderingContext2d, x: f64, y: f64, width: f64, height: f64) {
    ctx.set_fill_style(&JsValue::from("rgb(0,0,0)"));
    ctx.fill_rect(x, y, width, height);
}
