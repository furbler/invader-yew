use crate::input::KeyDown;
use crate::math::Vec2;
use web_sys::CanvasRenderingContext2d;
use web_sys::ImageBitmap;

pub struct Bullet {
    width: f64,                            // 描画サイズの幅 [pixel]
    height: f64,                           // 描画サイズの高さ [pixel]
    pos: Vec2,                             // 移動後の中心位置
    pre_pos: Vec2,                         // 前回描画時の中心位置
    live: bool,                            // 弾が画面中に存在しているか否か
    pub image_front: Option<ImageBitmap>,  // 表画像
    pub image_shadow: Option<ImageBitmap>, // 影画像
}
impl Bullet {
    fn empty() -> Self {
        Bullet {
            width: 0.,
            height: 0.,
            pos: Vec2::new(0., 0.),
            pre_pos: Vec2::new(0., 0.),
            live: false,
            image_front: None,
            image_shadow: None,
        }
    }
    fn new_image(image_front: ImageBitmap, image_shadow: ImageBitmap) -> Self {
        Bullet {
            width: image_front.width() as f64 * 3.,
            height: image_front.height() as f64 * 3.,
            pos: Vec2::new(0., 0.),
            pre_pos: Vec2::new(0., 0.),
            live: false,
            image_front: Some(image_front),
            image_shadow: Some(image_shadow),
        }
    }
    // 弾を消す
    fn remove(&mut self, ctx: &CanvasRenderingContext2d) {
        self.live = false;
        // 影画像(前回の部分を消す)
        ctx.draw_image_with_image_bitmap_and_dw_and_dh(
            &self.image_shadow.as_ref().unwrap(),
            self.pre_pos.x - self.width / 2.,
            self.pre_pos.y - self.height / 2.,
            self.width,
            self.height,
        )
        .unwrap();
    }

    fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        // プレイヤーの弾が画面上に存在する時のみ描画する
        if self.live {
            // 影画像(前回の部分を消す)
            ctx.draw_image_with_image_bitmap_and_dw_and_dh(
                &self.image_shadow.as_ref().unwrap(),
                self.pre_pos.x - self.width / 2.,
                self.pre_pos.y - self.height / 2.,
                self.width,
                self.height,
            )
            .unwrap();
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
    }
}

pub struct Player {
    width: f64,                            // 描画サイズの幅 [pixel]
    height: f64,                           // 描画サイズの高さ [pixel]
    pos: Vec2,                             // 移動後の中心位置
    pre_pos: Vec2,                         // 前回描画時の中心位置
    pub image_front: Option<ImageBitmap>,  // 表画像
    pub image_shadow: Option<ImageBitmap>, // 影画像
    pub bullet: Bullet,                    // 持ち弾(1発のみ)
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
            image_shadow: None,
            bullet: Bullet::empty(),
        }
    }
    pub fn new(
        pos: Vec2,
        image_front: ImageBitmap,
        image_shadow: ImageBitmap,
        image_bullet_front: ImageBitmap,
        image_bullet_shadow: ImageBitmap,
    ) -> Self {
        Player {
            width: image_front.width() as f64 * 3.,
            height: image_front.height() as f64 * 3.,
            pos,
            pre_pos: pos,
            image_front: Some(image_front),
            image_shadow: Some(image_shadow),
            bullet: Bullet::new_image(image_bullet_front, image_bullet_shadow),
        }
    }
    pub fn update(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        input_key: &KeyDown,
        canvas_width: f64,
    ) {
        // 一回(1フレーム)の移動距離
        let distance = 5.;
        if input_key.left && 0. < self.pos.x - self.width / 2. - distance {
            self.pos.x -= distance;
        }
        if input_key.right && self.pos.x + self.width / 2. + distance < canvas_width {
            self.pos.x += distance;
        }

        // 弾が画面上に存在していたら
        if self.bullet.live {
            // 弾の移動処理
            self.bullet.pos.y -= 13.;
            // 弾が画面上の外側に行ったら
            if self.bullet.pos.y < 0. {
                // 弾を消す
                self.bullet.remove(ctx);
            }
        }
        // 発射ボタンが押されている場合
        if input_key.shot {
            // 弾が画面上に存在しない場合
            if !self.bullet.live {
                // 弾をプレイヤーの少し上に置く
                self.bullet.pos.x = self.pos.x;
                self.bullet.pos.y = self.pos.y - self.height;
                self.bullet.live = true;
            }
        }
    }
    pub fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        // 影画像(前回の部分を消す)
        ctx.draw_image_with_image_bitmap_and_dw_and_dh(
            &self.image_shadow.as_ref().unwrap(),
            self.pre_pos.x - self.width / 2.,
            self.pre_pos.y - self.height / 2.,
            self.width,
            self.height,
        )
        .unwrap();
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
