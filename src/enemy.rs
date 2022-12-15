use std::collections::HashMap;
use web_sys::CanvasRenderingContext2d;
use web_sys::ImageBitmap;

use crate::dot_data::Color;
use crate::draw_background_rect;
use crate::load_image::ImageType;
use crate::math::Vec2;
use crate::pixel_ctrl;
use crate::player;
use crate::sound::Audio;

#[derive(Eq, Hash, PartialEq)]
enum EnemyType {
    Octopus,
    Crab,
    Squid,
}
struct Bullet {
    width: f64,    // 描画サイズの幅 [pixel]
    height: f64,   // 描画サイズの高さ [pixel]
    pos: Vec2,     // 移動後の中心位置
    pre_pos: Vec2, // 前回描画時の中心位置
    live: bool,    // 弾が画面中に存在しているか否か
    image: ImageBitmap,
    explosion: BulletExplosion,
}

impl Bullet {
    // 弾を指定された場所から発射
    fn set(&mut self, pos: Vec2) {
        self.pos = pos;
        self.pre_pos = self.pos;
        self.live = true;
    }

    fn update(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        canvas_width: f64,
        canvas_height: f64,
        player: &mut player::Player,
    ) {
        if !self.live {
            return;
        }
        //弾が存在していたら移動する
        self.pos.y += 3.;
        // 赤線の当たりに着弾した場合
        if self.pos.y > canvas_height - 52. {
            // 弾を消す
            self.live = false;
            draw_background_rect(
                ctx,
                self.pre_pos.x - self.width / 2.,
                self.pre_pos.y - self.height / 2.,
                self.width,
                self.height,
            );
            self.explosion.pos = self.pos;
            self.explosion.effect_cnt = Some(20);
            return;
        }
        //プレイヤーと衝突した場合
        if Vec2::new(self.pos.x, self.pos.y - self.height / 2.).collision(
            &player.pos,
            player.width,
            player.height,
        ) {
            //撃破後の状態でなければ
            if player.break_cnt == None {
                //プレイヤーを消す
                player.break_cnt = Some(player.revival_set_cnt);
                // 弾を消す
                self.live = false;
                draw_background_rect(
                    ctx,
                    self.pre_pos.x - self.width / 2.,
                    self.pre_pos.y - self.height / 2.,
                    self.width,
                    self.height,
                );
            }
        }

        // トーチカへの着弾確認
        // とりあえず弾の周りのデータまであれば十分
        // 弾の中心付近の座標
        let left_pos = Vec2::new(self.pos.x - self.width / 2. - 2., self.pos.y);
        let left_collision = pixel_ctrl::detect_pixel_diff(
            canvas_width,
            left_pos,
            Color::Red,
            ctx.get_image_data(0., 0., canvas_width, self.pos.y + self.height)
                .unwrap(),
        );

        let right_pos = Vec2::new(self.pos.x + self.width / 2. + 2., self.pos.y);
        let right_collision = pixel_ctrl::detect_pixel_diff(
            canvas_width,
            right_pos,
            Color::Red,
            ctx.get_image_data(0., 0., canvas_width, self.pos.y + self.height)
                .unwrap(),
        );
        //トーチカに触れていた場合
        if left_collision || right_collision {
            // 弾を消す
            self.live = false;
            draw_background_rect(
                ctx,
                self.pre_pos.x - self.width / 2.,
                self.pre_pos.y - self.height / 2.,
                self.width,
                self.height,
            );
            self.explosion.pos = self.pos;
            self.explosion.effect_cnt = Some(20);
        }
    }
    fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        if self.live {
            draw_background_rect(
                ctx,
                self.pre_pos.x - self.width / 2.,
                self.pre_pos.y - self.height / 2.,
                self.width,
                self.height,
            );
            // 表画像
            ctx.draw_image_with_image_bitmap_and_dw_and_dh(
                &self.image,
                self.pos.x - self.width / 2.,
                self.pos.y - self.height / 2.,
                self.width,
                self.height,
            )
            .unwrap();
            self.pre_pos = self.pos;
        }
        if let Some(cnt) = self.explosion.effect_cnt {
            //一定時間は表示
            if cnt > 0 {
                ctx.draw_image_with_image_bitmap_and_dw_and_dh(
                    self.explosion.image_front.as_ref().unwrap(),
                    self.explosion.pos.x - self.explosion.width / 2.,
                    self.explosion.pos.y - self.explosion.height / 2.,
                    self.explosion.width,
                    self.explosion.height,
                )
                .unwrap();
                self.explosion.effect_cnt = Some(cnt - 1);
            } else {
                //一定時間経過後は削除
                ctx.draw_image_with_image_bitmap_and_dw_and_dh(
                    self.explosion.image_shadow.as_ref().unwrap(),
                    self.explosion.pos.x - self.explosion.width / 2.,
                    self.explosion.pos.y - self.explosion.height / 2.,
                    self.explosion.width,
                    self.explosion.height,
                )
                .unwrap();
                self.explosion.effect_cnt = None;
            }
        }
    }
}
struct BulletExplosion {
    width: f64,
    height: f64,
    pos: Vec2,
    effect_cnt: Option<i32>,           //エフェクト表示中はSome(カウント)
    image_front: Option<ImageBitmap>,  // 着弾時の表画像
    image_shadow: Option<ImageBitmap>, // 着弾時の影画像
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
        player_bullet: &mut player::Bullet,
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
        move_down: bool,
        player_bullet: &mut player::Bullet,
        explosion: &mut Explosion,
        audio: &Audio,
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
                //点数を追加
                player_bullet.score.sum += {
                    match self.enemy_type {
                        EnemyType::Octopus => 10,
                        EnemyType::Crab => 20,
                        EnemyType::Squid => 30,
                    }
                };
                // 爆発エフェクトを生成
                explosion.create_effect(
                    self.pos,
                    explosion
                        .enemy_type_map
                        .get(&self.enemy_type)
                        .unwrap()
                        .clone(),
                );
                // インベーダー撃破音再生
                if let Some(sound) = &audio.invader_explosion {
                    audio.play_once_sound(sound);
                }
            }
        }
        // 動く時
        if self.move_turn {
            // 方向を考慮して動く
            self.pos.x += 10. * move_dir as f64;
            if move_down {
                self.pos.y += 23.;
            }
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
    // 移動方向反転時
    move_down: bool,
    // 種類に対応した画像を保存
    pub images_list: HashMap<ImageType, ImageBitmap>,
    // 敵一覧
    enemys_list: Vec<Enemy>,
    // 爆発エフェクト
    explosion: Explosion,
    // 敵弾3種類
    bullets: Vec<Bullet>,
    //敵の各縦列の中で一番下(射撃可能)の個体のインデックス番号
    can_shot_enemy: Vec<usize>,
    //射撃してからのフレーム数
    shot_interval: usize,
    // 前回再生した音番号
    play_sound_index: usize,
}

impl EnemyManage {
    pub fn default() -> Self {
        EnemyManage {
            left_border: 50.,
            right_border: 650.,
            move_dir: 1,
            move_dir_invert: false,
            move_down: false,
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
            bullets: Vec::new(),
            can_shot_enemy: core::array::from_fn::<usize, 11, _>(|i| i).to_vec(),
            shot_interval: 0,
            play_sound_index: 0,
        }
    }
    pub fn register_enemys(&mut self, canvas_height: f64) {
        let image_type1_front = self.images_list.get(&ImageType::OctopusOpen).unwrap();
        let image_type2_front = self.images_list.get(&ImageType::OctopusClose).unwrap();
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

        let image_type1_front = self.images_list.get(&ImageType::CrabBanzai).unwrap();
        let image_type2_front = self.images_list.get(&ImageType::CrabDown).unwrap();

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
        let image_type1_front = self.images_list.get(&ImageType::SquidOpen).unwrap();
        let image_type2_front = self.images_list.get(&ImageType::SquidClose).unwrap();

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
        };

        let image = self
            .images_list
            .get(&ImageType::EnemyBulletPlunger)
            .unwrap();
        let image_explosion_front = self
            .images_list
            .get(&ImageType::EnemyBulletExplosionFront)
            .unwrap();
        let image_explosion_shadow = self
            .images_list
            .get(&ImageType::EnemyBulletExplosionShadow)
            .unwrap();
        let bullet = Bullet {
            width: image.width() as f64 * 2.5,
            height: image.height() as f64 * 2.5,
            pos: Vec2::new(0., 0.),
            pre_pos: Vec2::new(0., 0.),
            live: false,
            image: image.clone(),
            explosion: BulletExplosion {
                width: image_explosion_front.width() as f64 * 3.,
                height: image_explosion_front.height() as f64 * 3.,
                pos: Vec2::new(0., 0.),
                effect_cnt: None,
                image_front: Some(image_explosion_front.clone()),
                image_shadow: Some(image_explosion_shadow.clone()),
            },
        };
        self.bullets.push(bullet);

        let image = self
            .images_list
            .get(&ImageType::EnemyBulletSquiggly)
            .unwrap();
        let image_explosion_front = self
            .images_list
            .get(&ImageType::EnemyBulletExplosionFront)
            .unwrap();
        let image_explosion_shadow = self
            .images_list
            .get(&ImageType::EnemyBulletExplosionShadow)
            .unwrap();
        let bullet = Bullet {
            width: image.width() as f64 * 2.5,
            height: image.height() as f64 * 2.5,
            pos: Vec2::new(0., 0.),
            pre_pos: Vec2::new(0., 0.),
            live: false,
            image: image.clone(),
            explosion: BulletExplosion {
                width: image_explosion_front.width() as f64 * 3.,
                height: image_explosion_front.height() as f64 * 3.,
                pos: Vec2::new(0., 0.),
                effect_cnt: None,
                image_front: Some(image_explosion_front.clone()),
                image_shadow: Some(image_explosion_shadow.clone()),
            },
        };
        self.bullets.push(bullet);

        let image = self
            .images_list
            .get(&ImageType::EnemyBulletRolling)
            .unwrap();
        let image_explosion_front = self
            .images_list
            .get(&ImageType::EnemyBulletExplosionFront)
            .unwrap();
        let image_explosion_shadow = self
            .images_list
            .get(&ImageType::EnemyBulletExplosionShadow)
            .unwrap();
        let bullet = Bullet {
            width: image.width() as f64 * 2.5,
            height: image.height() as f64 * 2.5,
            pos: Vec2::new(0., 0.),
            pre_pos: Vec2::new(0., 0.),
            live: false,
            image: image.clone(),
            explosion: BulletExplosion {
                width: image_explosion_front.width() as f64 * 3.,
                height: image_explosion_front.height() as f64 * 3.,
                pos: Vec2::new(0., 0.),
                effect_cnt: None,
                image_front: Some(image_explosion_front.clone()),
                image_shadow: Some(image_explosion_shadow.clone()),
            },
        };
        self.bullets.push(bullet);
    }

    pub fn update(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        canvas_width: f64,
        canvas_height: f64,
        player: &mut player::Player,
        audio: &Audio,
    ) {
        if let Some(_) = self.explosion.show {
            // 爆発エフェクト表示
            self.explosion.update_render(ctx, &mut player.bullet);
            //敵弾は動かす
            for bullet in &mut self.bullets {
                // 敵が全滅していたら発射しない
                if self.can_shot_enemy.len() == 0 {
                    return;
                }
                bullet.update(ctx, canvas_width, canvas_height, player);
            }
            // 爆発エフェクト表示中は敵の動きをすべて止める
            return;
        }

        // 各敵個体の移動処理
        self.enemys_list.iter_mut().for_each(|enemy| {
            enemy.update(
                self.move_dir,
                self.move_down,
                &mut player.bullet,
                &mut self.explosion,
                audio,
            );
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
                // 下へ移動する
                self.move_down = true;
            } else {
                // すべての個体が下への移動を終えた場合
                self.move_down = false;
            }
            // もう一週生きている個体を探す
            for (index, enemy) in self.enemys_list.iter().enumerate() {
                if enemy.live {
                    next_move_enemy_index = Some(index);
                    break;
                }
            }
            // サウンドが保存されていれば
            if audio.invader_move.len() > 0 {
                // 順番にループ
                self.play_sound_index = if self.play_sound_index >= audio.invader_move.len() {
                    0
                } else {
                    self.play_sound_index
                };
                audio.play_once_sound(&audio.invader_move[self.play_sound_index]);
                self.play_sound_index += 1;
            }
        }
        if let Some(i) = next_move_enemy_index {
            // 次に動く敵個体を指定
            self.enemys_list[i].move_turn = true;
        } else {
            log::info!("敵は全滅した。");
        }
        //射撃可能な敵個体から死んだ個体を削除
        self.update_can_shot_list();

        for bullet in &mut self.bullets {
            // 敵が全滅していたら発射しない
            if self.can_shot_enemy.len() == 0 {
                return;
            }
            //弾が消滅済みで、かつ前回の射撃から(3発の弾共通で)一定時間経過して、かつ弾の爆発エフェクト表示が終了していた場合
            if !bullet.live && self.shot_interval > 70 && bullet.explosion.effect_cnt == None {
                //疑似乱数で発射タイミングを決定(ここの発射タイミングのプログラムはあとで変える)
                let index = self.can_shot_enemy[self.shot_interval % self.can_shot_enemy.len()];
                bullet.set(Vec2::new(
                    self.enemys_list[index].pos.x,
                    self.enemys_list[index].pos.y + 20.,
                ));
                self.shot_interval = 0;
            }
            bullet.update(ctx, canvas_width, canvas_height, player);
        }
        self.shot_interval += 1;
    }
    pub fn render(&mut self, ctx: &CanvasRenderingContext2d) {
        self.enemys_list.iter_mut().for_each(|enemy| {
            enemy.render(ctx);
        });
        //弾
        for b in &mut self.bullets {
            b.render(ctx);
        }
    }
    //各縦列で射撃可能な個体の情報を更新
    fn update_can_shot_list(&mut self) {
        // 各縦列について
        for i in 0..self.can_shot_enemy.len() {
            //登録されている個体が死んでいた場合
            while !self.enemys_list[self.can_shot_enemy[i]].live {
                //一段上の個体を登録
                self.can_shot_enemy[i] += 11;
                //はみだしていたら、その縦一列は全滅状態
                if self.can_shot_enemy[i] >= self.enemys_list.len() {
                    break;
                }
            }
        }
        //はみだした部分(全滅した縦列)を消す
        self.can_shot_enemy.retain(|x| x < &self.enemys_list.len());
    }
}
