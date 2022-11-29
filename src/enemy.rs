use std::collections::HashMap;
use web_sys::CanvasRenderingContext2d;
use web_sys::ImageBitmap;

use crate::load_image::ImageType;
use crate::math::Vec2;
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
