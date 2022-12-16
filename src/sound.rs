use anyhow::anyhow;
use js_sys::ArrayBuffer;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, AudioBuffer, AudioBufferSourceNode, AudioContext, Response};

//サウンドを取得
pub async fn ret_audio() -> Audio {
    let mut audio = Audio::new();
    let mut se = Vec::new();
    // インベーダーの移動音は再生する順番に保存する
    se.push(audio.load_sound("sound/fastinvader1.wav").await.unwrap());
    se.push(audio.load_sound("sound/fastinvader2.wav").await.unwrap());
    se.push(audio.load_sound("sound/fastinvader3.wav").await.unwrap());
    se.push(audio.load_sound("sound/fastinvader4.wav").await.unwrap());
    audio.invader_move = se;

    audio.player_shot = Some(audio.load_sound("sound/shoot.wav").await.unwrap());
    audio.invader_explosion = Some(audio.load_sound("sound/invader_killed.wav").await.unwrap());
    audio.player_explosion = Some(
        audio
            .load_sound("sound/player_explosion.wav")
            .await
            .unwrap(),
    );
    audio.ufo_flying = Some(audio.load_sound("sound/ufo_flying.wav").await.unwrap());
    audio.ufo_explosion = Some(audio.load_sound("sound/ufo_explosion.wav").await.unwrap());

    audio
}

#[derive(Clone)]
pub struct Audio {
    context: AudioContext,
    pub invader_move: Vec<Sound>,
    pub player_shot: Option<Sound>,
    pub invader_explosion: Option<Sound>,
    pub player_explosion: Option<Sound>,
    pub ufo_flying: Option<Sound>,
    pub ufo_explosion: Option<Sound>,
}

impl Audio {
    pub fn new() -> Self {
        Audio {
            context: create_audio_context().unwrap(),
            invader_move: Vec::new(),
            player_shot: None,
            invader_explosion: None,
            player_explosion: None,
            ufo_flying: None,
            ufo_explosion: None,
        }
    }
    // ファイル名から音声データを取得
    pub async fn load_sound(&self, filename: &str) -> Result<Sound, ()> {
        let array_buffer = fetch_array_buffer(filename).await.unwrap();
        let audio_buffer = decode_audio_data(&self.context, &array_buffer)
            .await
            .map_err(|err| log::info!("error converting fetch to Response {:#?}", err))?;
        // 音声データ毎にデフォルトの音量を設定
        let default_volume: f32 = match filename {
            "sound/ufo_flying.wav" | "sound/ufo_explosion.wav" => 0.03,
            "sound/shoot.wav" => 0.1,
            "sound/invader_killed.wav" => 0.1,
            _ => 0.4,
        };
        Ok(Sound {
            buffer: audio_buffer,
            default_volume,
            volume: default_volume,
        })
    }
    //サウンドを一度だけ再生
    pub fn play_once_sound(&self, sound: &Sound) {
        self.play_sound(&sound.buffer, sound.volume, false);
    }
    //サウンドをループ再生
    pub fn play_looping_sound(&self, sound: &Sound) -> AudioBufferSourceNode {
        self.play_sound(&sound.buffer, sound.volume, true)
    }

    fn play_sound(
        &self,
        buffer: &AudioBuffer,
        volume: f32,
        looping: bool,
    ) -> AudioBufferSourceNode {
        let track_source = create_track_source(&self.context, buffer);
        let gain_node = self.context.create_gain().unwrap();
        // 音量設定
        gain_node.gain().set_value(volume);
        track_source.connect_with_audio_node(&gain_node).unwrap();
        gain_node
            .connect_with_audio_node(&self.context.destination())
            .unwrap();
        // ループ処理
        if looping {
            track_source.set_loop(true);
        }
        // 再生
        track_source
            .start()
            .map_err(|err| log::info!("Could not start sound! {:#?}", err))
            .unwrap();
        track_source
    }
    // 音量を初期値に戻す
    pub fn reset_volume(&mut self) {
        for invader in self.invader_move.iter_mut() {
            invader.volume = invader.default_volume;
        }
        if let Some(sound) = self.invader_explosion.as_mut() {
            sound.volume = sound.default_volume;
        }
        if let Some(sound) = self.player_shot.as_mut() {
            sound.volume = sound.default_volume;
        }
        if let Some(sound) = self.player_explosion.as_mut() {
            sound.volume = sound.default_volume;
        }
        if let Some(sound) = self.ufo_flying.as_mut() {
            sound.volume = sound.default_volume;
        }
        if let Some(sound) = self.ufo_explosion.as_mut() {
            sound.volume = sound.default_volume;
        }
    }
    // 音量を上げる
    pub fn all_volume_up(&mut self) {
        self.all_volume_change(0.1);
    }
    // 音量を下げる
    pub fn all_volume_down(&mut self) {
        self.all_volume_change(-0.1);
    }

    // 値が一定範囲内に収まるように変更する
    fn all_volume_change(&mut self, diff: f32) {
        // 最小音量値
        let min = 0.;
        // 最大音量値
        let max = 1.5;
        for invader in self.invader_move.iter_mut() {
            let volume = invader.volume + diff;
            invader.volume = Self::limit_value(volume, min, max);
        }
        if let Some(sound) = self.invader_explosion.as_mut() {
            let volume = sound.volume + diff;
            sound.volume = Self::limit_value(volume, min, max);
        }
        if let Some(sound) = self.player_shot.as_mut() {
            let volume = sound.volume + diff;
            sound.volume = Self::limit_value(volume, min, max);
        }
        if let Some(sound) = self.player_explosion.as_mut() {
            let volume = sound.volume + diff;
            sound.volume = Self::limit_value(volume, min, max);
        }
        if let Some(sound) = self.ufo_flying.as_mut() {
            let volume = sound.volume + diff;
            sound.volume = Self::limit_value(volume, min, max);
        }
        if let Some(sound) = self.ufo_explosion.as_mut() {
            let volume = sound.volume + diff;
            sound.volume = Self::limit_value(volume, min, max);
        }
    }
    // 指定した範囲に収めた値を返す
    fn limit_value(value: f32, min: f32, max: f32) -> f32 {
        if min > max {
            log::info!("範囲指定に間違いがあります。");
            value
        } else if value < min {
            min
        } else if max < value {
            max
        } else {
            value
        }
    }
}

#[derive(Clone)]
pub struct Sound {
    buffer: AudioBuffer,
    volume: f32,         // 再生時の音量
    default_volume: f32, // 音量の初期値
}

async fn fetch_array_buffer(resource: &str) -> Result<ArrayBuffer, JsValue> {
    let response = fetch_response(resource).await.unwrap();
    let array_buffer = response
        .array_buffer()
        .map_err(|err| log::info!("Error loading array buffer {:#?}", err))
        .unwrap();

    JsFuture::from(array_buffer).await.unwrap().dyn_into()
}
async fn fetch_response(resource: &str) -> Result<Response, ()> {
    fetch_with_str(resource)
        .await?
        .dyn_into()
        .map_err(|err| log::info!("error converting fetch to Response {:#?}", err))
}
async fn fetch_with_str(resource: &str) -> Result<JsValue, ()> {
    let promise = window().unwrap().fetch_with_str(resource);
    JsFuture::from(promise)
        .await
        .map_err(|err| log::info!("error fetching {:#?}", err))
}

fn create_audio_context() -> anyhow::Result<AudioContext> {
    AudioContext::new().map_err(|err| anyhow!("Could not create audio context: {:#?}", err))
}

fn create_buffer_source(ctx: &AudioContext) -> anyhow::Result<AudioBufferSourceNode> {
    ctx.create_buffer_source()
        .map_err(|err| anyhow!("Error creating buffer source {:#?}", err))
}

fn create_track_source(ctx: &AudioContext, buffer: &AudioBuffer) -> AudioBufferSourceNode {
    let track_source = create_buffer_source(ctx).unwrap();
    track_source.set_buffer(Some(buffer));
    track_source
}

//ArrayBufferをAudioBufferに変換する
async fn decode_audio_data(
    ctx: &AudioContext,
    array_buffer: &ArrayBuffer,
) -> anyhow::Result<AudioBuffer> {
    JsFuture::from(
        ctx.decode_audio_data(array_buffer)
            .map_err(|err| anyhow!("Could not decode audio from array buffer {:#?}", err))?,
    )
    .await
    .map_err(|err| anyhow!("Could not convert promise to future {:#?}", err))?
    .dyn_into()
    .map_err(|err| anyhow!("Could not cast into AudioBuffer {:#?}", err))
}
