//! 实体系统 — 玩家角色、NPC、精灵动画
//!
//! ## 设计约束
//! 采用**平铺 Entity + Option 字段**设计，为未来 ECS 升级预留通道。
//! 不要创建 `Player` / `Npc` 两个独立 struct，统一用 `Entity` + `kind` 区分。
//!
//! ## 子模块（Phase 2+）
//! - `player`: 玩家实体工厂方法 (`Entity::new_player()`)
//! - `npc`: NPC 实体 + 巡逻行为 (`WalkPattern`)
//! - `sprite`: 帧动画数据 (`AnimFrame`, `Animation`, `AnimState`)

/// 实体类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityKind {
    Player,
    Npc,
    // future: BattleUnit, Interactable
}

/// 朝向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up, Down, Left, Right,
}

/// 实体 — 平铺字段设计，靠 Option 区分能力
/// Phase 2 实现时填充具体字段
#[derive(Debug, Clone)]
pub struct Entity {
    pub id: u32,
    pub kind: EntityKind,
    pub pos: (f32, f32),        // 世界像素坐标
    pub facing: Direction,
    pub sprite_id: String,      // ResourceManager 纹理 key
    // ── 可选能力（None = 不具备此能力） ──
    pub walk_speed: Option<f32>,
    pub interact_radius: Option<f32>,
    pub dialogue_id: Option<String>,
}
