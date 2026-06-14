# Phase 2: 精灵系统与 NPC 实体

## 目标
实现角色帧动画、NPC 实体放置，以及 A 键交互（对话触发）。

## 共享类型引用（来自 Phase 0）
```rust
use golden_sun::engine::{Camera, GameState, InputState, FrameTime};
use golden_sun::engine::input::{InputBus, InputEvent};
use golden_sun::engine::constants::{SPRITE_SIZE, WALK_ANIM_FPS, NPC_INTERACT_RANGE};
use golden_sun::engine::resources::{ResourceManager, TextureData};
use golden_sun::map::TileKind;
use golden_sun::GameResult;
```
- `Camera::world_pos()` / `tile_index()` → 角色/NPC 坐标转换
- `InputBus::consume(InputEvent::Confirm)` → A 键交互
- `constants::NPC_INTERACT_RANGE(1.5)` → 交互距离阈值
- `ResourceManager::store_texture()` → 统一纹理注册

## 前置依赖
- Phase 1 完成（Mode7 地图可走、玩家可移动）

## 实体设计约束（为未来 ECS 升级预留）
所有实体（Player / Npc）采用**平铺字段 + Option** 设计，避免深层嵌套：

```rust
/// 推荐：平铺，靠 Option 区分能力
struct Entity {
    id: u32,
    kind: EntityKind,               // Player | Npc
    pos: (f32, f32),
    facing: Direction,
    sprite_id: String,              // 查 ResourceManager
    anim_state: AnimState,
    anim_timer: f32,
    // 可选能力
    walk_speed: Option<f32>,        // None = 不可移动
    interact_radius: Option<f32>,   // None = 不可交互
    dialogue_id: Option<String>,    // None = 无对话
}
```

```rust
/// 不推荐：深层嵌套（将来切 ECS 需大重构）
// struct Npc {
//     sprite: Sprite { animations: ..., timer: ... },  ← 嵌套太深
//     walk: WalkPattern { waypoints: ..., ... },
// }
```

> **为什么**：将来如果实体超过 50 个，需要切 bevy_ecs / hecs，平铺 Entity 可以直接拆成 `(Position, Sprite, Walkable?)` 三个 component。嵌套版本需要整体重构。

## 任务清单

### 2.1 帧动画系统
文件: `src/entity/sprite.rs`

```rust
pub struct AnimFrame {
    pub pixels: Vec<u8>,  // RGBA 像素数据 (SPRITE_SIZE × SPRITE_SIZE × 4)
    pub width: u32,       // = SPRITE_SIZE
    pub height: u32,      // = SPRITE_SIZE
}

pub struct Animation {
    pub frames: Vec<AnimFrame>,
    pub frame_duration: f32,  // = 1.0 / WALK_ANIM_FPS
    pub looping: bool,
}

/// 注意：Entity 的 anim_state / anim_timer 在 Entity 平铺字段中管理
/// Sprite 组件只负责纹理数据，不参与游戏循环更新
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub enum AnimState {
    IdleDown, IdleUp, IdleLeft, IdleRight,
    WalkDown, WalkUp, WalkLeft, WalkRight,
}
```

要求：
- 所有动画帧用程序化绘制（16x16 = SPRITE_SIZE 像素）
- 4 方向行走各 2 帧 + 4 方向站立各 1 帧 = 12 帧
- 动画切换跟随 `Camera.rotation` 方向（由 Entity 更新逻辑处理）
- 行走帧率使用 `WALK_ANIM_FPS`（8fps）

### 2.2 程序化绘制角色精灵
- 罗宾（主角）：红色兜帽 + 黄色头发
- NPC 通用模板：方块身体 + 不同颜色帽子
- 通过 `ResourceManager::store_texture()` 注册

### 2.3 实体与 NPC 系统
文件: `src/entity/npc.rs` 和 `src/entity/mod.rs`

> **使用上面定义的平铺 `Entity` 结构**，不要嵌套 `Sprite`。
> NPC 和 Player 是 Entity 的 `kind` 和可选字段不同，不是两个独立的 struct。

WalkPattern 作为 NPC 专有字段以 Option 形式挂在 Entity 上：

```rust
pub enum WalkPattern {
    Stationary,
    Patrol { waypoints: Vec<(f32, f32)>, speed: f32 },
    Random { radius: f32, pause_chance: f32 },
}

impl Entity {
    pub fn new_player(pos: (f32, f32)) -> Self { /* walk_speed = Some(...) */ }
    pub fn new_npc(pos: (f32, f32), name: &str, dialogue: &str) -> Self { /* ... */ }
}
```

要求：
- 至少 3 个 NPC 分布在 Vale 村地图上
- NPC 坐标使用 `Camera::tile_to_world()` 换算
- NPC 渲染在 Mode7 地面之上（RenderPhase::EntitiesLow/Entities）
- 所有实体存储在同一 `Vec<Entity>` 中，通过 `kind` 区分

### 2.4 玩家-NPC 交互
- 按 A 键（`InputBus::consume(InputEvent::Confirm)`）
- 检查玩家前方 `NPC_INTERACT_RANGE` tile 范围内是否有 NPC
- 若命中：切换到 `GameState::Dialog`，传入 `dialogue_id`

### 2.5 相机跟随优化
- 玩家移动时 `camera.target_x/y` 更新，`camera.update_lerp(dt)` 自动平滑
- 地图边缘越界保护

## 验收标准
- [ ] `cargo test` 全部通过
- [ ] 角色行走有 4 方向动画
- [ ] 至少 3 个 NPC 分布在地图上
- [ ] NPC 有巡逻/静止行为
- [ ] 按 A 键触发 NPC 交互
- [ ] 交互期间角色锁定移动
