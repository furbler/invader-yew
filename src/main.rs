use std::collections::HashMap;

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
struct Enemy {
    width: f64,      // 描画サイズの幅 [pixel]
    height: f64,     // 描画サイズの高さ [pixel]
    pos: Vec2,       // 中心位置
    move_turn: bool, // 動くか否か
    image: ImageBitmap,
}

impl Enemy {
    fn update(&mut self) {
        if self.move_turn {
            self.pos.x += 30.;
        }
    }

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

pub enum Msg {
    // ビットマップ画像を取得
    RetBitmapImage(Vec<EnemyType>, HashMap<EnemyType, ImageData>, Vec<Vec<u8>>),
    // ビットマップ画像を保存
    RegisterImage(
        Vec<EnemyType>,
        HashMap<EnemyType, ImageData>,
        Vec<Vec<u8>>,
        (EnemyType, ImageBitmap),
    ),
    RegisterCharacter,
    Render,
}
#[derive(Eq, Hash, PartialEq, Clone)]
pub enum EnemyType {
    Crab,
    Octopus,
    Squid,
}

impl EnemyType {
    fn ret_all_types() -> Vec<EnemyType> {
        vec![EnemyType::Crab, EnemyType::Octopus, EnemyType::Squid]
    }
}

struct EnemyManage {
    images_list: HashMap<EnemyType, ImageBitmap>,
    enemys_list: Vec<Enemy>,
}

impl EnemyManage {
    fn register_enemys(&mut self, canvas_height: f64) {
        let image = self.images_list.get(&EnemyType::Octopus).unwrap();
        let invader_column = 11;

        let mut invader_pos = Vec2::new(100., canvas_height - 300.);
        for _ in 0..2 {
            for _ in 0..invader_column {
                self.enemys_list.push(Enemy {
                    width: image.width() as f64 * 3.,
                    height: image.height() as f64 * 3.,
                    pos: invader_pos,
                    move_turn: false,
                    image: image.clone(),
                });
                invader_pos.x += 50.;
            }
            invader_pos.x = 100.;
            invader_pos.y -= 50.;
        }

        let image = self.images_list.get(&EnemyType::Crab).unwrap();
        for _ in 0..2 {
            for _ in 0..invader_column {
                self.enemys_list.push(Enemy {
                    width: image.width() as f64 * 3.,
                    height: image.height() as f64 * 3.,
                    pos: invader_pos,
                    move_turn: false,
                    image: image.clone(),
                });
                invader_pos.x += 50.;
            }
            invader_pos.x = 100.;
            invader_pos.y -= 50.;
        }

        let image = self.images_list.get(&EnemyType::Squid).unwrap();
        for _ in 0..invader_column {
            self.enemys_list.push(Enemy {
                width: image.width() as f64 * 3.,
                height: image.height() as f64 * 3.,
                pos: invader_pos,
                move_turn: false,
                image: image.clone(),
            });
            invader_pos.x += 50.;
        }

        // 一番左下の敵インベーダーから動く
        self.enemys_list[0].move_turn = true;
    }
    fn update(&mut self) {
        // 移動した敵インベーダーの個体番号を取得
        let mut move_enemy_index = 0;
        for (index, enemy) in self.enemys_list.iter().enumerate() {
            if enemy.move_turn {
                move_enemy_index = index;
                break;
            }
        }

        // 移動する個体を変える
        self.enemys_list[move_enemy_index].move_turn = false;

        move_enemy_index += 1;
        if move_enemy_index >= self.enemys_list.len() {
            // 最後の個体だったら、最初の個体に戻る
            self.enemys_list[0].move_turn = true;
        } else {
            // 次の個体を動かす
            self.enemys_list[move_enemy_index].move_turn = true;
        }
    }
}

struct AnimationCanvas {
    canvas: NodeRef,
    enemy_manage: EnemyManage,
    callback: Closure<dyn FnMut()>,
}

impl Component for AnimationCanvas {
    type Properties = ();
    type Message = Msg;
    fn create(ctx: &Context<Self>) -> Self {
        let mut image_data_list: HashMap<EnemyType, ImageData> = HashMap::new();
        // ダングリング防止のため、対応するImageDataがある間は保存する
        let mut image_rgb_list = Vec::new();

        let image_dot = dot_data::ret_dot_data("crab_banzai");
        let image_rgba = image_dot.create_color_dot_map("TURQUOISE");
        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&image_rgba),
            image_dot.width,
            image_dot.height,
        )
        .unwrap();
        image_data_list.insert(EnemyType::Crab, image_data);
        image_rgb_list.push(image_rgba);

        let image_dot = dot_data::ret_dot_data("octopus_open");
        let image_rgba = image_dot.create_color_dot_map("PURPLE");
        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&image_rgba),
            image_dot.width,
            image_dot.height,
        )
        .unwrap();
        image_data_list.insert(EnemyType::Octopus, image_data);
        image_rgb_list.push(image_rgba);

        let image_dot = dot_data::ret_dot_data("squid_open");
        let image_rgba = image_dot.create_color_dot_map("GREEN");
        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&image_rgba),
            image_dot.width,
            image_dot.height,
        )
        .unwrap();
        image_data_list.insert(EnemyType::Squid, image_data);
        image_rgb_list.push(image_rgba);

        ctx.link().send_future(async {
            Msg::RetBitmapImage(EnemyType::ret_all_types(), image_data_list, image_rgb_list)
        });

        let comp_ctx = ctx.link().clone();
        let callback =
            Closure::wrap(Box::new(move || comp_ctx.send_message(Msg::Render)) as Box<dyn FnMut()>);
        Self {
            canvas: NodeRef::default(),
            enemy_manage: EnemyManage {
                images_list: HashMap::new(),
                enemys_list: Vec::new(),
            },
            callback,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // すべてのビットマップ画像取得まで繰り返す
            Msg::RetBitmapImage(mut enemy_type_list, mut image_data_list, _image_rgb) => {
                if let Some(enemy_type) = enemy_type_list.pop() {
                    let image_data = image_data_list
                        .remove(&enemy_type)
                        .expect("ドットデータが読み込まれていないキャラクターがいます。");
                    ctx.link().send_future(async {
                        let image_bitmap = imagedata2bitmap(image_data).await.unwrap();
                        Msg::RegisterImage(
                            enemy_type_list,
                            image_data_list,
                            _image_rgb,
                            (enemy_type, image_bitmap),
                        )
                    });
                    true
                } else {
                    // すべての種類のキャラクター画像取得完了
                    ctx.link().send_message(Msg::RegisterCharacter);
                    true
                }
            }
            // 生成したビットマップ画像を保存
            Msg::RegisterImage(
                enemy_type_list,
                image_data_list,
                _image_rgb,
                (enemy_type, image_bitmap),
            ) => {
                self.enemy_manage
                    .images_list
                    .insert(enemy_type, image_bitmap);

                ctx.link().send_message(Msg::RetBitmapImage(
                    enemy_type_list,
                    image_data_list,
                    _image_rgb,
                ));
                true
            }
            // 敵キャラクターを生成
            Msg::RegisterCharacter => {
                let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
                let canvas_height = canvas.height() as f64;
                self.enemy_manage.register_enemys(canvas_height);

                ctx.link().send_message(Msg::Render);
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
                    width="800" height="600"
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

        self.enemy_manage.enemys_list.iter_mut().for_each(|enemy| {
            enemy.update();
            enemy.render(&ctx);
        });
        self.enemy_manage.update();

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
