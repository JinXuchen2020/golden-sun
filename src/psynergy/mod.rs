//! 精灵力（Psynergy）系统
//!
//! ## 子模块（Phase 3+）
//! - `effects`: 每种精灵力的地图交互逻辑
//!
//! ## 预定义（Phase 0 优化）
//! `PsynergyType` 和 `Element` 在此提前定义，使 Phase 5（战斗系统）
//! 可以在 Phase 3 实现之前引用这些类型，实现并行开发。

/// 元素类型 — 影响伤害克制与角色属性
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Element {
    /// 地（Venus）
    Venus,
    /// 水（Mercury）
    Mercury,
    /// 火（Mars）
    Mars,
    /// 风（Jupiter）
    Jupiter,
}

/// 7 种精灵力类型 — 每种有对应的元素和 PP 消耗
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PsynergyType {
    /// 旋风（风元素）— 清除藤蔓/启动风车 — PP:2
    Whirlwind,
    /// 生长（地元素）— 催发种子成藤蔓 — PP:3
    Growth,
    /// 冻结（水元素）— 冻结水面为冰 — PP:4
    Freeze,
    /// 力量（地元素）— 推动方块 — PP:3
    Force,
    /// 抓取（风元素）— 隔空取物 — PP:2
    Catch,
    /// 闪光（火元素）— 照亮暗区 — PP:2
    Flash,
    /// 透视（风元素）— 发现隐藏物品 — PP:1
    Reveal,
}

impl PsynergyType {
    /// 该精灵力对应的元素
    pub const fn element(self) -> Element {
        match self {
            PsynergyType::Whirlwind => Element::Jupiter,
            PsynergyType::Growth => Element::Venus,
            PsynergyType::Freeze => Element::Mercury,
            PsynergyType::Force => Element::Venus,
            PsynergyType::Catch => Element::Jupiter,
            PsynergyType::Flash => Element::Mars,
            PsynergyType::Reveal => Element::Jupiter,
        }
    }

    /// 该精灵力的 PP 消耗
    pub const fn pp_cost(self) -> u32 {
        match self {
            PsynergyType::Whirlwind => 2,
            PsynergyType::Growth => 3,
            PsynergyType::Freeze => 4,
            PsynergyType::Force => 3,
            PsynergyType::Catch => 2,
            PsynergyType::Flash => 2,
            PsynergyType::Reveal => 1,
        }
    }

    /// 所有精灵力类型的列表（遍历用）
    pub const fn all() -> &'static [PsynergyType] {
        &[
            PsynergyType::Whirlwind,
            PsynergyType::Growth,
            PsynergyType::Freeze,
            PsynergyType::Force,
            PsynergyType::Catch,
            PsynergyType::Flash,
            PsynergyType::Reveal,
        ]
    }
}

// TODO Phase 3: pub mod effects;
