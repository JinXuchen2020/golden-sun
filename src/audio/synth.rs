//! 方波合成器 — 生成 GBA 风格音效样本
//!
//! 生成 f32 PCM 样本数据供 `ResourceManager::store_audio()` 使用。

use crate::engine::constants::AUDIO_SAMPLE_RATE;
use crate::engine::resources::AudioSample;

/// 生成方波音频样本
///
/// - `freq`: 频率（Hz）
/// - `duration_s`: 持续时间（秒）
/// - `volume`: 音量 [0.0, 1.0]
pub fn generate_square_wave(freq: f32, duration_s: f32, volume: f32) -> AudioSample {
    let num_samples = (AUDIO_SAMPLE_RATE as f32 * duration_s) as usize;
    let mut data = Vec::with_capacity(num_samples);
    let phase_inc = freq / AUDIO_SAMPLE_RATE as f32;
    let inv_n = 1.0 / num_samples as f32;
    let mut phase = 0.0;

    for i in 0..num_samples {
        let envelope = 1.0 - i as f32 * inv_n;
        data.push(if phase < 0.5 { volume * envelope } else { -volume * envelope });
        phase = (phase + phase_inc).fract();
    }

    AudioSample { data, sample_rate: AUDIO_SAMPLE_RATE }
}
