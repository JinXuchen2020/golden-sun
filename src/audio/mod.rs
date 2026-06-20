//! 音频系统 — GBA 风格方波音效合成 + BGM 循环

pub mod synth;

use macroquad::audio::{self, Sound, PlaySoundParams};
use std::collections::HashMap;

// ── WAV 工具 ──

/// PCM f32 数据 → WAV 格式字节
fn pcm_to_wav(pcm: &[f32], sample_rate: u32) -> Vec<u8> {
    let data_size = pcm.len() * 2;
    let mut wav = Vec::with_capacity(44 + data_size);

    wav.extend(b"RIFF");
    wav.extend(&(36 + data_size as u32).to_le_bytes());
    wav.extend(b"WAVE");
    wav.extend(b"fmt ");
    wav.extend(&16u32.to_le_bytes());
    wav.extend(&1u16.to_le_bytes());    // PCM
    wav.extend(&1u16.to_le_bytes());    // mono
    wav.extend(&sample_rate.to_le_bytes());
    wav.extend(&(sample_rate * 2).to_le_bytes());
    wav.extend(&2u16.to_le_bytes());
    wav.extend(&16u16.to_le_bytes());
    wav.extend(b"data");
    wav.extend(&(data_size as u32).to_le_bytes());

    for &sample in pcm {
        let val = (sample * 32767.0) as i16;
        wav.extend(&val.to_le_bytes());
    }
    wav
}

/// 异步加载 PCM 为 macroquad Sound
async fn load_audio(pcm: Vec<f32>, sample_rate: u32) -> Sound {
    let wav = pcm_to_wav(&pcm, sample_rate);
    audio::load_sound_from_bytes(&wav).await.unwrap()
}

// ── BGM 音序 ──

const SAMPLE_RATE: u32 = 22050; // 降低采样率减少内存

/// 生成 Vale 村主题循环（4 小节，G 大调）
fn generate_vale_theme() -> Vec<f32> {
    // 音符频率表：G4 A4 B4 C5 D5 E5 F#5 G5
    let notes = [392.0, 440.0, 493.9, 523.3, 587.3, 659.3, 740.0, 784.0];
    // 音序：每拍 0.25s，4/4 拍 × 4 小节 = 16 拍
    let seq = [
        0,0,0,0, 1,1,2,2, 3,3,3,3, 4,4,5,5,  // 前半
        5,5,4,4, 3,3,2,2, 1,1,1,1, 0,0,0,0,  // 后半
    ];
    let beat_samples = (SAMPLE_RATE as f32 * 0.25) as usize;
    let total = seq.len() * beat_samples;
    let mut pcm = Vec::with_capacity(total);

    for &note_idx in &seq {
        let freq = notes[note_idx as usize];
        for i in 0..beat_samples {
            let t = i as f32 / beat_samples as f32;
            let phase = (i as f32 * freq / SAMPLE_RATE as f32) % 1.0;
            // 方波 + 衰减包络
            let envelope = 1.0 - t * 0.3;
            pcm.push(if phase < 0.5 { 0.2 * envelope } else { -0.2 * envelope });
        }
    }
    pcm
}

/// 生成战斗 BGM 循环（4 小节，快节奏）
fn generate_battle_theme() -> Vec<f32> {
    let notes = [130.8, 164.8, 196.0, 220.0, 261.6, 329.6, 392.0, 440.0];
    let seq = [
        0,4,0,4, 1,5,1,5, 2,6,2,6, 3,7,3,7,
        4,0,4,0, 5,1,5,1, 6,2,6,2, 7,3,7,3,
    ];
    let beat_samples = (SAMPLE_RATE as f32 * 0.15) as usize; // 更快
    let total = seq.len() * beat_samples;
    let mut pcm = Vec::with_capacity(total);

    for &note_idx in &seq {
        let freq = notes[note_idx as usize];
        for i in 0..beat_samples {
            let phase = (i as f32 * freq / SAMPLE_RATE as f32) % 1.0;
            let envelope = 1.0 - (i as f32 / beat_samples as f32) * 0.5;
            pcm.push(if phase < 0.5 { 0.3 * envelope } else { -0.3 * envelope });
        }
    }
    pcm
}

/// 生成 Boss 战斗 BGM 循环（低沉威胁感）
fn generate_boss_bgm() -> Vec<f32> {
    let sample_rate = 22050;
    let duration = 8.0;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut data = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let bass = (t * 110.0 * std::f32::consts::TAU).sin() * 0.4;
        let alt = (t * 220.0 * std::f32::consts::TAU).sin() * 0.3;
        let staccato = if (t * 4.0).fract() < 0.3 { 1.0 } else { 0.0 };
        let sample = (bass + alt * staccato) * 0.5;
        data.push(sample);
    }
    data
}

/// BGM 管理器
pub struct BgmPlayer {
    sounds: HashMap<&'static str, Sound>,
    current: Option<&'static str>,
}

impl BgmPlayer {
    /// 异步创建 BGM 播放器（在 macroquad 上下文就绪后调用）
    pub async fn new() -> Self {
        let mut sounds = HashMap::new();
        sounds.insert("vale", load_audio(generate_vale_theme(), SAMPLE_RATE).await);
        sounds.insert("battle", load_audio(generate_battle_theme(), SAMPLE_RATE).await);
        sounds.insert("boss", load_audio(generate_boss_bgm(), SAMPLE_RATE).await);
        Self { sounds, current: None }
    }

    /// 切换到指定 BGM（停止当前，播放新曲）
    pub fn play(&mut self, id: &'static str) {
        if self.current == Some(id) { return; }
        self.stop();
        if let Some(sound) = self.sounds.get(id) {
            audio::play_sound(sound, PlaySoundParams { looped: true, volume: 1.0 });
            self.current = Some(id);
        }
    }

    /// 停止当前 BGM
    pub fn stop(&mut self) {
        if let Some(id) = self.current {
            if let Some(sound) = self.sounds.get(id) {
                audio::stop_sound(sound);
            }
            self.current = None;
        }
    }
}

/// 播放一次性音效 — 使用预加载的 Sound
pub fn play_sfx(sound: &Sound) {
    macroquad::audio::play_sound_once(sound);
}

/// SFX 管理器 — 预加载音效 Sound 对象
pub struct SfxManager {
    sounds: HashMap<&'static str, Sound>,
}

impl SfxManager {
    /// 异步创建 SFX 管理器
    pub async fn new() -> Self {
        let mut sounds = HashMap::new();
        sounds.insert("confirm", generate_sfx_sound(440.0, 0.1, 0.5).await);
        sounds.insert("cancel", generate_sfx_sound(220.0, 0.08, 0.4).await);
        sounds.insert("hurt", generate_sfx_sound(180.0, 0.15, 0.6).await);
        sounds.insert("heal", generate_sfx_sound(660.0, 0.2, 0.4).await);
        sounds.insert("psynergy", generate_sfx_sound(550.0, 0.3, 0.5).await);
        sounds.insert("levelup", generate_sfx_sound(880.0, 0.3, 0.5).await);
        sounds.insert("shop_buy", generate_sfx_sound(660.0, 0.15, 0.4).await);
        sounds.insert("victory", generate_sfx_sound(440.0, 0.5, 0.6).await);
        sounds.insert("djinn", generate_sfx_sound(550.0, 0.25, 0.5).await);
        sounds.insert("summon", generate_sfx_sound(330.0, 0.5, 0.7).await);
        Self { sounds }
    }

    pub fn play(&self, name: &str) {
        if let Some(sound) = self.sounds.get(name) {
            play_sfx(sound);
        }
    }
}

/// 生成方波 PCM → WAV → Sound（异步）
async fn generate_sfx_sound(freq: f32, duration_s: f32, volume: f32) -> Sound {
    let num_samples = (SAMPLE_RATE as f32 * duration_s) as usize;
    let phase_inc = freq / SAMPLE_RATE as f32;
    let inv_n = 1.0 / num_samples as f32;
    let mut pcm = Vec::with_capacity(num_samples);
    let mut phase = 0.0;
    for i in 0..num_samples {
        let envelope = 1.0 - i as f32 * inv_n;
        pcm.push(if phase < 0.5 { volume * envelope } else { -volume * envelope });
        phase = (phase + phase_inc).fract();
    }
    let wav = pcm_to_wav(&pcm, SAMPLE_RATE);
    macroquad::audio::load_sound_from_bytes(&wav).await.unwrap()
}
