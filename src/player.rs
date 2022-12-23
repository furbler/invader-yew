use crate::dot_data::Color;
use crate::draw_background_rect;
use crate::input::KeyDown;
use crate::math::Vec2;
use crate::pixel_ctrl;
use crate::sound::Audio;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use web_sys::ImageBitmap;
//獲得点数
pub struct Score {
    pos: Vec2, //点数の表示位置
    //表示領域の大きさ
    width: f64,
    height: f64,
    pub sum: usize, //獲得点数
}

impl Score {
    fn render(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_fill_style(&JsValue::from("rgb(100,100,100)"));
        //文字は下にはみだしやすいため、少し下まで覆う
        draw_background_rect(
            ctx,
            self.pos.x - self.width / 2.,
            self.pos.y - self.height / 2.,
            self.width,
            self.height + 3.,
        );
        ctx.set_font(&format!("{}px monospace", self.height));
        ctx.set_fill_style(&JsValue::from("rgba(255, 255, 255)"));
        //5桁右詰めで表示
        ctx.fill_text(
            &format!("{:0>5}", self.sum),
            self.pos.x - self.width / 2.,
            self.pos.y + self.height / 2.,
        )
        .unwrap();
    }
}

pub struct Bullet {
    pub width: f64,               // 描画サイズの幅 [pixel]
    pub height: f64,              // 描画サイズの高さ [pixel]
    pub pos: Vec2,                // 移動後の中心位置
    pub pre_pos: Vec2,            // 前回描画時の中心位置
    pub live: bool,               // 弾が画面中に存在しているか否か
    pub can_shot: bool,           // 射撃可能ならば真
    pub shot_cnt: i32,            // ステージ開始からの累計射撃数
    land_effect_cnt: Option<i32>, // エフェクト表示の残りカウント
    pub remove: Option<Vec2>,     // 削除する際に残った描画を消す処理が必要であればSome(位置)で表す
    pub score: Score,
    pub image_front: Option<ImageBitmap>, // 表画像
    width_land_effect: f64,
    height_land_effect: f64,
    pub image_land_front: Option<ImageBitmap>, // 着弾時の表画像
    pub image_land_shadow: Option<ImageBitmap>, // 着弾時の影画像
}
impl Bullet {
    fn empty() -> Self {
        Bullet {
            width: 0.,
            height: 0.,
            pos: Vec2::new(0., 0.),
            pre_pos: Vec2::new(0., 0.),
            live: false,
            can_shot: true,
            shot_cnt: 0,
            remove: None,
            land_effect_cnt: None,
            score: Score {
                pos: Vec2::new(0., 0.),
                sum: 0,
                width: 0.,
                height: 0.,
            },
            width_land_effect: 0.,
            height_land_effect: 0.,
            image_front: None,
            image_land_front: None,
            image_land_shadow: None,
        }
    }
    fn new_image(
        image_front: ImageBitmap,
        image_land_front: ImageBitmap,
        image_land_shadow: ImageBitmap,
    ) -> Self {
        Bullet {
            width: image_front.width() as f64 * 2.5,
            height: image_front.height() as f64 * 2.5,
            pos: Vec2::new(0., 0.),
            pre_pos: Vec2::new(0., 0.),
            live: false,
            can_shot: true,
            land_effect_cnt: None,
            width_land_effect: image_land_front.width() as f64 * 2.5,
            height_land_effect: image_land_front.height() as f64 * 2.5,
            shot_cnt: 0,
            remove: None,
            score: Score {
                pos: Vec2::new(120., 30.),
                sum: 0,
                width: 100.,
                height: 30.,
            },
            image_front: Some(image_front),
            image_land_front: Some(image_land_front),
            image_land_shadow: Some(image_land_shadow),
        }
    }
    // 画面最上部またはトーチカへの着弾
    fn land_obstacle(&mut self) {
        // 弾を消す
        self.live = false;
        self.land_effect_cnt = Some(15);
        self.remove = Some(self.pre_pos);
    }

    fn update(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        input_key: &KeyDown,
        player_pos: Vec2,
        player_broken: Option<i32>,
        canvas_width: f64,
        audio: &Audio,
    ) {
        if self.live {
            // 弾が生きていたら更新処理を行う
            // 弾の移動処理
            self.pos.y -= 12.;
            // 弾が画面上に行ったら
            if self.pos.y < 20. {
                // 着弾処理
                self.land_obstacle();
            } else {
                // トーチカへの着弾確認
                // とりあえず弾の周りのデータまであれば十分
                let left_pos = Vec2::new(self.pos.x - self.width / 2. - 1., self.pos.y);
                let right_pos = Vec2::new(self.pos.x + self.width / 2. + 1., self.pos.y);
                // 敵の弾またはトーチカへの当たり判定
                let center_collision = pixel_ctrl::detect_pixel_diff(
                    canvas_width,
                    vec![left_pos, right_pos],
                    vec![Color::Yellow, Color::Red],
                    ctx.get_image_data(0., 0., canvas_width, self.pos.y + self.height)
                        .unwrap(),
                );
                //触れていた場合
                if center_collision {
                    // 着弾処理
                    self.land_obstacle();
                }
            }
        } else {
            // 弾が削除されている状態でのみ射撃可能
            // 射撃許可が出ていて、プレイヤーが破壊されていない状態で発射ボタンが押されている場合
            if self.can_shot && player_broken == None && input_key.shot {
                // 弾をプレイヤーの少し上に配置
                self.pos.x = player_pos.x;
                self.pos.y = player_pos.y - 24.;
                self.live = true;
                self.shot_cnt += 1;
                // 消えるまで射撃禁止
                self.can_shot = false;
                // 発射音再生
                if let Some(sound) = &audio.player_shot {
                    audio.play_once_sound(sound);
                }
            }
        }
    }

    fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        if let Some(land_pos) = self.remove {
            // 最後に残った部分を消す
            draw_background_rect(
                ctx,
                land_pos.x - self.width / 2.,
                land_pos.y - self.height / 2.,
                self.width,
                self.height,
            );
            self.remove = None;
        }
        // プレイヤーの弾が画面上に存在する時のみ描画する
        if self.live {
            // 影画像(前回の部分を消す)
            draw_background_rect(
                ctx,
                self.pre_pos.x - self.width / 2.,
                self.pre_pos.y - self.height / 2.,
                self.width,
                self.height,
            );
            // 表画像
            ctx.draw_image_with_image_bitmap_and_dw_and_dh(
                &self.image_front.as_ref().unwrap(),
                self.pos.x - self.width / 2.,
                self.pos.y - self.height / 2.,
                self.width,
                self.height,
            )
            .unwrap();
            self.pre_pos = self.pos;
        }
        // 着弾エフェクトを表示するか
        if let Some(cnt) = self.land_effect_cnt {
            // 着弾エフェクト表示
            ctx.draw_image_with_image_bitmap_and_dw_and_dh(
                &self.image_land_front.as_ref().unwrap(),
                self.pos.x - self.width_land_effect / 2.,
                self.pos.y - self.height_land_effect / 2.,
                self.width_land_effect,
                self.height_land_effect,
            )
            .unwrap();
            if cnt > 0 {
                self.land_effect_cnt = Some(cnt - 1);
            } else {
                // 着弾エフェクト削除
                ctx.draw_image_with_image_bitmap_and_dw_and_dh(
                    &self.image_land_shadow.as_ref().unwrap(),
                    self.pos.x - self.width_land_effect / 2.,
                    self.pos.y - self.height_land_effect / 2.,
                    self.width_land_effect,
                    self.height_land_effect,
                )
                .unwrap();
                self.land_effect_cnt = None;
                // 着弾エフェクトが消えてからプレイヤーの射撃可能とする
                self.can_shot = true;
            }
        }
    }
}

pub struct Player {
    pub width: f64,                       // 描画サイズの幅 [pixel]
    pub height: f64,                      // 描画サイズの高さ [pixel]
    pub pos: Vec2,                        // 移動後の中心位置
    pre_pos: Vec2,                        // 前回描画時の中心位置
    pub revival_set_cnt: i32,             //撃破されてから再出撃までのカウント設定を保存(定数)
    pub break_cnt: Option<i32>,           //再出撃までの残りカウント
    pub image_front: Option<ImageBitmap>, // 表画像
    pub bullet: Bullet,                   // 持ち弾(1発のみ)
    pub life: i32,                        // 自機含む残機(0になるとゲームオーバー)
    width_explosion: f64,
    height_explosion: f64,
    pub image_explosion_1: Option<ImageBitmap>,
    pub image_explosion_2: Option<ImageBitmap>,
    canvas_width: f64,
    canvas_height: f64,
}

impl Player {
    // 仮の値を返す
    pub fn empty() -> Self {
        Player {
            width: 0.,
            height: 0.,
            pos: Vec2 { x: 0., y: 0. },
            pre_pos: Vec2 { x: 0., y: 0. },
            revival_set_cnt: 0,
            break_cnt: None,
            image_front: None,
            life: 0,
            bullet: Bullet::empty(),
            width_explosion: 0.,
            height_explosion: 0.,
            image_explosion_1: None,
            image_explosion_2: None,
            canvas_width: 0.,
            canvas_height: 0.,
        }
    }
    pub fn new(
        canvas_width: f64,
        canvas_height: f64,
        image_front: ImageBitmap,
        image_bullet_front: ImageBitmap,
        image_land_bullet_front: ImageBitmap,
        image_land_bullet_shadow: ImageBitmap,
        image_explosion_1: ImageBitmap,
        image_explosion_2: ImageBitmap,
    ) -> Self {
        Player {
            width: image_front.width() as f64 * 2.5,
            height: image_front.height() as f64 * 2.5,
            pos: Vec2::new(70., canvas_height - 90.),
            pre_pos: Vec2::new(70., canvas_height - 90.),
            image_front: Some(image_front),
            revival_set_cnt: 130,
            break_cnt: None,
            life: 3,
            bullet: Bullet::new_image(
                image_bullet_front,
                image_land_bullet_front,
                image_land_bullet_shadow,
            ),
            width_explosion: image_explosion_1.width() as f64 * 3.,
            height_explosion: image_explosion_1.height() as f64 * 3.,
            image_explosion_1: Some(image_explosion_1),
            image_explosion_2: Some(image_explosion_2),
            canvas_width,
            canvas_height,
        }
    }
    pub fn reset(&mut self) {
        self.pos = Vec2::new(70., self.canvas_height - 100.);
        self.pre_pos = Vec2::new(70., self.canvas_height - 100.);
        self.bullet.shot_cnt = 0;
    }
    pub fn update(&mut self, ctx: &CanvasRenderingContext2d, input_key: &KeyDown, audio: &Audio) {
        //プレイヤーが撃破されてから一定時間
        if let Some(cnt) = self.break_cnt {
            if cnt < 0 {
                //一定時間経過したら復活
                self.break_cnt = None;
                self.pos.x = 70.;
                return;
            }
            if cnt == self.revival_set_cnt {
                //撃破直後にプレイヤーを消す
                draw_background_rect(
                    ctx,
                    self.pre_pos.x - self.width / 2.,
                    self.pre_pos.y - self.height / 2.,
                    self.width,
                    self.height,
                );
                // 自機撃破音再生
                if let Some(sound) = &audio.player_explosion {
                    audio.play_once_sound(sound);
                }
            }
            //撃破から一定時間は爆発エフェクトを表示
            if cnt > self.revival_set_cnt - 50 {
                //2種類の画像を交互に表示
                let image_explosion;
                if (cnt / 5) % 2 == 0 {
                    image_explosion = &self.image_explosion_1;
                } else {
                    image_explosion = &self.image_explosion_2;
                }
                ctx.draw_image_with_image_bitmap_and_dw_and_dh(
                    image_explosion.as_ref().unwrap(),
                    self.pre_pos.x - self.width_explosion / 2.,
                    self.pre_pos.y - self.height_explosion / 2.,
                    self.width_explosion,
                    self.height_explosion,
                )
                .unwrap();
                //表示画像切替時には消す
                if cnt % 5 == 0 {
                    draw_background_rect(
                        ctx,
                        self.pre_pos.x - self.width_explosion / 2.,
                        self.pre_pos.y - self.height_explosion / 2.,
                        self.width_explosion,
                        self.height_explosion,
                    );
                }
            } else if cnt == self.revival_set_cnt - 50 {
                //爆発エフェクトを最後に消す
                draw_background_rect(
                    ctx,
                    self.pre_pos.x - self.width_explosion / 2.,
                    self.pre_pos.y - self.height_explosion / 2.,
                    self.width_explosion,
                    self.height_explosion,
                );
            }
            //カウントを進める
            self.break_cnt = Some(cnt - 1);

            self.bullet.update(
                ctx,
                input_key,
                self.pos,
                self.break_cnt,
                self.canvas_width,
                audio,
            );

            return;
        }
        // 一回(1フレーム)の移動距離
        let distance = 3.5;
        if input_key.left && 0. < self.pos.x - self.width / 2. - distance {
            self.pos.x -= distance;
        }
        if input_key.right && self.pos.x + self.width / 2. + distance < self.canvas_width {
            self.pos.x += distance;
        }

        self.bullet.update(
            ctx,
            input_key,
            self.pos,
            self.break_cnt,
            self.canvas_width,
            audio,
        );
    }
    fn render_remain_life(&self, ctx: &CanvasRenderingContext2d) {
        let x = 20.;
        let y = 560.;
        // 残機表示
        ctx.set_fill_style(&JsValue::from("rgb(100,100,100)"));
        // 赤線より下をすべて消す
        draw_background_rect(ctx, x, y + 5., 600., 40.);
        ctx.set_font(&format!("25px sans-serif"));
        ctx.set_fill_style(&JsValue::from("rgba(68, 200, 210)"));
        ctx.fill_text(&format!("{}", self.life), x, y + 25.)
            .unwrap();

        // 数字表記-1 体のプレイヤー機を表示
        for i in 0..self.life - 1 {
            ctx.draw_image_with_image_bitmap_and_dw_and_dh(
                &self.image_front.as_ref().unwrap(),
                60. + 50. * i as f64,
                565.,
                self.width,
                self.height,
            )
            .unwrap();
        }
    }

    pub fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        //点数と残機は常に表示する
        self.bullet.score.render(ctx);
        self.render_remain_life(ctx);

        if let Some(_) = self.break_cnt {
            self.bullet.render(ctx);
            return;
        }
        // 影画像(前回の部分を消す)
        draw_background_rect(
            ctx,
            self.pre_pos.x - self.width / 2.,
            self.pre_pos.y - self.height / 2.,
            self.width,
            self.height,
        );
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
        self.bullet.render(ctx);
    }
}
