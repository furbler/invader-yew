use std::collections::HashMap;
use web_sys::CanvasRenderingContext2d;
use web_sys::ImageBitmap;

use crate::load_image::ImageType;
use crate::math::Vec2;
struct Enemy {
    width: f64,            // 描画サイズの幅 [pixel]
    height: f64,           // 描画サイズの高さ [pixel]
    pos: Vec2,             // 移動後の中心位置
    pre_pos: Vec2,         // 前回描画時の中心位置
    move_turn: bool,       // 動くか否か
    show_image_type: bool, // どちらの状態の画像を表示するか
    // 表画像
    image_type1_front: ImageBitmap,
    image_type2_front: ImageBitmap,
    // 影画像
    image_type1_shadow: ImageBitmap,
    image_type2_shadow: ImageBitmap,
}

impl Enemy {
    fn update(&mut self, move_dir: i32) {
        if !self.move_turn {
            // 動く時以外は何もしない
            return;
        }
        // 方向を考慮して動く
        self.pos.x += 10. * move_dir as f64;
        // 表示する画像を切り替える
        self.show_image_type = !self.show_image_type
    }

    fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        if !self.move_turn {
            // 動く時以外は描画しない
            return;
        }
        // 表示画像選択
        let show_image_front = if self.show_image_type {
            &self.image_type1_front
        } else {
            &self.image_type2_front
        };
        // 影の方は常に表画像と逆(動く時必ず画像タイプが切り替わるため)
        let show_image_shadow = if self.show_image_type {
            &self.image_type2_shadow
        } else {
            &self.image_type1_shadow
        };
        // 影画像(前回の部分を消す)
        ctx.draw_image_with_image_bitmap_and_dw_and_dh(
            show_image_shadow,
            self.pre_pos.x - self.width / 2.,
            self.pre_pos.y - self.height / 2.,
            self.width,
            self.height,
        )
        .unwrap();
        // 表画像
        ctx.draw_image_with_image_bitmap_and_dw_and_dh(
            show_image_front,
            self.pos.x - self.width / 2.,
            self.pos.y - self.height / 2.,
            self.width,
            self.height,
        )
        .unwrap();
        self.pre_pos = self.pos;
    }
}

pub struct EnemyManage {
    // 敵の左方向、右方向の移動範囲限界のx座標
    left_border: f64,
    right_border: f64,
    // 移動方向(右は1、左は-1)
    move_dir: i32,
    // 次フレームで移動方向を反転すべきか否か
    move_dir_invert: bool,
    pub images_list: HashMap<ImageType, ImageBitmap>,
    enemys_list: Vec<Enemy>,
}

impl EnemyManage {
    pub fn default() -> Self {
        EnemyManage {
            left_border: 50.,
            right_border: 730.,
            move_dir: 1,
            move_dir_invert: false,
            images_list: HashMap::new(),
            enemys_list: Vec::new(),
        }
    }
    pub fn register_enemys(&mut self, canvas_height: f64) {
        let image_type1_front = self.images_list.get(&ImageType::OctopusOpenFront).unwrap();
        let image_type2_front = self.images_list.get(&ImageType::OctopusCloseFront).unwrap();
        let image_type1_shadow = self.images_list.get(&ImageType::OctopusOpenShadow).unwrap();
        let image_type2_shadow = self
            .images_list
            .get(&ImageType::OctopusCloseShadow)
            .unwrap();

        let invader_column = 11;

        let mut invader_pos = Vec2::new(100., canvas_height - 300.);
        for _ in 0..2 {
            for _ in 0..invader_column {
                self.enemys_list.push(Enemy {
                    width: image_type1_front.width() as f64 * 3.,
                    height: image_type1_front.height() as f64 * 3.,
                    pos: invader_pos,
                    pre_pos: invader_pos,
                    move_turn: false,
                    show_image_type: true,
                    image_type1_front: image_type1_front.clone(),
                    image_type2_front: image_type2_front.clone(),
                    image_type1_shadow: image_type1_shadow.clone(),
                    image_type2_shadow: image_type2_shadow.clone(),
                });
                invader_pos.x += 50.;
            }
            invader_pos.x = 100.;
            invader_pos.y -= 50.;
        }

        let image_type1_front = self.images_list.get(&ImageType::CrabBanzaiFront).unwrap();
        let image_type2_front = self.images_list.get(&ImageType::CrabDownFront).unwrap();
        let image_type1_shadow = self.images_list.get(&ImageType::CrabBanzaiShadow).unwrap();
        let image_type2_shadow = self.images_list.get(&ImageType::CrabDownShadow).unwrap();

        for _ in 0..2 {
            for _ in 0..invader_column {
                self.enemys_list.push(Enemy {
                    width: image_type1_front.width() as f64 * 3.,
                    height: image_type1_front.height() as f64 * 3.,
                    pos: invader_pos,
                    pre_pos: invader_pos,
                    move_turn: false,
                    show_image_type: true,
                    image_type1_front: image_type1_front.clone(),
                    image_type2_front: image_type2_front.clone(),
                    image_type1_shadow: image_type1_shadow.clone(),
                    image_type2_shadow: image_type2_shadow.clone(),
                });
                invader_pos.x += 50.;
            }
            invader_pos.x = 100.;
            invader_pos.y -= 50.;
        }
        let image_type1_front = self.images_list.get(&ImageType::SquidOpenFront).unwrap();
        let image_type2_front = self.images_list.get(&ImageType::SquidCloseFront).unwrap();
        let image_type1_shadow = self.images_list.get(&ImageType::SquidOpenShadow).unwrap();
        let image_type2_shadow = self.images_list.get(&ImageType::SquidCloseShadow).unwrap();

        for _ in 0..invader_column {
            self.enemys_list.push(Enemy {
                width: image_type1_front.width() as f64 * 3.,
                height: image_type1_front.height() as f64 * 3.,
                pos: invader_pos,
                pre_pos: invader_pos,
                move_turn: false,
                show_image_type: true,
                image_type1_front: image_type1_front.clone(),
                image_type2_front: image_type2_front.clone(),
                image_type1_shadow: image_type1_shadow.clone(),
                image_type2_shadow: image_type2_shadow.clone(),
            });
            invader_pos.x += 50.;
        }

        // 一番左下の敵インベーダーから動く
        self.enemys_list[0].move_turn = true;
    }

    pub fn update(&mut self) {
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
    pub fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        self.enemys_list.iter_mut().for_each(|enemy| {
            enemy.render(ctx);
        });
    }
}
