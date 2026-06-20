# Phase 8 & 8s 任务追踪清单

> 项目：Golden Sun (Rust)  
> 来源：`prompts/08_golden_sun_complete.md` + `prompts/08s_story_content_expansion.md`  
> 创建日期：2026-06-20  
> 目标：补齐商店/旅馆/过场/战斗增强/内容补完等关键功能，使游戏逼近 GBA 原版体验

---

## Phase 8.1：商店系统（P0，~200 行）

- [ ] 8.1.1 新增 `ShopState` 到 `GameState` 枚举（`src/engine/game_state.rs`）
- [ ] 8.1.2 添加 `shop_inventory()` / `sell_price()` 函数（`src/game/mod.rs`）
- [ ] 8.1.3 实现 `draw_shop()` 商店 UI 绘制（`src/game/draw.rs`）
- [ ] 8.1.4 实现商店交互逻辑 - 购买/出售/标签切换（`src/game/update.rs`）
- [ ] 8.1.5 对话触发商店 - `DialogueAction::OpenShop` + Ivan 脚本修改（`src/dialogue/mod.rs` + `src/dialogue/script.rs`）

## Phase 8.2：旅馆/恢复系统（P0，~100 行）

- [ ] 8.2.1 新增 `Inn` 状态到 `GameState` 枚举（`src/engine/game_state.rs`）
- [ ] 8.2.2 新增旅馆老板娘 NPC + 对话脚本（`src/dialogue/script.rs` + `src/entity/mod.rs`）
- [ ] 8.2.3 实现旅馆逻辑 & UI - HP/PP 恢复动画（`src/game/update.rs` + `src/game/draw.rs`）

## Phase 8.3：过场引擎与开场动画（P1，~250 行）

- [ ] 8.3.1 新增 `Cutscene` 状态到 `GameState` 枚举（`src/engine/game_state.rs`）
- [ ] 8.3.2 创建 `src/data/cutscene.rs` - CutsceneCmd/Cutscene/overlapping cutscene 脚本
- [ ] 8.3.3 实现过场驱动逻辑 `execute_cutscene_step()`（`src/game/update.rs`）
- [ ] 8.3.4 `DialogueState` 增加 `auto_close` 字段支持自动推进（`src/dialogue/mod.rs`）
- [ ] 8.3.5 Title 状态开场触发逻辑（`src/game/update.rs`）
- [ ] 8.3.6 Cutscene UI 绘制注册（`src/game/draw.rs`）

## Phase 8.4：场景名称弹窗（P1，~50 行）

- [ ] 8.4.1 新增 `SceneName` 状态到 `GameState` 枚举（`src/engine/game_state.rs`）
- [ ] 8.4.2 `apply_scene_switch()` 中触发场景名称弹窗（`src/game/mod.rs`）
- [ ] 8.4.3 更新逻辑 & UI 绘制 - 金色渐隐效果（`src/game/update.rs` + `src/game/draw.rs`）
- [ ] 8.4.4 `SceneId::display_name()` 方法（`src/scene/mod.rs`）

## Phase 8.5：战斗增强（P1，~200 行）

- [ ] 8.5.1 战斗背景根据场景变化 `draw_battle_bg()`（`src/game/draw.rs`）
- [ ] 8.5.2 战斗中使用道具 - BattleMenu 状态 + Item 选项（`src/game/update.rs` + `src/game/draw.rs`）
- [ ] 8.5.3 战斗后 EXP 面板增强 - 每人 EXP + 金币 + 进度条（`src/game/draw.rs`）
- [ ] 8.5.4 战斗评级星级 `calculate_battle_grade()`（`src/game/draw.rs`）

## Phase 8.6：Djinn 收集增强（P2，~150 行）

- [ ] 8.6.1 `world_djinn()` 从 3 扩展到 8 个 Djinn（`src/data/djinn.rs`）
- [ ] 8.6.2 Djinn 收集光点显示 `draw_djinn_pickups()`（`src/game/draw.rs`）
- [ ] 8.6.3 Djinn 收集动画 `DjinnObtained` 状态 + 消息弹出（`src/game/update.rs` + `src/game/draw.rs`）

## Phase 8.7：音频扩展（P2，~100 行）

- [ ] 8.7.1 新增 SFX - levelup/shop_buy/victory/djinn/summon（`src/audio/mod.rs`）
- [ ] 8.7.2 Boss BGM `generate_boss_bgm()` + BgmPlayer 注册（`src/audio/mod.rs`）
- [ ] 8.7.3 场景 BGM 自动切换 `apply_scene_switch()` 中（`src/game/mod.rs`）

## Phase 8.8：故事补完（P2，~150 行）

- [ ] 8.8.1 主线对话增强 - Garsmin 告别页 + Garet 出发页（`src/dialogue/script.rs`）
- [ ] 8.8.2 完整任务链 `default_quests()` 替换（`src/data/quest.rs`）
- [ ] 8.8.3 任务自动推进 `update_story_progression()`（`src/game/update.rs`）

## Phase 8s.1：故事对话大扩充（P0，~200 行）

- [ ] 8s.1.1 Ivan 扩充至 6 页（追加 Page 5-6）
- [ ] 8s.1.1 Mia 扩充至 6 页（追加 Page 5-6）
- [ ] 8s.1.1 Garsmin 扩充至 10 页（追加 Page 8-10）
- [ ] 8s.1.1 Garet 扩充至 5 页（追加 Page 4-5）
- [ ] 8s.1.2 新增场景 NPC - forest_hermit / cave_prospector 对话脚本
- [ ] 8s.1.2 `create_npcs_for_scene()` 为 WildForest/Cave 增加 NPC 实体

## Phase 8s.2：任务链连贯化（P0，~80 行）

- [ ] 8s.2.1 QuestEntry 增加 `chapter` 字段 + 替换 `default_quests()` 为章节化任务链
- [ ] 8s.2.2 `auto_update_quests()` 函数实现任务自动推进
- [ ] 8s.2.3 QuestLog 新增辅助方法 `has()` / `is_completed()` / `unlock()`
- [ ] 8s.2.3 `QUEST_TEMPLATES` 常量定义

## Phase 8s.3：Djinn 收集扩展 + 地图布局（P1，~100 行）

- [ ] 8s.3.1 `world_djinn()` 扩展到 8 个 Djinn（同 8.6.1）
- [ ] 8s.3.2 Djinn 收集消息提示（同 8.6.3）
- [ ] 8s.3.3 Djinn 地图光点显示 `draw_djinn_hints()`（同 8.6.2）

## Phase 8s.4：敌人/战斗内容扩展（P1，~80 行）

- [ ] 8s.4.1 EnemyConfig 增加 `display_name` / `description` 字段 + 中文名配置
- [ ] 8s.4.2 战斗中道具使用 `BattleAction::UseItem` 实现

## Phase 8.9 / 8s.5：最终验证（必做）

- [ ] `cargo check` 编译检查（零错误）
- [ ] `cargo clippy --all-targets -- -D warnings`（零警告）
- [ ] `cargo test` 测试全绿
- [ ] 手动运行 `cargo run` 逐项验证清单
- [ ] `git add -A && git commit && git push`

---

## 依赖关系图

```
Phase 8.1 (商店) ──┐
Phase 8.2 (旅馆) ──┤  互不依赖，可并行
Phase 8.3 (过场) ──┼── 依赖 GameState 扩展
Phase 8.4 (场景名) ─┘
Phase 8.5 (战斗) ──┬── 依赖 Battle 状态
Phase 8.6 (Djinn) ─┤── 依赖 Djinn 系统
Phase 8.7 (音频) ──┼── 依赖 Audio 系统
Phase 8.8 (故事) ──┘
Phase 8s.1 (对话) ─┤── 依赖 Dialogue 系统
Phase 8s.2 (任务) ─┼── 依赖 QuestLog
Phase 8s.3 (Djinn) ─┤── 与 8.6 重叠
Phase 8s.4 (敌人) ─┘
                    ↓
Phase 8.9/8s.5 (验证) ── 最后执行，全部依赖上述任务
```

## 文件修改清单

| 文件 | 涉及 Phase | 预估改动 |
|------|-----------|---------|
| `src/engine/game_state.rs` | 8.1-8.4, 8.6 | ~50 行新增 |
| `src/game/mod.rs` | 8.1, 8.3, 8.4, 8.7 | ~100 行 |
| `src/game/update.rs` | 8.1-8.5, 8.7-8.8 | ~300 行 |
| `src/game/draw.rs` | 8.1, 8.2, 8.4-8.6 | ~250 行 |
| `src/dialogue/mod.rs` | 8.1, 8.3 | ~10 行 |
| `src/dialogue/script.rs` | 8.1, 8.8, 8s.1 | ~200 行 |
| `src/data/cutscene.rs` | 8.3 | **新建** ~100 行 |
| `src/data/quest.rs` | 8s.2 | ~80 行 |
| `src/data/djinn.rs` | 8.6, 8s.3 | ~30 行 |
| `src/data/mod.rs` | 8s.4 | ~60 行 |
| `src/scene/mod.rs` | 8.4 | ~10 行 |
| `src/audio/mod.rs` | 8.7 | ~50 行 |
| `src/entity/mod.rs` | 8s.1 | ~20 行 |
| **合计** | | **~1,200 行新增/修改** |

## 执行计划

### 第一阶段：核心系统（P0）
1. Phase 8.1 商店系统
2. Phase 8.2 旅馆系统

### 第二阶段：叙事引擎（P1）
3. Phase 8.3 过场引擎
4. Phase 8.4 场景名称弹窗

### 第三阶段：内容扩充（P1-P2）
5. Phase 8.5 战斗增强
6. Phase 8.6 Djinn 收集增强
7. Phase 8.7 音频扩展
8. Phase 8.8 故事补完
9. Phase 8s.1 故事对话大扩充
10. Phase 8s.2 任务链连贯化
11. Phase 8s.3 Djinn 收集扩展
12. Phase 8s.4 敌人/战斗内容扩展

### 第四阶段：验证（必做）
13. Phase 8.9/8s.5 最终验证
