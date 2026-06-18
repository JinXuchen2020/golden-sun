//! 精灵力地图交互 — 7 种精灵力对 TileKind 的转换逻辑
//!
//! 每种精灵力函数接收一个可变的 tile 引用，准备修改时持有 `&mut TileKind`。
//! 当前 Phase 1 地图数据是 const，无法直接修改——此处返回 `Option<TileKind>`
//! 表示"要将此位置替换为什么 tile"，由调用方（GameCtx）在持有地图写权限时执行。

use crate::map::TileKind;
use crate::PsynergyType;

/// 尝试对指定 tile 施加精灵力，返回替换后的 tile（None = 无效果）
pub fn apply_psynergy(tile: TileKind, psynergy: PsynergyType) -> Option<TileKind> {
    match psynergy {
        PsynergyType::Whirlwind => apply_whirlwind(tile),
        PsynergyType::Growth => apply_growth(tile),
        PsynergyType::Freeze => apply_freeze(tile),
        PsynergyType::Force => apply_force(tile),
        PsynergyType::Catch => apply_catch(tile),
        PsynergyType::Flash => None, // Flash 需要 3×3 范围信息，由调用方处理
        PsynergyType::Reveal => None, // Reveal 由调用方处理
    }
}

/// Force 对可破坏 tile 的效果
fn apply_force(tile: TileKind) -> Option<TileKind> {
    if tile.is_breakable() {
        Some(TileKind::Grass)
    } else {
        None
    }
}

/// 旋风：清除藤蔓、启动风车
fn apply_whirlwind(tile: TileKind) -> Option<TileKind> {
    match tile {
        TileKind::Vine => Some(TileKind::Grass),
        TileKind::Windmill => Some(TileKind::WindmillActive),
        _ => None,
    }
}

/// 生长：催发种子
fn apply_growth(tile: TileKind) -> Option<TileKind> {
    match tile {
        TileKind::Seed => Some(TileKind::VineClimbable),
        _ => None,
    }
}

/// 冻结：水面变冰面
fn apply_freeze(tile: TileKind) -> Option<TileKind> {
    match tile {
        TileKind::Water => Some(TileKind::Ice),
        _ => None,
    }
}

/// 抓取：打开隐藏宝箱
fn apply_catch(tile: TileKind) -> Option<TileKind> {
    match tile {
        TileKind::HiddenChest => Some(TileKind::OpenedChest),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whirlwind_clears_vine() {
        assert_eq!(apply_whirlwind(TileKind::Vine), Some(TileKind::Grass));
    }

    #[test]
    fn whirlwind_activates_windmill() {
        assert_eq!(apply_whirlwind(TileKind::Windmill), Some(TileKind::WindmillActive));
    }

    #[test]
    fn whirlwind_noop_on_grass() {
        assert_eq!(apply_whirlwind(TileKind::Grass), None);
    }

    #[test]
    fn growth_grows_seed() {
        assert_eq!(apply_growth(TileKind::Seed), Some(TileKind::VineClimbable));
    }

    #[test]
    fn growth_noop_on_grass() {
        assert_eq!(apply_growth(TileKind::Grass), None);
    }

    #[test]
    fn freeze_water_to_ice() {
        assert_eq!(apply_freeze(TileKind::Water), Some(TileKind::Ice));
    }

    #[test]
    fn freeze_noop_on_grass() {
        assert_eq!(apply_freeze(TileKind::Grass), None);
    }

    #[test]
    fn catch_opens_chest() {
        assert_eq!(apply_catch(TileKind::HiddenChest), Some(TileKind::OpenedChest));
    }

    #[test]
    fn catch_noop_on_grass() {
        assert_eq!(apply_catch(TileKind::Grass), None);
    }

    #[test]
    fn dispatch_all_psynergies() {
        // 验证 dispatch 函数正确路由到各子函数
        assert_eq!(apply_psynergy(TileKind::Vine, PsynergyType::Whirlwind), Some(TileKind::Grass));
        assert_eq!(apply_psynergy(TileKind::Seed, PsynergyType::Growth), Some(TileKind::VineClimbable));
        assert_eq!(apply_psynergy(TileKind::Water, PsynergyType::Freeze), Some(TileKind::Ice));
        assert_eq!(apply_psynergy(TileKind::HiddenChest, PsynergyType::Catch), Some(TileKind::OpenedChest));
        // Flash 和 Force/Reveal 由调用方处理，返回 None
        assert_eq!(apply_psynergy(TileKind::DarkArea, PsynergyType::Flash), None);
        assert_eq!(apply_psynergy(TileKind::Void, PsynergyType::Force), None);
        assert_eq!(apply_psynergy(TileKind::Void, PsynergyType::Reveal), None);
    }
}
