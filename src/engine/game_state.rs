//! 全局游戏状态机 + 过渡动画类型

/// 过渡动画类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionKind {
    FadeIn,
    FadeOut,
    Wipe,
}

/// 全局游戏状态机
///
/// 注意：包含 `f32` 字段（`Transition.timer`），因此不能派生 `Eq`
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum GameState {
    Title,
    WorldMap,
    Dialog,
    Battle,
    Menu,
    Psynergy,
    Transition { kind: TransitionKind, timer: f32, from: &'static str, to: &'static str },
}

impl GameState {
    /// 当前是否处于过渡动画（输入锁定）
    #[must_use]
    pub const fn is_transition(&self) -> bool {
        matches!(self, GameState::Transition { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_is_not_transition() {
        assert!(!GameState::Title.is_transition());
    }

    #[test]
    fn world_map_is_not_transition() {
        assert!(!GameState::WorldMap.is_transition());
    }

    #[test]
    fn transition_variant_is_transition() {
        let state = GameState::Transition {
            kind: TransitionKind::FadeIn,
            timer: 0.0,
            from: "Title",
            to: "Vale",
        };
        assert!(state.is_transition());
    }

    #[test]
    fn transition_kinds_are_distinct() {
        assert_ne!(TransitionKind::FadeIn as u8, TransitionKind::FadeOut as u8);
        assert_ne!(TransitionKind::FadeIn as u8, TransitionKind::Wipe as u8);
    }
}
