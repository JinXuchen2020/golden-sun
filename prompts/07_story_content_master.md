# Phase 7: 黄金太阳完整内容扩充 — 故事与玩法全面丰富

> 目标：基于现有 Phase 0-6 完整引擎框架，让游戏在**故事剧情**和**玩法深度**上大幅逼近 GBA 原版《黄金太阳》
> 执行方式：单 Agent 一次性完成所有任务，按顺序执行各 Phase

---

## 项目现状（仅供 Agent 上下文理解）

**已有架构**：40 个源文件，5300+ 行 Rust，176+ 测试零 Clippy 警告，desktop/wasm32 双端。

**已有功能**：
- Mode 7 伪 3D 渲染（3 张地图：Vale 32×32, WildForest 20×20, Cave 16×16）
- 23 种 TileKind（含交互型：Vine/Seed/PushBlock/DarkArea/HiddenChest/Waypoint 等）
- 7 种精灵力（Whirlwind/Growth/Freeze/Force/Catch/Flash/Reveal），含施法动画特效
- 回合制战斗系统（攻击/防御/精灵力/逃跑/Djinn 释放召回），含元素克制(4×4)、伤害数字弹出
- 对话系统（打字机效果 + 分支选择 + DialogueAction 副作用机制）
- Djinn 精灵系统（16 个 Djinn，4 元素各 4 个，含装备/释放/召回/职业切换）
- 任务日志系统（QuestLog，含 4 个默认任务）
- BGM 合成器（vale/battle 两首）+ SFX 音效（方波合成）
- 存档系统（bincode 序列化 + StorageBackend trait）
- 传送系统（Waypoint tile → Travel 菜单）
- 天气粒子系统 + CRT 扫描线滤镜 + 小地图 + HUD
- 输入录制/回放（debug 模式 R/P 键）
- 3 个对话 NPC（Ivan/Mia/Garsmin），仅 Garsmin 有分支

**最大缺失**：**没有真正的主线剧情**。当前只有 Vale 村基础对话，没有原版黄金太阳的任何故事元素（Sol Sanctum、炼金术封印、主角出发动机等）。

---

## 共享类型引用

```rust
use golden_sun::engine::{
    GameState, InputState, FrameTime, GameError, GameResult,
    TransitionKind, PsynergyAnim, Mode7Camera,
    Camera, TextureCache,
};
use golden_sun::engine::input::{InputBus, InputEvent};
use golden_sun::engine::constants::*;
use golden_sun::engine::resources::ResourceManager;
use golden_sun::engine::particle::ParticleSystem;
use golden_sun::engine::storage::{StorageBackend, FsStorage, create_storage};
use golden_sun::map::{TileKind, SceneMap, get_scene_map, tile_center, TileData};
use golden_sun::entity::{Entity, create_npcs_for_scene, create_vale_npcs};
use golden_sun::entity::sprite::AnimState;
use golden_sun::psynergy::{PsynergyType, Element, apply_psynergy};
use golden_sun::dialogue::{DialogueState, StoryFlags, DialogueAction};
use golden_sun::dialogue::script::{DialogueScript, DialoguePage, DialogueLine, DialogueChoice, get_script, NPC_SCRIPTS};
use golden_sun::battle::{Battle, Combatant, BattlePhase, AttackResult, DamagePopup, BattleAction};
use golden_sun::battle::calculator::{calculate_physical_damage, calculate_psynergy_damage};
use golden_sun::data::{SaveData, EnemyConfig, enemies_for_area};
use golden_sun::data::quest::{QuestLog, QuestEntry};
use golden_sun::data::djinn::{self, DjinnId, OwnedDjinn, SetBonus, Class, world_djinn, all_djinn_data};
use golden_sun::data::loader::{load_map_data, load_npc_data};
use golden_sun::scene::{SceneId, SceneRegistry};
use golden_sun::ui::{draw_hud, draw_pause_menu, draw_title_screen, draw_status_screen, draw_transition};
use golden_sun::audio::{SfxManager, BgmPlayer};
use crate::game::{GameCtx, Item, ItemType, PlayerStats, SpriteAtlas, WaypointDef};
```

---

## Phase 7.1：主线剧情框架 — Golden Sun 的故事（P0，~400 行）

### 核心叙事目标
将黄金太阳的经典开篇融入游戏，建立完整的故事驱动探索机制。剧情分三个章节，每个章节有独立 NPC 对话和地图事件。

### Chapter 1：Sol Sanctum（圣祭坛）— 序幕

#### 7.1.1 新增对话脚本文件
**文件**: `src/dialogue/script.rs` — 扩展 NPC_SCRIPTS

在现有 `NPC_SCRIPTS` 数组末尾追加以下脚本，保持已有脚本不变：

##### Vale 村民 — 丰富背景对话（每个 NPC 至少 3 页）

**Ivan（铁匠）— 4 页**（追加在已有 1 页之后）

```
Page 1 (已有): "你好！我是伊万…如果需要修理装备…" → SetFlag("met_ivan")
Page 2 (新增): require "met_ivan" 且 "talked_to_garsmin"
  "听说长老告诉了你关于精灵的事？
   确实，Vale 村自古就流传着这样的传说。
   山上的 Sol Sanctum 里封印着远古的力量…"
  → SetFlag("ivan_revealed")

Page 3 (新增): require "ivan_revealed"
  "最近山上的怪物越来越多了。
   据说是因为圣祭坛的封印在减弱…
   年轻人，你有责任去看看。"

Page 4 (新增): require flag "completed_sol_sanctum"
  "你从圣祭坛回来了？！
   天啊，这么说那些传说都是真的…
   世界可能会因此而改变。"
  → SetFlag("ivan_heard_news")
```

**Mia（池塘边少女）— 4 页**（追加在已有 1 页之后）

```
Page 1 (已有): "你喜欢这里的池塘吗…有时候有奇怪的闪光…" → SetFlag("met_mia")
Page 2 (新增): require "met_mia" 且 "met_ivan"
  "你也注意到水面闪光了吗？
   有人说那是 Sol Sanctum 里的精灵在呼唤…
   只有被选中的 Adept 才能听到。"
  → SetFlag("mia_revealed")

Page 3 (新增): require "mia_revealed"
  "Garet 经常在山脚下练习战斗。
   他说他想要变强，保护村子。
   你也许应该去找他聊聊。"

Page 4 (新增): require flag "completed_sol_sanctum"
  "所以世界真的要变了…
   传说当炼金术重新苏醒时，
   大地将会发生翻天覆地的变化。
   请保重，Isaac。"
```

**Garsmin（长老）— 新增 3 页**（追加在已有 4 页之后）

```
Page 5 (新增): Page 2 或 Page 3 完成后触发
  require flag "garsmin_bold" 或 "garsmin_revealed"
  "你体内有精灵力的天赋，我看得出来。
   Sol Sanctum 就在村后山的顶端。
   去那里看看吧，但小心山上的怪物。"
  → SetFlag("garsmin_sent_to_sanctum")

Page 6 (新增): require "garsmin_sent_to_sanctum" 且 "completed_sol_sanctum"
  "你去了 Sol Sanctum… 古老的封印被打开了。
   这不是你的错，这是命运的指引。
   现在，世界各地的 Elemental Stars 正在等待被找到。"
  → SetFlag("garsmin_complete_1")

Page 7 (新增): require "garsmin_complete_1"
  "带上 Garet，离开 Vale 吧。
   前往 Bilibin 镇，那里有更多的线索。
   记住，你和精灵之间的联系就是世界的希望。"
  → SetFlag("garsmin_farewell")
```

##### 新增 NPC：Garet（加雷特）

定义一个新的对话条目（在 NPC_SCRIPTS 中追加）：

```rust
("garet", &DialogueScript {
    pages: &[
        DialoguePage { lines: &[DialogueLine { text: GARET_1, actions: &[DialogueAction::SetFlag("met_garet")] }], choices: &[] },
        DialoguePage { lines: &[DialogueLine { text: GARET_2, actions: &[DialogueAction::SetFlag("garet_ready")] }], choices: &[] },
        DialoguePage { lines: &[DialogueLine { text: GARET_3, actions: &[] }], choices: &[] },
    ],
    start_flag: Some("talked_to_garet"),
})
```

文本内容：
```
GARET_1: "嘿，Isaac！你也是来找怪物练手的？
       山上的怪物越来越猖狂了。
       听说 Sol Sanctum 那边有动静…"

GARET_2: (require "met_garsmin")
      "长老也让你去 Sol Sanctum？
       好，我也正要去看看。
       等我准备好我们就出发！"
      → SetFlag("garet_ready")

GARET_3: (require "completed_sol_sanctum" 且 "met_garsmin")
      "我的天…圣祭坛里的东西…
       所以我们真的要踏上旅程了？
       Vale 永远是我的家，但外面的世界在召唤。"
```

#### 7.1.2 新增 Quest 任务链

**文件**: `src/data/quest.rs`

替换 `default_quests()` 函数，返回一个连贯的主线任务链（10 个任务）：

```rust
pub fn default_quests() -> Vec<QuestEntry> {
    vec![
        // ── Chapter 1 ──
        QuestEntry { id: "talk_to_villagers".into(), name: "与村民交谈",
                     description: "和 Vale 村的 Ivan、Mia、Garsmin 打招呼，了解村子近况。" },
        QuestEntry { id: "learn_psynergy".into(), name: "学会精灵力",
                     description: "听长老讲述精灵力的秘密，并尝试使用第一种精灵力。" },
        QuestEntry { id: "find_garet".into(), name: "与 Garet 会合",
                     description: "在山脚下找到 Garet，一起探索 Sol Sanctum。" },
        QuestEntry { id: "explore_sanctum".into(), name: "探索 Sol Sanctum",
                     description: "进入 Sol Sanctum 深处，揭开封印的秘密。" },
        QuestEntry { id: "survive_awakening".into(), name: "封印苏醒",
                     description: "面对苏醒的远古守卫，获得第一颗 Elemental Star。" },

        // ── Chapter 2 ──
        QuestEntry { id: "leave_vale".into(), name: "离开 Vale",
                     description: "告别村民，带着 Elemental Star 前往 Bilibin 镇。" },
        QuestEntry { id: "find_djinn".into(), name: "收集 Djinn",
                     description: "在旅途中寻找并收集散落在各地的 Djinn 精灵。" },

        // ── Chapter 3 ──
        QuestEntry { id: "defeat_boss".into(), name: "击败 Boss",
                     description: "在 Cave 深处击败镇守古老力量的 Boss 敌人。" },
        QuestEntry { id: "master_psynergy".into(), name: "精灵力大师",
                     description: "解锁并使用全部 7 种精灵力，证明你的 Adept 实力。" },
        QuestEntry { id: "become_adept".into(), name: "真正的 Adept",
                     description: "收集至少 5 个 Djinn，解锁高级职业，成为真正的精灵使。" },
    ]
}
```

#### 7.1.3 主线故事驱动逻辑

**文件**: `src/game/update.rs`

在现有 `update()` 函数的 WorldMap 分支末尾，追加故事进度检测逻辑：

```rust
// ── Phase 7: 故事进度自动推进 ──
fn update_story_progression(ctx: &mut GameCtx) {
    // 检测是否完成了全部三种对话
    if ctx.story_flags.get("met_ivan") && ctx.story_flags.get("met_mia")
        && ctx.story_flags.get("met_garsmin")
        && !ctx.story_flags.get("villagers_all_met")
    {
        ctx.story_flags.set("villagers_all_met");
        ctx.quest_log.complete("talk_to_villagers");
        ctx.quest_log.unlock("learn_psynergy");
        // 可选: 弹出系统消息 "任务完成: 与村民交谈"
    }

    // 检测是否使用过精灵力
    if ctx.unlocked_count > 0
        && ctx.story_flags.get("villagers_all_met")
        && !ctx.story_flags.get("psynergy_used")
    {
        // 在 execute_psynergy_effect() 中设置此 flag（见 7.2.1）
    }

    // 检测是否已和 Garet 对话
    if ctx.story_flags.get("met_garet")
        && ctx.story_flags.get("garsmin_sent_to_sanctum")
        && !ctx.story_flags.get("party_ready")
    {
        ctx.story_flags.set("party_ready");
        ctx.quest_log.complete("find_garet");
        ctx.quest_log.unlock("explore_sanctum");
    }

    // Chapter 2: 离开 Vale（进入 WildForest 时触发）
    if ctx.scene.current() == SceneId::WildForest
        && ctx.story_flags.get("completed_sol_sanctum")
        && !ctx.story_flags.get("left_vale")
    {
        ctx.story_flags.set("left_vale");
        ctx.quest_log.complete("leave_vale");
        ctx.quest_log.unlock("find_djinn");
    }
}
```

> **效果**：玩家依次完成对话 → 任务自动推进 → 下一任务解锁 → 形成连贯的新手引导

---

## Phase 7.2：新场景与内容扩充（P1，~400 行）

### 7.2.1 新增场景：Sol Sanctum（圣祭坛室内）

**文件**: `src/map/tilemap.rs`

在 `get_scene_map()` 函数中新增第 4 个场景入口：

```rust
SceneId::SolSanctum => {
    let data: &[u8] = &[
        // 16×16 室内迷宫地图
        // 外围墙壁 (5)，内部走廊和房间
        // 中央放置特殊 tile (249 = SanctumGate, 251 = ElementalStar)
        // 具体布局:
        // 5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,
        // 5,1,1,1,1,5,1,1,1,1,5,1,1,1,1,5,
        // 5,1,5,5,1,5,1,5,5,1,5,1,5,5,1,5,
        // 5,1,5,0,1,1,1,5,0,1,1,1,0,5,1,5,
        // 5,1,5,1,5,5,5,5,1,5,5,5,1,5,1,5,
        // 5,1,1,1,1,1,1,1,1,1,1,1,1,1,1,5,
        // 5,1,5,5,1,5,0,0,0,5,1,5,5,1,5,5,
        // 5,1,1,1,1,5,0,0,0,5,1,1,1,1,5,5,
        // 5,5,5,5,1,5,0,0,0,5,1,5,5,5,5,5,
        // 5,1,1,1,1,5,0,0,0,5,1,1,1,1,5,5,
        // 5,1,5,5,1,5,5,5,5,5,1,5,5,1,5,5,
        // 5,1,1,1,1,1,1,1,1,1,1,1,1,1,1,5,
        // 5,1,5,5,1,5,1,5,5,1,5,1,5,5,1,5,
        // 5,1,5,0,1,1,1,5,0,1,1,1,0,5,1,5,
        // 5,1,5,5,5,5,5,5,5,5,5,5,5,5,1,5,
        // 5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,5,
        // (第一行和最后一行全是墙，中间有 4 个房间通过走廊连接)
    ];
    SceneMap { data, width: 16, height: 16, encounter_rate: 0, encounter_enemies: &[] }
}
```

> **核心设计**：Sol Sanctum 是一个无遇敌的室内迷宫。中央房间放置两种特殊 tile：
> - `SanctumGate` (tile=249): 需要"解锁"(走到该位置触发剧情)  
> - `ElementalStar` (tile=250): 触碰后触发 Boss 战

同时需要在 `SceneId` 枚举中新增 `SolSanctum` 变体。

**文件**: `src/scene/mod.rs`

```rust
pub enum SceneId {
    Title,
    Vale,
    WildForest,
    Cave,
    SolSanctum,     // ← 新增
}
```

同时更新 `SceneRegistry` 的默认初始化、`get_scene_map()` 的模式匹配、`display_name()` 方法、以及所有 `match scene` 表达式来覆盖 `SolSanctum`。

### 7.2.2 新增 Sol Sanctum Boss 敌人 & 战斗

**文件**: `src/data/mod.rs`

在 `enemies_for_area()` 中新增：

```rust
"SolSanctum" => vec![
    EnemyConfig { name: "MythrilGolem", level: 8 },
],
```

**文件**: `src/data/quest.rs` — 新增剧情事件触发的 Boss 战配置

```rust
/// Boss 战配置
pub struct BossEncounter {
    pub name: &'static str,
    pub scene: &'static str,
    pub x: f32,
    pub y: f32,
    pub enemies: Vec<EnemyConfig>,
    pub trigger_flag: &'static str,
    pub completed_flag: &'static str,
}

pub fn boss_encounters() -> &'static [BossEncounter] {
    &[
        BossEncounter {
            name: "MythrilGolem",
            scene: "SolSanctum",
            x: 8.0, y: 8.0,
            enemies: vec![
                EnemyConfig { name: "MythrilGolem", level: 8 },
            ],
            trigger_flag: "at_sanctum_center",
            completed_flag: "completed_sol_sanctum",
        },
    ]
}
```

### 7.2.3 场景入口/出口管理与场景切换逻辑

**文件**: `src/game/update.rs`

在 `update_world_map()` 中的场景边界检测块里，增加以下连接：

```rust
fn check_scene_boundaries(ctx: &mut GameCtx) {
    let (px, py) = (ctx.camera.x, ctx.camera.y);
    let (w, h) = map_size(ctx.scene.current());

    // ── 场景互连映射 ──
    let transitions: &[(&str, f32, f32, &str, f32, f32)] = &[
        // Vale 底部 → WildForest 顶部
        ("Vale", -1.0, -1.0, "WildForest", 10.0, 0.5),
        // WildForest 顶部 → Vale 底部
        ("WildForest", -1.0, -1.0, "Vale", 16.0, hh - 1.0),
        // WildForest 右侧 → Cave 左侧
        ("WildForest", -1.0, -1.0, "Cave", 0.5, 10.0),
        // Cave 左侧 → WildForest 右侧
        ("Cave", -1.0, -1.0, "WildForest", ww - 1.0, 10.0),
        // Vale 顶部 → SolSanctum 底部
        ("Vale", -1.0, -1.0, "SolSanctum", 8.0, 15.0),
        // SolSanctum 底部 → Vale 顶部
        ("SolSanctum", -1.0, -1.0, "Vale", 16.0, 0.5),
    ];

    // 检测玩家走出边界 → 触发场景切换
    for &(from, fx, fy, to, tx, ty) in transitions {
        let current_name = format!("{:?}", ctx.scene.current());
        if current_name == from {
            let outside = (fx < 0.0 && px <= 0.0) || (fy < 0.0 && py <= 0.0)
                || (fx > 0.0 && px >= w as f32 - 1.0) || (fy > 0.0 && py >= h as f32 - 1.0);
            if outside {
                // 将实际边界退出映射到目标位置
                // 需要精确判断方向（上下左右哪个边界）
                if px <= 0.0 { /* left edge */ }
                else if px >= (w - 1) as f32 { /* right edge */ }
                // ... 实际实现时需要按方向映射
            }
        }
    }
}
```

**简化实现方案**：在 Vale 地图特定位置放置不可见的"出口触发区"。当 Camera 中心靠近触发区时，调用 `request_scene_switch()`：

```rust
// 在 update_world_map() 中：
fn check_scene_triggers(ctx: &mut GameCtx) {
    let px = ctx.camera.x;
    let py = ctx.camera.y;

    match ctx.scene.current() {
        SceneId::Vale => {
            // 上方出口 → SolSanctum
            if py <= 0.5 && px >= 14.0 && px <= 17.0 {
                ctx.request_scene_switch(SceneId::SolSanctum);
                ctx.camera.y = 14.0; // SolSanctum 底部入口
            }
            // 下方出口 → WildForest
            if py >= 30.5 && px >= 14.0 && px <= 17.0 {
                ctx.request_scene_switch(SceneId::WildForest);
                ctx.camera.y = 1.0;
            }
        }
        SceneId::WildForest => {
            // 上方出口 → Vale
            if py <= 0.5 && px >= 1.0 && px <= 3.0 {
                ctx.request_scene_switch(SceneId::Vale);
                ctx.camera.y = 30.0;
            }
            // 右侧出口 → Cave
            if px >= 18.5 && py >= 9.0 && py <= 11.0 {
                ctx.request_scene_switch(SceneId::Cave);
                ctx.camera.x = 1.0;
            }
        }
        SceneId::Cave => {
            // 左侧出口 → WildForest
            if px <= 0.5 && py >= 9.0 && py <= 11.0 {
                ctx.request_scene_switch(SceneId::WildForest);
                ctx.camera.x = 18.0;
            }
        }
        SceneId::SolSanctum => {
            // 底部出口 → Vale
            if py >= 14.5 && px >= 7.0 && px <= 9.0 {
                ctx.request_scene_switch(SceneId::Vale);
                ctx.camera.y = 0.5;
            }
        }
    }
}
```

### 7.2.4 新场景 NPC 配置

**文件**: `src/entity/mod.rs`

在 `create_npcs_for_scene()` 中补充所有场景的 NPC：

```rust
pub fn create_npcs_for_scene(scene: SceneId) -> Vec<Entity> {
    match scene {
        SceneId::Vale => create_vale_npcs(),
        SceneId::WildForest => vec![
            Entity::new(10, 7.0, 4.0, "forest_traveler"),   // 旅行者
        ],
        SceneId::Cave => vec![
            Entity::new(20, 12.0, 4.0, "cave_sage"),         // 洞穴贤者
        ],
        SceneId::SolSanctum => vec![
            // Sol Sanctum 内无 NPC（封闭场景）
        ],
    }
}
```

洞穴新增：`cave_sage` 的对话（在 NPC_SCRIPTS 追加）：

```rust
("cave_sage", &DialogueScript {
    pages: &[
        DialoguePage { lines: &[
            DialogueLine { text: "这洞穴深处沉睡着古老的力量…\n你能感受到大地的脉动吗？", actions: &[DialogueAction::SetFlag("met_cave_sage")] },
        ], choices: &[] },
        DialoguePage { lines: &[
            DialogueLine { text: "深处有一个强大的守卫。\n如果你想测试自己的实力，就去挑战吧！", actions: &[] },
        ], choices: &[] },
    ],
    start_flag: Some("talked_to_cave_sage"),
})
```

### 7.2.5 新增敌人种类

**文件**: `src/data/mod.rs` — 扩展 `enemies_for_area()`

增加敌人种类和难度梯度（在原基础上增加新敌人或替换部分敌人）：

```rust
pub fn enemies_for_area(area: &str) -> Vec<EnemyConfig> {
    match area {
        "Vale" => vec![
            EnemyConfig { name: "Wolf", level: 3 },
            EnemyConfig { name: "Bat", level: 2 },
            EnemyConfig { name: "Goblin", level: 4 },
        ],
        "WildForest" => vec![
            EnemyConfig { name: "Wolf", level: 4 },
            EnemyConfig { name: "Spider", level: 3 },
            EnemyConfig { name: "Goblin", level: 5 },
            EnemyConfig { name: "Treant", level: 6 },
            EnemyConfig { name: "Slime", level: 2 },    // ← 新增
        ],
        "Cave" => vec![
            EnemyConfig { name: "Bat", level: 3 },
            EnemyConfig { name: "Golem", level: 7 },
            EnemyConfig { name: "Spider", level: 5 },
            EnemyConfig { name: "Ghost", level: 6 },    // ← 新增
            EnemyConfig { name: "RatKing", level: 8 },  // ← 新增
        ],
        "SolSanctum" => vec![
            EnemyConfig { name: "MythrilGolem", level: 10 },
            EnemyConfig { name: "AncientGuard", level: 9 },  // ← 新增
        ],
        _ => vec![
            EnemyConfig { name: "Wolf", level: 3 },
            EnemyConfig { name: "Bat", level: 2 },
        ],
    }
}
```

### 7.2.6 剧情事件触发系统（Sol Sanctum 中心剧情）

**文件**: `src/game/update.rs`

在 `WorldMap` update 中新增剧情触发检测（放在 `update_story_progression` 之后）：

```rust
// ── Phase 7.2: 场景内触发事件 ──
fn check_scene_events(ctx: &mut GameCtx) {
    if ctx.state != GameState::WorldMap { return; }

    let px = ctx.camera.x.floor() as i32;
    let py = ctx.camera.y.floor() as i32;

    match ctx.scene.current() {
        SceneId::SolSanctum => {
            // 检测是否走到中心位置（8,8 附近）
            let center_dist = ((px - 8).abs() + (py - 8).abs()) as f32;
            if center_dist <= 1.5 && !ctx.story_flags.get("at_sanctum_center") {
                ctx.story_flags.set("at_sanctum_center");
                // 触发 Boss 战
                // 实际上需要先弹出剧情对话，然后进入战斗
                // 这里委托对话系统
            }

            // Boss 战后检查
            if ctx.story_flags.get("completed_sol_sanctum")
                && !ctx.story_flags.get("sanctum_aftermath")
            {
                ctx.story_flags.set("sanctum_aftermath");
                // 弹出后续剧情文字
            }
        }
        _ => {}
    }
}
```

**完整剧情事件实现**：利用现有的 DialogueAction 机制。在 `src/dialogue/script.rs` 中定义新的"系统脚本"：

```rust
// Sol Sanctum 中心剧情对话
const SANCTUM_CENTER: &str = "你走到大厅中央，看到一座古老的祭坛。
祭坛上悬浮着一颗闪耀着金色光芒的宝石…
这就是 Elemental Star！
但就在这时，地面开始震动——
一个巨大的石像从祭坛前升起！";

// Sol Sanctum Boss 后剧情
const SANCTUM_AFTER: &str = "石像崩塌了，Elemental Star 落在你手中。
你能感受到它蕴含的无穷力量。
远处传来古老的声音：
「Elemental Star 的守护者已被击败…
但真正的旅程才刚刚开始…」";

// 新增 NPC 脚本 ID 用于系统事件
("_sanctum_center", &DialogueScript {
    pages: &[
        DialoguePage {
            lines: &[DialogueLine {
                text: SANCTUM_CENTER,
                actions: &[DialogueAction::SetFlag("_sanctum_boss_ready"), DialogueAction::StartBattle],
            }],
            choices: &[],
        },
    ],
    start_flag: Some("_sanctum_entered"),
})

("_sanctum_aftermath", &DialogueScript {
    pages: &[
        DialoguePage {
            lines: &[DialogueLine {
                text: SANCTUM_AFTER,
                actions: &[DialogueAction::SetFlag("_sanctum_complete")],
            }],
            choices: &[],
        },
    ],
    start_flag: Some("_sanctum_after"),
})
```

**脚本 ID 约定**：以 `_` 开头的脚本 ID 为系统触发脚本（不由 NPC 对话触发，而由场景事件触发）。

---

## Phase 7.3：装备与商店系统（P1，~200 行）

### 7.3.1 装备类型定义

**文件**: `src/game/mod.rs`

```rust
/// 装备类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EquipmentSlot {
    Weapon,
    Armor,
    Accessory,
}

/// 装备
#[derive(Debug, Clone)]
pub struct Equipment {
    pub name: &'static str,
    pub slot: EquipmentSlot,
    pub atk_bonus: u32,
    pub def_bonus: u32,
    pub hp_bonus: u32,
    pub price: u32,
    pub description: &'static str,
}

impl Equipment {
    pub fn new(name: &'static str, slot: EquipmentSlot, atk: u32, def: u32, hp: u32, price: u32, desc: &'static str) -> Self {
        Self { name, slot, atk_bonus: atk, def_bonus: def, hp_bonus: hp, price, description: desc }
    }
}

impl Default for Equipment {
    fn default() -> Self {
        Self { name: "None", slot: EquipmentSlot::Weapon, atk_bonus: 0, def_bonus: 0, hp_bonus: 0, price: 0, description: "" }
    }
}
```

### 7.3.2 装备数据库

**文件**: `src/game/mod.rs` 或新建 `src/game/equipment.rs`

```rust
pub fn all_equipment() -> Vec<Equipment> {
    vec![
        // ── 武器 ──
        Equipment::new("短剑", EquipmentSlot::Weapon, 3, 0, 0, 50, "一把普通的短剑"),
        Equipment::new("铁剑", EquipmentSlot::Weapon, 6, 0, 0, 120, "铁匠打造的实用长剑"),
        Equipment::new("长剑", EquipmentSlot::Weapon, 10, 0, 0, 250, "锋利的精钢长剑"),
        Equipment::new("精灵之刃", EquipmentSlot::Weapon, 15, 2, 5, 500, "灌注了精灵之力的魔法剑"),
        // ── 防具 ──
        Equipment::new("布甲", EquipmentSlot::Armor, 0, 3, 0, 40, "轻便的布制防具"),
        Equipment::new("皮甲", EquipmentSlot::Armor, 0, 6, 5, 100, "鞣制皮革制成的胸甲"),
        Equipment::new("锁子甲", EquipmentSlot::Armor, 0, 10, 10, 200, "铁环编织的结实护甲"),
        Equipment::new("精灵护甲", EquipmentSlot::Armor, 2, 15, 20, 450, "精灵祝福过的魔法护甲"),
        // ── 饰品 ──
        Equipment::new("守护戒指", EquipmentSlot::Accessory, 0, 5, 10, 80, "提升防御力和生命值"),
        Equipment::new("力量手环", EquipmentSlot::Accessory, 5, 0, 0, 150, "增强佩戴者的攻击力"),
        Equipment::new("精灵徽章", EquipmentSlot::Accessory, 3, 3, 15, 300, "共鸣着精灵能量的徽章"),
    ]
}
```

### 7.3.3 GameCtx 添加装备字段

在 `GameCtx` 结构体末尾追加：

```rust
// ── Phase 7.3: 装备系统 ──
equipped_weapon: Option<usize>,      // all_equipment() 索引
equipped_armor: Option<usize>,
equipped_accessory: Option<usize>,
```

初始化时全部设为 `None`。

在 `apply_equipment_bonuses()` 中实现装备属性加成应用到 `PlayerStats`：

```rust
fn apply_equipment_bonuses(&mut self) {
    let eqs = all_equipment();
    let mut atk = 0u32; let mut def = 0u32; let mut hp = 0u32;
    
    for slot_idx in [self.equipped_weapon, self.equipped_armor, self.equipped_accessory] {
        if let Some(idx) = slot_idx {
            if idx < eqs.len() {
                atk += eqs[idx].atk_bonus;
                def += eqs[idx].def_bonus;
                hp += eqs[idx].hp_bonus;
            }
        }
    }
    
    // 装备属性是加在基础属性之上的
    // 需要区分基础值和装备加成
    // 简化实现：直接在 battle 中 Combatant 创建时应用
}
```

> **简化方案**：在创建战斗 Combatant 时直接从 GameCtx 读取当前装备加成，不在 PlayerStats 中持久化装备属性。战斗外只影响显示。

### 7.3.4 商店 UI & 交互

**文件**: `src/game/draw.rs` — 新增 `draw_shop()` 函数

```rust
/// 商店界面
/// items: 在售商品列表（equipment 索引或道具类型）
/// selection: 当前选择
/// gold: 玩家金币数
pub fn draw_shop(equipment: &[usize], items: &[ItemType], selection: usize, gold: u32) {
    // 绘制半透明背景框
    // 左侧：商品列表（名称 + 价格 + 属性加成）
    // 右侧：选中商品的详细描述
    // 底部：玩家金币显示 + 操作提示
    // 使用像素字体绘制
}
```

**文件**: `src/game/update.rs` — 新增 shop 交互逻辑

```rust
fn handle_shop_interaction(ctx: &mut GameCtx) {
    // 当玩家站在 Shop 类型 tile 上时按 Confirm 触发
    // GameState::Shop 变体
}
```

**文件**: `src/engine/game_state.rs` — 新增 Shop 状态

```rust
/// 商店状态
pub struct ShopState {
    pub npc_id: u32,
    pub selection: usize,
    pub equipment_for_sale: Vec<usize>,  // all_equipment() 索引
    pub items_for_sale: Vec<ItemType>,
}
```

### 7.3.5 Vale 村商店

在 Vale 村地图 (src/map/tilemap.rs) 中，将某个 Grass tile 替换为 Shop tile（tile=248）。当玩家站在 Shop tile 上按 Confirm 时触发商店界面。

```rust
// 新增 tile 编码
// TradeShop = 248 （商店 tile，不可通行但玩家可站在相邻格触发）
```

**简化方案**：不用新增 tile，而是让 Ivan（铁匠）的对话末尾出现一个商店选项：

```rust
（在 Ivan 的对话 choices 中追加）
DialogueChoice { label: "看看商品", target_page: 5, require_flag: Some("met_ivan"),
    require_affinity: None, set_flag: None },

// Page 5 = 商店 UI 触发页
// 在对话动作中新增 ShopAction
DialogueAction::OpenShop,
```

---

## Phase 7.4：Boss 战机制（P1，~150 行）

### 7.4.1 Boss 行动模式

**文件**: `src/battle/state.rs`

在 `Battle::enemy_decision()` 或等效函数中，增加 Boss 特殊行动逻辑：

```rust
/// 敌人（含 Boss）决策
pub fn enemy_decision(&self, enemy_idx: usize) -> BattleAction {
    let enemy = &self.enemies[enemy_idx];
    let is_boss = enemy.level >= 8;  // 等级≥8视为Boss

    if is_boss {
        // Boss 模式：
        // 50% 概率攻击，30% 概率使用强力技能，20% 概率防御
        match fastrand::u32(0..100) {
            0..=49 => BattleAction::Attack(0),              // 攻击玩家1
            50..=79 => BattleAction::Psynergy(PsynergyType::Force, 0),  // Boss 技能
            _ => BattleAction::Defend,                      // 防御
        }
    } else {
        // 普通敌人：70% 攻击，30% 防御
        let target = fastrand::u32(0..self.party.len() as u32) as usize;
        match fastrand::u32(0..100) {
            0..=69 => BattleAction::Attack(target),
            _ => BattleAction::Defend,
        }
    }
}
```

### 7.4.2 Boss 特殊属性

在 Combatant 结构体或战斗数据中，增加 Boss 特有的字段：

```rust
#[derive(Debug, Clone)]
pub struct Combatant {
    // ... 现有字段 ...
    pub is_boss: bool,
    pub phase_threshold: u32,       // 血量阶段触发值（如 50% 血时切换形态）
}
```

在 `apply_damage()` 中检测血量百分比切换阶段：

```rust
pub fn apply_damage(&mut self, target_idx: usize, damage: u32) -> AttackResult {
    let target = &mut self.enemies[target_idx];
    // ...
    if target.is_boss && target.phase_threshold > 0 {
        let hp_pct = target.hp as f32 / target.max_hp as f32;
        if hp_pct <= 0.5 && !target.status_has("phase2") {
            // Boss 第二阶段：攻击力提升
            target.attack = (target.attack as f32 * 1.5) as u32;
            self.logs.push("Boss 进入了愤怒状态！".into());
        }
    }
}
```

### 7.4.3 战斗奖励翻倍

Boss 战后金币和 EXP 翻倍。在战斗胜利判定后：

```rust
if self.phase == BattlePhase::Victory {
    let boss_mult = if self.enemies.iter().any(|e| e.is_boss) { 2 } else { 1 };
    self.total_exp *= boss_mult;
    self.total_coins *= boss_mult;
}
```

---

## Phase 7.5：召唤系统（P2，~100 行）

### 7.5.1 召唤数据结构

**新文件**: `src/data/summon.rs`

```rust
/// 召唤技能 — 黄金太阳经典特色
#[derive(Debug, Clone)]
pub struct Summon {
    pub name: &'static str,
    pub element: Element,
    pub power: u32,           // 威力
    pub pp_cost: u32,         // PP 消耗
    pub djinn_required: u32,  // 需要几个 Djinn 处于 Standby 状态
    pub description: &'static str,
}

impl Summon {
    pub fn base_damage(&self, level: u32) -> u32 {
        (self.power * level / 2 + self.power * 3) as u32
    }
}

/// 所有召唤列表
pub fn all_summons() -> &'static [Summon] {
    &[
        Summon { name: "Venus",  element: Element::Venus,   power: 30, pp_cost: 8,  djinn_required: 2, description: "召唤大地精灵 Venus" },
        Summon { name: "Ramses", element: Element::Venus,   power: 50, pp_cost: 15, djinn_required: 3, description: "召唤巨像 Ramses 碾压敌人" },
        Summon { name: "Cybele", element: Element::Venus,   power: 80, pp_cost: 25, djinn_required: 4, description: "召唤大地之母 Cybele 的愤怒" },
        Summon { name: "Neptune",element: Element::Mercury, power: 30, pp_cost: 8,  djinn_required: 2, description: "召唤水之精灵 Neptune" },
        Summon { name: "Boreas", element: Element::Mercury, power: 60, pp_cost: 18, djinn_required: 3, description: "召唤极寒之力 Boreas" },
        Summon { name: "Mars",   element: Element::Mars,    power: 35, pp_cost: 9,  djinn_required: 2, description: "召唤火之精灵 Mars" },
        Summon { name: "Meteor", element: Element::Mars,    power: 65, pp_cost: 20, djinn_required: 3, description: "召唤陨石 Meteor 天降" },
        Summon { name: "Jupiter",element: Element::Jupiter, power: 30, pp_cost: 8,  djinn_required: 2, description: "召唤风之精灵 Jupiter" },
        Summon { name: "Atlas",  element: Element::Jupiter, power: 70, pp_cost: 22, djinn_required: 4, description: "召唤巨人 Atlas 席卷一切" },
    ]
}
```

### 7.5.2 战斗中添加 Summon 行动

**文件**: `src/battle/state.rs`

在 `BattleAction` 枚举中追加：

```rust
pub enum BattleAction {
    Attack(usize),
    Defend,
    Psynergy(PsynergyType, usize),
    ReleaseDjinn(DjinnId),
    RecallDjinn(DjinnId),
    Summon(usize),       // ← 新增：summons 数组索引
    Flee,
}
```

在 `execute_turn()` 中追加：

```rust
BattleAction::Summon(summon_idx) => {
    let summons = golden_sun::data::summon::all_summons();
    if let Some(summon) = summons.get(summon_idx) {
        // 检查是否有足够 Djinn 处于 Standby
        let standby_count = battle_state.collect_standby_djinn_count();
        if standby_count >= summon.djinn_required {
            let base_dmg = summon.base_damage(party[attacker_idx].level);
            // 对所有敌人造成伤害
            for (ei, _) in battle_state.enemies.iter().enumerate() {
                let dmg = ((base_dmg as f32) * (1.0 - battle_state.enemies[ei].defense as f32 / 100.0).max(0.1)) as u32;
                battle_state.apply_raw_damage(ei, dmg, false);
                battle_state.results.push(AttackResult {
                    attacker: party[attacker_idx].id,
                    target: battle_state.enemies[ei].id,
                    damage: dmg,
                    element: summon.element,
                    modifier: 1.5,  // 召唤总是优势
                    killed: false,
                });
            }
            // 消耗 PP
            party[attacker_idx].pp = party[attacker_idx].pp.saturating_sub(summon.pp_cost);
            // 消耗 Djinn Standby
            battle_state.consume_standby_djinn(summon.djinn_required);
            battle_state.logs.push(format!("{} 召唤了 {}！", party[attacker_idx].name, summon.name));
        } else {
            battle_state.logs.push("Djinn 不够！需要 {} 个 Standby 状态 Djinn".to_string());
        }
    }
}
```

### 7.5.3 战斗 UI 添加 Summon 选项

**文件**: `src/game/draw.rs` — 在 `draw_battle()` 的 Action 菜单中增加 "Summon" 选项

```rust
// 在战斗菜单选项数组中追加
const BATTLE_ACTIONS: &[&str] = &["Attack", "Defend", "Psynergy", "Summon", "Djinn", "Flee"];

// 当选中 Summon 时，切换到召唤列表子菜单
// 显示所有可用的召唤技能（每元素颜色标注）
// 灰色显示需要更多 Djinn 的召唤
```

---

## Phase 7.6：升级特效与更多战斗反馈（P2，~100 行）

### 7.6.1 升级动画

**文件**: `src/game/update.rs`

在 `add_exp()` 调用处检测等级变化，触发升级状态：

```rust
// 在 PlayerStats::add_exp() 或 GameCtx::add_exp() 中：
fn check_level_up(ctx: &mut GameCtx) {
    let old_level = ctx.player_stats.level;
    ctx.player_stats.add_exp(amount);
    if ctx.player_stats.level > old_level {
        // 设置升级动画状态
        ctx.state = GameState::LevelUp {
            old_level,
            new_level: ctx.player_stats.level,
            timer: 0.0,
        };
    }
}
```

**文件**: `src/engine/game_state.rs` — 新增 LevelUp 状态

```rust
GameState::LevelUp { old_level: u32, new_level: u32, timer: f32 },
```

**文件**: `src/game/draw.rs` — 升级动画绘制

```rust
fn draw_level_up(ctx: &GameCtx, old_lv: u32, new_lv: u32, timer: f32) {
    if let GameState::LevelUp { .. } = ctx.state {
        // 1. 全屏金色闪光（0-0.5s）
        // 2. "LEVEL UP!" 大字弹出（0.5s）
        // 3. 属性增长列表逐一显示（0.5-2.0s）
        //   "HP +8" "ATK +2" "DEF +1"
        // 4. timer >= 2.5s 时切回 WorldMap
    }
}
```

### 7.6.2 战斗统计

**文件**: `src/battle/state.rs`

```rust
pub struct BattleStats {
    pub turns: u32,
    pub damage_dealt: u32,
    pub damage_taken: u32,
    pub items_used: u32,
    pub djinn_released: u32,
}

impl Battle {
    pub fn stats(&self) -> BattleStats {
        BattleStats {
            turns: self.turn_index as u32,
            damage_dealt: self.results.iter().filter(|r| r.damage > 0).map(|r| r.damage).sum(),
            damage_taken: 0, // 需要从攻击玩家方统计
            items_used: 0,
            djinn_released: 0,
        }
    }
}
```

### 7.6.3 战斗评级（胜利时显示）

**文件**: `src/game/draw.rs`

在 `draw_battle()` 中，`BattlePhase::Victory` 时显示评级面板：

```rust
fn draw_victory_panel(ctx: &GameCtx) {
    if let Some(ref battle) = ctx.battle {
        if battle.phase != BattlePhase::Victory { return; }
        
        // 绘制胜利信息面板：
        // "战斗胜利！"
        // "获得 EXP: XXX"
        // "获得 金币: XXX"
        // 如有 Djinn 释放："Djinn: XXX 召唤已使用"
        // "按 Z 继续"
    }
}
```

---

## Phase 7.7：NPC 交互丰富化（P2，~100 行）

### 7.7.1 好感度效果扩展

**文件**: `src/game/update.rs`

在 NPC 对话结束时增加好感度变化：

```rust
// 在 WorldMap update 中对话结束后的处理：
fn after_dialogue(ctx: &mut GameCtx, npc_id: u32) {
    // 每次对话好感度 +1
    let entry = ctx.affinity.entry(npc_id).or_insert(0);
    *entry += 1;
    
    // 好感度解锁特殊对话
    if *entry >= 3 {
        // 触发好感度对话
        ctx.story_flags.set("affinity_3");
    }
}
```

### 7.7.2 NPC 问候（重复对话变化）

在 `NPC_SCRIPTS` 中，为每个 NPC 增加"再次对话"页：

```rust
// 为每个已有 NPC 追加"已经说过话"后的重复问候页
// 规则：如果 start_flag 已设置，自动跳到最后一页
// 在 get_script() 的调用方处理：
// 如果 story_flags.get(start_flag)，从最后一页开始
```

**实现方式**：在 `src/dialogue/script.rs` 中新增函数：

```rust
/// 获取 NPC 的"重复对话"脚本页
pub fn get_repeat_script(id: &str) -> Option<&'static DialoguePage> {
    match id {
        "ivan" => Some(&DialoguePage {
            lines: &[DialogueLine {
                text: "又是你！需要修理装备吗？\n随时欢迎。",
                actions: &[],
            }],
            choices: &[],
        }),
        "mia" => Some(&DialoguePage {
            lines: &[DialogueLine {
                text: "你还在村子里啊！池塘里好像有什么东西…\n要不要去看看？",
                actions: &[],
            }],
            choices: &[],
        }),
        "garsmin" => Some(&DialoguePage {
            lines: &[DialogueLine {
                text: "你的冒险怎么样了？\n记住，力量越大责任越大。",
                actions: &[],
            }],
            choices: &[],
        }),
        "garet" => Some(&DialoguePage {
            lines: &[DialogueLine {
                text: "Isaac！我们去下一站吧！\n世界在等着我们！",
                actions: &[],
            }],
            choices: &[],
        }),
        _ => None,
    }
}
```

### 7.7.3 世界氛围 NPC 闲聊

为 WildForest 和 Cave 的 NPC 增加多页对话：

```rust
("forest_traveler", &DialogueScript {
    pages: &[
        DialoguePage { lines: &[DialogueLine {
            text: "这条路通往 Cave，里面据说很危险。\n不过你有精灵力的话应该没问题。", actions: &[DialogueAction::SetFlag("met_traveler")],
        }], choices: &[] },
    ],
    start_flag: Some("talked_to_traveler"),
})
```

---

## Phase 7.8：测试验证（P2，~100 行）

### 7.8.1 对话脚本测试

**文件**: `tests/dialogue_bdd.rs` — 展开现有的骨架

使用 BDD 风格测试新增对话：

```rust
#[test]
fn ivan_has_4_pages() {
    let script = get_script("ivan").unwrap();
    assert_eq!(script.pages.len(), 4);
}

#[test]
fn mia_has_4_pages() {
    let script = get_script("mia").unwrap();
    assert_eq!(script.pages.len(), 4);
}

#[test]
fn garsmin_has_7_pages() {
    let script = get_script("garsmin").unwrap();
    assert_eq!(script.pages.len(), 7);
}

#[test]
fn garet_script_exists() {
    let script = get_script("garet").unwrap();
    assert!(script.pages.len() >= 3);
}

#[test]
fn system_scripts_start_with_underscore() {
    // 所有以 _ 开头的脚本是系统触发
}

#[test]
fn new_enemies_can_be_spawned() {
    let enemies = enemies_for_area("SolSanctum");
    assert_eq!(enemies.len(), 2);
    assert!(enemies.iter().any(|e| e.name == "MythrilGolem"));
}
```

### 7.8.2 场景测试

```rust
#[test]
fn sol_sanctum_map_exists() {
    let map = get_scene_map(SceneId::SolSanctum);
    assert_eq!(map.width, 16);
    assert_eq!(map.height, 16);
    assert!(map.data.len() > 0);
}

#[test]
fn sol_sanctum_has_center_open_area() {
    let map = get_scene_map(SceneId::SolSanctum);
    let center_idx = (8 * map.width + 8) as usize;  // (8,8)
    assert_eq!(map.data[center_idx], 1);  // Grass，可通行
}
```

### 7.8.3 装备测试

```rust
#[test]
fn all_equipment_has_unique_names() {
    let eqs = all_equipment();
    let mut names: Vec<&str> = eqs.iter().map(|e| e.name).collect();
    names.sort();
    names.dedup();
    assert_eq!(names.len(), eqs.len());
}

#[test]
fn summon_requires_djinn() {
    let summons = all_summons();
    for s in summons {
        assert!(s.djinn_required >= 1);
        assert!(s.djinn_required <= 4);
    }
}
```

### 7.8.4 BDD 特性文件

**在 tests/features/ 中补充**（可选，建议优先实现 Rust 单元测试）：

`story_progression.feature`：
```gherkin
Feature: 故事进度推进

  Scenario: 完成村民对话解锁任务
    Given 玩家在 Vale 村
    When 玩家与 Ivan, Mia, Garsmin 分别对话后
    Then "talk_to_villagers" 任务被标记为完成
    And "learn_psynergy" 任务被解锁

  Scenario: 进入 Sol Sanctum 触发剧情
    Given 玩家已获得精灵力
    When 玩家进入 Sol Sanctum
    Then 剧情 flag "at_sanctum_center" 在走到中心时被设置
```

---

## Phase 7.9：最终验证（必做）

### 执行顺序

1. **先完成 Phase 7.1-7.8 的所有代码修改**
2. **不要合并、不要新文件跳过** — 每个 Phase 按文件逐步修改
3. **每次修改后不要编译** — 全部改完后再编译

### 验证清单

```bash
# 1. 编译检查（必须零错误）
cargo check 2>&1

# 2. Clippy 检查（必须零警告）
cargo clippy --all-targets -- -D warnings 2>&1

# 3. 测试（必须全绿）
cargo test 2>&1

# 4. 运行桌面端（手动检查）
cargo run 2>&1
# 需确认：
# - [ ] 标题画面正常
# - [ ] Vale 村 NPC 新对话可触发
# - [ ] 长老对话推进剧情
# - [ ] Garet 对话正常
# - [ ] Sol Sanctum 可进入（Vale 上方出口）
# - [ ] Sol Sanctum 中心触发 Boss 战
# - [ ] Boss 战后剧情推进
# - [ ] 新敌人种类出现
# - [ ] 任务链按进度解锁/完成
# - [ ] 召唤系统在战斗中可用
# - [ ] 升级动画正常显示
# - [ ] 战斗胜利/失败页面正常
# - [ ] 装备系统生效（如未实现商店，跳过商店验证）

# 5. 测试全部通过
cargo test --test '*'
```

### 提交

```bash
# 确认一切正常后：
git add -A
git commit -m "Phase 7: 主线剧情 + Sol Sanctum + Boss + 召唤 + 装备系统"

# 推送
git push
```

---

## 重要约定提醒

### 代码规范
- 所有常量集中在 `src/engine/constants.rs`，禁止在各模块硬编码魔数
- 颜色通过 constants.rs 的 Color 常量引用，禁止原始 RGBA 元组
- 输入永远通过 `InputBus::consume()` / `has()`，禁止直接读 macroquad is_key_*
- TileKind 在 `src/map/mod.rs` 统一管理（当前 23 种），新增 tile 必须在这里定义
- 所有 `#[non_exhaustive]` 枚举（GameState, TileKind）新增变体后必须更新所有 match

### SceneId 变更警告
- 在 `SceneId` 中新增 `SolSanctum` 变体后，必须遍历以下全部位置确保处理：

| 文件 | 需要更新的 match 位置 |
|------|---------------------|
| `src/scene/mod.rs` | `display_name()`, `SceneRegistry::new()` 默认 |
| `src/map/tilemap.rs` | `get_scene_map()`, `map_size()`, 旧兼容函数 |
| `src/game/mod.rs` | `start_random_battle()`, `save_game()`, `check_djinn_pickup()` |
| `src/game/update.rs` | `update_world_map()`, `trigger_random_encounter()`, 场景边界检查 |
| `src/game/draw.rs` | `draw_hud()` 中的地点名显示 |
| `src/entity/mod.rs` | `create_npcs_for_scene()` |
| `src/data/mod.rs` | `enemies_for_area()` |

如果漏掉任何 match，编译器会报 `non-exhaustive patterns` 错误，逐一补全即可。

### 对话脚本数据位置
- NPC 对话定义在 `src/dialogue/script.rs` 的 `NPC_SCRIPTS` 数组中
- 系统事件脚本（以 `_` 开头）也放在同一个数组
- `get_script()` 函数通过线性查找匹配 ID
- DialogueAction 的 `StartBattle` 动作需要由 update 处理

### 测试
- 测试文件在 `tests/` 目录下：`tilekind_bdd.rs` (38 测试), `core.rs` (6 测试), 以及各骨架文件
- BDD 测试：`.feature` 文件在 `tests/features/`，`_bdd.rs` 在 `tests/`
- 新增对话直接在 `src/dialogue/script.rs` 已有的 `#[cfg(test)]` 块中添加 `#[test]` 函数
- 场景/装备测试直接在 `src/map/tilemap.rs` / `src/game/mod.rs` 的 `#[cfg(test)]` 模块中添加
- 战斗测试在 `tests/combat_bdd.rs` 中展开

---

## 全部任务总览

| 阶段 | 核心任务 | 文件数 | 预估行数 | 优先级 |
|------|---------|--------|---------|--------|
| 7.1 主线剧情 | NPC 对话丰富 + 任务链 + 故事驱动 | 4 | ~400 | P0 |
| 7.2 新场景 | Sol Sanctum + Boss + 场景连接 | 5 | ~400 | P1 |
| 7.3 装备与商店 | 装备/道具系统 + 商店 UI | 4 | ~200 | P1 |
| 7.4 Boss 机制 | 特殊行动 + 阶段切换 + 奖励 | 2 | ~150 | P1 |
| 7.5 召唤系统 | 召唤数据结构 + 战斗整合 | 2 | ~100 | P2 |
| 7.6 升级特效 | 升级动画 + 战斗统计 | 2 | ~100 | P2 |
| 7.7 NPC 丰富 | 好感度 + 闲聊 + 问候 | 2 | ~100 | P2 |
| 7.8 测试验证 | 对话/场景/装备/召唤测试 | 4 | ~100 | P2 |
| **总计** | | | **~1,550** | |

> **Agent 执行顺序**：按 7.1 → 7.2 → 7.3 → 7.4 → 7.5 → 7.6 → 7.7 → 7.8 → 7.9 逐段实现
> 每完成一个子任务，用 `cargo check` 验证无编译错误。全部完成后执行完整验证。
