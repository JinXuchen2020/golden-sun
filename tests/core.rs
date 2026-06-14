//! 核心类型单元测试 — Phase 0
//!
//! 每个 Phase 在各自模块中添加 `#[cfg(test)] mod tests { ... }`。

use golden_sun::engine::input::{InputBus, InputEvent};
use golden_sun::engine::mode7_camera::Mode7Camera;
use golden_sun::engine::{Camera, FrameTime, InputState};
use golden_sun::map::TileKind;

// ── 坐标转换测试 ──

#[test]
fn test_camera_tile_world_roundtrip() {
    let tile = 5.0;
    let world = Camera::tile_to_world(tile);
    let back = Camera::world_to_tile(world);
    assert!((back - tile).abs() < 0.001);
}

#[test]
fn test_map_world_to_tile_index_floor() {
    assert_eq!(golden_sun::map::world_to_tile_index(47.9), 1);
    assert_eq!(golden_sun::map::world_to_tile_index(64.0), 2);
}

#[test]
fn test_map_tile_center() {
    let center = golden_sun::map::tile_center(5);
    assert!((center - 176.0).abs() < 0.001); // 5*32 + 16
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

// ── Camera 移动/旋转 ──

#[test]
fn test_camera_move_forward() {
    let mut cam = Camera::new(10.0, 10.0);
    cam.move_forward(2.0);
    assert!((cam.x - 12.0).abs() < 0.001);
    assert!((cam.y - 10.0).abs() < 0.001);
}

#[test]
fn test_camera_move_backward() {
    let mut cam = Camera::new(10.0, 10.0);
    cam.move_backward(3.0);
    assert!((cam.x - 7.0).abs() < 0.001);
}

#[test]
fn test_camera_rotate() {
    let mut cam = Camera::new(0.0, 0.0);
    cam.rotate(std::f32::consts::FRAC_PI_2);
    assert!((cam.rotation - std::f32::consts::FRAC_PI_2).abs() < 0.001);
}

#[test]
fn test_camera_rotate_wraparound() {
    let mut cam = Camera::new(0.0, 0.0);
    cam.rotate(std::f32::consts::TAU);
    assert!(cam.rotation < 0.001);
}

#[test]
fn test_camera_set_target_and_lerp() {
    let mut cam = Camera::new(0.0, 0.0);
    cam.set_target(10.0, 20.0);
    cam.update_lerp(1.0);
    assert!(cam.x > 0.0);
    assert!(cam.y > 0.0);
}

#[test]
fn test_camera_snap_to_target() {
    let mut cam = Camera::new(5.0, 5.0);
    cam.set_target(100.0, 200.0);
    cam.snap_to_target();
    assert!((cam.x - 100.0).abs() < 0.001);
    assert!((cam.y - 200.0).abs() < 0.001);
}

// ── Camera strafe ──

#[test]
fn test_camera_strafe_moves_perpendicular() {
    let mut cam = Camera::new(0.0, 0.0);
    cam.strafe(1.0);
    // rotation=0 → strafe moves in +y direction (perpendicular)
    assert!((cam.x).abs() < 0.001);
    assert!((cam.y - 1.0).abs() < 0.001);
}

// ── InputBus 测试 ──

#[test]
fn test_inputbus_consume_returns_true_when_event_present() {
    let mut bus = InputBus::new();
    let mut state = InputState::new();
    state.a = true;
    bus.poll(&state);
    assert!(bus.consume(InputEvent::Confirm));
}

#[test]
fn test_inputbus_consume_removes_event() {
    let mut bus = InputBus::new();
    let mut state = InputState::new();
    state.a = true;
    bus.poll(&state);
    assert!(bus.consume(InputEvent::Confirm));
    assert!(!bus.consume(InputEvent::Confirm));
}

#[test]
fn test_inputbus_consume_returns_false_when_absent() {
    let mut bus = InputBus::new();
    let state = InputState::new();
    bus.poll(&state);
    assert!(!bus.consume(InputEvent::Confirm));
}

#[test]
fn test_inputbus_has_any() {
    let mut bus = InputBus::new();
    let state = InputState::new();
    bus.poll(&state);
    assert!(!bus.has_any());
}

#[test]
fn test_inputbus_has_event() {
    let mut bus = InputBus::new();
    let mut state = InputState::new();
    state.start = true;
    bus.poll(&state);
    assert!(bus.has(InputEvent::Menu));
    assert!(!bus.has(InputEvent::Confirm));
}

#[test]
fn test_inputbus_multiple_events() {
    let mut bus = InputBus::new();
    let mut state = InputState::new();
    state.up = true;
    state.a = true;
    state.start = true;
    bus.poll(&state);
    assert!(bus.consume(InputEvent::Up));
    assert!(bus.consume(InputEvent::Confirm));
    assert!(bus.consume(InputEvent::Menu));
    assert!(!bus.consume(InputEvent::Down));
}

// ── FrameTime 测试 ──

#[test]
fn test_frametime_defaults() {
    let ft = FrameTime::new();
    assert!((ft.delta - 0.0).abs() < f32::EPSILON);
    assert!((ft.elapsed - 0.0).abs() < f32::EPSILON);
}

#[test]
fn test_frametime_elapsed_increases() {
    let mut ft = FrameTime::new();
    ft.delta = 0.016;
    ft.elapsed += ft.delta;
    assert!((ft.elapsed - 0.016).abs() < 0.001);
}

// ── Mode7Camera 测试 ──

#[test]
fn test_mode7_camera_world_coords() {
    let cam = Camera::new(10.0, 20.0);
    let m7 = Mode7Camera::new(&cam);
    assert!((m7.world_x() - 320.0).abs() < 0.001);
    assert!((m7.world_z() - 640.0).abs() < 0.001);
}

#[test]
fn test_prepare_scanline_returns_none_above_horizon() {
    let cam = Camera::new(0.0, 0.0);
    let m7 = Mode7Camera::new(&cam);
    assert!(m7.prepare_scanline(100.0, 640.0, 50.0).is_none());
    assert!(m7.prepare_scanline(100.0, 640.0, 100.0).is_none());
}

#[test]
fn test_prepare_scanline_returns_some_below_horizon() {
    let cam = Camera::new(0.0, 0.0);
    let m7 = Mode7Camera::new(&cam);
    let ctx = m7.prepare_scanline(100.0, 640.0, 200.0);
    assert!(ctx.is_some());
    let ctx = ctx.unwrap();
    assert!((ctx.z - cam.height / 100.0).abs() < 0.001);
}

#[test]
fn test_fog_factor_full_at_zero_depth() {
    let cam = Camera::new(0.0, 0.0);
    let m7 = Mode7Camera::new(&cam);
    assert!((m7.fog_factor(0.0) - 1.0).abs() < 0.001);
}

#[test]
fn test_fog_factor_min_at_max_depth() {
    let cam = Camera::new(0.0, 0.0);
    let m7 = Mode7Camera::new(&cam);
    let f = m7.fog_factor(200.0);
    assert!((f - 0.3).abs() < 0.001);
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
fn test_tilekind_color_returns_non_black_for_walkable_tiles() {
    for kind in &[
        TileKind::Grass, TileKind::Dirt, TileKind::Sand,
        TileKind::Snow, TileKind::Bridge, TileKind::Ice,
    ] {
        let c = kind.color();
        assert!(c.r > 0.0 || c.g > 0.0 || c.b > 0.0, "{:?} color is black", kind);
    }
}

#[test]
fn test_is_walkable() {
    assert!(TileKind::Grass.is_walkable());
    assert!(TileKind::Dirt.is_walkable());
    assert!(TileKind::Bridge.is_walkable());
    assert!(!TileKind::Water.is_walkable());
    assert!(!TileKind::Wall.is_walkable());
    assert!(!TileKind::Forest.is_walkable());
    assert!(TileKind::Ice.is_walkable());
}

#[test]
fn test_is_interactive() {
    assert!(TileKind::Vine.is_interactive());
    assert!(TileKind::Seed.is_interactive());
    assert!(TileKind::PushBlock.is_interactive());
    assert!(!TileKind::Grass.is_interactive());
    assert!(!TileKind::Wall.is_interactive());
}
