//! 全局游戏状态机 + 过渡动画类型

use crate::PsynergyType;
use crate::dialogue::script::{DialogueChoice, DialogueScript};

/// 过渡动画类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionKind {
    FadeIn,
    FadeOut,
    Wipe,
}

/// 精灵力施法动画状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PsynergyAnim {
    /// 动画计时器（秒），PSYNERGY_ANIM_DURATION 时结束
    pub timer: f32,
    /// 正在施放的精灵力类型
    pub psynergy: PsynergyType,
    /// 目标 tile 坐标
    pub tx: i32,
    pub ty: i32,
}

impl PsynergyAnim {
    /// 施法动画持续时长（秒）
    pub const DURATION: f32 = 1.2;

    /// 动画进度 0.0 → 1.0
    #[must_use]
    pub const fn progress(&self) -> f32 {
        (self.timer / Self::DURATION).clamp(0.0, 1.0)
    }

    /// 动画是否已结束
    #[must_use]
    pub const fn is_finished(&self) -> bool {
        self.timer >= Self::DURATION
    }
}

/// 全局游戏状态机
///
/// 注意：包含 `f32` 字段（`Transition.timer`、`PsynergyAnim.timer`），因此不能派生 `Eq`
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum GameState {
    Title,
    WorldMap,
    Dialog,
    Battle,
    Menu,
    Psynergy,
    Transition { kind: TransitionKind, timer: f32, from: &'static str, to: &'static str },
    /// 精灵力施法动画 — 动画期间锁定输入，结束后才修改 tile
    PsynergyAnim { anim: PsynergyAnim },
    /// 对话分支选择 — 显示可选项列表
    DialogueChoices { choices: &'static [DialogueChoice], script: DialogueScript },
    /// 传送菜单 — 显示已激活的传送点列表
    Travel { selection: usize },
    /// Djinn 菜单 — 显示已收集的 Djinn 列表，允许装备/卸下
    DjinnMenu { selection: usize, page: usize, character_select: u32 },
}

impl GameState {
    /// 当前是否处于过渡动画（输入锁定）
    #[must_use]
    pub const fn is_transition(&self) -> bool {
        matches!(self, GameState::Transition { .. })
    }

    /// 当前是否处于精灵力施法动画（输入锁定）
    #[must_use]
    pub const fn is_psynergy_anim(&self) -> bool {
        matches!(self, GameState::PsynergyAnim { .. })
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

    #[test]
    fn psynergy_anim_state_blocks_input() {
        let state = GameState::PsynergyAnim {
            anim: PsynergyAnim { timer: 0.0, psynergy: PsynergyType::Whirlwind, tx: 10, ty: 10 },
        };
        assert!(state.is_psynergy_anim());
        assert!(!state.is_transition());
    }

    #[test]
    fn psynergy_anim_progress() {
        let anim = PsynergyAnim { timer: 0.0, psynergy: PsynergyType::Growth, tx: 0, ty: 0 };
        assert_eq!(anim.progress(), 0.0);

        let anim = PsynergyAnim { timer: PsynergyAnim::DURATION * 0.5, psynergy: PsynergyType::Freeze, tx: 0, ty: 0 };
        assert_eq!(anim.progress(), 0.5);

        let anim = PsynergyAnim { timer: PsynergyAnim::DURATION * 2.0, psynergy: PsynergyType::Force, tx: 0, ty: 0 };
        assert_eq!(anim.progress(), 1.0);
    }

    #[test]
    fn psynergy_anim_finished() {
        let early = PsynergyAnim { timer: 0.5, psynergy: PsynergyType::Whirlwind, tx: 0, ty: 0 };
        assert!(!early.is_finished());

        let done = PsynergyAnim { timer: PsynergyAnim::DURATION, psynergy: PsynergyType::Flash, tx: 0, ty: 0 };
        assert!(done.is_finished());
    }
}
