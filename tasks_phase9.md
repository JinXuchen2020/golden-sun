# Phase 9 任务追踪清单 — 深度内容扩充

> 来源：`prompts/09_golden_sun_depth_expansion.md`  
> 创建日期：2026-06-20  
> 目标：扩展世界地图/叙事深度/玩法密度，逼近 GBA 原版体验  
> 预估新增行数：~1,800 行，15+ 文件

---

## Phase 9.1：序幕开场与故事完整化（P0，~350 行）⬜

### 9.1.1 开场序幕过场 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.1.1 | `src/data/cutscene.rs` | 25 | 追加 `opening_prologue()` 函数，三段故事文字+Fade过渡 |

### 9.1.2 标题画面开场触发 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.1.2 | `src/game/update.rs` | 15 | Title 状态首次 Confirm → 触发开场过场，`opening_seen` flag 控制 |

### 9.1.3 开场后剧情触发 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.1.3 | `src/game/update.rs` | 20 | 开场完成后自动进 Vale、设位置、`opening_done` flag |

### 9.1.4 追加 Garsmin 起始对话 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.1.4 | `src/dialogue/script.rs` | 10 | Garsmin Page 0 — 开场后首次对话引导玩家见 Ivan/Mia |

### 9.1.5 任务链深化 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.1.5a | `src/data/quest.rs` | 80 | 替换 `default_quests()` 为 15 个任务、4 章节（Act 1-4） |
| 9.1.5b | `src/data/quest.rs` | 20 | 同步替换 `QUEST_TEMPLATES` |

### 9.1.6 故事推进逻辑增强 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.1.6a | `src/game/update.rs` | 100 | 替换 `update_story_progression()` — Act 1 推进（开场→Garsmin→Ivan/Mia→精灵力→Garet→Sol Sanctum→MythrilGolem） |
| 9.1.6b | `src/game/update.rs` | 40 | Act 2 推进（离开Vale→WildForest→Djinn收集→Cave） |
| 9.1.6c | `src/game/update.rs` | 40 | Act 3-4 推进（Cave贤者→全精灵力→召唤→10 Djinn→12 Djinn） |

### 9.1.6 辅助 Flag 检查 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.1.6d | `src/data/djinn.rs` | 5 | 在 Djinn 拾取处设 `collected_any_djinn` flag |
| 9.1.6e | `src/psynergy/mod.rs` | 5 | 在 Psynergy 使用处设 `psynergy_used` flag |
| 9.1.6f | `src/battle/state.rs` | 5 | 在召唤执行处设 `summon_used_in_battle` flag |

| **合计** | | **~350 行** | |

---

## Phase 9.2：场景深度扩展（P1，~550 行）⬜

### 9.2.1 新增 SceneId 变体 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.2.1a | `src/scene/mod.rs` | 5 | `SceneId` 追加 `Bilibin`、`KolimaForest` |
| 9.2.1b | `src/scene/mod.rs` | 5 | `display_name()` 追加 `"Bilibin 镇"`、`"柯利玛森林"` |
| 9.2.1c | `src/game/mod.rs` | 10 | `start_random_battle()` match 补全 |
| 9.2.1d | `src/game/mod.rs` | 5 | `save_game()` match 补全 |
| 9.2.1e | `src/game/mod.rs` | 10 | `check_djinn_pickup()` match 补全 |
| 9.2.1f | `src/game/draw.rs` | 5 | `draw_hud()` 地点名 match 补全 |
| 9.2.1g | `src/game/update.rs` | 10 | 场景边界检查 & `update_story_progression()` match 补全 |
| 9.2.1h | `src/entity/mod.rs` | 5 | `create_npcs_for_scene()` match 补全 |
| 9.2.1i | `src/data/mod.rs` | 5 | `enemies_for_area()` match 补全 |
| 9.2.1j | `src/map/tilemap.rs` | 10 | `get_scene_map()`、`map_size()` 等 match 补全 |

### 9.2.2 新增地图：Bilibin 镇 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.2.2 | `src/map/tilemap.rs` | 45 | 20×20 Bilibin 镇地图数据（中心广场+街道+建筑） |

### 9.2.3 新增地图：KolimaForest ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.2.3 | `src/map/tilemap.rs` | 50 | 24×24 KolimaForest 森林迷宫地图数据 |

### 9.2.4 场景出入口连接 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.2.4a | `src/game/update.rs` | 30 | `check_scene_boundaries()` 追加 WildForest↔Bilibin↔KolimaForest 连接 |
| 9.2.4b | `src/game/update.rs` | 10 | 确保场景切换时 camera 位置正确 |

### 9.2.5 新场景的敌人配置 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.2.5a | `src/data/mod.rs` | 5 | Bilibin 敌人：Rat(Lv2)、Spider(Lv3) |
| 9.2.5b | `src/data/mod.rs` | 8 | KolimaForest 敌人：Wolf(Lv5)、Treant(Lv6)、Slime(Lv3)、Mandrake(Lv7)、Moth(Lv4) |

### 9.2.6 新场景的 NPC 配置 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.2.6a | `src/entity/mod.rs` | 10 | Bilibin NPC ×4（长老/商人/旅行者/守卫） |
| 9.2.6b | `src/entity/mod.rs` | 8 | KolimaForest NPC ×2（流浪者/贤者） |

### 9.2.7 新场景的 Djinn 位置 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.2.7 | `src/data/djinn.rs` | 8 | Bilibin +2 Djinn（Titania/Laguna）、KolimaForest +2 Djinn（Amduscia/Grendel） |

| **合计** | | **~550 行** | |

---

## Phase 9.3：Bilibin 镇的 NPC 与故事内容（P1，~250 行）⬜

### 9.3.1 Bilibin NPC 对话脚本 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.3.1a | `src/dialogue/script.rs` | 30 | `bilibin_elder` 对话脚本（2 页，介绍 Cave 情报） |
| 9.3.1b | `src/dialogue/script.rs` | 30 | `bilibin_merchant` 对话脚本（2 页 + 商店选择） |
| 9.3.1c | `src/dialogue/script.rs` | 25 | `bilibin_traveler` 对话脚本（2 页，介绍 Kolima 森林） |
| 9.3.1d | `src/dialogue/script.rs` | 30 | `bilibin_guard` 对话脚本（2 页，警告危险） |

### 9.3.2 Kolima Forest NPC 对话脚本 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.3.2a | `src/dialogue/script.rs` | 20 | `kolima_wanderer` 对话脚本（2 页，Growth 提示） |
| 9.3.2b | `src/dialogue/script.rs` | 20 | `kolima_sage` 对话脚本（2 页，Djinn 线索） |

### 9.3.3 Bilibin 商店整合 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.3.3a | `src/game/mod.rs` | 30 | 新增 `handle_shop_trigger()` 区分 Vale/Bilibin 商店 |
| 9.3.3b | `src/game/mod.rs` | 20 | Bilibin 商品列表：长剑/精灵之刃/锁子甲/精灵护甲/力量手环/精灵徽章+Potion/Ether/GoldRing |

### 9.3.4 Kolima Forest 的 Puzzle ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.3.4a | `src/game/update.rs` | 20 | KolimaForest 场景事件：多种子生长→生成新路径 |
| 9.3.4b | `src/map/tilemap.rs` | 10 | KolimaForest 地图放置 Seed/HiddenChest tile |

| **合计** | | **~250 行** | |

---

## Phase 9.4：Vale 村 NPC 扩充（P2，~150 行）⬜

### 9.4.1 Vale 村新增 NPC ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.4.1 | `src/entity/mod.rs` | 8 | Vale 追加 NPC ×4：小孩(6)/农民(7)/渔夫(8)/老妇人(9) |

### 9.4.2 Vale 新增 NPC 对话脚本 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.4.2a | `src/dialogue/script.rs` | 20 | `vale_child` 对话脚本（3 页） |
| 9.4.2b | `src/dialogue/script.rs` | 20 | `vale_farmer` 对话脚本（3 页） |
| 9.4.2c | `src/dialogue/script.rs` | 18 | `vale_fisher` 对话脚本（3 页，池塘闪光线索） |
| 9.4.2d | `src/dialogue/script.rs` | 22 | `vale_old_woman` 对话脚本（4 页，身世线索） |

### 9.4.3 修复缺失的 NPC 脚本 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.4.3a | `src/dialogue/script.rs` | 15 | `forest_traveler` 对话脚本（2 页）— 检查是否缺失，如需追加 |
| 9.4.3b | `src/dialogue/script.rs` | 18 | `cave_sage` 对话脚本（2 页）— 检查是否缺失，如需追加 |

| **合计** | | **~150 行** | |

---

## Phase 9.5：装备与战斗系统扩展（P1，~180 行）⬜

### 9.5.1 新增装备 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.5.1 | `src/game/mod.rs` | 15 | `all_equipment()` 追加 5 件：破甲剑/战斗斧/铁甲/治愈戒指/力量腰带 |

### 9.5.2 战斗道具系统扩展 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.5.2a | `src/game/mod.rs` | 10 | `ItemType` 枚举追加 Elixir/Antidote/Nut |
| 9.5.2b | `src/game/mod.rs` | 30 | `Item::use_on_player()` 实现各类道具效果（HP/PP 恢复/永久提升） |
| 9.5.2c | `src/game/mod.rs` | 10 | 游戏内初始道具或 Bilibin 商店包含新道具 |

### 9.5.3 战斗内召唤 UI 集成 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.5.3a | `src/game/draw.rs` | 25 | 战斗 Summon 子菜单绘制（元素颜色/可用性/PP消耗） |
| 9.5.3b | `src/game/update.rs` | 15 | Summon 选择处理、BattleAction::Summon 路由 |
| 9.5.3c | `src/battle/state.rs` | 20 | 确认/实现 `collect_standby_djinn_count()` 和 `consume_standby_djinn()` 方法 |

### 9.5.4 战斗加速功能确认 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.5.4 | `src/game/update.rs` | 10 | Battle update 中检测 B 键 → 设置 `turbo` 加速动画 |

### 9.5.4 检查点：Bilibin 商店商品清单同步 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.5.4x | `src/game/mod.rs` | 5 | Bilibin 商店含新装备索引（破甲剑/战斗斧/铁甲/力量腰带/治愈戒指） |

| **合计** | | **~180 行** | |

---

## Phase 9.6：Game Over 与标题增强（P2，~100 行）⬜

### 9.6.1 Game Over 状态 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.6.1a | `src/engine/game_state.rs` | 5 | `GameState` 追加 `GameOver { timer, retry }` 变体 |
| 9.6.1b | `src/game/update.rs` | 15 | Battle 全灭检测 → 切换到 GameOver 状态 |
| 9.6.1c | `src/game/update.rs` | 25 | GameOver 更新逻辑：2秒后可 Confirm 读档或 Cancel 回标题 |
| 9.6.1d | `src/game/draw.rs` | 20 | GameOver 绘制（暗红背景 + GAME OVER 大字 + 操作提示） |
| 9.6.1e | `src/game/mod.rs` | 5 | GameState match 补充 GameOver 分支 |

### 9.6.2 标题画面增强 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.6.2a | `src/game/draw.rs` | 20 | `draw_title_screen()` 增强：渐变背景+金色标题+闪烁提示+存档检测 |
| 9.6.2b | `src/game/draw.rs` | 10 | 底部版本信息显示 |

| **合计** | | **~100 行** | |

---

## Phase 9.7：音频与视觉增强（P2，~120 行）⬜

### 9.7.1 新增 BGM ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.7.1a | `src/audio/mod.rs` | 50 | `generate_bilibin_bgm()` — E 大调悠闲城镇 BGM |
| 9.7.1b | `src/audio/mod.rs` | 40 | `generate_forest_bgm()` — C 小调神秘森林 BGM |
| 9.7.1c | `src/audio/mod.rs` | 5 | BgmPlayer 注册新 BGM："bilibin" 和 "forest" |
| 9.7.1d | `src/game/mod.rs` | 10 | `apply_scene_switch()` 中 BGM 自动切换逻辑 |

### 9.7.2 新增 SFX ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.7.2a | `src/audio/mod.rs` | 8 | `game_over` SFX — 440Hz→110Hz 下滑音 |
| 9.7.2b | `src/audio/mod.rs` | 8 | `opening` SFX — 220Hz→880Hz 上升音 |

### 9.7.3 更多天气粒子效果 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.7.3a | `src/engine/particle.rs` | 10 | `ParticleKind` 追加 Sparkle（精灵光点）、Leaf（落叶） |
| 9.7.3b | `src/engine/particle.rs` | 15 | Leaf 粒子生成函数（棕色椭圆、横向飘落） |
| 9.7.3c | `src/game/draw.rs` | 8 | KolimaForest 场景自动触发 Leaf 粒子系统 |
| 9.7.3d | `src/game/draw.rs` | 5 | Djinn 附近自动触发 Sparkle 光点 |

| **合计** | | **~120 行** | |

---

## Phase 9.8：测试与验证（P2，~100 行）⬜

### 9.8.1 对话脚本测试 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.8.1a | `tests/dialogue_bdd.rs` | 15 | Bilibin 4 NPC 脚本存在性+页数测试 |
| 9.8.1b | `tests/dialogue_bdd.rs` | 10 | Kolima 2 NPC 脚本存在性测试 |
| 9.8.1c | `tests/dialogue_bdd.rs` | 15 | Vale 4 新增 NPC 脚本存在性测试 |
| 9.8.1d | `tests/dialogue_bdd.rs` | 10 | forest_traveler+cave_sage 脚本存在性测试 |

### 9.8.2 场景测试 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.8.2a | `src/scene/mod.rs` | 15 | `#[cfg(test)]` — 新场景地图宽度/高度/非空测试 |
| 9.8.2b | `src/entity/mod.rs` | 10 | `#[cfg(test)]` — 新场景 NPC 数量测试 |

### 9.8.3 任务链测试 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.8.3 | `src/data/quest.rs` | 10 | `#[cfg(test)]` — 15 任务+ID 唯一性测试 |

### 9.8.4 Game Over 测试 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.8.4 | `tests/core.rs` | 10 | GameOver 变体存在性测试（编译检查） |

### 9.8.5 装备测试 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.8.5 | `src/game/mod.rs` | 15 | `#[cfg(test)]` — 16 件装备、价格正数、属性检查 |

### 9.8.6 敌人配置测试 ⬜

| 子任务 | 文件 | 行数 | 说明 |
|--------|------|------|------|
| 9.8.6 | `src/data/mod.rs` | 10 | `#[cfg(test)]` — Bilibin/KolimaForest 有敌人配置 |

| **合计** | | **~100 行** | |

---

## Phase 9.9：最终验证（必做）⬜

### 执行顺序

1. 按 Phase 9.1 → 9.2 → 9.3 → 9.4 → 9.5 → 9.6 → 9.7 → 9.8 顺序逐一实现
2. 每个子 Phase 实现后 **不要单独编译**，全部改完后统一编译
3. 所有 `match` 表达式必须覆盖新变体（编译器会提示，逐一补全即可）

### 验证清单

```bash
# 1. 编译检查（必须零错误）
cargo check 2>&1

# 2. Clippy 检查（必须零警告）
cargo clippy --all-targets -- -D warnings 2>&1

# 3. 测试（必须全绿）
cargo test 2>&1
# 预期：260+ 测试全部通过

# 4. 手动运行验证
cargo run 2>&1

# 手动验证项：
# - [ ] 开场序幕动画正常显示
# - [ ] 开场后自动进入 Vale，Garsmin 有起始对话
# - [ ] 任务链按对话/事件正确推进
# - [ ] Vale 新增 4 个 NPC 的对话可触发
# - [ ] Bilibin 镇可进入（WildForest 左侧出口）
# - [ ] Bilibin 4 个 NPC 对话正常
# - [ ] Bilibin 商店可访问
# - [ ] Bilibin 有敌人（Rat/Spider）
# - [ ] KolimaForest 可进入（Bilibin 下方出口）
# - [ ] KolimaForest 2 个 NPC 对话正常
# - [ ] KolimaForest 有敌人（Wolf/Treant/Slime/Mandrake/Moth）
# - [ ] forest_traveler 和 cave_sage 对话可触发
# - [ ] 新装备在商店中可购买
# - [ ] 道具（Elixir/Nut）可在战斗外使用
# - [ ] 战斗内召唤选项可用
# - [ ] 战斗全灭后显示 Game Over 画面
# - [ ] Game Over 后可读档/回标题
# - [ ] 标题画面增强（渐变色/闪烁提示）
# - [ ] Bilibin/KolimaForest BGM 自动切换
# - [ ] 新 Djinn 在地图上可收集（Bilibin 2 + Kolima 2）
# - [ ] 新敌人战斗正常
# - [ ] 15 个任务全部可查看

# 5. 测试全部通过
cargo test --test '*' 2>&1

# 6. 提交
git add -A
git commit -m "Phase 9: 深度内容扩充 — 开场/Bilibin/Kolima/GameOver/12NPC/15个任务/16件装备/16Djinn"
```

---

## 统计概览

| Phase | 内容 | 子任务数 | 文件数 | 预估行数 | 优先级 |
|-------|------|---------|--------|---------|--------|
| **9.1** | 开场与故事完整化 | 10 | 5 | ~350 | P0 |
| **9.2** | 场景深度扩展 | 12 | 6 | ~550 | P1 |
| **9.3** | Bilibin 故事内容 | 7 | 3 | ~250 | P1 |
| **9.4** | Vale NPC 扩充 | 6 | 2 | ~150 | P2 |
| **9.5** | 装备与战斗扩展 | 7 | 4 | ~180 | P1 |
| **9.6** | Game Over 与标题 | 7 | 4 | ~100 | P2 |
| **9.7** | 音频与视觉增强 | 7 | 3 | ~120 | P2 |
| **9.8** | 测试验证 | 9 | 5 | ~100 | P2 |
| **9.9** | 最终验证 | 6 项 | — | — | **必做** |
| **总计** | | **65** | **15+** | **~1,800** | |

## 关键交付物

| 类别 | Phase 8 完成时 | Phase 9 完成后 |
|------|---------------|---------------|
| 场景 (SceneId) | 5 | **7** (+Bilibin, +KolimaForest) |
| 地图 | 5 | **7** |
| NPC (所有场景) | 8 | **14** (Vale +4, Bilibin +4, Kolima +2) |
| 有脚本 NPC | 6 (2 个无脚本) | **14** (全部有脚本) |
| 敌人种类 | 11 | **17** (+Rat, +Mandrake, +Moth, +Mandrake, +Moth) |
| Djinn (地图可收集) | 8 / 16 | **12 / 16** |
| 任务 | 10 (3 章) | **15 (4 章)** |
| 装备 | 11 | **16** |
| BGM | 3 | **5** (+bilibin, +forest) |
| SFX | 10 | **12** (+game_over, +opening) |
| 粒子种类 | 2 (雨/雪) | **4** (+Sparkle, +Leaf) |
| Game Over | 无 | **有** |
| 开场动画 | 无 | **有** |
| 召唤系统 | 数据存在但 UI 未接入 | **UI 完全集成** |
