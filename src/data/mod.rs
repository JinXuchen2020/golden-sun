//! 游戏数据模型 — 存档/读档的序列化契约
//!
//! 所有 Phase 共享此结构：Phase 1-2 填充位置/实体，
//! Phase 3 填充精灵力，Phase 5 填充战斗数据，Phase 6 实现序列化。
//! Phase 6.7: 敌人配置表
//! Phase 6.10: QuestLog 任务日志系统
//! Phase 6.9: Djinn 精灵系统

pub mod djinn;
pub mod quest;
pub mod loader;
pub mod summon;
pub mod cutscene;

use std::collections::HashMap;

/// 敌人配置 — 按区域定义可用敌人
#[derive(Debug, Clone)]
pub struct EnemyConfig {
    pub name: &'static str,
    pub level: u32,
    pub display_name: &'static str,
    pub description: &'static str,
}

/// 获取指定区域的敌人编队
pub fn enemies_for_area(area: &str) -> Vec<EnemyConfig> {
    match area {
        "Vale" => vec![
            EnemyConfig { name: "Wolf", level: 3, display_name: "野狼", description: "山脚常见的野兽" },
            EnemyConfig { name: "Bat", level: 2, display_name: "蝙蝠", description: "洞穴中成群出没" },
            EnemyConfig { name: "Goblin", level: 4, display_name: "哥布林", description: "喜欢恶作剧的小妖怪" },
        ],
        "WildForest" => vec![
            EnemyConfig { name: "Wolf", level: 4, display_name: "野狼", description: "" },
            EnemyConfig { name: "Spider", level: 3, display_name: "毒蜘蛛", description: "森林深处的危险生物" },
            EnemyConfig { name: "Goblin", level: 5, display_name: "哥布林", description: "" },
            EnemyConfig { name: "Treant", level: 6, display_name: "树精", description: "古老森林的守护者" },
            EnemyConfig { name: "Slime", level: 2, display_name: "史莱姆", description: "元素凝聚成的软体生物" },
        ],
        "Cave" => vec![
            EnemyConfig { name: "Bat", level: 3, display_name: "蝙蝠", description: "" },
            EnemyConfig { name: "Golem", level: 7, display_name: "石巨人", description: "洞穴深处的岩石守卫" },
            EnemyConfig { name: "Spider", level: 5, display_name: "毒蜘蛛", description: "" },
            EnemyConfig { name: "Ghost", level: 6, display_name: "幽灵", description: "游荡在黑暗中的灵体" },
            EnemyConfig { name: "RatKing", level: 8, display_name: "鼠王", description: "洞穴鼠群的统帅" },
        ],
        "SolSanctum" => vec![
            EnemyConfig { name: "MythrilGolem", level: 10, display_name: "密银巨像", description: "圣祭坛的远古守卫" },
            EnemyConfig { name: "AncientGuard", level: 9, display_name: "古代守卫", description: "迷途者的灵魂被赋予了形体" },
        ],
        _ => vec![
            EnemyConfig { name: "Wolf", level: 3, display_name: "野狼", description: "" },
            EnemyConfig { name: "Bat", level: 2, display_name: "蝙蝠", description: "" },
        ],
    }
}

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
    /// 玩家等级
    pub player_level: u32,
    /// 玩家攻击力
    pub player_attack: u32,
    /// 玩家防御力
    pub player_defense: u32,
    /// 已收集的 Djinn ID 列表
    pub collected_djinn: Vec<String>,
    /// 已装备的 Djinn (djinn_id, slot_index)
    pub equipped_djinn: Vec<(String, u32)>,
    /// 已装备武器索引
    pub equipped_weapon: Option<usize>,
    /// 已装备护甲索引
    pub equipped_armor: Option<usize>,
    /// 已装备饰品索引
    pub equipped_accessory: Option<usize>,
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
            player_level: 1,
            player_attack: 10,
            player_defense: 8,
            collected_djinn: Vec::new(),
            equipped_djinn: Vec::new(),
            equipped_weapon: None,
            equipped_armor: None,
            equipped_accessory: None,
            timestamp: 0,
        }
    }
}
