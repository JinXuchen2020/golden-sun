//! 实体系统 — 玩家角色、NPC、精灵动画
//!
//! ## 设计约束
//! 采用**平铺 Entity + Option 字段**设计，为未来 ECS 升级预留通道。
//! 不要创建 `Player` / `Npc` 两个独立 struct，统一用 `Entity` + `kind` 区分。
//!
//! ## 子模块
//! - `sprite`: 帧动画数据 (`AnimFrame`, `Animation`, `AnimState`)

pub mod sprite;

use crate::engine::constants::{ANIM_FRAME_DURATION, NPC_INTERACT_RANGE, TILE_SIZE};
use sprite::AnimState;

/// 朝向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up, Down, Left, Right,
}

/// 行走模式
#[derive(Debug, Clone)]
pub enum WalkPattern {
    Patrol { waypoints: Vec<(f32, f32)>, speed: f32, index: usize, pause: f32 },
}

/// 实体 — 平铺字段设计，靠 Option 区分能力
#[derive(Debug, Clone)]
pub struct Entity {
    pub id: u32,
    pub pos: (f32, f32),
    pub facing: Direction,
    pub anim_state: AnimState,
    pub anim_timer: f32,
    pub interact_radius: Option<f32>,
    pub dialogue_id: Option<String>,
    pub walk_pattern: Option<WalkPattern>,
}

impl Entity {
    #[must_use]
    pub fn new_player(pos: (f32, f32)) -> Self {
        Self {
            id: 0,
            pos,
            facing: Direction::Down,
            anim_state: AnimState::IdleDown,
            anim_timer: 0.0,
            interact_radius: None,
            dialogue_id: None,
            walk_pattern: None,
        }
    }

    #[must_use]
    pub fn new_npc(id: u32, pos: (f32, f32), facing: Direction, dialogue: &str, pattern: Option<WalkPattern>) -> Self {
        Self {
            id,
            pos,
            facing,
            anim_state: AnimState::from_dir(facing, false),
            anim_timer: 0.0,
            interact_radius: Some(NPC_INTERACT_RANGE),
            dialogue_id: Some(dialogue.to_string()),
            walk_pattern: pattern,
        }
    }

    /// 计算当前动画帧索引
    pub fn current_frame_index(&self, num_frames: usize) -> usize {
        if num_frames <= 1 { return 0; }
        ((self.anim_timer / ANIM_FRAME_DURATION).floor() as usize) % num_frames
    }

    /// 转换 tile 坐标到世界像素坐标
    #[must_use]
    pub fn tile_to_world(tile_x: f32, tile_y: f32) -> (f32, f32) {
        (tile_x * TILE_SIZE + TILE_SIZE * 0.5, tile_y * TILE_SIZE + TILE_SIZE * 0.5)
    }
}

/// 创建 Vale 村 NPC 列表（世界像素坐标）
#[must_use]
pub fn create_vale_npcs() -> Vec<Entity> {
    vec![
        // 伊万 — 站在房屋 1 前
        Entity::new_npc(
            1, Entity::tile_to_world(7.0, 8.0),
            Direction::Down, "你好！我是伊万，Vale 村的铁匠。",
            None,
        ),
        // 米娅 — 站在池塘边
        Entity::new_npc(
            2, Entity::tile_to_world(20.0, 20.0),
            Direction::Left, "你喜欢这里的池塘吗？夏天有很多鱼。",
            None,
        ),
        // 加斯敏 — 在房屋 3 附近巡逻
        Entity::new_npc(
            3, Entity::tile_to_world(14.0, 14.0),
            Direction::Right, "我是村里的长老。小心山上的怪物！",
            Some(WalkPattern::Patrol {
                waypoints: vec![
                    Entity::tile_to_world(14.0, 14.0),
                    Entity::tile_to_world(18.0, 14.0),
                    Entity::tile_to_world(18.0, 12.0),
                ],
                speed: 1.0,
                index: 0,
                pause: 0.0,
            }),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_created_with_id_zero() {
        let p = Entity::new_player((100.0, 200.0));
        assert_eq!(p.id, 0);
        assert_eq!(p.facing, Direction::Down);
    }

    #[test]
    fn npc_has_dialogue() {
        let npc = Entity::new_npc(1, (300.0, 400.0), Direction::Up, "hi", None);
        assert_eq!(npc.interact_radius, Some(NPC_INTERACT_RANGE));
        assert_eq!(npc.dialogue_id, Some("hi".to_string()));
    }

    #[test]
    fn patrol_npc_has_waypoints() {
        let npc = Entity::new_npc(2, (0.0, 0.0), Direction::Left, "hello",
            Some(WalkPattern::Patrol { waypoints: vec![(0.0, 0.0), (32.0, 0.0)], speed: 1.0, index: 0, pause: 0.0 }));
        assert!(npc.walk_pattern.is_some());
    }

    #[test]
    fn vale_has_three_npcs() {
        let npcs = create_vale_npcs();
        assert_eq!(npcs.len(), 3);
    }
}
