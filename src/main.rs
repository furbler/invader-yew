use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{window, ImageBitmap, ImageData};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::prelude::*;

use enemy::*;
use load_image::ImageType;
use pause::Pause;
use player::Player;
use sound::Audio;
use title::Title;
use ufo::Ufo;

mod dot_data;
mod enemy;
mod input;
mod load_image;
mod math;
mod pause;
mod pixel_ctrl;
mod player;
mod sound;
mod title;
mod ufo;

enum Scene {
    Title,            // タイトル画面
    Pause,            // 一時停止状態
    Play,             // ゲーム実行中
    LaunchStage(i32), // ゲーム開始後、プレイヤーが操作可能になるまで
    GameOver(i32),
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
    RetAudio,
    RegisterAudio(Audio),
    AudioVolumeUp,
    AudioVolumeDown,
    AudioVolumeReset,
    ResetCanvas,
    Initialize,
    MainLoop,
}

struct AnimationCanvas {
    canvas: NodeRef,
    player: Player,
    enemy_manage: EnemyManage,
    torchika: Option<ImageBitmap>,
    ufo: Ufo,
    audio: Audio,
    stage_number: usize, // 最初は1、最終は9
    callback: Closure<dyn FnMut()>,
    input_key_down: Rc<RefCell<input::KeyDown>>,
    need_to_screen_init: bool, // 真ならば画面全体の初期化が必要
    new_game: bool,            // 真ならば残機、点数などをすべてリセットする
    canvas_width: f64,
    canvas_height: f64,
    pause: Pause,
    scene: Scene,
    title: Title,
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
            audio: Audio::new(),
            input_key_down: Rc::new(RefCell::new(input::KeyDown {
                left: false,
                right: false,
                shot: false,
                pause: false,
            })),
            need_to_screen_init: true,
            new_game: true,
            stage_number: 1,
            canvas_width: 0.,
            canvas_height: 0.,
            pause: Pause::new(),
            title: Title::new(0., 0.),
            scene: Scene::Title,
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
                    ImageType::PlayerExplosion1 => {
                        self.player.image_explosion_1 = Some(image_bitmap)
                    }
                    ImageType::PlayerExplosion2 => {
                        self.player.image_explosion_2 = Some(image_bitmap)
                    }
                    ImageType::Torchika => self.torchika = Some(image_bitmap),
                    ImageType::Ufo => self.ufo.image = Some(image_bitmap),
                    ImageType::UfoExplosion => self.ufo.explosion.image = Some(image_bitmap),
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
            Msg::ResetCanvas => {
                // Click Thisボタンを削除
                let document = window().unwrap().document().unwrap();
                let audio_enable_button_element =
                    document.get_element_by_id("parent-audio-button").unwrap();
                audio_enable_button_element.remove();

                ctx.link().send_message(Msg::Initialize);
                true
            }
            // 初期化
            Msg::Initialize => {
                let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
                (self.canvas_width, self.canvas_height) =
                    (canvas.width() as f64, canvas.height() as f64);
                self.title = Title::new(self.canvas_width, self.canvas_height);
                // 敵インベーダーの初期化
                self.enemy_manage
                    .register_enemys(self.canvas_width, self.canvas_height);
                // プレイヤーの初期化
                self.player = Player::new(
                    self.canvas_width,
                    self.canvas_height,
                    self.player.image_front.clone().unwrap(),
                    self.player.bullet.image_front.clone().unwrap(),
                    self.player.bullet.image_land_front.clone().unwrap(),
                    self.player.bullet.image_land_shadow.clone().unwrap(),
                    self.player.image_explosion_1.clone().unwrap(),
                    self.player.image_explosion_2.clone().unwrap(),
                );
                self.ufo = Ufo::new(
                    self.ufo.image.clone().unwrap(),
                    self.ufo.explosion.image.clone().unwrap(),
                );
                // キー入力情報初期化
                input::input_setup(&self.input_key_down);

                ctx.link().send_message(Msg::RetAudio);
                true
            }
            // 音データを取得
            Msg::RetAudio => {
                if self.audio.invader_move.len() == 0 {
                    ctx.link()
                        .send_future(async { Msg::RegisterAudio(sound::ret_audio().await) });
                }
                false
            }
            // 音データを保存
            Msg::RegisterAudio(audio) => {
                self.audio = audio;
                ctx.link().send_message(Msg::MainLoop);
                false
            }
            Msg::AudioVolumeUp => {
                self.audio.all_volume_up();
                false
            }
            Msg::AudioVolumeDown => {
                self.audio.all_volume_down();
                false
            }
            Msg::AudioVolumeReset => {
                self.audio.reset_volume();
                false
            }
            // ループ
            Msg::MainLoop => {
                self.main_loop();
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div id="parent-audio-button">
                    <button id="audio-button" onclick={ctx.link().callback(|_| Msg::ResetCanvas)}>{ "Click This" }</button>
                </div>
            // キャンバスのサイズはここで指定
                <canvas
                    id="canvas"
                    width="540" height="600"
                    ref={self.canvas.clone()}/>
                <div class="volume-buttons-list">
                    <button class="volume-button" onclick={ctx.link().callback(|_| Msg::AudioVolumeUp)}>{ "Volume Up" }</button>
                    <button class="volume-button" onclick={ctx.link().callback(|_| Msg::AudioVolumeReset)}>{ "Reset Volume" }</button>
                    <button class="volume-button" onclick={ctx.link().callback(|_| Msg::AudioVolumeDown)}>{ "Volume Down" }</button>
                </div>
            </div>
        }
    }
}

impl AnimationCanvas {
    fn main_loop(&mut self) {
        let canvas: HtmlCanvasElement = self.canvas.cast().unwrap();
        let ctx: CanvasRenderingContext2d =
            canvas.get_context("2d").unwrap().unwrap().unchecked_into();
        match self.scene {
            Scene::Title => {
                // スタートボタンが押されたらゲーム開始
                if self.input_key_down.borrow().shot {
                    self.need_to_screen_init = true;
                    self.new_game = true;
                    self.scene = Scene::LaunchStage(120);
                } else {
                    self.title.render(&ctx);
                }
            }
            Scene::Pause => {
                // ポーズボタンが押されたらゲーム再開
                if self.pause.toggle_pause(self.input_key_down.borrow().pause) {
                    self.scene = Scene::Play;
                }
            }
            Scene::LaunchStage(cnt) => {
                // インベーダーを全滅させた後は休憩のため長めに間をおく
                if self.stage_number > 1 && cnt > 120 {
                    self.scene = Scene::LaunchStage(cnt - 1);

                    window()
                        .unwrap()
                        .request_animation_frame(self.callback.as_ref().unchecked_ref())
                        .unwrap();
                    return;
                }
                ctx.set_global_alpha(1.);
                // 画像のぼやけを防ぐ
                ctx.set_image_smoothing_enabled(false);

                // 画面全体の初期化
                if self.need_to_screen_init {
                    ctx.set_fill_style(&JsValue::from("rgb(0,0,0)"));
                    ctx.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
                    // プレイヤーの下に赤線を描く
                    ctx.set_stroke_style(&JsValue::from("rgb(180,0,0)"));
                    ctx.set_line_width(2.);
                    ctx.begin_path();
                    ctx.move_to(0., self.canvas_height - 40.);
                    ctx.line_to(self.canvas_width - 0., self.canvas_height - 40.);
                    ctx.stroke();
                    // トーチカの描画サイズ
                    let (torchika_width, torchika_height) = (
                        self.torchika.as_ref().unwrap().width() as f64 * 3.,
                        self.torchika.as_ref().unwrap().height() as f64 * 3.,
                    );
                    let torchika_start = self.canvas_width / 2. - 175.;
                    // トーチカ描画
                    for i in 0..4 {
                        ctx.draw_image_with_image_bitmap_and_dw_and_dh(
                            &self.torchika.as_ref().unwrap(),
                            torchika_start + 120. * i as f64 - torchika_width / 2.,
                            self.canvas_height - 180.,
                            torchika_width,
                            torchika_height,
                        )
                        .unwrap();
                    }
                    // 新しくゲーム開始
                    if self.new_game {
                        self.stage_number = 1;
                        self.player.all_reset();
                    } else {
                        // ステージが進む
                        self.player.stage_reset();
                    }
                    self.enemy_manage.reset(&ctx, self.stage_number);
                    self.ufo.reset(&ctx);

                    // 初期化は最初のみ
                    self.need_to_screen_init = false;
                    self.new_game = false;
                }

                // 敵インベーダーの処理
                // プレイヤーが操作可能になるまで敵は動くが攻撃しない
                self.enemy_manage.set_shot_interval(0);
                self.enemy_manage
                    .update(&ctx, &mut self.player, &self.audio);
                self.enemy_manage.render(&ctx);

                // 一定時間経過するまで繰り返す
                if cnt < 0 {
                    self.scene = Scene::Play;
                    // UFOの出現タイマーを初期化する
                    self.ufo.reset_timer();

                    log::info!("Stage{} start.", self.stage_number);
                } else {
                    self.scene = Scene::LaunchStage(cnt - 1);
                }
            }
            Scene::Play => {
                // ポーズボタンが押されたらゲーム一時停止
                if self.pause.toggle_pause(self.input_key_down.borrow().pause) {
                    self.scene = Scene::Pause;
                }
                ctx.set_global_alpha(1.);
                // 画像のぼやけを防ぐ
                ctx.set_image_smoothing_enabled(false);
                // プレイヤーの処理
                self.player
                    .update(&ctx, &self.input_key_down.borrow(), &self.audio);
                // 敵インベーダーの処理
                self.enemy_manage
                    .update(&ctx, &mut self.player, &self.audio);

                // UFOの処理
                self.ufo.update(
                    &ctx,
                    self.canvas_width,
                    &mut self.player.bullet,
                    &self.audio,
                );

                self.player.render(&ctx);
                self.enemy_manage.render(&ctx);
                self.ufo.render(&ctx);

                if let Some(enemy_pos_y) = self.enemy_manage.nadir_y() {
                    // 敵インベーダーがプレイヤーの高さまで侵攻した場合
                    if self.player.pos.y - self.player.height / 2. < enemy_pos_y {
                        // プレイヤーは破壊される
                        self.player.break_cnt = Some(self.player.revival_set_cnt);
                        // ゲームオーバー
                        self.scene = Scene::GameOver(140);
                    }
                } else {
                    // インベーダーが全滅した場合
                    // 画面を初期化する
                    self.need_to_screen_init = true;
                    self.new_game = false;
                    // ステージは9面の次は2面に戻る
                    self.stage_number = if self.stage_number >= 9 {
                        2
                    } else {
                        self.stage_number + 1
                    };
                    self.scene = Scene::LaunchStage(240);
                }
                // プレイヤーの残機が無くなったら
                if self.player.life <= 0 {
                    // ゲームオーバー
                    self.scene = Scene::GameOver(140);
                }
            }
            Scene::GameOver(cnt) => {
                self.new_game = true;
                // プレイヤーの爆発エフェクトを最後まで表示
                if let Some(explosion_cnt) = self.player.break_cnt {
                    self.player
                        .update(&ctx, &self.input_key_down.borrow(), &self.audio);
                    // 爆発エフェクト表示が終わった後のプレイヤー復活はしない
                    if explosion_cnt > 0 {
                        self.player.render(&ctx);
                    }
                } else {
                    // プレイヤーの爆発エフェクト表示が終わったら一定時間ゲームオーバー表示
                    ctx.set_font("80px monospace");
                    ctx.set_fill_style(&JsValue::from("rgba(200, 10, 10)"));
                    ctx.fill_text(
                        "GAME OVER",
                        self.canvas_width / 2. - 180.,
                        self.canvas_height / 4.,
                    )
                    .unwrap();
                    self.scene = Scene::GameOver(cnt - 1);
                }
                if cnt < 0 {
                    // 画面クリア
                    ctx.set_fill_style(&JsValue::from("rgb(0,0,0)"));
                    ctx.fill_rect(0.0, 0.0, self.canvas_width, self.canvas_height);
                    self.ufo.reset(&ctx);
                    // タイトルに戻る
                    self.scene = Scene::Title;
                }
            }
        }
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
    // firefox以外のブラウザで、画像描画範囲に対し塗りつぶし範囲が僅かにずれる
    // その対策として、塗りつぶし範囲を1pixel増やす
    ctx.fill_rect(x - 1., y - 1., width + 2., height + 2.);
}
