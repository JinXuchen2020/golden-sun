//! 渲染层序 — 定义 WorldMap 状态下各层的绘制顺序

/// 渲染层序
///
/// 各 Phase 的渲染模块按此枚举顺序依次调用 draw():
/// ```text
/// Sky(天空渐变) → Terrain(Mode7地面) → EntitiesLow(远NPC) → Entities(玩家/近NPC)
/// → Effects(粒子/精灵力特效) → Overlay(精灵力UI) → HUD(HP/PP) → Debug(FPS等)
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum RenderPhase {
    Sky = 0,
    Terrain = 1,
    EntitiesLow = 2,
    Entities = 3,
    Effects = 4,
    Overlay = 5,
    HUD = 6,
    Debug = 7,
}
