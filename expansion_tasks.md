# 黄金太阳 Rust 复刻 — 扩展任务清单

> 基于 `expansion_plan.md` 拆分，按实施路径排序

---

## Phase 6.5：视觉反馈（P0）

### A1 — 精灵力施法动画 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| A1.1 新增 PsynergyAnim 状态 | `game_state.rs` | 10 | 添加 `PsynergyAnim { timer, psynergy, tx, ty }` 变体 |
| A1.2 update 路由动画推进 | `update.rs` | 20 | 动画期间锁定输入，timer 推进，结束后执行 tile 修改 |
| A1.3 draw 渲染施法特效 | `draw.rs` | 40 | 7 种精灵力对应不同特效（旋风/生长/闪光/抖动） |
| A1.4 try_use 改为触发动画 | `update.rs` | 15 | 将直接 tile 修改改为设置动画状态 |
| A1.5 测试 | — | 10 | 动画状态推进/结束/输入锁定测试 |
| **合计** | | **~95 行** | |

### A2 — 战斗精灵渲染 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| A2.1 生成战斗角色纹理 | `sprite.rs` | 20 | 新增 `generate_battle_sprites()` 调用现有绘制函数 |
| A2.2 SpriteAtlas 持有战斗纹理 | `game/mod.rs` | 10 | 新增 `battle_sprites: [Texture2D; 4]` 字段 |
| A2.3 draw_battle 绘制精灵 | `draw.rs` | 40 | 玩家左半屏 2 列，敌人右半屏 1-2 列 |
| A2.4 攻击抖动动画 | `draw.rs` | 15 | 攻击时目标角色水平偏移 3 帧复位 |
| A2.5 测试 | — | 10 | 角色纹理非空检查 |
| **合计** | | **~95 行** | |

### A3 — 伤害数字弹出 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| A3.1 DamagePopup 结构体 | `state.rs` | 10 | `{ damage, x, y, timer }` |
| A3.2 execute_turn 推入 popup | `state.rs` | 5 | 每次 Attack/Psynergy 产生伤害时追加 |
| A3.3 update 更新 timer/位置 | `game/update.rs` | 10 | 每帧 timer += dt, y -= 0.5（上飘） |
| A3.4 draw 渲染数字 | `game/draw.rs` | 15 | 根据 timer 控制 alpha 渐隐（1 秒渐出） |
| A3.5 清理已完成的 popup | `update.rs` | 5 | timer > 1.0 时移除 |
| **合计** | | **~45 行** | |

---

## Phase 6.6：交互反馈（P1）

### B1 — NPC 头上气泡提示 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| B1.1 render_npcs 检测距离 | `draw.rs` | 10 | 计算玩家与 NPC 距离，< 1.5 tile 标记 |
| B1.2 绘制气泡图标 | `draw.rs` | 10 | 像素字体绘制 "!" 在 NPC 头顶 |
| **合计** | | **~20 行** | |

### B2 — 菜单音效 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| B2.1 ResourceManager 获取 SFX | `game/mod.rs` | 5 | 添加 `sfx` 字段到 GameCtx |
| B2.2 菜单打开/关闭播放 | `update.rs` | 10 | Menu/Psynergy/Dialog 进入/退出时播放 |
| B2.3 确认选择播放 | `update.rs` | 5 | Confirm 事件处理时播放 confirm 音效 |
| **合计** | | **~20 行** | |

### B3 — 遇敌闪光效果 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| B3.1 遇敌时设置 Transition | `update.rs` | 10 | `start_transition("Flash")` 然后设置 battle |
| B3.2 过渡结束时进入战斗 | `update.rs` | 5 | transition 完成后 state = Battle |
| **合计** | | **~15 行** | |

---

## Phase 6.7：内容扩展（P2）

### C1 — 多场景地图 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| C1.1 地图数据模块化 | `tilemap.rs` | 30 | 将 Vale 村数据提取为函数，支持按场景名加载 |
| C1.2 新增野外/洞穴地图 | `tilemap.rs` | 60 | 2-3 张新地图（16×16~32×32） |
| C1.3 场景切换机制 | `game/update.rs` | 20 | 地图边缘检测 → Transition → 切换 |
| C1.4 场景 NPC 配置 | `entity/mod.rs` | 20 | 每场景独立 NPC 列表 |
| C1.5 新地图的遇敌配置 | `game/update.rs` | 10 | 不同地区不同敌人池 |
| **合计** | | **~140 行** | |

### C2 — 道具/升级系统 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| C2.1 宝箱拾取逻辑 | `update.rs` | 15 | 走到 OpenedChest 旁按 A 获取道具 |
| C2.2 战斗奖励金币/EXP | `state.rs` | 20 | Victory 触发 gold/exp 计算，升级属性提升 |
| C2.3 道具 UI | `draw.rs` | 20 | 道具列表 + 使用/丢弃 |
| C2.4 Inventory 数据结构 | `game/mod.rs` | 15 | `items: Vec<Item>` 字段 |
| **合计** | | **~70 行** | |

### C3 — 多敌人种类 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| C3.1 敌人配置表 | `data/mod.rs` | 30 | 按区域定义敌人编队列表 |
| C3.2 按区域选取敌人 | `game/update.rs` | 10 | `enemies_for_area("ValeForest")` |
| C3.3 新敌人属性平衡 | `data/mod.rs` | 15 | 5-8 种不同属性/等级的敌人 |
| **合计** | | **~55 行** | |

---

## Phase 6.8：地图交互深度（P2）

### D1 — 昼夜系统 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| D1.1 game_time 字段 + 累加 | `game/mod.rs` | 10 | `game_time: f32`，每帧 +delta |
| D1.2 阶段切换逻辑 | `update.rs` | 15 | 每 600s 切换白天→黄昏→夜晚 |
| D1.3 天空颜色映射 | `mode7.rs` | 10 | 从常量表中按阶段选天空色 |
| D1.4 夜间 tile 亮度减半 | `mode7.rs` | 10 | color_map 构建时亮度系数 |
| **合计** | | **~45 行** | |

### D2 — 动态水面 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| D2.1 波浪颜色偏移 | `mode7.rs` | 15 | water color 叠加 `sin(time)` 蓝色偏移 |
| D2.2 mode7 接收 time 参数 | `mode7.rs` | 10 | render() 增加 time: f32 参数 |
| **合计** | | **~25 行** | |

### D3 — 可破坏场景 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| D3.1 CanBreak tile 属性 | `map/mod.rs` | 5 | TileKind 新增 `is_breakable()` |
| D3.2 Force 撞墙/宝箱逻辑 | `effects.rs` | 20 | PushBlock 推到目标触发额外效果 |
| **合计** | | **~25 行** | |

---

## Phase 6.9：战斗深度（P1）

### E1 — Djinn 精灵系统 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| E1.1 Djinn 数据结构 | `data/djinn.rs` | 30 | `Djinn { id, name, element, bonus }` |
| E1.2 Djinn 管理器 | `game/mod.rs` | 30 | 收集、装备、释放/召回状态机 |
| E1.3 战斗内 Djinn 指令 | `battle/state.rs` | 40 | DjinnRelease(DjinnId, target) 行动 |
| E1.4 职业切换 + 新精灵力 | `psynergy/mod.rs` | 40 | 精灵组合 → 角色职业 → 可用技能变化 |
| E1.5 地图 Djinn 放置 | `entity/mod.rs` | 30 | Vale 村 2-3 个可收集 Djinn |
| E1.6 测试 | — | 30 | Djinn 收集/装备/释放测试 |
| **合计** | | **~200 行** | |

### E2 — 属性克制动画 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| E2.1 克制闪红特效 | `draw.rs` | 15 | modifier > 1.0 时伤害文字变红 + 放大 |
| E2.2 被克 WEAK 标记 | `draw.rs` | 10 | modifier < 1.0 时显示 "WEAK" |
| E2.3 免疫 IMMUNE 显示 | `draw.rs` | 10 | modifier = 0 时弹跳 "IMMUNE" |
| **合计** | | **~35 行** | |

### E3 — 战斗加速 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| E3.1 turbo 字段 | `state.rs` | 5 | `turbo: bool` |
| E3.2 B 键检测 | `update.rs` | 10 | 按住 B 时设置 turbo |
| E3.3 加速渲染 | `draw.rs` | 10 | turbo 时跳过帧间延迟 |
| **合计** | | **~25 行** | |

---

## Phase 6.10：RPG 系统（P2）

### F1 — 任务/日志 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| F1.1 QuestLog 结构体 | `data/quest.rs` | 20 | `{ entries: Vec<QuestEntry> }` |
| F1.2 关键事件推进 | `update.rs` | 20 | 获取精灵力/通关/对话时更新任务 |
| F1.3 HUD 任务提示 | `draw.rs` | 15 | 右上角显示当前任务名称 |
| **合计** | | **~55 行** | |

### F2 — NPC 小地图 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| F2.1 地图 1:4 采样 | `draw.rs` | 15 | 从 MAP_DATA 采样生成缩略图 |
| F2.2 NPC/玩家标注 | `draw.rs` | 15 | 彩色圆点标注位置 |
| **合计** | | **~30 行** | |

### F3 — 对话分支 + 好感度 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| F3.1 Faction 好感度系统 | `game/mod.rs` | 15 | `affinity: HashMap<u32, i32>` |
| F3.2 DialogueChoice 接入 | `dialogue/script.rs` | 20 | 为 NPC 脚本添加 choice 分支 |
| F3.3 选择 UI | `draw.rs` | 20 | 选项列表 + 高亮 + 确认 |
| F3.4 好感度影响对话 | `dialogue/mod.rs` | 15 | 根据 affinity 选择不同对话页 |
| **合计** | | **~70 行** | |

### F4 — 传送点 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| F4.1 Waypoint tile 定义 | `map/mod.rs` | 5 | 新增 TileKind::Waypoint |
| F4.2 触碰激活 | `update.rs` | 15 | 玩家走到 Waypoint 时加入已激活列表 |
| F4.3 传送菜单 | `draw.rs` | 20 | 菜单新增 "Travel" 选项 → 传送点列表 |
| F4.4 场景切换 | `update.rs` | 10 | 选择后 Transition → 切 Camera 位置 |
| **合计** | | **~50 行** | |

---

## Phase 6.11：工程化（P3）

### G1 — wasm32 一键部署 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| G1.1 index.html shell | `index.html` | 30 | 加载 wasm 的 HTML 页面 |
| G1.2 GitHub Actions workflow | `.github/workflows/deploy.yml` | 20 | 构建 + 部署到 Pages |
| **合计** | | **~50 行** | |

### G2 — 游戏数据外置 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| G2.1 地图 JSON 格式 | `data/maps/` | 30 | Vale 村 32×32 导出为 JSON |
| G2.2 JSON 加载器 | `data/loader.rs` | 40 | `load_map("vale")` 从 JSON 解析 |
| G2.3 NPC/对话 JSON | `data/npcs/` | 30 | NPC 数据外置 |
| **合计** | | **~100 行** | |

### G3 — 地图编辑器 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| G3.1 编辑器 GUI | `tools/editor.rs` | 80 | 20×20 网格 + 调色板 + 鼠标交互 |
| G3.2 JSON 导出 | `tools/editor.rs` | 30 | 地图数据序列化为 JSON |
| G3.3 内嵌开发模式 | `game/mod.rs` | 20 | debug 模式按 E 启动编辑器 |
| **合计** | | **~130 行** | |

### G4 — 回放系统 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| G4.1 InputFrame 环形缓冲区 | `engine/replay.rs` | 25 | 记录每帧输入状态 |
| G4.2 录制/回放控制 | `game/update.rs` | 20 | R 录制，P 回放 |
| G4.3 状态重置 | `game/mod.rs` | 15 | 回放前重置到录制起点 |
| **合计** | | **~60 行** | |

---

## Phase 6.12：视觉风格（P2）

### H1 — 扫描线 CRT 滤镜 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| H1.1 水平线覆盖层 | `draw.rs` | 10 | 每偶数行降低亮度 |
| H2.2 RGB 错位（可选） | `draw.rs` | 10 | 1px 偏移制造色散效果 |
| **合计** | | **~20 行** | |

### H2 — 像素字体粗细变体 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| H2.1 6×8 紧凑字形集 | `font.rs` | 30 | 数字/字母的 6×8 紧凑版 |
| H2.2 字体选择 API | `font.rs` | 15 | `draw_text_variant(text, x, y, variant)` |
| **合计** | | **~45 行** | |

### H3 — 天气粒子系统 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| H3.1 Particle 结构体 | `engine/particle.rs` | 20 | `{ x, y, speed, lifetime, kind }` |
| H3.2 降雨生成器 | `engine/particle.rs` | 15 | 每帧生成 5-10 个雨滴 |
| H3.3 降雪生成器 | `engine/particle.rs` | 15 | 每帧生成 3-5 个雪片 |
| H3.4 粒子渲染 | `draw.rs` | 15 | 在 Mode7 之上、HUD 之下渲染 |
| **合计** | | **~65 行** | |

---

## 总览

| 阶段 | 任务数 | 预估行数 | 优先级 |
|------|--------|---------|--------|
| Phase 6.5 视觉反馈 | 3 | ~235 | P0 |
| Phase 6.6 交互反馈 | 3 | ~55 | P1 |
| Phase 6.7 内容扩展 | 3 | ~265 | P2 |
| Phase 6.8 地图交互 | 3 | ~95 | P2 |
| Phase 6.9 战斗深度 | 3 | ~260 | P1 |
| Phase 6.10 RPG 系统 | 4 | ~205 | P2 |
| Phase 6.11 工程化 | 4 | ~340 | P3 |
| Phase 6.12 视觉风格 | 3 | ~130 | P2 |
| **总计** | **26** | **~1,585** | |

---

## 开发顺序建议

```
Week 1: A1 施法动画 → A3 伤害数字（视觉反馈，快速见效）
Week 2: A2 战斗精灵 → B2 菜单音效（完善战斗体验）
Week 3: B1 NPC气泡 → B3 遇敌闪光（小优化收尾）
Week 4+: C1 多场景 → C2 道具 → C3 多敌人（内容扩展）
```
