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
    move_turn: bool,       // 動く順番がきたら真
    live: bool,            // 生死
    remove: bool,          // 削除時に残った描画処理の必用がある場合真
    show_image_type: bool, // どちらの状態の画像を表示するか
    // 表画像
    image_type1_front: ImageBitmap,
    image_type2_front: ImageBitmap,
    // 影画像
    image_type1_shadow: ImageBitmap,
    image_type2_shadow: ImageBitmap,
}

impl Enemy {
    fn update(&mut self, move_dir: i32, player_bullet: &mut crate::player::Bullet) {
        if !self.live {
            // 死んでいたら何もしない
            return;
        }
        // プレイヤーの弾が画面上に存在して
        if player_bullet.live {
            // 自身と衝突していた場合
            if player_bullet
                .pos
                .collision(&self.pos, self.width, self.height)
            {
                // 自分を削除
                self.live = false;
                self.remove = true;
                // プレイヤーの弾も削除
                player_bullet.live = false;
                player_bullet.remove = Some(player_bullet.pre_pos);
            }
        }
        // 動く時
        if self.move_turn {
            // 方向を考慮して動く
            self.pos.x += 10. * move_dir as f64;
            // 表示する画像を切り替える
            self.show_image_type = !self.show_image_type
        }
    }

    fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        // 削除処理
        if self.remove {
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
            // 削除処理完了
            self.remove = false;
        }

        // 動く時以外または死んでいる場合は描画しない
        if !self.move_turn || !self.live {
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
                    live: true,
                    remove: false,
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
                    live: true,
                    remove: false,
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
                live: true,
                remove: false,
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

    pub fn update(&mut self, player_bullet: &mut crate::player::Bullet) {
        // 各敵個体の移動処理
        self.enemys_list.iter_mut().for_each(|enemy| {
            enemy.update(self.move_dir, player_bullet);
        });

        // 移動した敵インベーダーの個体番号を取得
        let mut moved_enemy_index = 0;
        for (index, enemy) in self.enemys_list.iter().enumerate() {
            if enemy.move_turn {
                moved_enemy_index = index;
                break;
            }
        }
        // 動いた個体が制限範囲外に出た場合
        if self.enemys_list[moved_enemy_index].pos.x < self.left_border
            || self.right_border < self.enemys_list[moved_enemy_index].pos.x
        {
            // 移動方向反転フラグを立てる
            self.move_dir_invert = true;
        }
        // 移動する個体を変える
        self.enemys_list[moved_enemy_index].move_turn = false;

        let mut next_move_enemy_index = None;
        for index in (moved_enemy_index + 1)..self.enemys_list.len() {
            if self.enemys_list[index].live {
                next_move_enemy_index = Some(index);
                break;
            }
        }
        // 動いた個体より後がすべて死んでいた場合
        if next_move_enemy_index == None {
            // 移動方向反転フラグが立っている場合
            if self.move_dir_invert {
                // 移動方向を反転
                self.move_dir *= -1;
                // 移動方向反転フラグをリセット
                self.move_dir_invert = false;
            }
            // もう一週生きている個体を探す
            for (index, enemy) in self.enemys_list.iter().enumerate() {
                if enemy.live {
                    next_move_enemy_index = Some(index);
                    break;
                }
            }
        }
        if let Some(i) = next_move_enemy_index {
            // 次に動く敵個体を指定
            self.enemys_list[i].move_turn = true;
        } else {
            log::info!("敵は全滅した。");
        }
    }
    pub fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        self.enemys_list.iter_mut().for_each(|enemy| {
            enemy.render(ctx);
        });
    }
}
