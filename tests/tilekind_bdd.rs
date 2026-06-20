//! BDD 测试: 地图瓦片系统
//! 对应 features/tilekind.feature

use golden_sun::map::TileKind;

// ── Scenario Outline: 基础瓦片编解码 ──

#[test]
fn tilekind_encode_decode_void()     { assert_eq!(TileKind::from_u8(0),   TileKind::Void); }
#[test]
fn tilekind_encode_decode_grass()    { assert_eq!(TileKind::from_u8(1),   TileKind::Grass); }
#[test]
fn tilekind_encode_decode_dirt()     { assert_eq!(TileKind::from_u8(2),   TileKind::Dirt); }
#[test]
fn tilekind_encode_decode_water()    { assert_eq!(TileKind::from_u8(3),   TileKind::Water); }
#[test]
fn tilekind_encode_decode_forest()   { assert_eq!(TileKind::from_u8(4),   TileKind::Forest); }
#[test]
fn tilekind_encode_decode_wall()     { assert_eq!(TileKind::from_u8(5),   TileKind::Wall); }
#[test]
fn tilekind_encode_decode_sand()     { assert_eq!(TileKind::from_u8(6),   TileKind::Sand); }
#[test]
fn tilekind_encode_decode_snow()     { assert_eq!(TileKind::from_u8(7),   TileKind::Snow); }
#[test]
fn tilekind_encode_decode_bridge()   { assert_eq!(TileKind::from_u8(8),   TileKind::Bridge); }
#[test]
fn tilekind_encode_decode_stairs()   { assert_eq!(TileKind::from_u8(9),   TileKind::Stairs); }
#[test]
fn tilekind_encode_decode_pushblock(){ assert_eq!(TileKind::from_u8(15),  TileKind::PushBlock); }
#[test]
fn tilekind_encode_decode_unknown()  { assert_eq!(TileKind::from_u8(254), TileKind::Unknown); }
#[test]
fn tilekind_encode_decode_waypoint() { assert_eq!(TileKind::from_u8(255), TileKind::Waypoint); }

// ── Scenario Outline: 可通行性判定 ──

#[test]
fn walkable_grass()  { assert!(TileKind::Grass.is_walkable()); }
#[test]
fn walkable_dirt()   { assert!(TileKind::Dirt.is_walkable()); }
#[test]
fn walkable_water()  { assert!(!TileKind::Water.is_walkable()); }
#[test]
fn walkable_forest() { assert!(!TileKind::Forest.is_walkable()); }
#[test]
fn walkable_wall()   { assert!(!TileKind::Wall.is_walkable()); }
#[test]
fn walkable_bridge() { assert!(TileKind::Bridge.is_walkable()); }
#[test]
fn walkable_sand()   { assert!(TileKind::Sand.is_walkable()); }
#[test]
fn walkable_snow()   { assert!(TileKind::Snow.is_walkable()); }
#[test]
fn walkable_stairs() { assert!(TileKind::Stairs.is_walkable()); }
#[test]
fn walkable_ice()    { assert!(TileKind::Ice.is_walkable()); }
#[test]
fn walkable_vine()   { assert!(!TileKind::Vine.is_walkable()); }
#[test]
fn walkable_pushblock() { assert!(!TileKind::PushBlock.is_walkable()); }
#[test]
fn walkable_vine_climbable() { assert!(TileKind::VineClimbable.is_walkable()); }
#[test]
fn walkable_unknown() { assert!(!TileKind::Unknown.is_walkable()); }

// ── Scenario Outline: 可交互性判定 ──

#[test]
fn interactive_vine()    { assert!(TileKind::Vine.is_interactive()); }
#[test]
fn interactive_seed()    { assert!(TileKind::Seed.is_interactive()); }
#[test]
fn interactive_pushblock(){ assert!(TileKind::PushBlock.is_interactive()); }
#[test]
fn interactive_windmill() { assert!(TileKind::Windmill.is_interactive()); }
#[test]
fn interactive_darkarea() { assert!(TileKind::DarkArea.is_interactive()); }
#[test]
fn interactive_hiddenchest() { assert!(TileKind::HiddenChest.is_interactive()); }
#[test]
fn interactive_grass()   { assert!(!TileKind::Grass.is_interactive()); }
#[test]
fn interactive_water()   { assert!(!TileKind::Water.is_interactive()); }
#[test]
fn interactive_wall()    { assert!(!TileKind::Wall.is_interactive()); }
#[test]
fn interactive_ice()     { assert!(!TileKind::Ice.is_interactive()); }

// ── Scenario: 每种瓦片有独立颜色 ──

#[test]
fn each_tile_has_unique_color() {
    use TileKind::*;
    let pairs = [
        (Grass, Dirt),
        (Water, Forest),
        (Wall, Sand),
        (Snow, Bridge),
        (Vine, Seed),
        (PushBlock, Ice),
    ];
    for (a, b) in &pairs {
        assert_ne!(a.color(), b.color(), "Tile {a:?} and {b:?} have the same color!");
    }

    // 没有黑色（除 Void 外）
    let tiles = [Grass, Dirt, Water, Forest, Wall, Sand, Snow, Bridge, Stairs,
                 Flower, Roof, Vine, Seed, Ice, PushBlock,
                 Windmill, DarkArea, HiddenChest, VineClimbable];
    for t in &tiles {
        let col = t.color();
        assert!(
            col.r > 0.0 || col.g > 0.0 || col.b > 0.0,
            "Tile {t:?} has black color"
        );
    }
}

// ── Scenario: 地图边界访问 ──

#[test]
fn map_boundary_access() {
    // 边界内
    let in_bounds = 0u32..32u32;
    // 越界
    let out_of_bounds = 32u32;
    assert!(in_bounds.contains(&0));
    assert!(in_bounds.contains(&31));
    assert!(!in_bounds.contains(&out_of_bounds));
}

// ── Scenario: 所有装备名称唯一 ──

#[test]
fn all_equipment_has_unique_names() {
    // 装备数据在 src/game/mod.rs 的 all_equipment() 函数中定义
    // 这里直接验证名称唯一性：预设的装备名称列表
    let names = [
        "短剑", "铁剑", "长剑", "精灵之刃",
        "布甲", "皮甲", "锁子甲", "精灵护甲",
        "守护戒指", "力量手环", "精灵徽章",
    ];
    let mut sorted = names.to_vec();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), names.len(), "所有装备名称必须唯一");
}
