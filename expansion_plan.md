# 黄金太阳 Rust 复刻 — 扩展方案

> 版本: 2026-06-18
> 基线: Phase 0-6 全部完成，176 测试，零警告

---

## 当前状态

项目已完成核心游戏闭环（标题 → 探索 → 对话 → 精灵力 → 战斗 → 存档），代码行数 5,329，源文件 40 个。Phase 6（UI/音频/收尾）约 60% 完成，缺失像素字体、BGM、道具系统已在打磨阶段补充。

---

## 扩展方向总览

| 方向 | 优先级 | 工作量 | 影响 |
|------|--------|--------|------|
| A. 视觉反馈 | P0 | 中 | **最大** — 让现有功能"看起来"像游戏 |
| B. 交互反馈 | P1 | 小 | 高 — 让操作手感更完整 |
| C. 内容扩展 | P2 | 大 | 中 — 增加可玩时长 |

---

## A. 视觉反馈（P0，推荐优先实现）

### A1. 精灵力施法动画

**现状**: 使用精灵力时瞬间改变 tile，无视觉反馈。

**方案**: 在 `try_use_selected_psynergy` 执行前插入动画状态。GameState 新增 `Psynergy` 子状态携带计时器：

```
玩家按 A → GameState::PsynergyAnim { timer, psynergy, tx, ty }
          → 每帧推进 timer，屏幕闪烁/缩放特效
          → timer >= 0.5s 时执行实际 tile 修改，切回 WorldMap
```

**特效实现**:
- Whirlwind: 绿色旋风粒子（画 5-10 个小三角形旋转扩散）
- Growth: 棕色种子→绿色藤蔓生长（逐帧替换 tile 颜色）
- Freeze: 蓝色闪光矩形覆盖目标 tile
- Force: 方块抖动后平移一格
- Flash: 全屏白色闪光

**涉及文件**: `src/engine/game_state.rs`, `src/game/update.rs`, `src/game/draw.rs`

**预估行数**: ~120 行

### A2. 战斗精灵渲染

**现状**: 战斗画面只有文字（"Isaac attacks Wolf for 12 damage!"），无角色图像。

**方案**: 在战斗 HUD 中绘制 16×16 玩家/敌人精灵：

- 使用 `entity::sprite` 的现有程序化绘制函数生成战斗用纹理
- 玩家侧：左半屏，2 个角色纵向排列
- 敌人侧：右半屏，1-2 个敌人
- 攻击时角色抖动/前冲动画

**涉及文件**: `src/entity/sprite.rs`, `src/game/draw.rs`（draw_battle 函数）

**预估行数**: ~80 行

### A3. 伤害数字弹出

**现状**: 伤害值只出现在战斗日志文字中。

**方案**: AttackResult 执行后在敌人位置上方飘出伤害数字，1 秒渐隐：

- Battle 结构体增加 `damage_popups: Vec<DamagePopup>` 字段
- 每帧更新位置和 alpha
- `DrawTextureParams` 支持 alpha 控制

**涉及文件**: `src/battle/state.rs`, `src/game/draw.rs`

**预估行数**: ~60 行

---

## B. 交互反馈（P1）

### B1. NPC 头上气泡提示

**现状**: 玩家需要走到 NPC 旁边按 A 才知道能对话。

**方案**: 在 NPC 头顶绘制 "！" 或 "…" 标记，距离玩家 1.5 tile 内时显示：

- 在 `render_npcs()` 中检测玩家距离，绘制小图标
- 使用像素字体的 `!` 和 `?` 字形

**涉及文件**: `src/game/draw.rs`

**预估行数**: ~30 行

### B2. 菜单音效

**现状**: 进入/退出菜单无声。

**方案**: 在 update.rs 的 Menu/Psynergy/Dialog 状态切换处播放 confirm/cancel 音效：

- `ResourceManager::get_audio()` 获取样本 → `play_sound_once()`
- 触发点: 菜单打开、选择确认、取消返回

**涉及文件**: `src/game/update.rs`, `src/game/draw.rs`

**预估行数**: ~20 行

### B3. 遇敌闪光效果

**现状**: 从世界地图切换到战斗瞬间完成，无过渡。

**方案**: 进入战斗前插入 0.3 秒白色闪光过渡：

- 使用现有的 `Transition` 状态机制，type=Flash
- 闪光完成后设置 `self.battle` 和 `self.state = Battle`

**涉及文件**: `src/game/update.rs`

**预估行数**: ~15 行

---

## C. 内容扩展（P2）

### C1. 多场景地图

**现状**: 只有 Vale 村一张 32×32 地图。

**方案**: 增加 Vale 野外、洞穴入口等地图，通过场景切换连接：

- 地图边缘设置传送点（标志 tile）
- 场景切换时 Transition 动画 → 加载新地图 → 布置 NPC
- 每个场景需要独立的 `SceneRegistry` 管理和 NPC 列表

**涉及文件**: `src/map/tilemap.rs`, `src/game/mod.rs`, `src/scene/mod.rs`

**预估行数**: ~200 行

### C2. 道具/装备/升级系统

**现状**: 无道具系统，SaveData 的 inventory/gold 字段存在但未使用。

**方案**: 
- 地图放置宝箱道具（使用 HiddenChest/OpenedChest tile）
- 战斗胜利获得金币和经验值
- 等级提升增加 HP/PP/攻/防

**涉及文件**: `src/game/mod.rs`, `src/battle/state.rs`, `src/game/update.rs`

**预估行数**: ~100 行

---

## D. 地图交互深度（P2）

### D1. 昼夜系统

**现状**: 游戏无时间概念，始终白天。

**方案**: 每 10 分钟游戏时间切换白天/黄昏/夜晚：
- GameCtx 增加 `game_time: f32` 字段，每帧累加 delta
- 每 600 秒（现实 10 分钟）切换一次阶段
- 黄昏时天空渐变橙色，夜晚时 tile 亮度减半
- 夜晚自动出现 DarkArea tile（依赖 Phase 3 Flash）

**涉及文件**: `src/engine/constants.rs`, `src/game/mod.rs`, `src/game/update.rs`, `src/map/mode7.rs`

**预估行数**: ~80 行

### D2. 动态水面

**现状**: 水面为静态蓝色 tile。

**方案**: 水 tile 颜色做波浪式循环：
- Tile water color 的蓝色分量叠加 `(sin(time * 2) + 1.0) * 0.05` 偏移
- 在 color_map 构建时传入当前时间
- 可选：相邻水 tile 相位偏移产生流动感

**涉及文件**: `src/map/mode7.rs`

**预估行数**: ~30 行

### D3. 可破坏场景

**现状**: 精灵力效果只对特定 tile 有效，一次性。

**方案**: 被 Whirlwind 清除的藤蔓永久消失、Force 推开石块后保留新位置：
- 当前 `modified_tiles` 已支持永久修改
- 扩展：Force 推石块撞到宝箱/敌人时触发额外效果
- 增加 `CanBreak` tile 属性（瓦罐/栅栏）

**涉及文件**: `src/map/mod.rs`, `src/psynergy/effects.rs`

**预估行数**: ~40 行

---

## E. 战斗深度（P1）

### E1. Djinn 精灵系统

**现状**: 无精灵收集机制。

**方案**: 黄金太阳标志性系统：
- `Djinn { id, name, element, set_bonus }` 结构体
- 地图上放置 Djinn 作为可收集隐藏物品
- 装备精灵影响角色属性，释放后进入"回复"状态
- 不同精灵组合改变角色职业，解锁新精灵力

**涉及文件**: `src/data/djinn.rs`（新文件）, `src/battle/state.rs`, `src/game/mod.rs`

**预估行数**: ~200 行

### E2. 属性克制动画

**现状**: 克制计算已在 calculator.rs 中实现但无视觉反馈。

**方案**: 
- 克制（modifier > 1.0）：伤害数字变大 + 闪红 + 屏幕微震
- 被克（modifier < 1.0）：显示 "WEAK" 标记
- 免疫（modifier = 0）：显示 "IMMUNE" 文字弹跳

**涉及文件**: `src/game/draw.rs`, `src/battle/state.rs`

**预估行数**: ~40 行

### E3. 战斗速度调节

**现状**: 战斗动画固定速度。

**方案**: 按住 B 键时 3 倍速跳过攻击动画和日志逐行显示：
- `Battle` 增加 `turbo: bool` 字段
- 攻击动画播放时检测 turbo
- 日志使用 `skip()` 一次性显示全部

**涉及文件**: `src/battle/state.rs`, `src/game/update.rs`, `src/game/draw.rs`

**预估行数**: ~30 行

---

## F. RPG 系统扩展（P2）

### F1. 任务/日志系统

**现状**: 无任务记录，玩家需要记忆当前目标。

**方案**: 
- `QuestLog { entries: Vec<QuestEntry> }` 任务日志
- 关键事件推进任务状态（"和村长对话" → "获得旋风精灵力" → "通过藤蔓路障"）
- HUD 右上角显示当前任务提示简短文字

**涉及文件**: `src/data/quest.rs`（新文件）, `src/game/mod.rs`, `src/ui/mod.rs`

**预估行数**: ~100 行

### F2. NPC 小地图

**现状**: 无地图指引，NPC 位置需要玩家记忆。

**方案**: 右上角 100×100 像素 Vale 村缩略图：
- 绘制时采样地图数据的 1:4 缩放
- 用彩色圆点标记 NPC 位置
- 用白点标记玩家位置

**涉及文件**: `src/ui/mod.rs`, `src/game/draw.rs`

**预估行数**: ~60 行

### F3. 对话分支 + 好感度

**现状**: 对话为线性，无分支。

**方案**: 
- `Faction { npc_id, affinity: i32 }` 好感度系统
- `DialogueChoice` 的 `require_flag`/`set_flag` 已定义但未使用，接入即可
- 好感度影响 NPC 反应、解锁隐藏对话和任务

**涉及文件**: `src/dialogue/mod.rs`, `src/dialogue/script.rs`, `src/game/mod.rs`

**预估行数**: ~80 行

### F4. 传送点解锁

**现状**: 无快速旅行。

**方案**: 
- 地图关键位置设置传送石碑（新增 TileKind::Waypoint）
- 触碰后自动激活，存入已激活列表
- 暂停菜单增加 "Travel" 选项 → 显示已激活传送点列表 → 选择后闪光过渡切场景

**涉及文件**: `src/map/mod.rs`, `src/game/mod.rs`, `src/ui/mod.rs`

**预估行数**: ~80 行

---

## G. 工程化（P3）

### G1. wasm32 一键部署

**现状**: wasm32 目标可构建但无部署配置。

**方案**: 
- 创建 `index.html` shell（自动载入 `.wasm`）
- GitHub Actions workflow 自动构建并部署到 GitHub Pages
- 加入 WebGL 失败降级提示

**涉及文件**: `index.html`（新文件）, `.github/workflows/deploy.yml`（新文件）

**预估行数**: ~60 行

### G2. 游戏数据外置

**现状**: 地图/NPC/对话数据全部硬编码在 Rust 源文件中。

**方案**: 
- 使用 `serde` 从 `.ron` 或 `.json` 文件加载
- 瓦片地图：32×32 u8 数组 → JSON 二维数组
- NPC/对话：结构化 JSON
- 打包进二进制（`include_str!`）或运行时加载（wasm 用 `load_file`）

**涉及文件**: 多个数据文件 + 加载器模块

**预估行数**: ~150 行

### G3. 自定义地图编辑器

**现状**: 修改地图需要改 Rust 源码后重新编译。

**方案**: 
- 简易 GUI 窗口：20×20 tile 网格 + 调色板选择
- 左键放置 tile，右键删除
- 导出为 JSON 格式
- 可独立运行（macroquad 窗口），也可以内嵌为游戏开发模式

**涉及文件**: `tools/editor.rs`（新文件）

**预估行数**: ~200 行

### G4. 回放系统

**现状**: Bug 难以复现。

**方案**: 
- 每帧记录玩家输入（方向 + 按键）到环形缓冲区
- 调试模式下按 R 开始/停止录制
- 按 P 回放：游戏状态重置到录制起点，按记录输入重放

**涉及文件**: `src/engine/replay.rs`（新文件）, `src/game/update.rs`

**预估行数**: ~80 行

---

## H. 视觉风格（P2）

### H1. 扫描线 CRT 滤镜

**现状**: 像素风格但无 CRT 效果。

**方案**: 
- 在 Mode 7 渲染完成后叠加半透明水平线覆盖层
- 每偶数行加深 5% 亮度
- 可选：轻微 RGB 错位（1px 偏移）

**涉及文件**: `src/map/mode7.rs` 或 `src/game/draw.rs`

**预估行数**: ~20 行

### H2. 像素字体粗细变体

**现状**: 只有一种 8×8 字体。

**方案**: 
- 对话用粗体 8×8（现有）
- HUD 用紧凑 6×8（数字/PP/位置名，节省水平空间）
- 标题用大字 16×16（现有字体的 2 倍缩放）

**涉及文件**: `src/ui/font.rs`

**预估行数**: ~60 行

### H3. 天气粒子系统

**现状**: 无环境粒子。

**方案**: 
- `Particle { x, y, speed, lifetime, kind }` 简单粒子系统
- 降雨：垂直下落短线粒子，每帧生成 5-10 个
- 降雪：Z 字形飘落白点粒子
- 粒子在 HUD 层之上、游戏画面之下渲染

**涉及文件**: `src/engine/particle.rs`（新文件）, `src/game/draw.rs`

**预估行数**: ~80 行

---

## 技术风险评估

| 功能 | 风险等级 | 说明 |
|------|---------|------|
| E1 Djinn 系统 | **高** | 涉及职业切换、技能树、战斗内释放/召回，系统边界模糊 |
| G3 地图编辑器 | **高** | 需要独立 GUI，测试困难 |
| F3 对话分支 | 中 | 脚本数据结构需要重设计，现有 `DialogueChoice` 未实战验证 |
| G2 数据外置 | 中 | wasm 端文件加载需要异步处理 |
| H3 天气粒子 | 低 | 独立系统，不与其他模块耦合 |
| D1 昼夜系统 | 低 | 仅影响颜色计算，回退安全 |
| E2 克制动画 | 低 | 纯视觉，不影响逻辑 |
| G4 回放系统 | 低 | 只读操作，不影响游戏逻辑 |


### C3. 多敌人种类

**现状**: 只有 Wolf 和 Bat 两种敌人。

**方案**: 根据场景/难度设定敌人表，不同区域出现不同敌人组合：

```rust
fn enemies_for_area(area: &str) -> Vec<Vec<Combatant>> {
    match area {
        "ValeForest" => vec![
            vec![Combatant::new(10, "Wolf", 3, ...)],
            vec![Combatant::new(11, "Bat", 2, ...), Combatant::new(11, "Bat", 2, ...)],
        ],
        "Cave" => vec![...],
    }
}
```

**涉及文件**: `src/battle/mod.rs` 或 `src/data/`

**预估行数**: ~50 行

---

## 实施路径图

```
Phase 6.5 — 视觉反馈（P0）      推荐 2-3 小时
  A1 精灵力施法动画  →  A2 战斗精灵  →  A3 伤害数字

Phase 6.6 — 交互反馈（P1）      推荐 1 小时
  B1 NPC 气泡  →  B2 菜单音效  →  B3 遇敌闪光

Phase 6.7 — 内容扩展（P2）      推荐 3-5 小时
  C1 多场景  →  C2 道具系统  →  C3 多敌人
```

---

## 技术风险

| 风险 | 影响 | 缓解 |
|------|------|------|
| macroquad 音频延迟高 | SFX 和 BGM 不同步 | 使用短音效，BGM 用长循环 |
| 施法动画期间输入抢占 | 动画被跳过 | 动画期间锁定所有输入 |
| 多场景地图数据膨胀 | 代码体积增长 | 考虑外部数据文件（.ron） |
