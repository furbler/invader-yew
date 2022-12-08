use crate::dot_data::Color;
use crate::draw_background_rect;
use crate::input::KeyDown;
use crate::math::Vec2;
use crate::pixel_ctrl;
use web_sys::CanvasRenderingContext2d;
use web_sys::ImageBitmap;

pub struct Bullet {
    pub width: f64,                       // 描画サイズの幅 [pixel]
    pub height: f64,                      // 描画サイズの高さ [pixel]
    pub pos: Vec2,                        // 移動後の中心位置
    pub pre_pos: Vec2,                    // 前回描画時の中心位置
    pub live: bool,                       // 弾が画面中に存在しているか否か
    pub can_shot: bool,                   // 射撃可能ならば真
    land_effect_cnt: Option<i32>,         // エフェクト表示の残りカウント
    pub remove: Option<Vec2>, // 削除する際に残った描画を消す処理が必要であればSome(位置)で表す
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
            remove: None,
            land_effect_cnt: None,
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
            width: image_front.width() as f64 * 3.,
            height: image_front.height() as f64 * 3.,
            pos: Vec2::new(0., 0.),
            pre_pos: Vec2::new(0., 0.),
            live: false,
            can_shot: true,
            land_effect_cnt: None,
            width_land_effect: image_land_front.width() as f64 * 2.5,
            height_land_effect: image_land_front.height() as f64 * 2.5,
            remove: None,
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
        canvas_width: f64,
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
                // 弾の中心より少し上の座標
                let left_pos = Vec2::new(self.pos.x - self.width / 2. - 2., self.pos.y);
                let left_collision = pixel_ctrl::detect_pixel_diff(
                    canvas_width,
                    left_pos,
                    Color::RED,
                    ctx.get_image_data(0., 0., canvas_width, self.pos.y + self.height)
                        .unwrap(),
                );

                let right_pos = Vec2::new(self.pos.x + self.width / 2. + 2., self.pos.y);
                let right_collision = pixel_ctrl::detect_pixel_diff(
                    canvas_width,
                    right_pos,
                    Color::RED,
                    ctx.get_image_data(0., 0., canvas_width, self.pos.y + self.height)
                        .unwrap(),
                );
                //トーチカに触れていた場合
                if left_collision || right_collision {
                    // 着弾処理
                    self.land_obstacle();
                }
            }
        } else {
            // 弾が削除されている状態でのみ射撃可能
            // 射撃許可が出ていて、発射ボタンが押されている場合
            if self.can_shot && input_key.shot {
                // 弾をプレイヤーの少し上に配置
                self.pos.x = player_pos.x;
                self.pos.y = player_pos.y - 24.;
                self.live = true;
                // 消えるまで射撃禁止
                self.can_shot = false;
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
    width: f64,                           // 描画サイズの幅 [pixel]
    height: f64,                          // 描画サイズの高さ [pixel]
    pos: Vec2,                            // 移動後の中心位置
    pre_pos: Vec2,                        // 前回描画時の中心位置
    pub image_front: Option<ImageBitmap>, // 表画像
    pub bullet: Bullet,                   // 持ち弾(1発のみ)
}

impl Player {
    // 仮の値を返す
    pub fn empty() -> Self {
        Player {
            width: 0.,
            height: 0.,
            pos: Vec2 { x: 0., y: 0. },
            pre_pos: Vec2 { x: 0., y: 0. },
            image_front: None,
            bullet: Bullet::empty(),
        }
    }
    pub fn new(
        pos: Vec2,
        image_front: ImageBitmap,
        image_bullet_front: ImageBitmap,
        image_land_bullet_front: ImageBitmap,
        image_land_bullet_shadow: ImageBitmap,
    ) -> Self {
        Player {
            width: image_front.width() as f64 * 3.,
            height: image_front.height() as f64 * 3.,
            pos,
            pre_pos: pos,
            image_front: Some(image_front),
            bullet: Bullet::new_image(
                image_bullet_front,
                image_land_bullet_front,
                image_land_bullet_shadow,
            ),
        }
    }
    pub fn update(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        input_key: &KeyDown,
        canvas_width: f64,
    ) {
        // 一回(1フレーム)の移動距離
        let distance = 3.5;
        if input_key.left && 0. < self.pos.x - self.width / 2. - distance {
            self.pos.x -= distance;
        }
        if input_key.right && self.pos.x + self.width / 2. + distance < canvas_width {
            self.pos.x += distance;
        }

        self.bullet.update(ctx, input_key, self.pos, canvas_width);
    }
    pub fn render(&mut self, ctx: &CanvasRenderingContext2d) {
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
