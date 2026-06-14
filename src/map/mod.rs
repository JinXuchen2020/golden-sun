//! 地图系统 — 瓦片类型、调色板、地图数据、Mode 7 渲染

pub mod mode7;
pub mod tilemap;

use crate::engine::constants::{TILE_SIZE, WORLD_TO_TILE};

// ── 瓦片类型（17 种，覆盖 Phase 1~3） ──

use macroquad::prelude::Color;

/// 瓦片类型 — 单一真源，各 Phase 在此基础上扩展
///
/// 与各 Phase 的对应：
/// | Phase | 需要的类型 |
/// |-------|-----------|
/// | 1     | Void, Grass, Dirt, Water, Forest(=Tree), Wall, Sand, Bridge, Flower, Roof |
/// | 2     | (碰撞检测复用) |
/// | 3     | Vine, Seed, Ice, PushBlock (精灵力交互) |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
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
    #[must_use]
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
    #[must_use]
    pub const fn is_walkable(self) -> bool {
        matches!(self, TileKind::Grass
            | TileKind::Dirt
            | TileKind::Sand
            | TileKind::Snow
            | TileKind::Bridge
            | TileKind::Stairs
            | TileKind::Flower
            | TileKind::Ice
            | TileKind::VineClimbable
            | TileKind::WindmillActive)
    }

    /// 该 tile 是否有交互动作（Phase 3 用）
    #[must_use]
    pub const fn is_interactive(self) -> bool {
        matches!(self, TileKind::Vine
            | TileKind::Seed
            | TileKind::PushBlock
            | TileKind::Windmill
            | TileKind::DarkArea
            | TileKind::HiddenChest)
    }

    /// GBA 风格调色板颜色
    /// 直接返回 macroquad `Color`，调用方无需再次转换
    #[must_use]
    pub fn color(self) -> Color {
        fn c(r: u8, g: u8, b: u8) -> Color { Color::from_rgba(r, g, b, 255) }
        match self {
            TileKind::Void => c(0, 0, 0),
            TileKind::Grass => c(86, 130, 36),
            TileKind::Dirt => c(160, 120, 60),
            TileKind::Water => c(40, 100, 200),
            TileKind::Forest => c(20, 80, 20),
            TileKind::Wall => c(120, 110, 100),
            TileKind::Sand => c(220, 200, 140),
            TileKind::Snow => c(230, 240, 250),
            TileKind::Bridge => c(140, 100, 60),
            TileKind::Stairs => c(100, 80, 60),
            TileKind::Flower => c(200, 100, 150),
            TileKind::Roof => c(180, 80, 40),
            TileKind::Vine => c(60, 140, 40),
            TileKind::Seed => c(180, 160, 100),
            TileKind::Ice => c(180, 220, 255),
            TileKind::PushBlock => c(160, 140, 120),
            TileKind::Windmill => c(150, 150, 150),
            TileKind::WindmillActive => c(100, 200, 100),
            TileKind::DarkArea => c(10, 10, 20),
            TileKind::HiddenChest => c(200, 180, 50),
            TileKind::OpenedChest => c(120, 100, 60),
            TileKind::VineClimbable => c(80, 160, 60),
            TileKind::Unknown => c(255, 0, 255),
        }
    }
}

/// 将世界坐标（像素）转换为 tile 索引（i32，向下取整）
pub fn world_to_tile_index(world: f32) -> i32 {
    (world * WORLD_TO_TILE).floor() as i32
}

/// 将 tile 索引转换为世界坐标（tile 中心点）
pub fn tile_center(tile: i32) -> f32 {
    tile as f32 * TILE_SIZE + TILE_SIZE * 0.5
}
