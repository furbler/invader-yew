use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::Clamped;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{window, ImageBitmap, ImageData};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

mod dot_data;

#[derive(Clone, Copy)]
struct Vec2 {
    x: f64,
    y: f64,
}
impl Vec2 {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

// 対応するキーが押されている時真
#[derive(Debug)]
struct KeyDown {
    left: bool,  // プレイヤーを左へ移動させる
    right: bool, // プレイヤーを右へ移動させる
    shot: bool,  // プレイヤーが弾を撃つ
}
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
    fn ret_all_types() -> Vec<ImageType> {
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

struct Player {
    width: f64,  // 描画サイズの幅 [pixel]
    height: f64, // 描画サイズの高さ [pixel]
    pos: Vec2,   // 中心位置
    image: Option<ImageBitmap>,
}

impl Player {
    fn update(&mut self, input_key: &KeyDown) {
        if input_key.left {
            self.pos.x -= 5.;
        }
        if input_key.right {
            self.pos.x += 5.;
        }
    }
    fn render(&self, ctx: &CanvasRenderingContext2d) {
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

struct Enemy {
    width: f64,            // 描画サイズの幅 [pixel]
    height: f64,           // 描画サイズの高さ [pixel]
    pos: Vec2,             // 中心位置
    move_turn: bool,       // 動くか否か
    show_image_type: bool, // どちらの状態の画像を表示するか
    image_type1: ImageBitmap,
    image_type2: ImageBitmap,
}

impl Enemy {
    fn update(&mut self, move_dir: i32) {
        if !self.move_turn {
            // 動く時以外は何もしない
            return;
        }

        // 方向を考慮して動く
        self.pos.x += 20. * move_dir as f64;
        // 表示する画像を切り替える
        self.show_image_type = !self.show_image_type
    }

    fn render(&self, ctx: &CanvasRenderingContext2d) {
        // 表示画像選択
        let show_image = if self.show_image_type {
            &self.image_type1
        } else {
            &self.image_type2
        };
        ctx.draw_image_with_image_bitmap_and_dw_and_dh(
            show_image,
            self.pos.x - self.width / 2.,
            self.pos.y - self.height / 2.,
            self.width,
            self.height,
        )
        .unwrap();
    }
}

struct EnemyManage {
    // 敵の左方向、右方向の移動範囲限界のx座標
    left_border: f64,
    right_border: f64,
    // 移動方向(右は1、左は-1)
    move_dir: i32,
    // 次フレームで移動方向を反転すべきか否か
    move_dir_invert: bool,
    images_list: HashMap<ImageType, ImageBitmap>,
    enemys_list: Vec<Enemy>,
}

impl EnemyManage {
    fn register_enemys(&mut self, canvas_height: f64) {
        let image_1 = self.images_list.get(&ImageType::OctopusOpen).unwrap();
        let image_2 = self.images_list.get(&ImageType::OctopusClose).unwrap();
        let invader_column = 11;

        let mut invader_pos = Vec2::new(100., canvas_height - 300.);
        for _ in 0..2 {
            for _ in 0..invader_column {
                self.enemys_list.push(Enemy {
                    width: image_1.width() as f64 * 3.,
                    height: image_1.height() as f64 * 3.,
                    pos: invader_pos,
                    move_turn: false,
                    show_image_type: true,
                    image_type1: image_1.clone(),
                    image_type2: image_2.clone(),
                });
                invader_pos.x += 50.;
            }
            invader_pos.x = 100.;
            invader_pos.y -= 50.;
        }

        let image_1 = self.images_list.get(&ImageType::CrabBanzai).unwrap();
        let image_2 = self.images_list.get(&ImageType::CrabDown).unwrap();
        for _ in 0..2 {
            for _ in 0..invader_column {
                self.enemys_list.push(Enemy {
                    width: image_1.width() as f64 * 3.,
                    height: image_1.height() as f64 * 3.,
                    pos: invader_pos,
                    move_turn: false,
                    show_image_type: true,
                    image_type1: image_1.clone(),
                    image_type2: image_2.clone(),
                });
                invader_pos.x += 50.;
            }
            invader_pos.x = 100.;
            invader_pos.y -= 50.;
        }

        let image_1 = self.images_list.get(&ImageType::SquidOpen).unwrap();
        let image_2 = self.images_list.get(&ImageType::SquidClose).unwrap();
        for _ in 0..invader_column {
            self.enemys_list.push(Enemy {
                width: image_1.width() as f64 * 3.,
                height: image_1.height() as f64 * 3.,
                pos: invader_pos,
                move_turn: false,
                show_image_type: true,
                image_type1: image_1.clone(),
                image_type2: image_2.clone(),
            });
            invader_pos.x += 50.;
        }

        // 一番左下の敵インベーダーから動く
        self.enemys_list[0].move_turn = true;
    }
    fn update(&mut self) {
        // 各敵個体の移動処理
        self.enemys_list.iter_mut().for_each(|enemy| {
            enemy.update(self.move_dir);
        });

        // 移動した敵インベーダーの個体番号を取得
        let mut move_enemy_index = 0;
        for (index, enemy) in self.enemys_list.iter().enumerate() {
            if enemy.move_turn {
                move_enemy_index = index;
                break;
            }
        }
        // 動いた個体が制限範囲外に出た場合
        if self.enemys_list[move_enemy_index].pos.x < self.left_border
            || self.right_border < self.enemys_list[move_enemy_index].pos.x
        {
            // 移動方向反転フラグを立てる
            self.move_dir_invert = true;
        }

        // 移動する個体を変える
        self.enemys_list[move_enemy_index].move_turn = false;

        move_enemy_index += 1;
        if move_enemy_index >= self.enemys_list.len() {
            // 最後の個体だったら、最初の個体に戻る
            self.enemys_list[0].move_turn = true;
            // 移動方向反転フラグが立っている場合
            if self.move_dir_invert {
                // 移動方向を反転
                self.move_dir *= -1;
                // 移動方向反転フラグをリセット
                self.move_dir_invert = false;
            }
        } else {
            // 次の個体を動かす
            self.enemys_list[move_enemy_index].move_turn = true;
        }
    }
    fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        self.enemys_list.iter_mut().for_each(|enemy| {
            enemy.render(ctx);
        });
    }
}

struct AnimationCanvas {
    canvas: NodeRef,
    player: Player,
    enemy_manage: EnemyManage,
    callback: Closure<dyn FnMut()>,
    input_key_down: Rc<RefCell<KeyDown>>,
}

impl Component for AnimationCanvas {
    type Properties = ();
    type Message = Msg;
    fn create(ctx: &Context<Self>) -> Self {
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

        ctx.link().send_future(async {
            Msg::RetBitmapImage(ImageType::ret_all_types(), image_data_list, image_rgb_list)
        });

        let comp_ctx = ctx.link().clone();
        let callback = Closure::wrap(
            Box::new(move || comp_ctx.send_message(Msg::MainLoop)) as Box<dyn FnMut()>
        );
        Self {
            canvas: NodeRef::default(),
            player: Player {
                width: 0.,
                height: 0.,
                pos: Vec2 { x: 0., y: 0. },
                image: None,
            },
            enemy_manage: EnemyManage {
                left_border: 50.,
                right_border: 730.,
                move_dir: 1,
                move_dir_invert: false,
                images_list: HashMap::new(),
                enemys_list: Vec::new(),
            },
            callback,
            input_key_down: Rc::new(RefCell::new(KeyDown {
                left: false,
                right: false,
                shot: false,
            })),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // すべてのビットマップ画像取得まで繰り返す
            Msg::RetBitmapImage(mut image_type_list, mut image_data_list, _image_rgb) => {
                if let Some(image_type) = image_type_list.pop() {
                    let image_data = image_data_list
                        .remove(&image_type)
                        .expect("ドットデータが読み込まれていないキャラクターがいます。");
                    ctx.link().send_future(async {
                        let image_bitmap = imagedata2bitmap(image_data).await.unwrap();
                        Msg::RegisterImage(
                            image_type_list,
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
            // 生成したビットマップ画像を保存
            Msg::RegisterImage(
                image_type_list,
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
                    image_type_list,
                    image_data_list,
                    _image_rgb,
                ));
                true
            }
            // 初期化
            Msg::Initialize => {
                let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
                let (canvas_width, canvas_height) = (canvas.width() as f64, canvas.height() as f64);
                self.enemy_manage.register_enemys(canvas_height);

                let image_player = self.player.image.clone().unwrap();
                self.player = Player {
                    width: image_player.width() as f64 * 3.,
                    height: image_player.height() as f64 * 3.,
                    pos: Vec2::new(canvas_width / 2., canvas_height - 120.),
                    image: Some(image_player),
                };

                // キー入力処理
                let key_down = Rc::clone(&self.input_key_down);
                let document = web_sys::window().unwrap().document().unwrap();
                let body = document.body().unwrap();
                let canvas = document
                    .create_element("canvas")
                    .unwrap()
                    .dyn_into::<HtmlCanvasElement>()
                    .unwrap();
                body.append_child(&canvas).unwrap();
                // キー押し下げ
                let closure_key_down = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                    input_key_down(e, &mut key_down.borrow_mut());
                }) as Box<dyn FnMut(_)>);
                body.add_event_listener_with_callback(
                    "keydown",
                    closure_key_down.as_ref().unchecked_ref(),
                )
                .unwrap();
                closure_key_down.forget();

                // キー押し上げ
                let key_up = Rc::clone(&self.input_key_down);
                let closure_key_up = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                    input_key_up(e, &mut key_up.borrow_mut());
                }) as Box<dyn FnMut(_)>);
                body.add_event_listener_with_callback(
                    "keyup",
                    closure_key_up.as_ref().unchecked_ref(),
                )
                .unwrap();
                closure_key_up.forget();

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
        // ここで実行時エラーが起きないか不安
        self.player.update(&self.input_key_down.borrow());
        self.player.render(&ctx);

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

// ImageData画像をImageBitmap画像に変換
async fn imagedata2bitmap(image_data: ImageData) -> Result<ImageBitmap, JsValue> {
    let promise = window()
        .unwrap()
        .create_image_bitmap_with_image_data(&image_data);
    let result = wasm_bindgen_futures::JsFuture::from(promise.unwrap())
        .await
        .unwrap();

    Ok(result.dyn_into::<ImageBitmap>()?)
}

fn main() {
    // デバッグ出力用
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
// キー押し下げ時に呼び出し
fn input_key_down(event: web_sys::KeyboardEvent, input_data: &mut KeyDown) {
    match &*event.key() {
        "ArrowLeft" | "a" => {
            input_data.left = true;
        }
        "ArrowRight" | "d" => {
            input_data.right = true;
        }
        "Space" | "Enter" => {
            input_data.shot = true;
        }
        _ => (),
    };
}

// キー押し上げ時に呼び出し
fn input_key_up(event: web_sys::KeyboardEvent, input_data: &mut KeyDown) {
    match &*event.key() {
        "ArrowLeft" | "a" => {
            input_data.left = false;
        }
        "ArrowRight" | "d" => {
            input_data.right = false;
        }
        "Space" | "Enter" => {
            input_data.shot = false;
        }
        _ => (),
    };
}
