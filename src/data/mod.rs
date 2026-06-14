//! 游戏数据模型 — 存档/读档的序列化契约
//!
//! 所有 Phase 共享此结构：Phase 1-2 填充位置/实体，
//! Phase 3 填充精灵力，Phase 5 填充战斗数据，Phase 6 实现序列化。

use std::collections::HashMap;

/// 可保存的游戏状态 — 各 Phase 按需求扩展字段
///
/// 已标注 Serialize/Deserialize，Phase 6 可直接用 bincode 序列化。
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SaveData {
    /// 当前场景
    pub scene: String,
    /// 玩家 tile 坐标 (x, y)
    pub player_x: f32,
    pub player_y: f32,
    /// 玩家朝向弧度
    pub player_rotation: f32,
    /// 游戏剧情/状态标记
    pub flags: HashMap<String, bool>,
    /// 道具栏 (id, count)
    pub inventory: Vec<(String, u32)>,
    /// 已解锁精灵力列表
    pub psynergies: Vec<String>,
    /// 金币
    pub gold: u32,
    /// 玩家 HP
    pub player_hp: u32,
    /// 玩家 PP
    pub player_pp: u32,
    /// 存档时间戳
    pub timestamp: u64,
}

impl Default for SaveData {
    fn default() -> Self {
        Self::new()
    }
}

impl SaveData {
    /// 创建默认存档（新游戏）
    pub fn new() -> Self {
        Self {
            scene: "Vale".into(),
            player_x: 16.0,
            player_y: 16.0,
            player_rotation: 0.0,
            flags: HashMap::new(),
            inventory: Vec::new(),
            psynergies: Vec::new(),
            gold: 0,
            player_hp: 100,
            player_pp: 30,
            timestamp: 0,
        }
    }
}
