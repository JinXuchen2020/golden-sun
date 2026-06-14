# Phase 3: 精灵力（Psynergy）系统

## 目标
实现黄金太阳标志性的 7 种精灵力，每种都能与地图进行独特交互。

## 共享类型引用（来自 Phase 0）
```rust
use golden_sun::engine::{GameState, InputState, FrameTime};
use golden_sun::engine::input::{InputBus, InputEvent};
use golden_sun::engine::constants::{PP_RECOVER_INTERVAL, PP_RECOVER_AMOUNT};
use golden_sun::map::TileKind;
use golden_sun::GameResult;
```
- `GameState::Psynergy` — B/Secondary 键切换到此状态
- `InputBus::consume(InputEvent::Secondary)` — B/X 进入精灵力选择
- `TileKind` — Phase 0 已预定义 `Vine/Seed/Ice/PushBlock/Windmill/DarkArea/HiddenChest...`
- `TileKind::is_interactive()` — 已有，判断 tile 是否可被精灵力影响
- `constants::PP_RECOVER_INTERVAL/AMOUNT` — PP 恢复参数

## 前置依赖
- Phase 2 完成（NPC + 交互 + 动画系统）

## 任务清单

### 3.1 精灵力类型枚举
文件: `src/psynergy/types.rs`

```rust
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum PsynergyType {
    Whirlwind,   // 旋风：Vine→Grass, Windmill→WindmillActive
    Growth,      // 生长：Seed→VineClimbable
    Freeze,      // 冻结：Water→Ice
    Force,       // 推：PushBlock 沿朝向移动一格
    Catch,       // 抓：隔空取物
    Flash,       // 闪：DarkArea 照亮 3x3
    Reveal,      // 透视：显示 HiddenChest
}
```

### 3.2 精灵力选择 UI
- 按 B/Secondary → `GameState::Psynergy`
- 屏幕底部精灵力快捷栏（横向图标）
- 左右键（`InputEvent::Left/Right`）切换选择
- A 键（`InputEvent::Confirm`）确认使用
- B 键（`InputEvent::Cancel`）返回地图

### 3.3 地图交互逻辑
文件: `src/psynergy/effects.rs`

各精灵力效果使用 `TileKind` 枚举值（非数字）：

| 精灵力 | 目标 tile | 效果 | 新 tile |
|--------|-----------|------|---------|
| Whirlwind | `Vine` | 清除藤蔓 | `Grass` |
| Whirlwind | `Windmill` | 激活风车 | `WindmillActive` |
| Growth | `Seed` | 生长 | `VineClimbable` |
| Freeze | `Water` | 冻结 | `Ice` |
| Force | `PushBlock` | 推动一格 | `PushBlock`(新位置) |
| Catch | `HiddenChest` | 拉近打开 | `OpenedChest` |
| Flash | `DarkArea` | 照亮 3x3 | 周围 tile 恢复可见 |
| Reveal | — | 显示隐藏 tile | 标记 map marker |

### 3.4 施法动画
- 每种精灵力有简单施法动画（闪烁/缩放/粒子）
- 动画期间锁定输入
- 动画结束后执行地图修改

### 3.5 PP 管理
- 每次使用消耗 PP（不同精灵力不同消耗）
- PP 不足时提示
- 行走时每 `PP_RECOVER_INTERVAL` 秒恢复 `PP_RECOVER_AMOUNT` 点

## 验收标准
- [ ] `cargo test` 全部通过
- [ ] B 键进入精灵力选择，左/右切换，A 确认，B 取消
- [ ] 7 种精灵力均可使用
- [ ] 每种精灵力对 `TileKind` 产生正确的 tile 转换
- [ ] PP 消耗/恢复正常
- [ ] 至少一处需要精灵力解谜的障碍
