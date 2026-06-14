//! 帧时序信息

use crate::engine::constants;
use macroquad::prelude::*;

/// 帧时序信息
#[derive(Debug, Clone, Copy)]
pub struct FrameTime {
    pub delta: f32,
    pub elapsed: f32,
}

impl FrameTime {
    pub const fn new() -> Self {
        Self { delta: 0.0, elapsed: 0.0 }
    }

    /// 从 macroquad 帧状态刷新（带 delta 裁剪保护）
    pub fn poll(&mut self) {
        let raw = get_frame_time();
        self.delta = raw.clamp(constants::DELTA_MIN, constants::DELTA_MAX);
        self.elapsed += self.delta;
    }
}

impl Default for FrameTime {
    fn default() -> Self {
        Self::new()
    }
}
