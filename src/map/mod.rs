//! 地图系统 — 瓦片类型、调色板、地图数据
//!
//! 统一的 TileKind 枚举覆盖 Phase 1~3 全部需求：
//! - Phase 1: Mode 7 世界渲染
//! - Phase 2: NPC 碰撞检测
//! - Phase 3: 精灵力地图交互

use crate::engine::constants::TILE_SIZE;

// ── 瓦片类型（17 种，覆盖 Phase 1~3） ──

/// 瓦片类型 — 单一真源，各 Phase 在此基础上扩展
///
/// 与各 Phase 的对应：
/// | Phase | 需要的类型 |
/// |-------|-----------|
/// | 1     | Void, Grass, Dirt, Water, Forest(=Tree), Wall, Sand, Bridge, Flower, Roof |
/// | 2     | (碰撞检测复用) |
/// | 3     | Vine, Seed, Ice, PushBlock (精灵力交互) |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileKind {
    /// 0: 空白/不可见（地图边缘）
    Void,
    /// 1: 草地（可通行）
    Grass,
    /// 2: 土路（可通行）
    Dirt,
    /// 3: 水面（不可通行，Phase 3 Freeze→Ice 可变）
    Water,
    /// 4: 树林（不可通行）
    Forest,
    /// 5: 石墙（不可通行）
    Wall,
    /// 6: 沙滩（可通行）
    Sand,
    /// 7: 雪地（可通行）
    Snow,
    /// 8: 木桥（可通行）
    Bridge,
    /// 9: 台阶（可通行）
    Stairs,
    /// 10: 花丛（可通行，纯装饰）
    Flower,
    /// 11: 屋顶（不可通行）
    Roof,
    // ── Phase 3 交互 tile ──
    /// 12: 藤蔓（不可通行，Whirlwind→Grass）
    Vine,
    /// 13: 种子（Whirlwind→Grass）
    Seed,
    /// 14: 冰面（可通行，Freeze 结果）
    Ice,
    /// 15: 可推方块（不可通行，Force 移动）
    PushBlock,
    // ── 预留给 Phase 3 扩展 ──
    /// 16: 风车（Wind→WindmillActive 切换）
    Windmill,
    /// 17: 激活的风车
    WindmillActive,
    /// 18: 暗区（需 Flash 照亮）
    DarkArea,
    /// 19: 隐藏宝箱（需 Reveal 发现）
    HiddenChest,
    /// 20: 宝箱（已打开）
    OpenedChest,
    /// 21: 可攀爬藤蔓（Growth 结果）
    VineClimbable,
    /// 255: 未知（fallback）
    Unknown,
}

impl TileKind {
    /// 从 u8 数字解码（Phase 1 地图数据用）
    pub const fn from_u8(v: u8) -> Self {
        match v {
            0 => TileKind::Void,
            1 => TileKind::Grass,
            2 => TileKind::Dirt,
            3 => TileKind::Water,
            4 => TileKind::Forest,
            5 => TileKind::Wall,
            6 => TileKind::Sand,
            7 => TileKind::Snow,
            8 => TileKind::Bridge,
            9 => TileKind::Stairs,
            10 => TileKind::Flower,
            11 => TileKind::Roof,
            12 => TileKind::Vine,
            13 => TileKind::Seed,
            14 => TileKind::Ice,
            15 => TileKind::PushBlock,
            16 => TileKind::Windmill,
            17 => TileKind::WindmillActive,
            18 => TileKind::DarkArea,
            19 => TileKind::HiddenChest,
            20 => TileKind::OpenedChest,
            21 => TileKind::VineClimbable,
            _ => TileKind::Unknown,
        }
    }

    /// 碰撞检测：该 tile 是否可以行走通过
    pub const fn is_walkable(self) -> bool {
        match self {
            TileKind::Grass
            | TileKind::Dirt
            | TileKind::Sand
            | TileKind::Snow
            | TileKind::Bridge
            | TileKind::Stairs
            | TileKind::Flower
            | TileKind::Ice
            | TileKind::VineClimbable
            | TileKind::WindmillActive => true,
            _ => false,
        }
    }

    /// 该 tile 是否有交互动作（Phase 3 用）
    pub const fn is_interactive(self) -> bool {
        match self {
            TileKind::Vine
            | TileKind::Seed
            | TileKind::PushBlock
            | TileKind::Windmill
            | TileKind::DarkArea
            | TileKind::HiddenChest => true,
            _ => false,
        }
    }

    /// GBA 风格调色板颜色 (R, G, B)
    pub const fn color(self) -> (u8, u8, u8) {
        match self {
            TileKind::Void => (0, 0, 0),
            TileKind::Grass => (86, 130, 36),
            TileKind::Dirt => (160, 120, 60),
            TileKind::Water => (40, 100, 200),
            TileKind::Forest => (20, 80, 20),
            TileKind::Wall => (120, 110, 100),
            TileKind::Sand => (220, 200, 140),
            TileKind::Snow => (230, 240, 250),
            TileKind::Bridge => (140, 100, 60),
            TileKind::Stairs => (100, 80, 60),
            TileKind::Flower => (200, 100, 150),
            TileKind::Roof => (180, 80, 40),
            TileKind::Vine => (60, 140, 40),
            TileKind::Seed => (180, 160, 100),
            TileKind::Ice => (180, 220, 255),
            TileKind::PushBlock => (160, 140, 120),
            TileKind::Windmill => (150, 150, 150),
            TileKind::WindmillActive => (100, 200, 100),
            TileKind::DarkArea => (10, 10, 20),
            TileKind::HiddenChest => (200, 180, 50),
            TileKind::OpenedChest => (120, 100, 60),
            TileKind::VineClimbable => (80, 160, 60),
            TileKind::Unknown => (255, 0, 255),
        }
    }
}

/// 将世界坐标（像素）转换为 tile 索引
pub fn world_to_tile(world: f32) -> i32 {
    (world / TILE_SIZE).floor() as i32
}

/// 将 tile 索引转换为世界坐标（tile 中心点）
pub fn tile_to_world(tile: i32) -> f32 {
    tile as f32 * TILE_SIZE + TILE_SIZE * 0.5
}
