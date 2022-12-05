use crate::draw_background_rect;
use crate::math::Vec2;
use crate::player;
use instant::Instant;
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
}

impl Explosion {
    fn create_effect(&mut self, pos: Vec2) {
        self.pos = pos;
        self.count = 20;
        self.live = true;
    }
    fn update(&mut self, ctx: &CanvasRenderingContext2d) {
        if !self.live {
            return;
        }
        // エフェクト表示
        ctx.draw_image_with_image_bitmap_and_dw_and_dh(
            &self.image.as_ref().unwrap(),
            self.pos.x - self.width / 2.,
            self.pos.y - self.height / 2.,
            self.width,
            self.height,
        )
        .unwrap();

        self.count -= 1;
        // 一定時間経過したら
        if self.count < 0 {
            // エフェクト削除
            draw_background_rect(
                ctx,
                self.pos.x - self.width / 2.,
                self.pos.y - self.height / 2.,
                self.width,
                self.height,
            );
            self.live = false;
        }
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
            },
            lapse_time: Instant::now(),
            move_dir: 0,
        }
    }
    pub fn new(image: ImageBitmap, image_explosion: ImageBitmap) -> Self {
        Ufo {
            width: image.width() as f64 * 3.,
            height: image.height() as f64 * 3.,
            // ここで高さを指定する
            pos: Vec2 { x: -10., y: 70. },
            pre_pos: Vec2 { x: -10., y: -10. },
            image: Some(image),
            explosion: Explosion {
                width: image_explosion.width() as f64 * 3.,
                height: image_explosion.height() as f64 * 3.,
                pos: Vec2::new(0., 0.),
                live: false,
                image: Some(image_explosion),
                count: 0,
            },
            lapse_time: Instant::now(),
            move_dir: -1, // 最初は右から左
        }
    }
    fn remove(&mut self, ctx: &CanvasRenderingContext2d) {
        // 描画を削除
        self.remove_shadow(ctx);
        // 時間をリセット
        self.lapse_time = Instant::now();
        // 方向反転
        self.move_dir *= -1;
        self.pos.x = -10.;
    }

    pub fn update(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        canvas_width: f64,
        player_bullet: &mut player::Bullet,
    ) {
        self.explosion.update(ctx);
        if self.lapse_time.elapsed().as_secs() < 5 {
            // 一定時間経過するまでは何もしない
            return;
        }
        if player_bullet.live {
            // 弾と衝突していた場合
            if player_bullet
                .pos
                .collision(&self.pos, self.width, self.height)
            {
                // UFOの位置に爆発エフェクト生成
                self.explosion.create_effect(self.pos);
                // UFOを消す
                self.remove(ctx);
                // プレイヤーの弾を消す
                player_bullet.live = false;
                player_bullet.remove = Some(player_bullet.pre_pos);
                player_bullet.can_shot = true;
            }
        }

        // 出現する瞬間
        if self.pos.x < 0. {
            // 右から左へ動く
            if self.move_dir < 0 {
                self.pos.x = canvas_width - self.width / 2.;
            } else {
                // 左から右へ動く
                self.pos.x = self.width / 2.;
            }
            self.pre_pos = self.pos;
        } else {
            // 移動
            self.pos.x += 2.5 * self.move_dir as f64;
        }
        // 外に行った場合
        if self.pos.x - self.width / 2. < 0. || canvas_width < self.pos.x + self.width / 2. {
            self.remove(ctx);
        }
    }

    pub fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        if self.lapse_time.elapsed().as_secs() < 5 || self.pos.x < 0. {
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
