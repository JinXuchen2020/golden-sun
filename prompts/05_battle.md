# Phase 5: 战斗系统

## 目标
实现黄金太阳风格的回合制战斗，支持 3v3 对位、精灵召唤、伤害计算。

## 共享类型引用（来自 Phase 0 & Phase 3）
```rust
use golden_sun::engine::{GameState, InputState, FrameTime};
use golden_sun::engine::input::{InputBus, InputEvent};
use golden_sun::engine::constants::{
    PHYSICAL_ATK_MULTIPLIER, PHYSICAL_DEF_MULTIPLIER,
    CRIT_RATE_COEFFICIENT, CRIT_RATE_MAX, CRIT_DAMAGE_MULTIPLIER,
    ELEMENT_ADVANTAGE_MULTIPLIER, ELEMENT_RESISTANCE_MULTIPLIER,
    FLEE_SPEED_COEFFICIENT, FLEE_MAX_CHANCE,
};
use golden_sun::psynergy::types::PsynergyType;  // Phase 3: 精灵力枚举
use golden_sun::GameResult;
```
- `GameState::Battle` — 遇敌/事件触发时切换
- `InputBus` — 战斗 UI 中消费方向/Confirm/Cancel
- `constants::*` — 伤害公式、暴击率、元素倍率、逃跑率参数
- `PsynergyType` — 精灵召唤时引用元素类型

## 前置依赖
- Phase 1 完成（地图系统 — 战斗场景入口）
- Phase 3 完成（精灵力系统 — `PsynergyType` 枚举 + 元素定义）

## 任务清单

### 5.1 战斗数据模型
文件: `src/battle/state.rs`

```rust
pub struct BattleUnit {
    pub name: String,
    pub hp: i32, pub max_hp: i32,
    pub pp: i32, pub max_pp: i32,
    pub element: Element,
    pub level: u32,
    pub attack: u32, pub defense: u32, pub speed: u32,
    pub status_effects: Vec<StatusEffect>,
    pub position: BattlePosition,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Element { Earth, Fire, Wind, Water }

pub enum BattlePhase {
    Intro,
    PlayerSelect, PlayerTarget,
    ExecuteActions,
    EnemyTurn,
    ResolveEffects,
    Victory, Defeat, Flee,
}
```

### 5.2 伤害计算器（使用 constants）
文件: `src/battle/calculator.rs`

```
物理攻击:
  base = (attack × PHYSICAL_ATK_MULTIPLIER - defense × PHYSICAL_DEF_MULTIPLIER) × random(0.85, 1.0)
  暴击率 = (attacker.speed / defender.speed) × CRIT_RATE_COEFFICIENT, cap CRIT_RATE_MAX
  暴击时 damage ×= CRIT_DAMAGE_MULTIPLIER

精灵召唤:
  damage = (power × (level + 10) / (defense + 10)) × element_mult × random(0.9, 1.0)

元素克制:
  地→水→火→风→地 循环 → ×ELEMENT_ADVANTAGE_MULTIPLIER(1.25)
  相同元素 → ×ELEMENT_RESISTANCE_MULTIPLIER(0.75)
  其他 → ×1.0
```

### 5.3 战斗渲染
- 背景层（战斗场景纯色+渐变）
- 角色层（我方左 3，敌方右 3 站位）
- 特效层（攻击动画/精灵召唤）
- UI 层（HP/PP 条、指令菜单、伤害数字）

### 5.4 战斗指令 UI
- 菜单: 攻击 / 精灵 / 防御 / 道具 / 逃跑
- 使用 `InputBus` 消费方向 + Confirm/Cancel

### 5.5 AI 对手逻辑
- HP > 70%: 40% 攻击/30% 精灵/30% 防御
- HP 30-70%: 50% 攻击/40% 精灵/10% 道具
- HP < 30%: 60% 精灵/30% 道具/10% 逃跑
- 优先攻击 HP 最低单位

### 5.6 胜负判定
- 全部敌人 HP=0 → Victory（经验值+金币）
- 我方全灭 → Defeat
- 逃跑率 = (我方平均速度/敌方平均速度) × `FLEE_SPEED_COEFFICIENT(0.5)`, 最大 `FLEE_MAX_CHANCE(0.9)`

## 验收标准
- [ ] `cargo test` 全部通过
- [ ] 战斗 UI 完整（HP/PP/指令菜单）
- [ ] 伤害计算正确（用 constants 参数验证）
- [ ] 暴击/元素克制生效
- [ ] AI 有基本策略
- [ ] 胜利/失败/逃跑流程正常
