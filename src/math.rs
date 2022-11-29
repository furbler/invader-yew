#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}
impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    // 幅width、高さheight、中心位置center_posの矩形の中にselfの点が存在する場合に真を返す
    pub fn collision(&self, center_pos: &Vec2, width: f64, height: f64) -> bool {
        let left = center_pos.x - width / 2.;
        let right = center_pos.x + width / 2.;
        let top = center_pos.y - height / 2.;
        let bottom = center_pos.y + height / 2.;

        if left < self.x && self.x < right && top < self.y && self.y < bottom {
            true
        } else {
            false
        }
    }
}
