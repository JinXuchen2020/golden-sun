//! 核心类型单元测试 — Phase 0
//!
//! 每个 Phase 在各自模块中添加 `#[cfg(test)] mod tests { ... }`。

use golden_sun::engine::Camera;
use golden_sun::map::TileKind;

// ── 坐标转换测试 ──

#[test]
fn test_tile_to_world_roundtrip() {
    let tile = 5.0;
    let world = Camera::tile_to_world(tile);
    let back = Camera::world_to_tile(world);
    // 浮点误差容许
    assert!((back - tile).abs() < 0.001);
}

#[test]
fn test_camera_tile_index() {
    let cam = Camera::new(3.7, 8.2);
    assert_eq!(cam.tile_index(), (3, 8));
}

#[test]
fn test_camera_world_pos() {
    let cam = Camera::new(2.0, 4.0);
    let (wx, wy) = cam.world_pos();
    assert!((wx - 64.0).abs() < 0.001);
    assert!((wy - 128.0).abs() < 0.001);
}

// ── TileKind 测试 ──

#[test]
fn test_tilekind_from_u8() {
    assert_eq!(TileKind::from_u8(0), TileKind::Void);
    assert_eq!(TileKind::from_u8(1), TileKind::Grass);
    assert_eq!(TileKind::from_u8(15), TileKind::PushBlock);
    assert_eq!(TileKind::from_u8(255), TileKind::Unknown);
}

#[test]
fn test_is_walkable() {
    assert!(TileKind::Grass.is_walkable());
    assert!(TileKind::Dirt.is_walkable());
    assert!(TileKind::Bridge.is_walkable());
    assert!(!TileKind::Water.is_walkable());
    assert!(!TileKind::Wall.is_walkable());
    assert!(!TileKind::Forest.is_walkable());
    assert!(TileKind::Ice.is_walkable()); // Phase 3: Freeze 结果
}

#[test]
fn test_is_interactive() {
    assert!(TileKind::Vine.is_interactive());
    assert!(TileKind::Seed.is_interactive());
    assert!(TileKind::PushBlock.is_interactive());
    assert!(!TileKind::Grass.is_interactive());
    assert!(!TileKind::Wall.is_interactive());
}
