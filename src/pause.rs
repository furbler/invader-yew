pub struct Pause {
    // pause: bool,
    pre_pause_button: bool,
}

impl Pause {
    pub fn new() -> Self {
        Pause {
            // pause: false,
            pre_pause_button: false,
        }
    }
    // ポーズ停止または解除する瞬間のみ真を返す
    pub fn toggle_pause(&mut self, pause_button: bool) -> bool {
        // ポーズボタンが押された
        if !self.pre_pause_button && pause_button {
            // ポーズ開始/解除
            // self.pause = !self.pause;
            self.pre_pause_button = true;
            return true;
        }
        if self.pre_pause_button && !pause_button {
            // ポーズボタンが押されなくなった
            self.pre_pause_button = false;
        }
        false
    }
}
