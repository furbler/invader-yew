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

mod dot_data;
mod enemy;
mod input;
mod load_image;
mod math;
mod player;

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
    callback: Closure<dyn FnMut()>,
    input_key_down: Rc<RefCell<input::KeyDown>>,
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
            // まだ画像読み込みをしていないため、仮の値を入れる
            player: Player::default(),
            enemy_manage: EnemyManage::default(),
            callback,
            input_key_down: Rc::new(RefCell::new(input::KeyDown {
                left: false,
                right: false,
                shot: false,
            })),
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
                if image_type == ImageType::Player {
                    self.player.image = Some(image_bitmap);
                } else {
                    self.enemy_manage
                        .images_list
                        .insert(image_type, image_bitmap);
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
                    Vec2::new(canvas_width / 2., canvas_height - 120.),
                    self.player.image.clone().unwrap(),
                );

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
        // 画面全体クリア
        ctx.set_global_alpha(1.);
        // 描画確認のため背景はグレーにしておく
        ctx.set_fill_style(&JsValue::from("rgb(100,100,100)"));
        ctx.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
        // 画像のぼやけを防ぐ
        ctx.set_image_smoothing_enabled(false);

        log::info!("key down state = {:?}", self.input_key_down);
        // プレイヤーの処理
        self.player
            .update(&self.input_key_down.borrow(), canvas.width() as f64);
        self.player.render(&ctx);
        // 敵インベーダーの処理
        self.enemy_manage.update();
        self.enemy_manage.render(&ctx);

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
