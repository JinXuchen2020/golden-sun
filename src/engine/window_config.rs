//! 窗口配置 — 一处修改全局生效

use crate::engine::constants;

/// 窗口配置
#[derive(Debug, Clone, Copy)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
    pub fullscreen: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: constants::WINDOW_WIDTH,
            height: constants::WINDOW_HEIGHT,
            fullscreen: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_width_matches_constant() {
        let cfg = WindowConfig::default();
        assert!((cfg.width - constants::WINDOW_WIDTH).abs() < f32::EPSILON);
    }

    #[test]
    fn default_height_matches_constant() {
        let cfg = WindowConfig::default();
        assert!((cfg.height - constants::WINDOW_HEIGHT).abs() < f32::EPSILON);
    }

    #[test]
    fn default_fullscreen_is_false() {
        let cfg = WindowConfig::default();
        assert!(!cfg.fullscreen);
    }
}
