use std::collections::HashMap;
use web_sys::CanvasRenderingContext2d;
use web_sys::ImageBitmap;

use crate::draw_background_rect;
use crate::load_image::ImageType;
use crate::math::Vec2;

#[derive(Eq, Hash, PartialEq)]
enum EnemyType {
    Octopus,
    Crab,
    Squid,
}

struct Explosion {
    // 爆発エフェクト表示中は
    show: Option<ImageBitmap>,
    pos: Vec2,
    // 表示カウント(0になったら消滅)
    count: i32,
    // 表示幅
    width: f64,
    // 表示高さ
    height: f64,
    enemy_type_map: HashMap<EnemyType, ImageBitmap>,
    shadow: Option<ImageBitmap>,
}
impl Explosion {
    fn create_effect(&mut self, pos: Vec2, image: ImageBitmap) {
        self.show = Some(image);
        self.pos = pos;
        self.count = 15;
    }
    fn update_render(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        player_bullet: &mut crate::player::Bullet,
    ) {
        // エフェクト表示中であれば
        if let Some(image) = self.show.as_ref() {
            ctx.draw_image_with_image_bitmap_and_dw_and_dh(
                image,
                self.pos.x - self.width / 2.,
                self.pos.y - self.height / 2.,
                self.width,
                self.height,
            )
            .unwrap();
            self.count -= 1;
        }
        // 一定フレーム経過したら
        if self.count < 0 {
            // 削除
            self.show = None;
            // 爆発エフェクトを消す
            ctx.draw_image_with_image_bitmap_and_dw_and_dh(
                self.shadow.as_ref().unwrap(),
                self.pos.x - self.width / 2.,
                self.pos.y - self.height / 2.,
                self.width,
                self.height,
            )
            .unwrap();
            // 爆発エフェクトが消えてからプレイヤーの射撃を可能とする
            player_bullet.can_shot = true;
        }
    }
}

struct Enemy {
    enemy_type: EnemyType,
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
}

impl Enemy {
    fn update(
        &mut self,
        move_dir: i32,
        player_bullet: &mut crate::player::Bullet,
        explosion: &mut Explosion,
    ) {
        if !self.live {
            // 死んでいたら何もしない
            return;
        }
        // プレイヤーの弾が画面上に存在して
        if player_bullet.live {
            // 弾と衝突していた場合
            if player_bullet
                .pre_pos
                .collision(&self.pos, self.width, self.height)
            {
                // 自分を削除
                self.live = false;
                self.remove = true;
                // プレイヤーの弾を消す
                player_bullet.live = false;
                player_bullet.remove = Some(player_bullet.pre_pos);
                // 爆発エフェクトを生成
                explosion.create_effect(
                    self.pos,
                    explosion
                        .enemy_type_map
                        .get(&self.enemy_type)
                        .unwrap()
                        .clone(),
                );
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
            // 影画像(前回の部分を消す)
            draw_background_rect(
                ctx,
                self.pre_pos.x - self.width / 2.,
                self.pre_pos.y - self.height / 2.,
                self.width,
                self.height,
            );
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
    // 種類に対応した画像を保存
    pub images_list: HashMap<ImageType, ImageBitmap>,
    // 敵一覧
    enemys_list: Vec<Enemy>,
    // 爆発エフェクト
    explosion: Explosion,
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
            explosion: Explosion {
                show: None,
                pos: Vec2 { x: 0., y: 0. },
                count: 0,
                width: 0.,
                height: 0.,
                enemy_type_map: HashMap::new(),
                shadow: None,
            },
        }
    }
    pub fn register_enemys(&mut self, canvas_height: f64) {
        let image_type1_front = self.images_list.get(&ImageType::OctopusOpenFront).unwrap();
        let image_type2_front = self.images_list.get(&ImageType::OctopusCloseFront).unwrap();
        let invader_column = 11;
        // 縦横の間隔
        let gap_x = 47.;
        let gap_y = 45.;

        let mut invader_pos = Vec2::new(100., canvas_height - 300.);
        for _ in 0..2 {
            for _ in 0..invader_column {
                self.enemys_list.push(Enemy {
                    enemy_type: EnemyType::Octopus,
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
                });
                invader_pos.x += gap_x;
            }
            invader_pos.x = 100.;
            invader_pos.y -= gap_y;
        }

        let image_type1_front = self.images_list.get(&ImageType::CrabBanzaiFront).unwrap();
        let image_type2_front = self.images_list.get(&ImageType::CrabDownFront).unwrap();

        for _ in 0..2 {
            for _ in 0..invader_column {
                self.enemys_list.push(Enemy {
                    enemy_type: EnemyType::Crab,
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
                });
                invader_pos.x += gap_x;
            }
            invader_pos.x = 100.;
            invader_pos.y -= gap_y;
        }
        let image_type1_front = self.images_list.get(&ImageType::SquidOpenFront).unwrap();
        let image_type2_front = self.images_list.get(&ImageType::SquidCloseFront).unwrap();

        for _ in 0..invader_column {
            self.enemys_list.push(Enemy {
                enemy_type: EnemyType::Squid,
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
            });
            invader_pos.x += gap_x;
        }

        self.enemys_list[0].move_turn = true;
        // 爆発画像を登録
        let explosion_turquoise = self
            .images_list
            .get(&ImageType::ExplosionTurquoise)
            .unwrap();
        let explosion_purple = self.images_list.get(&ImageType::ExplosionPurple).unwrap();
        let explosion_green = self.images_list.get(&ImageType::ExpolsionGreen).unwrap();
        let explosion_shadow = self.images_list.get(&ImageType::ExplosionShadow).unwrap();
        let mut explosion_image = HashMap::new();
        explosion_image.insert(EnemyType::Octopus, explosion_purple.clone());
        explosion_image.insert(EnemyType::Crab, explosion_turquoise.clone());
        explosion_image.insert(EnemyType::Squid, explosion_green.clone());
        self.explosion = Explosion {
            show: None,
            width: explosion_turquoise.width() as f64 * 3.,
            height: explosion_turquoise.height() as f64 * 3.,
            enemy_type_map: explosion_image,
            shadow: Some(explosion_shadow.clone()),
            ..self.explosion
        }
    }

    pub fn update(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        player_bullet: &mut crate::player::Bullet,
    ) {
        if let Some(_) = self.explosion.show {
            // 爆発エフェクト表示
            self.explosion.update_render(ctx, player_bullet);
            // 爆発エフェクト表示中は敵の動きをすべて止める
            return;
        }

        // 各敵個体の移動処理
        self.enemys_list.iter_mut().for_each(|enemy| {
            enemy.update(self.move_dir, player_bullet, &mut self.explosion);
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
