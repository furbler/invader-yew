pub struct Pause {
    pause: bool,
    pre_pause_button: bool,
}

impl Pause {
    pub fn new() -> Self {
        Pause {
            pause: false,
            pre_pause_button: false,
        }
    }
    pub fn toggle_pause(&mut self, pause_button: bool) -> bool {
        // ポーズボタンが押された
        if !self.pre_pause_button && pause_button {
            // ポーズ開始/解除
            self.pause = !self.pause;
            self.pre_pause_button = true;
        }
        // ポーズボタンが押されなくなった
        if self.pre_pause_button && !pause_button {
            self.pre_pause_button = false;
        }
        self.pause
    }
}
