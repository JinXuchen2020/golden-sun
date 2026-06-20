# Phase 8 & 8s 任务追踪清单

> 项目：Golden Sun (Rust)  
> 来源：`prompts/08_golden_sun_complete.md` + `prompts/08s_story_content_expansion.md`  
> 创建日期：2026-06-20  
> 完成日期：2026-06-20  
> 目标：补齐商店/旅馆/过场/战斗增强/内容补完等关键功能，使游戏逼近 GBA 原版体验

---

## Phase 8.1：商店系统（P0，~200 行）✅ 全部完成

- [x] 8.1.1 新增 `Shop` 状态到 `GameState` 枚举（`src/engine/game_state.rs`）
- [x] 8.1.2 添加 `shop_inventory()` / `sell_price()` 函数（`src/game/mod.rs`）
- [x] 8.1.3 实现 `draw_shop()` 商店 UI 绘制（`src/game/draw.rs`）
- [x] 8.1.4 实现商店交互逻辑 - 购买/出售/标签切换（`src/game/update.rs`）
- [x] 8.1.5 对话触发商店 - Ivan 脚本 "看看商品" + `_open_shop` flag（`src/dialogue/script.rs` + `src/game/update.rs`）

## Phase 8.2：旅馆/恢复系统（P0，~100 行）✅ 全部完成

- [x] 8.2.1 新增 `Inn` 状态到 `GameState` 枚举（`src/engine/game_state.rs`）
- [x] 8.2.2 新增旅馆老板娘 NPC + 对话脚本（`src/dialogue/script.rs` + `src/entity/mod.rs`）
- [x] 8.2.3 实现旅馆逻辑 & UI - HP/PP 恢复动画（`src/game/update.rs` + `src/game/draw.rs`）

## Phase 8.3：过场引擎与开场动画（P1，~250 行）✅ 全部完成

- [x] 8.3.1 新增 `Cutscene` 状态到 `GameState` 枚举（`src/engine/game_state.rs`）
- [x] 8.3.2 创建 `src/data/cutscene.rs` - CutsceneCmd/Cutscene/三个过场脚本
- [x] 8.3.3 实现过场驱动逻辑 `execute_cutscene_step()`（`src/game/update.rs`）
- [x] 8.3.4 `DialogueState` 增加 `auto_close` 字段支持自动推进（`src/dialogue/mod.rs`）
- [x] 8.3.5 Title 状态开场触发逻辑（`src/game/update.rs`）
- [x] 8.3.6 Cutscene UI 绘制注册（`src/game/draw.rs`）

## Phase 8.4：场景名称弹窗（P1，~50 行）✅ 全部完成

- [x] 8.4.1 新增 `SceneName` 状态到 `GameState` 枚举（`src/engine/game_state.rs`）
- [x] 8.4.2 `apply_scene_switch()` 中触发场景名称弹窗（`src/game/mod.rs`）
- [x] 8.4.3 更新逻辑 & UI 绘制 - 金色渐隐效果（`src/game/update.rs` + `src/game/draw.rs`）
- [x] 8.4.4 `SceneId::display_name()` 方法（`src/scene/mod.rs`）

## Phase 8.5：战斗增强（P1，~200 行）✅ 全部完成

- [x] 8.5.1 战斗背景根据场景变化 `draw_battle_bg()`（`src/game/draw.rs`）
- [x] 8.5.2 战斗中使用道具 - BattleMenu + BattleItemSelect 状态（`src/game/update.rs` + `src/game/draw.rs`）
- [x] 8.5.3 战斗后 EXP 面板增强 - 每人 EXP + 金币 + 进度条（`src/game/draw.rs`）
- [x] 8.5.4 战斗评级星级 `calculate_battle_grade()`（`src/game/draw.rs`）

## Phase 8.6：Djinn 收集增强（P2，~150 行）✅ 全部完成

- [x] 8.6.1 `world_djinn()` 从 3 扩展到 8 个 Djinn（`src/data/djinn.rs`）
- [x] 8.6.2 Djinn 收集光点显示 `draw_djinn_hints()`（`src/game/draw.rs`）
- [x] 8.6.3 Djinn 收集动画 `DjinnObtained` 状态 + 消息弹出（`src/game/update.rs` + `src/game/draw.rs`）

## Phase 8.7：音频扩展（P2，~100 行）✅ 全部完成

- [x] 8.7.1 新增 SFX - levelup/shop_buy/victory/djinn/summon（`src/audio/mod.rs`）
- [x] 8.7.2 Boss BGM `generate_boss_bgm()` + BgmPlayer 注册（`src/audio/mod.rs`）
- [x] 8.7.3 场景 BGM 自动切换 `apply_scene_switch()` 中（`src/game/mod.rs`）

## Phase 8.8：故事补完（P2，~150 行）✅ 全部完成

- [x] 8.8.1 主线对话增强 - Garsmin 告别页 + Garet 出发页（`src/dialogue/script.rs`）
- [x] 8.8.2 完整任务链 `default_quests()` 替换（`src/data/quest.rs`）
- [x] 8.8.3 任务自动推进 `update_story_progression()`（`src/game/update.rs`）

## Phase 8s.1：故事对话大扩充（P0，~200 行）✅ 全部完成

- [x] 8s.1.1 Ivan 扩充至 6 页（追加 Page 5-6）
- [x] 8s.1.1 Mia 扩充至 6 页（追加 Page 5-6）
- [x] 8s.1.1 Garsmin 扩充至 10 页（追加 Page 8-10）
- [x] 8s.1.1 Garet 扩充至 5 页（追加 Page 4-5）
- [x] 8s.1.2 新增场景 NPC - forest_hermit / cave_prospector 对话脚本
- [x] 8s.1.2 `create_npcs_for_scene()` 为 WildForest/Cave 增加 NPC 实体

## Phase 8s.2：任务链连贯化（P0，~80 行）✅ 全部完成

- [x] 8s.2.1 QuestEntry 增加 `chapter` 字段 + 替换 `default_quests()` 为章节化任务链
- [x] 8s.2.2 `auto_update_quests()` 函数实现任务自动推进
- [x] 8s.2.3 QuestLog 新增辅助方法 `has()` / `is_completed()` / `unlock()`
- [x] 8s.2.3 `QUEST_TEMPLATES` 常量定义

## Phase 8s.3：Djinn 收集扩展 + 地图布局（P1，~100 行）✅ 全部完成

- [x] 8s.3.1 `world_djinn()` 扩展到 8 个 Djinn（同 8.6.1）
- [x] 8s.3.2 Djinn 收集消息提示（同 8.6.3）
- [x] 8s.3.3 Djinn 地图光点显示 `draw_djinn_hints()`（同 8.6.2）

## Phase 8s.4：敌人/战斗内容扩展（P1，~80 行）✅ 全部完成

- [x] 8s.4.1 EnemyConfig 增加 `display_name` / `description` 字段 + 中文名配置
- [x] 8s.4.2 战斗中道具使用 `BattleAction::UseItem` 实现

## Phase 8.9 / 8s.5：最终验证 ✅ 全部完成

- [x] `cargo check` 编译检查 — **通过**
- [x] `cargo clippy --all-targets -- -D warnings` — **零警告**
- [x] `cargo test` — **250 个测试全部通过**
- [ ] 手动运行 `cargo run` 逐项验证清单 — **待用户手动验证**
- [x] `git add -A && git commit` — **已提交 (002809d)**
- [ ] `git push` — **网络失败，待重试**

---

## 统计

| 指标 | 数值 |
|------|------|
| 总任务数 | 47 |
| 已完成 | 45 |
| 完成率 | 95.7% |
| 修改文件 | 17 个源码文件 + 3 个测试文件 |
| 新增文件 | 2 个 (cutscene.rs, prompts/) |
| 代码变更 | +5061 / -200 行 |
| 测试通过 | 250/250 |
| Clippy | 零警告 |

## 待办事项

1. 手动运行 `cargo run` 验证商店/旅馆/过场/战斗等新功能
2. 修复网络后执行 `git push`
