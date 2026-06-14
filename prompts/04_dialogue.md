# Phase 4: 剧情引擎与对话系统

## 目标
实现对话树引擎、事件触发器、基本的场景过场。

## 共享类型引用（来自 Phase 0）
```rust
use golden_sun::engine::{GameState, InputState, FrameTime, Camera};
use golden_sun::engine::input::{InputBus, InputEvent};
use golden_sun::engine::constants::TYPEWRITER_INTERVAL;
use golden_sun::map::TileKind;
use golden_sun::psynergy::types::PsynergyType;  // Phase 3 的枚举
use golden_sun::GameResult;
```
- `GameState::Dialog` — 交互触发时切换
- `InputBus::consume(InputEvent::Confirm)` — A 键推进文字
- `InputBus::consume(InputEvent::Cancel)` — B 键退出
- `constants::TYPEWRITER_INTERVAL(0.05s)` — 打字机间隔
- `FrameTime.delta` — 字符显示计时
- `PsynergyType` — 对话结束时可能解锁精灵力

## 前置依赖
- Phase 2 完成（NPC 放置 + A 键交互）

## 任务清单

### 4.1 对话树数据结构
文件: `src/scene/dialogue.rs`

```rust
pub struct DialogueNode {
    pub id: String,
    pub speaker: String,
    pub text: String,
    pub options: Vec<DialogueOption>,
    pub on_end: Option<DialogueAction>,
}

pub struct DialogueOption {
    pub text: String,
    pub next_node_id: String,
    pub condition: Option<Box<dyn Fn(&GameState) -> bool>>,
}

pub enum DialogueAction {
    None,
    GiveItem { item_id: String, count: u32 },
    StartBattle { enemy_group: String },
    SetFlag { flag: String, value: bool },
    Teleport { map_x: f32, map_z: f32 },
    UnlockPsynergy { psynergy: PsynergyType },
}
```

### 4.2 对话引擎
- 打字机效果：使用 `TYPEWRITER_INTERVAL` 定时逐字显示
- A 键（`Confirm`）加速/进入下一段
- B 键（`Cancel`）退出对话
- 选项：方向键选择，A 确认

### 4.3 对话 UI
- GBA 风格：底部半透明黑色条 + 白色文字
- 说话者名称左上角
- 选项列表下方，当前选项高亮

### 4.4 事件触发器系统
文件: `src/scene/events.rs`

```rust
pub enum EventType {
    OnTile { x: i32, z: i32 },
    OnNpcInteract { npc_id: String },
    OnFlagSet { flag: String },
    OnEnterRegion { x1: i32, z1: i32, x2: i32, z2: i32 },
}
```

### 4.5 剧情 Flag 系统
- `HashMap<String, bool>` 追踪游戏进度
- 对话条件、NPC 出现条件

### 4.6 过场动画
- 相机插值移动（复用 `Camera::set_target()` + `update_lerp()`）
- 屏幕淡入/淡出

## 验收标准
- [ ] `cargo test` 全部通过
- [ ] 打字机效果逐字显示
- [ ] 多分支选项正常
- [ ] 对话结束可触发 `DialogueAction`（道具/精灵力/传送）
- [ ] 事件触发器按条件正确触发
- [ ] 至少 3 段不同对话内容
