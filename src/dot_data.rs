use std::io::Write;

pub enum Color {
    Turquoise,  // 青緑色
    Purple,     // 紫色
    Green,      // 緑色
    Red,        // 赤色
    Yellow,     //黄色
    Background, // 背景色
}
pub struct DotShape {
    pub width: u32,              // 幅[ドット]
    pub height: u32,             // 高さ[ドット]
    pub dot_map: Vec<Vec<bool>>, // 描画部分を真、透過部分を偽で表す
}

impl DotShape {
    // 真偽値で表されたドットマップを、1pixelをrgbaの4バイトで表すVec<u8>に変換
    pub fn create_color_dot_map(&self, color: Color) -> Vec<u8> {
        // 指定されたサイズと実際のドットマップのサイズが一致しているか確認
        if self.height as usize != self.dot_map.len() {
            panic!("指定されたドットマップの高さが実際のデータと異なります。");
        }
        if self.width as usize != self.dot_map[0].len() {
            panic!("指定されたドットマップの幅が実際のデータと異なります。");
        }
        // ドットマップの幅が異なる行が無いか確認
        let map_width = self.dot_map[0].len();
        for l in &self.dot_map {
            if l.len() != map_width {
                panic!("ドットマップの形が不正です。");
            }
        }
        // ドット絵を描画する部分
        let color = set_color(color);
        // 背景を透過する部分
        let transparent = vec![0, 0, 0, 0];

        let mut bytes: Vec<u8> = Vec::new();
        for line in &self.dot_map {
            for c in line {
                if *c {
                    bytes.write(&color).unwrap();
                } else {
                    bytes.write(&transparent).unwrap();
                }
            }
        }
        bytes
    }
}

// ドットデータを変更する際はこの中身のみ変更する
pub fn ret_dot_data(name: &str) -> DotShape {
    let player = DotShape {
        width: 13,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ _ # _ _ _ _ _ _",
            "_ _ _ _ _ # # # _ _ _ _ _",
            "_ _ _ _ _ # # # _ _ _ _ _",
            "_ # # # # # # # # # # # _",
            "# # # # # # # # # # # # #",
            "# # # # # # # # # # # # #",
            "# # # # # # # # # # # # #",
            "# # # # # # # # # # # # #",
        ]),
    };
    let bullet_player = DotShape {
        width: 1,
        height: 6,
        dot_map: convert_dot_map(vec!["#", "#", "#", "#", "#", "#"]),
    };
    let crab_down = DotShape {
        width: 11,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ # _ _ _ _ _ # _ _",
            "_ _ _ # _ _ _ # _ _ _",
            "_ _ # # # # # # # _ _",
            "_ # # _ # # # _ # # _",
            "# # # # # # # # # # #",
            "# _ # # # # # # # _ #",
            "# _ # _ _ _ _ _ # _ #",
            "_ _ _ # # _ # # _ _ _",
        ]),
    };
    let crab_banzai = DotShape {
        width: 11,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ # _ _ _ _ _ # _ _",
            "# _ _ # _ _ _ # _ _ #",
            "# _ # # # # # # # _ #",
            "# # # _ # # # _ # # #",
            "# # # # # # # # # # #",
            "_ # # # # # # # # # _",
            "_ _ # _ _ _ _ _ # _ _",
            "_ # _ _ _ _ _ _ _ # _",
        ]),
    };

    let octopus_open = DotShape {
        width: 12,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ # # # # _ _ _ _",
            "_ # # # # # # # # # # _",
            "# # # # # # # # # # # #",
            "# # # _ _ # # _ _ # # #",
            "# # # # # # # # # # # #",
            "_ _ _ # # _ _ # # _ _ _",
            "_ _ # # _ # # _ # # _ _",
            "# # _ _ _ _ _ _ _ _ # #",
        ]),
    };

    let octopus_close = DotShape {
        width: 12,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ # # # # _ _ _ _",
            "_ # # # # # # # # # # _",
            "# # # # # # # # # # # #",
            "# # # _ _ # # _ _ # # #",
            "# # # # # # # # # # # #",
            "_ _ # # # _ _ # # # _ _",
            "_ # # _ _ # # _ _ # # _",
            "_ _ # # _ _ _ _ # # _ _",
        ]),
    };
    let squid_open = DotShape {
        width: 8,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ # # _ _ _",
            "_ _ # # # # _ _",
            "_ # # # # # # _",
            "# # _ # # _ # #",
            "# # # # # # # #",
            "_ _ # _ _ # _ _",
            "_ # _ # # _ # _",
            "# _ # _ _ # _ #",
        ]),
    };
    let squid_close = DotShape {
        width: 8,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ _ # # _ _ _",
            "_ _ # # # # _ _",
            "_ # # # # # # _",
            "# # _ # # _ # #",
            "# # # # # # # #",
            "_ # _ # # _ # _",
            "# _ _ _ _ _ _ #",
            "_ # _ _ _ _ # _",
        ]),
    };
    let explosion = DotShape {
        width: 13,
        height: 7,
        dot_map: convert_dot_map(vec![
            "_ # _ _ # _ _ _ # _ _ # _",
            "_ _ # _ _ # _ # _ _ # _ _",
            "_ _ _ # _ _ _ _ _ # _ _ _",
            "# # _ _ _ _ _ _ _ _ _ # #",
            "_ _ _ # _ _ _ _ _ # _ _ _",
            "_ _ # _ _ # _ # _ _ # _ _",
            "_ # _ _ # _ _ _ # _ _ # _",
        ]),
    };

    let land_player_bullet = DotShape {
        width: 8,
        height: 8,
        dot_map: convert_dot_map(vec![
            "# _ _ _ # _ _ #",
            "_ _ # _ _ _ # _",
            "_ # # # # # # _",
            "# # # # # # # #",
            "# # # # # # # #",
            "_ # # # # # # _",
            "_ _ # _ _ # _ _",
            "# _ _ # _ _ _ #",
        ]),
    };

    let torchika = DotShape {
        width: 20,
        height: 15,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ # # # # # # # # # # # # _ _ _ _",
            "_ _ _ # # # # # # # # # # # # # # _ _ _",
            "_ _ # # # # # # # # # # # # # # # # _ _",
            "_ # # # # # # # # # # # # # # # # # # _",
            "# # # # # # # # # # # # # # # # # # # #",
            "# # # # # # # # # # # # # # # # # # # #",
            "# # # # # # # # # # # # # # # # # # # #",
            "# # # # # # # # # # # # # # # # # # # #",
            "# # # # # # # # # # # # # # # # # # # #",
            "# # # # # # # # # # # # # # # # # # # #",
            "# # # # # # # # # # # # # # # # # # # #",
            "# # # # # # # _ _ _ _ _ _ # # # # # # #",
            "# # # # # # _ _ _ _ _ _ _ _ # # # # # #",
            "# # # # # _ _ _ _ _ _ _ _ _ _ # # # # #",
            "# # # # # _ _ _ _ _ _ _ _ _ _ # # # # #",
        ]),
    };

    let ufo = DotShape {
        width: 16,
        height: 7,
        dot_map: convert_dot_map(vec![
            "_ _ _ _ _ # # # # # # _ _ _ _ _",
            "_ _ _ # # # # # # # # # # _ _ _",
            "_ _ # # # # # # # # # # # # _ _",
            "_ # # _ # # _ # # _ # # _ # # _",
            "# # # # # # # # # # # # # # # #",
            "_ _ # # # _ _ # # _ _ # # # _ _",
            "_ _ _ # _ _ _ _ _ _ _ _ # _ _ _",
        ]),
    };

    let ufo_explosion = DotShape {
        width: 21,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ # _ _ # _ # _ _ _ _ _ _ # _ # # _ # _",
            "_ _ _ # _ _ _ _ _ _ _ _ # # _ _ _ _ # _ _",
            "# _ # _ _ _ # # # # _ _ _ # # _ _ _ _ _ _",
            "_ _ _ _ _ # # # # # # # _ _ # # # _ _ # _",
            "_ _ _ _ # # # _ # _ # _ # _ _ # # # _ _ #",
            "_ _ # _ _ _ # # # # # _ _ _ # # _ _ _ _ _",
            "# _ _ _ _ _ _ # _ # _ _ _ # # _ _ _ # _ _",
            "_ _ # _ _ _ # _ _ _ # _ _ _ _ # _ _ _ _ _",
        ]),
    };
    // ジグザク型
    let enemy_bullet_squiggly = DotShape {
        width: 3,
        height: 7,
        dot_map: convert_dot_map(vec![
            "_ # _", "# _ _", "_ # _", "_ _ #", "_ # _", "# _ _", "_ # _",
        ]),
    };
    // T字型
    let enemy_bullet_plunger = DotShape {
        width: 3,
        height: 7,
        dot_map: convert_dot_map(vec![
            "_ # _", "_ # _", "# # #", "_ # _", "_ # _", "_ # _", "_ # _",
        ]),
    };
    // 螺旋状
    let enemy_bullet_rolling = DotShape {
        width: 3,
        height: 7,
        dot_map: convert_dot_map(vec![
            "_ # #", "_ # _", "# # _", "_ # #", "_ # _", "# # _", "_ # _",
        ]),
    };

    let enemy_bullet_explosion = DotShape {
        width: 6,
        height: 8,
        dot_map: convert_dot_map(vec![
            "_ _ # _ _ _",
            "# _ _ _ # _",
            "_ _ # # _ #",
            "_ # # # # _",
            "# _ # # # _",
            "_ # # # # #",
            "# _ # # # _",
            "_ # _ # _ #",
        ]),
    };
    match name {
        "player" => player,
        "player_bullet" => bullet_player,
        "crab_down" => crab_down,
        "crab_banzai" => crab_banzai,
        "octopus_open" => octopus_open,
        "octopus_close" => octopus_close,
        "squid_open" => squid_open,
        "squid_close" => squid_close,
        "explosion" => explosion,
        "land_player_bullet" => land_player_bullet,
        "torchika" => torchika,
        "ufo" => ufo,
        "ufo_explosion" => ufo_explosion,
        "enemy_bullet_squiggly" => enemy_bullet_squiggly,
        "enemy_bullet_plunger" => enemy_bullet_plunger,
        "enemy_bullet_rolling" => enemy_bullet_rolling,
        "enemy_bullet_explosion" => enemy_bullet_explosion,
        _ => panic!(
            "{}のドットマップ取得に失敗しました。プログラムを終了します。",
            name
        ), // ドットマップ取得失敗
    }
}
// 描画部分を真、非描画部分を偽とするドットマップを返す
fn convert_dot_map(dot_map: Vec<&str>) -> Vec<Vec<bool>> {
    let mut bool_map = Vec::new();
    for line in dot_map {
        let mut bool_line = Vec::new();
        // 空白をすべて削除
        let space_removed = line.replace(' ', "");
        for c in space_removed.chars() {
            if c == '#' {
                bool_line.push(true);
            } else if c == '_' {
                bool_line.push(false);
            }
        }
        bool_map.push(bool_line);
    }
    bool_map
}

// 指定した色に対応するrgbaの値を返す
pub fn set_color(color: Color) -> Vec<u8> {
    match color {
        Color::Turquoise => vec![68, 200, 210, 255], // 青緑色
        Color::Purple => vec![219, 85, 221, 255],    // 紫色
        Color::Green => vec![98, 222, 109, 255],     // 緑色
        Color::Red => vec![210, 0, 0, 255],          // 赤色
        Color::Yellow => vec![190, 180, 80, 255],    //黄色
        Color::Background => vec![0, 0, 0, 255],     // 背景色
    }
}
