//! 场景管理 — 场景标识、对话引擎、过场动画
//!
//! ## 子模块
//! - `dialogue`: 对话树引擎 (Phase 4)

/// 场景标识 — 每个可加载的地图/场景对应一个 ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SceneId {
    Title,
    Vale,
    WildForest,
    Cave,
    SolSanctum,
}

impl SceneId {
    pub fn display_name(&self) -> &'static str {
        match self {
            SceneId::Title => "标题画面",
            SceneId::Vale => "Vale 村",
            SceneId::WildForest => "密林",
            SceneId::Cave => "洞穴",
            SceneId::SolSanctum => "Sol Sanctum",
        }
    }
}

/// 场景注册表 — 管理场景切换、加载和资源生命周期
///
/// 骨架实现，Phase 1 填充具体逻辑。
#[derive(Debug)]
pub struct SceneRegistry {
    current: SceneId,
    pending: Option<SceneId>,
}

impl SceneRegistry {
    pub const fn new(initial: SceneId) -> Self {
        Self { current: initial, pending: None }
    }

    /// 请求切换到新场景（过渡动画触发）
    pub fn request_switch(&mut self, target: SceneId) {
        self.pending = Some(target);
    }

    /// 执行场景切换（过渡动画完成后调用）
    pub fn commit_switch(&mut self) {
        if let Some(next) = self.pending.take() {
            self.current = next;
        }
    }

    #[must_use]
    pub const fn current(&self) -> SceneId {
        self.current
    }

    #[must_use]
    pub const fn is_pending(&self) -> bool {
        self.pending.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_registry_has_initial_scene() {
        let reg = SceneRegistry::new(SceneId::Title);
        assert_eq!(reg.current(), SceneId::Title);
        assert!(!reg.is_pending());
    }

    #[test]
    fn request_switch_sets_pending() {
        let mut reg = SceneRegistry::new(SceneId::Title);
        reg.request_switch(SceneId::Vale);
        assert!(reg.is_pending());
        assert_eq!(reg.current(), SceneId::Title);
    }

    #[test]
    fn commit_switch_applies_pending() {
        let mut reg = SceneRegistry::new(SceneId::Title);
        reg.request_switch(SceneId::Vale);
        reg.commit_switch();
        assert!(!reg.is_pending());
        assert_eq!(reg.current(), SceneId::Vale);
    }

    #[test]
    fn commit_switch_without_pending_is_noop() {
        let mut reg = SceneRegistry::new(SceneId::Vale);
        reg.commit_switch();
        assert_eq!(reg.current(), SceneId::Vale);
    }
}

// TODO Phase 4: pub mod dialogue;
