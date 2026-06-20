# Phase 8s: 黄金太阳故事内容深度扩充（纯内容版）

> 目标：在不修改引擎核心的前提下，通过对话脚本、任务数据、地图布局、NPC 配置和音频资源，
> 大幅丰富游戏的**叙事深度**和**内容密度**，使其接近 GBA 原版《黄金太阳》
> 执行方式：单 Agent 一次性完成所有任务
> 预估修改：~600 行，修改 10+ 个数据/内容文件（不碰引擎）

---

## 项目现状（仅供 Agent 上下文理解）

**不修改**的引擎文件：
- `src/engine/` 全部 — 不新增 GameState 变体，不修改核心循环
- `src/game/mod.rs` — 不新增 GameCtx 字段
- `src/battle/` — 不修改战斗核心逻辑
- `src/map/mod.rs` — 不新增 TileKind

**只修改**的内容文件：
- `src/dialogue/script.rs` — NPC 对话脚本
- `src/data/quest.rs` — 任务链
- `src/data/djinn.rs` — Djinn 地图位置
- `src/data/mod.rs` — 敌人配置
- `src/dialogue/mod.rs` — DialogueAction（仅新增变体）
- `src/entity/mod.rs` — NPC 位置/新 NPC
- `src/audio/mod.rs` — 新增音效
- `src/game/update.rs` — 故事推进逻辑（仅新增函数，不改变现有分支）
- `src/game/draw.rs` — 故事相关 UI 元素（仅新增函数）

---

## 共享类型引用

```rust
use golden_sun::scene::SceneId;
use golden_sun::entity::{Entity, create_npcs_for_scene};
use golden_sun::dialogue::script::{DialogueScript, DialoguePage, DialogueLine, DialogueChoice, NPC_SCRIPTS};
use golden_sun::dialogue::{DialogueAction, StoryFlags};
use golden_sun::data::quest::{QuestLog, QuestEntry, QuestStatus};
use golden_sun::data::djinn::{self, DjinnId, world_djinn};
use golden_sun::data::{EnemyConfig, enemies_for_area};
use golden_sun::Element;
use crate::game::GameCtx;
```

---

## Phase 8s.1：故事对话大扩充（P0，~200 行）

### 8s.1.1 核心 NPC 深度对话

**文件**: `src/dialogue/script.rs`

目标：让每个 NPC 的对话有 6-8 页，覆盖故事全程。在 NPC_SCRIPTS 数组中只更新已有 NPC，不新增。

#### Ivan 扩充至 6 页（追加在现有 4 页之后）

```
Page 5: require "completed_sol_sanctum" && "garet_ready"
  "你们要去 Bilibin？好选择。
   我年轻时去过一次，是个热闹的镇子。
   记得在路上的旅馆休息，山道很危险。"
  → SetFlag("ivan_travel_tip")

Page 6: require "ivan_travel_tip" && "opening_seen"
  "传说中，四个 Elemental Stars 分别藏在世界的四个角落。
   如果你真的找到了一个，那么剩下的三个也在呼唤你。
   世界之轮已经开始转动了…"
  → SetFlag("ivan_revealed_stars")
```

#### Mia 扩充至 6 页（追加在现有 4 页之后）

```
Page 5: require "completed_sol_sanctum"
  "池塘里的闪光…现在我知道那是精灵的力量了。
   也许我也能感受到它。
   保重，Isaac。"
  → SetFlag("mia_farewell")

Page 6: require "mia_farewell" && "collected_any_djinn"
  "你收集到 Djinn 了！
   它们是很特别的精灵，只选择有缘人。
   看来你是真正的 Adept。"
  → SetFlag("mia_djinn_talk")
```

#### Garsmin 扩充至 10 页（追加在现有 7 页之后）

```
Page 8: require "completed_sol_sanctum"
  "Elemental Star…你做到了。
   在圣祭坛沉睡了数百年的力量终于重现于世。
   你知道吗？传说中一共有四颗 Elemental Stars。
   每一颗都代表着一种元素力量。"
  → SetFlag("garsmin_revealed_stars")

Page 9: require "garsmin_revealed_stars"
  "数百年前，伟大的贤者们将炼金术封印在
   四颗 Elemental Stars 之中。
   你的祖先就是其中一位守护者。
   现在，这责任落在了你肩上。"
  → SetFlag("garsmin_ancestor")

Page 10: require "garsmin_ancestor"
  "离开 Vale 吧，孩子。
   世界在等待着你。
   记住——精灵的力量来自于内心，
   而不是来自于 Elemental Stars。"
  → SetFlag("garsmin_final_blessing")
```

#### Garet 扩充至 5 页（追加在现有 3 页之后）

```
Page 4: require "garsmin_final_blessing"
  "长老说得对！我们该出发了。
   我已经准备好了，你呢？
   Isaac，一起闯荡世界吧！"
  → SetFlag("garet_ready_to_go")

Page 5: require "garet_ready_to_go" && "left_vale"
  "哇，外面的空气真不一样！
   你看那边的森林，比 Vale 周围的茂密多了。
   我感觉我们很快就会遇到有趣的事！"
  → SetFlag("garet_excited")
```

### 8s.1.2 新增场景 NPC

**文件**: `src/entity/mod.rs`

在 `create_npcs_for_scene()` 中为 WildForest 和 Cave 增加 NPC：

```rust
pub fn create_npcs_for_scene(scene: SceneId) -> Vec<Entity> {
    match scene {
        SceneId::Vale => create_vale_npcs(),
        SceneId::WildForest => vec![
            Entity::new(10, 7.0, 4.0, "forest_traveler"),   // 旅行者（移至NPC_SCRIPTS中定义对话）
            Entity::new(11, 14.0, 8.0, "forest_hermit"),    // 隐士（新）
        ],
        SceneId::Cave => vec![
            Entity::new(20, 12.0, 4.0, "cave_sage"),        // 洞穴贤者（对话已在Phase 7定义）
            Entity::new(21, 5.0, 10.0, "cave_prospector"),  // 探矿者（新）
        ],
        SceneId::SolSanctum => vec![],  // 封闭场景，无NPC
    }
}
```

**在 NPC_SCRIPTS 中定义新NPC对话**：

```rust
// ── 场景 NPC — WildForest ──
("forest_hermit", &DialogueScript {
    pages: &[
        DialoguePage { lines: &[DialogueLine {
            text: "嘘…小点声，我在冥想。
森林里住着许多古老的生灵，
它们不喜欢被打扰。",
            actions: &[DialogueAction::SetFlag("met_hermit")],
        }], choices: &[] },
        DialoguePage { lines: &[DialogueLine {
            text: "你身上有精灵力的气息…
你是 Adept 吧？难怪森林对你这么友好。
向东走有个洞穴，那里的能量很不稳定。",
            actions: &[DialogueAction::SetFlag("hermit_hint")],
        }], choices: &[] },
    ],
    start_flag: Some("talked_to_hermit"),
})

// ── 场景 NPC — Cave ──
("cave_prospector", &DialogueScript {
    pages: &[
        DialoguePage { lines: &[DialogueLine {
            text: "嘿！你也是来挖宝的？
别误会，这洞里没什么值钱的东西…
不过据说深处有奇怪的光芒。",
            actions: &[DialogueAction::SetFlag("met_prospector")],
        }], choices: &[] },
        DialoguePage { lines: &[DialogueLine {
            text: "你要往深处走？那小心点。
我听到过沉重的脚步声，像是有什么大家伙在睡觉。
……也许你该带上些恢复药。",
            actions: &[DialogueAction::SetFlag("prospector_warning")],
        }], choices: &[] },
    ],
    start_flag: Some("talked_to_prospector"),
})
```

---

## Phase 8s.2：任务链连贯化（P0，~80 行）

### 8s.2.1 替换任务定义

**文件**: `src/data/quest.rs`

将 `default_quests()` 替换为按章节组织的完整任务链。同时为 QuestEntry 增加 `chapter` 字段（在 QuestEntry 结构体中增加 `pub chapter: u32`）：

```rust
pub fn default_quests() -> Vec<QuestEntry> {
    vec![
        // ── 第一章：Vale 村 — 觉醒 ──
        QuestEntry { id: "talk_to_villagers", name: "初访村民",
            hint: "和 Vale 村的 Ivan、Mia、Garsmin 聊聊", chapter: 1, completed: false },
        QuestEntry { id: "learn_psynergy", name: "觉醒的精灵力",
            hint: "使用一次精灵力，感受体内的力量", chapter: 1, completed: false },
        QuestEntry { id: "meet_garet", name: "与 Garet 会合",
            hint: "去村口找 Garet，一起探索 Sol Sanctum", chapter: 1, completed: false },
        QuestEntry { id: "explore_sanctum", name: "圣祭坛之谜",
            hint: "进入 Vale 后山的 Sol Sanctum", chapter: 1, completed: false },
        QuestEntry { id: "defeat_mythrilgolem", name: "古石像",
            hint: "击败 Sol Sanctum 深处的 MythrilGolem 守护者", chapter: 1, completed: false },

        // ── 第二章：踏上旅程 ──
        QuestEntry { id: "leave_vale", name: "告别 Vale",
            hint: "带着 Elemental Star 离开 Vale", chapter: 2, completed: false },
        QuestEntry { id: "collect_first_djinn", name: "初遇 Djinn",
            hint: "在野外找到并收集第一个 Djinn", chapter: 2, completed: false },

        // ── 第三章：力量之路 ──
        QuestEntry { id: "collect_three_djinn", name: "精灵使之道",
            hint: "收集 3 个 Djinn，感受精灵的力量", chapter: 3, completed: false },
        QuestEntry { id: "master_psynergy", name: "精灵力大师",
            hint: "解锁全部 7 种精灵力", chapter: 3, completed: false },
        QuestEntry { id: "become_adept", name: "真正的 Adept",
            hint: "收集 5 个 Djinn 并激活高级职业", chapter: 3, completed: false },
    ]
}
```

### 8s.2.2 任务推进逻辑

**文件**: `src/game/mod.rs` 或 `src/game/update.rs`

在 GameCtx 中新增函数 `auto_update_quests()`，在 `update_world_map()` 末尾调用：

```rust
/// 根据故事标志自动推进任务进度
pub fn auto_update_quests(&mut self) {
    // ★ 第一章：Vale 篇
    // 与三位村民对话
    if self.story_flags.get("met_ivan") && self.story_flags.get("met_mia")
        && self.story_flags.get("met_garsmin")
        && self.quest_log.has("talk_to_villagers") && !self.quest_log.is_completed("talk_to_villagers")
    {
        self.quest_log.complete("talk_to_villagers");
    }

    // 使用精灵力
    if self.unlocked_count > 0 && self.quest_log.has("learn_psynergy")
        && !self.quest_log.is_completed("learn_psynergy")
    {
        self.quest_log.complete("learn_psynergy");
        self.quest_log.unlock("meet_garet");
    }

    // 与 Garet 对话
    if self.story_flags.get("met_garet") && self.story_flags.get("garsmin_sent_to_sanctum")
        && self.quest_log.has("meet_garet") && !self.quest_log.is_completed("meet_garet")
    {
        self.quest_log.complete("meet_garet");
        self.quest_log.unlock("explore_sanctum");
    }

    // 进入 SolSanctum
    if self.scene.current() == SceneId::SolSanctum
        && self.quest_log.has("explore_sanctum") && !self.quest_log.is_completed("explore_sanctum")
    {
        self.quest_log.complete("explore_sanctum");
        self.quest_log.unlock("defeat_mythrilgolem");
    }

    // Boss 战后
    if self.story_flags.get("completed_sol_sanctum")
        && self.quest_log.has("defeat_mythrilgolem") && !self.quest_log.is_completed("defeat_mythrilgolem")
    {
        self.quest_log.complete("defeat_mythrilgolem");
        self.quest_log.unlock("leave_vale");
    }

    // 离开 Vale
    if self.scene.current() != SceneId::Vale
        && self.quest_log.has("leave_vale") && !self.quest_log.is_completed("leave_vale")
    {
        self.quest_log.complete("leave_vale");
        self.quest_log.unlock("collect_first_djinn");
    }

    // ★ 第二章：收集篇
    if self.collected_djinn.len() >= 1
        && self.quest_log.has("collect_first_djinn") && !self.quest_log.is_completed("collect_first_djinn")
    {
        self.quest_log.complete("collect_first_djinn");
        self.quest_log.unlock("collect_three_djinn");
    }

    // ★ 第三章：力量篇
    if self.collected_djinn.len() >= 3
        && self.quest_log.has("collect_three_djinn") && !self.quest_log.is_completed("collect_three_djinn")
    {
        self.quest_log.complete("collect_three_djinn");
        self.quest_log.unlock("become_adept");
    }

    if self.unlocked_count >= 7
        && self.quest_log.has("master_psynergy") && !self.quest_log.is_completed("master_psynergy")
    {
        self.quest_log.complete("master_psynergy");
    }

    if self.collected_djinn.len() >= 5
        && self.quest_log.has("become_adept") && !self.quest_log.is_completed("become_adept")
    {
        self.quest_log.complete("become_adept");
    }
}
```

### 8s.2.3 QuestLog 新增辅助方法

**文件**: `src/data/quest.rs`

```rust
impl QuestLog {
    /// 检查某任务是否存在
    pub fn has(&self, id: &str) -> bool {
        self.entries.iter().any(|e| e.id == id)
    }

    /// 检查某任务是否已完成
    pub fn is_completed(&self, id: &str) -> bool {
        self.entries.iter().find(|e| e.id == id).map_or(false, |e| e.completed)
    }

    /// 解锁新任务（若不存在则添加）
    pub fn unlock(&mut self, id: &str) {
        if let Some(entry) = QUEST_TEMPLATES.iter().find(|e| e.id == id) {
            self.add(QuestEntry { id: entry.id, name: entry.name, hint: entry.hint, chapter: entry.chapter, completed: false });
        }
    }
}

/// 任务模板（用于 unlock 时快速查找）
pub const QUEST_TEMPLATES: &[QuestEntry] = &[
    QuestEntry { id: "talk_to_villagers", name: "初访村民", hint: "和 Vale 村的 Ivan、Mia、Garsmin 聊聊", chapter: 1, completed: false },
    QuestEntry { id: "learn_psynergy", name: "觉醒的精灵力", hint: "使用一次精灵力，感受体内的力量", chapter: 1, completed: false },
    QuestEntry { id: "meet_garet", name: "与 Garet 会合", hint: "去村口找 Garet", chapter: 1, completed: false },
    QuestEntry { id: "explore_sanctum", name: "圣祭坛之谜", hint: "进入 Vale 后山的 Sol Sanctum", chapter: 1, completed: false },
    QuestEntry { id: "defeat_mythrilgolem", name: "古石像", hint: "击败 Sol Sanctum 深处的 MythrilGolem", chapter: 1, completed: false },
    QuestEntry { id: "leave_vale", name: "告别 Vale", hint: "带着 Elemental Star 离开 Vale", chapter: 2, completed: false },
    QuestEntry { id: "collect_first_djinn", name: "初遇 Djinn", hint: "在野外找到并收集第一个 Djinn", chapter: 2, completed: false },
    QuestEntry { id: "collect_three_djinn", name: "精灵使之道", hint: "收集 3 个 Djinn", chapter: 3, completed: false },
    QuestEntry { id: "master_psynergy", name: "精灵力大师", hint: "解锁全部 7 种精灵力", chapter: 3, completed: false },
    QuestEntry { id: "become_adept", name: "真正的 Adept", hint: "收集 5 个 Djinn", chapter: 3, completed: false },
];
```

---

## Phase 8s.3：Djinn 收集扩展 + 地图布局（P1，~100 行）

### 8s.3.1 扩大 Djinn 可收集区域

**文件**: `src/data/djinn.rs`

修改 `world_djinn()`，将可收集 Djinn 从 3 个扩展到 8 个：

```rust
pub fn world_djinn() -> &'static [(DjinnId, &'static str, f32, f32)] {
    &[
        (DjinnId::Fafnir,  "Vale",        8.0,  11.0),   // Vale 村左池塘边
        (DjinnId::Belian,  "Vale",        24.0, 20.0),   // Vale 村右上角
        (DjinnId::Gnome,   "WildForest",  5.0,  5.0),    // 密林入口附近
        (DjinnId::Vermin,  "WildForest",  15.0, 10.0),   // 密林深处
        (DjinnId::Malina,  "Cave",        10.0, 12.0),   // 洞穴中心
        (DjinnId::Betsy,   "Cave",        3.0,  3.0),    // 洞穴入口旁
        (DjinnId::Laguna,  "SolSanctum",  4.0,  3.0),    // 圣殿左上
        (DjinnId::Undine,  "SolSanctum",  12.0, 10.0),   // 圣殿右下
    ]
}
```

### 8s.3.2 Djinn 收集消息提示

在 `src/game/update.rs` 的 `check_djinn_pickup()` 中，收集成功后发送系统消息。利用现有的 `DialogueState` 弹出提示：

```rust
// 在 collect_djinn() 成功返回 true 后：
if self.collect_djinn(djinn_id) {
    // 弹出 Djinn 收集消息
    let name = djinn_id.as_str();
    let msg = format!("获得了 Djinn：{}！\n元素：{} 属性加成：攻击+{} 防御+{} HP+{}",
        name,
        djinn_id.element().as_str(),
        djinn.atk_bonus, djinn.def_bonus, djinn.hp_bonus);
    self.dialogue = Some(DialogueState::new(msg));
    self.state = GameState::Dialog;
    self.play_sfx("confirm");
}
```

### 8s.3.3 Djinn 地图光点显示

**文件**: `src/game/draw.rs`

在 `draw_world_map()` 中的 NPC 渲染之后、HUD 绘制之前新增 Djinn 光点绘制：

```rust
fn draw_djinn_hints(&self) {
    let px = self.camera.x;
    let py = self.camera.y;
    let current = self.scene.current();

    for &(djinn_id, scene_name, tx, ty) in djinn::world_djinn() {
        // 只绘制当前场景且未被收集的 Djinn
        if scene_name != match current {
            SceneId::Vale => "Vale",
            SceneId::WildForest => "WildForest",
            SceneId::Cave => "Cave",
            SceneId::SolSanctum => "SolSanctum",
            _ => "",
        } { continue; }
        if self.collected_djinn.iter().any(|d| d.djinn.id == djinn_id) { continue; }

        let dist_sq = (px - tx).powi(2) + (py - ty).powi(2);
        if dist_sq < 9.0 {  // 3 tiles 内可见
            let blink = (self.game_time * 3.0).sin().abs();
            let (r, g, b) = match djinn_id.element() {
                Element::Venus => (0.2, 1.0, 0.2),
                Element::Mercury => (0.2, 0.5, 1.0),
                Element::Mars => (1.0, 0.3, 0.2),
                Element::Jupiter => (0.7, 0.2, 1.0),
            };
            // 在 Mode 7 地面上方绘制光点（在 draw_world_map 的 Mode7 渲染之后）
            let screen_x = RENDER_TARGET_W as f32 / 2.0;
            let screen_y = RENDER_TARGET_H as f32 * 0.5 - 50.0 - blink * 8.0;
            draw_circle(screen_x, screen_y, 3.0, Color::new(r, g, b, blink * 0.8));
        }
    }
}
```

---

## Phase 8s.4：敌人/战斗内容扩展（P1，~80 行）

### 8s.4.1 敌人配置扩展

**文件**: `src/data/mod.rs`

为敌人增加中文名和描述：

```rust
pub struct EnemyConfig {
    pub name: &'static str,
    pub level: u32,
    pub display_name: &'static str,  // 战斗中显示的中文名
    pub description: &'static str,    // 战斗中显示的简介
}

pub fn enemies_for_area(area: &str) -> Vec<EnemyConfig> {
    match area {
        "Vale" => vec![
            EnemyConfig { name: "Wolf", level: 3, display_name: "野狼", description: "山脚常见的野兽" },
            EnemyConfig { name: "Bat", level: 2, display_name: "蝙蝠", description: "洞穴中成群出没" },
            EnemyConfig { name: "Goblin", level: 4, display_name: "哥布林", description: "喜欢恶作剧的小妖怪" },
        ],
        "WildForest" => vec![
            EnemyConfig { name: "Wolf", level: 4, display_name: "野狼", description: "" },
            EnemyConfig { name: "Spider", level: 3, display_name: "毒蜘蛛", description: "森林深处的危险生物" },
            EnemyConfig { name: "Goblin", level: 5, display_name: "哥布林", description: "" },
            EnemyConfig { name: "Treant", level: 6, display_name: "树精", description: "古老森林的守护者" },
            EnemyConfig { name: "Slime", level: 2, display_name: "史莱姆", description: "元素凝聚成的软体生物" },
        ],
        "Cave" => vec![
            EnemyConfig { name: "Bat", level: 3, display_name: "蝙蝠", description: "" },
            EnemyConfig { name: "Golem", level: 7, display_name: "石巨人", description: "洞穴深处的岩石守卫" },
            EnemyConfig { name: "Spider", level: 5, display_name: "毒蜘蛛", description: "" },
            EnemyConfig { name: "Ghost", level: 6, display_name: "幽灵", description: "游荡在黑暗中的灵体" },
            EnemyConfig { name: "RatKing", level: 8, display_name: "鼠王", description: "洞穴鼠群的统帅" },
        ],
        "SolSanctum" => vec![
            EnemyConfig { name: "MythrilGolem", level: 10, display_name: "密银巨像", description: "圣祭坛的远古守卫" },
            EnemyConfig { name: "AncientGuard", level: 9, display_name: "古代守卫", description: "迷途者的灵魂被赋予了形体" },
        ],
        _ => vec![
            EnemyConfig { name: "Wolf", level: 3, display_name: "野狼", description: "" },
            EnemyConfig { name: "Bat", level: 2, display_name: "蝙蝠", description: "" },
        ],
    }
}
```

### 8s.4.2 战斗中使用道具恢复

在 battle 中，新增一个 `UseItem` 类型的中间状态让玩家能在战斗中喝药。在 `src/battle/state.rs` 的 `BattleAction` 中：

```rust
pub enum BattleAction {
    // ... 现有变体 ...
    UseItem(ItemType, usize),  // (道具类型, 目标索引)
    // ...
}
```

在 `execute_turn()` 中处理：

```rust
BattleAction::UseItem(item_type, target_idx) => {
    let target = &mut battle_state.party[target_idx];
    match item_type {
        ItemType::Potion => {
            let heal = 30u32;
            let actual = (target.hp + heal).min(target.max_hp) - target.hp;
            target.hp = (target.hp + heal).min(target.max_hp);
            battle_state.logs.push(format!("{} 使用了药水，恢复了 {} HP！", target.name, actual));
        }
        ItemType::Ether => {
            let recover = 10u32;
            let actual = (target.pp + recover).min(target.max_pp) - target.pp;
            target.pp = (target.pp + recover).min(target.max_pp);
            battle_state.logs.push(format!("{} 使用了以太，恢复了 {} PP！", target.name, actual));
        }
        _ => {}
    }
}
```

---

## Phase 8s.5：最终验证（必做）

### 执行顺序

1. **按 8s.1 → 8s.2 → 8s.3 → 8s.4 顺序实现**
2. 每个步骤间建议 `cargo check` 验证

### 验证清单

```bash
cargo check 2>&1
cargo clippy --all-targets -- -D warnings 2>&1
cargo test 2>&1
cargo run 2>&1
```

运行后检查：
- [ ] Ivan 对话有 6 页，涵盖故事各个阶段
- [ ] Mia 对话有 6 页，合理过渡情绪
- [ ] Garsmin 有 10 页，完整讲述背景故事
- [ ] Garet 有 5 页，表现出发的兴奋
- [ ] WildForest 有 2 个 NPC（traveler + hermit）
- [ ] Cave 有 2 个 NPC（sage + prospector）
- [ ] 任务链按章节连贯推进
- [ ] 8 个 Djinn 在地图上各有位置
- [ ] Djinn 附近 3 tiles 内可见光点闪烁
- [ ] 收集 Djinn 弹出对话框提示
- [ ] 敌人有中文名显示
- [ ] 战斗中可使用 Potion/Ether 恢复
- [ ] 存档/读档后故事 flag 和任务进度保留

### 提交

```bash
git add -A
git commit -m "Phase 8s: 故事内容深度扩充 — 对话丰富/任务链连贯/Djinn收集扩展/敌人本地化"
git push
```
