# Phase 1: Mode 7 世界地图 + 玩家移动（MVP）

## 目标
实现 Vale 村地图的伪 3D 渲染，玩家可在世界中自由移动和旋转视角。

## 共享类型引用（来自 Phase 0）
```rust
use golden_sun::engine::{Camera, GameState, InputState, FrameTime, RenderPhase};
use golden_sun::engine::input::{InputBus, InputEvent};
use golden_sun::engine::texture::TextureCache;  // ← V3 新增
use golden_sun::engine::constants;
use golden_sun::map::{TileKind, world_to_tile, tile_to_world};
use golden_sun::GameResult;
```
- `Camera` → 坐标转换：`Camera::tile_to_world(t)` / `Camera::world_to_tile(w)` / `tile_index()` / `world_pos()`
- `TileKind` → **已统一定义在 `map/mod.rs`**，17 种，勿重复定义。Phase 1 使用：`Void/Grass/Dirt/Water/Forest/Wall/Sand/Bridge/Flower/Roof`
- `constants::*` → 所有魔数：`TILE_SIZE(32)`, `WINDOW_WIDTH/HEIGHT`, `HORIZON_RATIO`, `FOG_*`, `CAMERA_*`, `PLAYER_*` 等
- `InputBus` → 统一输入分发，调用 `input_bus.consume(InputEvent::Up)` 消费方向键
- `RenderPhase` → 渲染层序：Sky(0)→Terrain(1)→EntitiesLow(2)→Entities(3)→...

## 前置依赖
- Phase 0 完成（已含：TileKind 统一、坐标系统、InputBus、ResourceManager、constants）

## 任务清单

### 1.1 地图数据与调色板
文件: `src/map/tilemap.rs`

**注意：TileKind 已在 `src/map/mod.rs` 定义，不要重复声明。直接使用 `use golden_sun::map::TileKind`。**

```rust
use golden_sun::map::TileKind;
use golden_sun::engine::constants::{MAP_WIDTH, MAP_HEIGHT};

/// 世界地图：32x32 的 u8 编码
/// 编码对应 TileKind::from_u8()：
///   0=Void, 1=Grass, 2=Dirt, 3=Water, 4=Forest, 5=Wall,
///   7=Bridge, 8=Sand, 9=Stairs, 10=Flower, 11=Roof
pub const WORLD_MAP: [[u8; MAP_WIDTH as usize]; MAP_HEIGHT as usize] = [
    // Vale 村示例地图
];
```

> **色彩不再需要独立调色板常量** — 使用 `TileKind::color()` 返回 `(r,g,b)`。

要求：
- 编写 Vale 村地图数据（32x32，用 u8 编码）
- 地图中标注出：房屋墙壁(Roof/Wall)、门口(Dirt)、道路(Dirt)、水面(Water)、树林(Forest)

### 1.2 相机系统
文件: `src/engine/camera.rs`

**Camera 已在 `src/engine/mod.rs` 中完定义，无需新建结构体。** Phase 1 直接在 `world_map` 状态下使用 `ctx.camera`。

`Camera` 已包含的 Phase 1 方法：
- `move_forward(distance)` / `move_backward(distance)` — 沿朝向移动（tile 单位）
- `strafe(distance)` — 横向平移（tile 单位）
- `rotate(radians)` — 旋转视角
- `tile_index()` → `(i32, i32)` — 当前 tile 网格索引
- `world_pos()` → `(f32, f32)` — 世界像素坐标
- `update_lerp(dt)` — lerp 插值
- `z`, `fov` — Mode 7 透视参数

移动速度使用 `constants::PLAYER_SPEED(3.0 tile/s)` × `constants::PLAYER_SPRINT_MULTIPLIER(1.8)` 加速。

### 1.3 Mode 7 渲染器
文件: `src/map/mode7.rs`

核心算法（逐行扫描渲染）：

```
对于屏幕上每一行 y（从 horizon_y 到屏幕底部）:
  dy = y - horizon_y
  z = camera.z / dy         ← 透视深度（使用 Camera.z，单位为像素高度）
  scale_factor = z / camera_fov_scale

  对于该行每一列 x（0..screen_width）:
    rel_x = (x - screen_width/2) * scale_factor
    rel_z = z

    // 施加旋转
    world_x = camera.world_x + rel_x * cos(rotation) - rel_z * sin(rotation)
    world_z = camera.world_z + rel_x * sin(rotation) + rel_z * cos(rotation)

    tile_x = world_to_tile(world_x)   // 使用 map::world_to_tile()
    tile_z = world_to_tile(world_z)

    if tile_x,z 在边界内:
      kind = tilemap[tile_z][tile_x]  // u8 → TileKind::from_u8()
      color = kind.color()
      雾化: fog = clamp(1.0 - z/FOG_END, FOG_MIN_ALPHA, 1.0)
      写入像素缓冲区
```

实现要求：
- 每帧通过 `ctx.textures.world_map_image_mut()` 获取 CPU 端像素缓冲区
- 用 `get_image_data_mut()` 直接写 RGBA 字节
- 渲染完成后调用 `ctx.textures.upload_world_map()` 上传到 GPU
- 用 `draw_texture(ctx.textures.world_map_texture(), 0, 0, WHITE)` 绘制
- 渲染目标尺寸从 `constants::RENDER_TARGET_W/H` 取，勿硬编码
- 纹理滤镜已设为 `FilterMode::Nearest`（GBA 像素风格）
- `FrameTime.delta` 已内置裁剪保护（`1ms ~ 33ms`），直接用即可

### 1.4 输入控制（使用 InputBus）
```rust
// 在 update() 的 WorldMap 分支中：
if ctx.input_bus.consume(InputEvent::Up) {
    ctx.camera.move_forward(speed * dt);
}
if ctx.input_bus.consume(InputEvent::Down) {
    ctx.camera.move_backward(speed * dt);
}
if ctx.input_bus.consume(InputEvent::Left) {
    ctx.camera.rotate(-PLAYER_TURN_SPEED * dt);
}
if ctx.input_bus.consume(InputEvent::Right) {
    ctx.camera.rotate(PLAYER_TURN_SPEED * dt);
}
let sprint = ctx.input.state.a;  // A 键加速（方向已被消费，用 raw InputState 读持续按下）
```

### 1.5 碰撞检测
- 移动前检查目标 tile：`TileKind::is_walkable()`
- 用 `world_to_tile()` 获取目标 tile 索引
- 不可通行则回退移动

### 1.6 玩家角色渲染
文件: `src/entity/player.rs`（可先作为 Phase 1 的简易占位）
- 在 Mode 7 渲染完成后，在玩家脚底绘制菱形标记（`draw_poly()`）
- 玩家世界位置 = `camera.world_pos()`

### 1.7 集成到主循环
将以上所有模块集成到 `main.rs` 的 `update()` / `draw()` 分支：
1. `update()`: InputBus 消费 → 碰撞检测 → 更新 Camera
2. `draw()`: 调用 mode7 渲染 → 绘制玩家标记
3. Debug 信息（`#[cfg(debug_assertions)]`）已集成在 main.rs 中

## 验收标准
- [x] `cargo test` 全部通过（87 passed）
- [x] 运行后看到伪 3D 俯视视角的 Vale 村地图（天空渐变 + Mode 7 透视地面）
- [x] 方向键行走，WASD 等价（通过 InputBus::consume 消费）
- [x] 地图有近大远小的透视效果（Mode7Camera::prepare_scanline + project_pixel）
- [x] 远处 tile 带雾化淡出（Mode7Camera::fog_factor + 地平线色混合）
- [x] 角色脚底有菱形标记（draw_player_marker: 两个三角形拼合）
- [x] 走到墙/树/水边会被挡住（try_move_to → tilemap::is_walkable）
- [x] 60fps 稳定运行（release 构建优化后）
- [x] 按 A 键加速行走（InputState.a × PLAYER_SPRINT_MULTIPLIER）
- [x] 按 Space/Start 暂停菜单不报错（Phase 6 实现 Menu 状态）
