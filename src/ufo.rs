use crate::draw_background_rect;
use crate::math::Vec2;
use crate::player;
use crate::sound::Audio;
use instant::Instant;
use wasm_bindgen::JsValue;
use web_sys::AudioBufferSourceNode;
use web_sys::CanvasRenderingContext2d;
use web_sys::ImageBitmap;

pub struct Explosion {
    width: f64,                     // 描画サイズの幅 [pixel]
    height: f64,                    // 描画サイズの高さ [pixel]
    pos: Vec2,                      // 移動後の中心位置
    live: bool,                     // 生死
    pub image: Option<ImageBitmap>, // 爆発画像
    // 表示カウント(0になったら消滅)
    count: i32,
    got_score: usize, // 獲得した点数
}

impl Explosion {
    fn create_effect(&mut self, pos: Vec2) {
        self.pos = pos;
        self.count = 120;
        self.live = true;
    }
    fn update(&mut self, ctx: &CanvasRenderingContext2d) {
        if !self.live {
            return;
        }
        if self.count == 120 {
            // エフェクト表示
            ctx.draw_image_with_image_bitmap_and_dw_and_dh(
                &self.image.as_ref().unwrap(),
                self.pos.x - self.width / 2.,
                self.pos.y - self.height / 2.,
                self.width,
                self.height,
            )
            .unwrap();
        }
        if 0 < self.count && self.count < 100 {
            // 一定時間経過したら
            // 爆発エフェクト削除
            draw_background_rect(
                ctx,
                self.pos.x - self.width / 2.,
                self.pos.y - self.height / 2.,
                self.width,
                self.height,
            );
            // 獲得得点表示
            ctx.set_font(&format!("22px monospace"));
            ctx.set_fill_style(&JsValue::from("rgba(219, 85, 221)"));
            ctx.fill_text(
                &format!("{}", self.got_score),
                self.pos.x - 14.,
                self.pos.y + 8.,
            )
            .unwrap();
        }
        if self.count < 0 {
            // 獲得得点表示削除
            draw_background_rect(
                ctx,
                self.pos.x - self.width / 2.,
                self.pos.y - self.height / 2.,
                self.width,
                self.height,
            );
            self.live = false;
        }
        self.count -= 1;
    }
}

pub struct Ufo {
    width: f64,                     // 描画サイズの幅 [pixel]
    height: f64,                    // 描画サイズの高さ [pixel]
    pos: Vec2,                      // 移動後の中心位置
    pre_pos: Vec2,                  // 前回描画時の中心位置
    pub image: Option<ImageBitmap>, // 表画像
    pub explosion: Explosion,
    lapse_time: Instant, // 前回に出現してからの経過時間
    move_dir: i32,       // 移動方向
    flying_sound: Option<AudioBufferSourceNode>,
    score_table: [usize; 15], // 獲得得点の表(プレイヤーの発射数の合計で決める)
}

impl Ufo {
    // 仮の値を返す
    pub fn empty() -> Self {
        Ufo {
            width: 0.,
            height: 0.,
            pos: Vec2 { x: 0., y: 0. },
            pre_pos: Vec2 { x: 0., y: 0. },
            image: None,
            explosion: Explosion {
                width: 0.,
                height: 0.,
                pos: Vec2::new(0., 0.),
                live: false,
                image: None,
                count: 0,
                got_score: 0,
            },
            lapse_time: Instant::now(),
            move_dir: 0,
            flying_sound: None,
            score_table: [0; 15],
        }
    }
    pub fn new(image: ImageBitmap, image_explosion: ImageBitmap) -> Self {
        Ufo {
            width: image.width() as f64 * 2.3,
            height: image.height() as f64 * 2.3,
            // ここで高さを指定する
            pos: Vec2 { x: -10., y: 80. },
            pre_pos: Vec2 { x: -10., y: -10. },
            image: Some(image),
            explosion: Explosion {
                width: image_explosion.width() as f64 * 2.3,
                height: image_explosion.height() as f64 * 2.3,
                pos: Vec2::new(0., 0.),
                live: false,
                image: Some(image_explosion),
                count: 0,
                got_score: 0,
            },
            lapse_time: Instant::now(),
            move_dir: -1, // 最初は右から左
            flying_sound: None,
            score_table: [
                50, 50, 100, 150, 100, 100, 50, 300, 100, 100, 100, 50, 150, 100, 100,
            ],
        }
    }
    // 出現タイミング用タイマーをリセット
    pub fn reset_timer(&mut self) {
        self.lapse_time = Instant::now();
    }
    fn remove(&mut self, ctx: &CanvasRenderingContext2d) {
        // 描画を削除
        self.remove_shadow(ctx);
        // 時間をリセット
        self.lapse_time = Instant::now();
        self.pos.x = -10.;
        // 飛行音のループ再生を止める
        if let Some(sound_node) = &self.flying_sound {
            sound_node.stop().unwrap();
        }
    }
    // 新しいステージに進むときなどに残った表示を消す
    pub fn reset(&mut self, ctx: &CanvasRenderingContext2d) {
        self.reset_timer();
        self.remove(ctx);
    }

    pub fn update(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        canvas_width: f64,
        player_bullet: &mut player::Bullet,
        audio: &Audio,
    ) {
        self.explosion.update(ctx);
        if self.lapse_time.elapsed().as_secs() < 25 {
            // 一定時間経過するまでは何もしない
            return;
        }
        if player_bullet.live
            && player_bullet
                .pos
                .collision(&self.pos, self.width, self.height)
        {
            // 弾と衝突していた場合
            // UFOの位置に爆発エフェクト生成
            self.explosion.create_effect(self.pos);
            // UFOを消す
            self.remove(ctx);
            // プレイヤーの弾を消す
            player_bullet.live = false;
            player_bullet.remove = Some(player_bullet.pre_pos);
            player_bullet.can_shot = true;
            // 表を参考に点数を加算
            let got_score = self.score_table[(player_bullet.shot_cnt - 1) as usize % 15];
            player_bullet.score.sum += got_score;
            // 表示用に点数保存
            self.explosion.got_score = got_score;
            // UFO撃破音再生
            if let Some(sound) = &audio.ufo_explosion {
                audio.play_once_sound(sound);
            }
            return;
        }
        if self.pos.x < 0. {
            // UFOが出現する瞬間
            // プレイヤーの発射数が偶数であれば右から左へ動く
            if player_bullet.shot_cnt % 2 == 0 {
                self.pos.x = canvas_width - self.width / 2.;
                self.move_dir = -1;
            } else {
                // 奇数ならば左から右へ動く
                self.pos.x = self.width / 2.;
                self.move_dir = 1;
            }
            self.pre_pos = self.pos;
            // UFO飛行音ループ再生開始
            if let Some(sound) = &audio.ufo_flying {
                self.flying_sound = Some(audio.play_looping_sound(sound));
            }
            return;
        }
        // 移動
        self.pos.x += 2.5 * self.move_dir as f64;

        // 外に出た場合
        if self.pos.x - self.width / 2. < 0. || canvas_width < self.pos.x + self.width / 2. {
            self.remove(ctx);
        }
    }

    pub fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        if self.lapse_time.elapsed().as_secs() < 25 || self.pos.x < 0. {
            return;
        }
        // 一定時間経過して、かつupdate関数が実行されていた場合
        // 前回の描画を削除
        self.remove_shadow(ctx);
        // 表画像
        ctx.draw_image_with_image_bitmap_and_dw_and_dh(
            &self.image.as_ref().unwrap(),
            self.pos.x - self.width / 2.,
            self.pos.y - self.height / 2.,
            self.width,
            self.height,
        )
        .unwrap();
        self.pre_pos = self.pos;
    }
    fn remove_shadow(&self, ctx: &CanvasRenderingContext2d) {
        // 前回の描画を削除
        draw_background_rect(
            ctx,
            self.pre_pos.x - self.width / 2.,
            self.pre_pos.y - self.height / 2.,
            self.width,
            self.height,
        );
    }
}
