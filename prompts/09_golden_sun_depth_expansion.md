# Phase 9: 黄金太阳深度内容扩充 — 故事/地图/玩法/打磨

> 目标：在 Phase 0-8 的完整引擎框架上，大幅扩展游戏世界的地图规模、叙事深度和玩法密度，使其更接近 GBA 原版《黄金太阳》的丰富体验
> 执行方式：单 Agent 一次性完成所有任务，按顺序执行各 Phase
> 预估修改：~1,800 行，15+ 文件
> 注意：本 prompt 假设项目已包含 Phase 8（商店/旅馆/过场/战斗增强/Djinn 收集扩展）的全部功能

---

## 项目现状（仅供 Agent 上下文理解）

**已有架构**：42+ 源文件，10,000+ 行 Rust，250 测试零 Clippy 警告，desktop/wasm32 双端。

**已有功能**：
- Mode 7 伪 3D 渲染（5 张地图：Vale 32×32, WildForest 20×20, Cave 16×16, SolSanctum 16×16）
- 23 种 TileKind，7 种精灵力（含施法动画），8 种职业，6 种套装加成
- 回合制战斗（攻击/防御/精灵力/逃跑/Djinn 释放召回/召唤），元素克制 4×4
- 对话系统（打字机 + 分支 + DialogueAction），30+ StoryFlags
- Djinn 系统（16 个 Djinn，含装备/释放/召回/职业切换）
- 任务日志（10 个任务，章节化推进）
- 商店/旅馆/传送/存档/升级动画/装备（11 件 3 槽）/过场过渡/场景名称弹出
- BGM 合成器（vale/battle/boss 三首）+ SFX 音效（10 种）
- CRT 扫描线/天气粒子/小地图/战斗精灵/遇敌闪光/属性克制动画
- 8 个 NPC（Ivan/Mia/Garsmin 各 6-10 页，Garet 5 页，旅馆 2 页，隐士 2 页，探矿者 2 页）
- 4 个区域敌人表（共 11 种敌人）

**仍有缺失**（Phase 9 目标补全）：
- 开场序幕 — 游戏直接进入标题，无叙事背景
- 地图规模 — 仅 5 张地图，原版黄金太阳有 20+ 场景
- 故事闭环 — 当前故事止于 Sol Sanctum，缺少第二幕
- 部分 NPC 脚本缺失 — `forest_traveler` 和 `cave_sage` 无对话脚本
- 召唤系统未完全接入 UI — data 层有，但 battle UI 不可用
- 解谜元素 — 仅有基本 Psynergy tile 交互，缺少连贯谜题链
- Game Over — 战斗失败后直接无反馈
- 极简 NPC — Vale 仅 4 个 NPC，原版有 10+

---

## 共享类型引用

```rust
use golden_sun::engine::{
    GameState, InputState, FrameTime, GameError, GameResult,
    TransitionKind, PsynergyAnim, Mode7Camera,
    Camera, TextureCache, WindowConfig,
};
use golden_sun::engine::input::{InputBus, InputEvent};
use golden_sun::engine::constants::*;
use golden_sun::engine::resources::ResourceManager;
use golden_sun::engine::particle::ParticleSystem;
use golden_sun::engine::storage::{StorageBackend, FsStorage, create_storage};

use golden_sun::map::{TileKind, SceneMap, get_scene_map, tile_center, TileData};
use golden_sun::scene::{SceneId, SceneRegistry};

use golden_sun::entity::{Entity, create_npcs_for_scene, create_vale_npcs};
use golden_sun::entity::sprite::AnimState;

use golden_sun::psynergy::{PsynergyType, Element, apply_psynergy};

use golden_sun::dialogue::{DialogueState, StoryFlags, DialogueAction};
use golden_sun::dialogue::script::{DialogueScript, DialoguePage, DialogueLine, DialogueChoice, get_script, NPC_SCRIPTS};

use golden_sun::battle::{Battle, Combatant, BattlePhase, AttackResult, DamagePopup, BattleAction};
use golden_sun::battle::calculator::{calculate_physical_damage, calculate_psynergy_damage};

use golden_sun::data::{SaveData, EnemyConfig, enemies_for_area};
use golden_sun::data::quest::{QuestLog, QuestEntry, QuestStatus};
use golden_sun::data::djinn::{self, DjinnId, OwnedDjinn, SetBonus, Class, world_djinn, all_djinn_data};
use golden_sun::data::summon::{Summon, all_summons};
use golden_sun::data::loader::{load_map_data, load_npc_data};
use golden_sun::data::cutscene::{CutsceneCmd, Cutscene};

use golden_sun::scene::SceneId::{Title, Vale, WildForest, Cave, SolSanctum};

use golden_sun::ui::{draw_hud, draw_pause_menu, draw_title_screen, draw_status_screen, draw_transition};
use golden_sun::audio::{SfxManager, BgmPlayer};

use crate::game::{GameCtx, Item, ItemType, PlayerStats, SpriteAtlas, EquipmentSlot, Equipment, all_equipment, WaypointDef};
```

---

## 实现顺序与依赖关系

```
Phase 9.1 → 9.2 → 9.3 → 9.4 → 9.5 → 9.6 → 9.7  → 9.8
 序幕/开场   新地图   故事闭环   NPC 扩展   解谜/玩法    UI 打磨   音频扩展   测试验证
```

---

## Phase 9.1：序幕开场与故事完整化（P0，~350 行）

### 9.1.1 开场序幕过场

**文件**: `src/data/cutscene.rs`

在 `cutscene()` 函数末尾追加一场完整的开场序幕过场。通过现有 CutsceneCmd 描述事件序列：

```rust
pub fn opening_prologue() -> Cutscene {
    Cutscene {
        id: "opening_prologue",
        commands: &[
            // ── 1. 全屏黑 0.3s ──
            CutsceneCmd::FadeIn(0.3),
            // ── 2. 星空背景上逐行显示故事文字 ──
            CutsceneCmd::ShowText(0.0, "在遥远的古代，世界的炼金术力量\n被封印在四个 Elemental Stars 之中..."),
            CutsceneCmd::Wait(2.0),
            CutsceneCmd::FadeOut(0.3),
            CutsceneCmd::ShowText(0.0, "Vale 村，一个被群山环绕的宁静村落。\n据说山巅的 Sol Sanctum 中，\n沉睡着远古的秘密..."),
            CutsceneCmd::Wait(2.5),
            CutsceneCmd::FadeOut(0.3),
            CutsceneCmd::FadeIn(0.5),
            // ── 3. 显示游戏标题 ──
            CutsceneCmd::ShowText(0.0, "Golden Sun\n— Rust Edition —"),
            CutsceneCmd::Wait(1.5),
            CutsceneCmd::FadeOut(0.5),
        ],
    }
}
```

> **说明**：使用已有的 `CutsceneCmd` 枚举变体。不修改 `cutscene()` 原有内容，只追加新增函数。

### 9.1.2 标题画面开场触发

**文件**: `src/game/update.rs`

在 `Title` 状态处理中，首次按 Confirm 时触发开场过场而非直接切 WorldMap：

```rust
// 在 Title 状态的 Confirm 处理中：
// 原：直接切到 WorldMap（Vale）
// 改为：
if !ctx.story_flags.get("opening_seen") {
    ctx.story_flags.set("opening_seen");
    // 设置开场过场
    ctx.cutscene.start(opening_prologue());
    ctx.state = GameState::Cutscene;
} else {
    // 已有存档 / 已看过开场 → 直接进 Vale
    ctx.scene.request_switch(SceneId::Vale);
    ctx.state = GameState::WorldMap;
}
```

### 9.1.3 开场后剧情触发

在开场过场结束后，自动进入 Vale 村并弹出任务提示：

**文件**: `src/game/update.rs` — 在 cutscene 完成处理中：

```rust
// 开场过场完成后：
if ctx.cutscene_completed() && ctx.story_flags.get("opening_seen")
    && !ctx.story_flags.get("opening_done")
{
    ctx.story_flags.set("opening_done");
    ctx.scene.request_switch(SceneId::Vale);
    ctx.camera.x = 16.0; // Vale 中心
    ctx.camera.y = 15.0;
    ctx.state = GameState::WorldMap;
    // 弹出对话：Garsmin 在等你
    // 触发自动对话（通过系统脚本来实现）
}
```

### 9.1.4 追加 Garsmin 起始对话

在 Garsmin 的脚本中追加起始页（Page 0，首次进入 Vale 时触发）：

**文件**: `src/dialogue/script.rs`

在 garsmin 的 NPC_SCRIPTS 条目中追加新 Page：

```rust
// Page 0 (起始页): require "opening_done" && !"talked_to_garsmin"
// 条件：开场完成后第一次走向 Garsmin
DialoguePage {
    lines: &[DialogueLine {
        text: "啊，Isaac，你来了。\n我感觉到你体内的力量在苏醒。\n世界正在改变，年轻人。",
        actions: &[DialogueAction::SetFlag("garsmin_opening")],
    }, DialogueLine {
        text: "村里的 Ivan 和 Mia 也察觉到了异常。\n去和他们聊聊吧，了解村子的近况。\n然后来找我，我有重要的事要告诉你。",
        actions: &[],
    }],
    choices: &[],
},
// 原有 Page 1-7 保持不变
```

### 9.1.5 任务链深化

**文件**: `src/data/quest.rs`

替换 `default_quests()` 和 `QUEST_TEMPLATES`，扩展为 15 个任务、4 个章节：

```rust
pub fn default_quests() -> Vec<QuestEntry> {
    vec![
        // ── Act 1: 命运的开端 ──
        QuestEntry { id: "wake_up".into(), name: "苏醒",
            description: "和 Garsmin 长老谈谈，了解村子近况。" },
        QuestEntry { id: "meet_villagers".into(), name: "交談",
            description: "和 Ivan、Mia 聊聊，收集线索。" },
        QuestEntry { id: "first_psynergy".into(), name: "觉醒",
            description: "按长老的指导使用一次精灵力。" },
        QuestEntry { id: "meet_garet_again".into(), name: "同伴",
            description: "到村口找 Garet。" },
        QuestEntry { id: "enter_sanctum".into(), name: "圣祭坛",
            description: "和 Garet 一起进入 Sol Sanctum。" },
        QuestEntry { id: "defeat_guardian".into(), name: "守护者",
            description: "击败 Sol Sanctum 的古老守卫 MythrilGolem。" },

        // ── Act 2: 远行 ──
        QuestEntry { id: "leave_vale".into(), name: "告别",
            description: "与 Vale 的村民告别，踏上旅程。" },
        QuestEntry { id: "explore_forest".into(), name: "密林深处",
            description: "穿越 WildForest，前往新天地。" },
        QuestEntry { id: "find_djinn".into(), name: "Djinn 收集",
            description: "在世界各处寻找并收集 Djinn 精灵。" },
        QuestEntry { id: "find_bilibin".into(), name: "比里宾",
            description: "离开 WildForest，寻找 Bilibin 镇。" },

        // ── Act 3: 挑战 ──
        QuestEntry { id: "cave_depth".into(), name: "洞穴深渊",
            description: "深入 Cave 最深处，揭开封存的秘密。" },
        QuestEntry { id: "master_abilities".into(), name: "掌握之力",
            description: "收集至少 5 个 Djinn 并解锁全部 7 种精灵力。" },
        QuestEntry { id: "summon_mastery".into(), name: "召唤大师",
            description: "通过 Djinn 的 Standby 状态发动一次召唤。" },

        // ── Act 4: 考验 ──
        QuestEntry { id: "ultimate_djinn".into(), name: "精灵王者",
            description: "收集 10 个 Djinn，成为真正的 Adept。" },
        QuestEntry { id: "true_adept".into(), name: "真正的精灵使",
            description: "在战斗中展现你的全部力量，成为传说的 Adept。" },
    ]
}
```

### 9.1.6 故事推进逻辑增强

**文件**: `src/game/update.rs`

替换/扩充 `update_story_progression()` 函数，覆盖 Act 1-4 的全部任务推进：

```rust
fn update_story_progression(ctx: &mut GameCtx) {
    // ── Act 1 ──
    // 开场完成 = 苏醒任务解锁（已在 9.1.3 中设置）
    if ctx.story_flags.get("garsmin_opening") && !ctx.story_flags.get("act1_started") {
        ctx.story_flags.set("act1_started");
        ctx.quest_log.unlock("wake_up");
    }

    // 和 Garsmin 对话 = 完成苏醒，解锁交谈
    if ctx.story_flags.get("talked_to_garsmin") && !ctx.story_flags.get("wake_done") {
        ctx.story_flags.set("wake_done");
        ctx.quest_log.complete("wake_up");
        ctx.quest_log.unlock("meet_villagers");
    }

    // 见了 Ivan 和 Mia = 完成交谈
    if ctx.story_flags.get("talked_to_ivan") && ctx.story_flags.get("talked_to_mia")
        && !ctx.story_flags.get("villagers_met")
    {
        ctx.story_flags.set("villagers_met");
        ctx.quest_log.complete("meet_villagers");
    }

    // 完成交谈 + 解锁精灵力 = 觉醒任务解锁（由 Garsmin 对话自动触发）
    if ctx.story_flags.get("villagers_met") && !ctx.story_flags.get("first_psynergy_unlocked") {
        ctx.story_flags.set("first_psynergy_unlocked");
        ctx.quest_log.unlock("first_psynergy");
    }

    // 使用一次精灵力 = 完成觉醒
    if ctx.story_flags.get("psynergy_used")
        && !ctx.story_flags.get("first_psynergy_done")
    {
        ctx.story_flags.set("first_psynergy_done");
        ctx.quest_log.complete("first_psynergy");
        ctx.quest_log.unlock("meet_garet_again");
    }

    // 见了 Garet = 完成同伴
    if ctx.story_flags.get("talked_to_garet") && !ctx.story_flags.get("garet_met") {
        ctx.story_flags.set("garet_met");
        ctx.quest_log.complete("meet_garet_again");
        ctx.quest_log.unlock("enter_sanctum");
    }

    // 进入 Sol Sanctum = 完成圣祭坛
    if ctx.scene.current() == SceneId::SolSanctum
        && ctx.story_flags.get("garet_met")
        && !ctx.story_flags.get("entered_sanctum")
    {
        ctx.story_flags.set("entered_sanctum");
        ctx.quest_log.complete("enter_sanctum");
        ctx.quest_log.unlock("defeat_guardian");
    }

    // 击败 MythrilGolem = 完成守护者
    if ctx.story_flags.get("completed_sol_sanctum")
        && !ctx.story_flags.get("guardian_defeated")
    {
        ctx.story_flags.set("guardian_defeated");
        ctx.quest_log.complete("defeat_guardian");

        // Act 2 解锁
        ctx.quest_log.unlock("leave_vale");
    }

    // ── Act 2 ──
    // 离开 Vale 进入 WildForest
    if ctx.scene.current() == SceneId::WildForest
        && ctx.story_flags.get("completed_sol_sanctum")
    {
        if !ctx.story_flags.get("left_vale_flag") {
            ctx.story_flags.set("left_vale_flag");
            ctx.quest_log.complete("leave_vale");
            ctx.quest_log.unlock("explore_forest");
        }
    }

    // WildForest 遇到第一个 Djinn
    if ctx.story_flags.get("collected_any_djinn")
        && !ctx.story_flags.get("first_djinn_done")
    {
        ctx.story_flags.set("first_djinn_done");
        ctx.quest_log.complete("explore_forest");
        ctx.quest_log.unlock("find_djinn");
    }

    // 进入 Cave
    if ctx.scene.current() == SceneId::Cave
        && ctx.story_flags.get("first_djinn_done")
        && !ctx.story_flags.get("entered_cave_flag")
    {
        ctx.story_flags.set("entered_cave_flag");
        ctx.quest_log.complete("find_djinn");
        ctx.quest_log.unlock("cave_depth");
    }

    // ── Act 3 ──
    // Cave 深处遇到洞穴贤者
    if ctx.story_flags.get("talked_to_cave_sage")
        && !ctx.story_flags.get("cave_sage_met")
    {
        ctx.story_flags.set("cave_sage_met");
        ctx.quest_log.complete("cave_depth");
        ctx.quest_log.unlock("master_abilities");
    }

    // 解锁全部精灵力 + 5 Djinn
    if ctx.unlocked_count >= 7
        && ctx.collected_djinn.len() >= 5
        && !ctx.story_flags.get("mastered_abilities")
    {
        ctx.story_flags.set("mastered_abilities");
        ctx.quest_log.complete("master_abilities");
        ctx.quest_log.unlock("summon_mastery");
    }

    // 战斗中发动过一次召唤（由战斗结算检测）
    if ctx.story_flags.get("summon_used_in_battle")
        && !ctx.story_flags.get("summon_mastered")
    {
        ctx.story_flags.set("summon_mastered");
        ctx.quest_log.complete("summon_mastery");
    }

    // ── Act 4 ──
    if ctx.collected_djinn.len() >= 10 && !ctx.story_flags.get("ten_djinn_collected") {
        ctx.story_flags.set("ten_djinn_collected");
        ctx.quest_log.complete("ultimate_djinn");
        ctx.quest_log.unlock("true_adept");
    }

    // 收集 12 个 Djinn = 真正 Adept
    if ctx.collected_djinn.len() >= 12 && !ctx.story_flags.get("true_adept_unlocked") {
        ctx.story_flags.set("true_adept_unlocked");
        ctx.quest_log.complete("true_adept");
    }
}
```

> **注意**：`collected_any_djinn` flag 需要由 Djinn 拾取逻辑设置。检查现有 `check_djinn_pickup()` 函数中是否已设置此 flag。
> 同样，`psynergy_used` 需要由 `use_psynergy()` 相关逻辑设置。检查 `try_use_selected_psynergy()` 或等效函数。

---

## Phase 9.2：场景深度扩展（P1，~550 行）

### 9.2.1 新增 SceneId 变体

**文件**: `src/scene/mod.rs`

在 `SceneId` 枚举中追加两个新变体：

```rust
pub enum SceneId {
    Title,
    Vale,
    WildForest,
    Cave,
    SolSanctum,
    Bilibin,      // ← 新增：比里宾镇
    KolimaForest, // ← 新增：柯利玛森林
}
```

**同时需要在以下文件中更新所有 `match scene` 表达式**：
- `src/scene/mod.rs` — `display_name()`: `Bilibin => "Bilibin 镇"`, `KolimaForest => "柯利玛森林"`
- `src/scene/mod.rs` — `SceneRegistry::new()`: 默认场景改为 `Vale`（保持不变）
- `src/map/tilemap.rs` — `get_scene_map()`, `map_size()`, 所有旧兼容函数
- `src/entity/mod.rs` — `create_npcs_for_scene()`
- `src/data/mod.rs` — `enemies_for_area()`
- `src/game/mod.rs` — `start_random_battle()`, `save_game()`, `check_djinn_pickup()`
- `src/game/draw.rs` — `draw_hud()` 中的地点名显示
- `src/game/update.rs` — `update_story_progression()`, `trigger_random_encounter()`, 场景边界检查

### 9.2.2 新增地图：Bilibin 镇

**文件**: `src/map/tilemap.rs`

在 `get_scene_map()` 中新增第 6 个场景：

```rust
SceneId::Bilibin => {
    // 20×20 小镇地图
    // 设计：中心广场 + 4 条街道 + 房屋 + 商店 + 旅馆 + 通往 Vale 路
    // tile 编码: 0=Grass, 1=Path, 2=Water, 3=Tree, 4=Flower, 5=Wall,
    //            6=Roof, 7=PathDark, 8=Well, 9=Fence, 10=Signpost, 11=Bridge
    let data: &[u8] = &[
        // 外围为树木+栅栏，内部为道路+建筑
        // 左上区域: 2 栋房屋，右侧: 商店+旅馆，下方: 通往 Vale 的路口
        3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,
        3,1,1,1,1,5,5,1,1,1,1,1,5,5,1,1,1,1,1,3,
        3,1,6,6,1,5,5,1,3,3,3,1,5,5,1,6,6,6,1,3,
        3,1,1,1,1,0,1,1,3,0,3,1,1,0,1,1,1,1,1,3,
        3,1,1,1,1,0,1,1,3,0,3,1,1,0,1,1,1,1,1,3,
        3,3,3,1,0,0,0,0,0,0,0,0,0,0,0,0,1,3,3,3,
        3,0,0,1,0,3,0,3,3,0,3,3,0,3,0,0,1,0,0,3,
        3,1,1,1,0,3,0,0,0,0,0,0,0,3,0,0,1,1,1,3,
        3,1,6,1,0,3,0,1,1,1,1,1,0,3,0,0,1,6,1,3,
        3,1,1,1,0,3,0,1,10,1,1,1,0,3,0,0,1,1,1,3,
        3,0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,0,3,
        3,1,1,1,0,3,0,1,1,1,1,1,0,3,0,0,1,1,1,3,
        3,1,6,1,0,3,0,0,0,0,0,0,0,3,0,0,1,6,1,3,
        3,1,1,1,0,3,0,3,3,0,3,3,0,3,0,0,1,1,1,3,
        3,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,
        3,1,1,1,1,0,1,1,1,0,1,1,1,0,1,1,1,1,1,3,
        3,1,6,1,1,0,1,6,1,8,1,6,1,0,1,1,6,1,1,3,
        3,1,1,1,1,0,1,1,1,0,1,1,1,0,1,1,1,1,1,3,
        3,1,1,1,1,0,1,1,1,0,1,1,1,0,1,1,1,1,1,3,
        3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,
    ];
    SceneMap { data, width: 20, height: 20, encounter_rate: 0, encounter_enemies: &[] }
}
```

### 9.2.3 新增地图：KolimaForest

**文件**: `src/map/tilemap.rs`

```rust
SceneId::KolimaForest => {
    // 24×24 森林迷宫地图
    // 设计：密集树木+狭窄通道+隐藏岔路
    // 添加特殊 tile 放置：种子(15)、藤蔓(18)、宝箱(16)等
    let data: &[u8] = &[
        // 外围树木，内部蜿蜒通道
        // 左上方入口 → 向右向下蜿蜒 → 右下方出口
        // 含隐藏岔路和宝箱
        3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,
        3,0,0,0,0,3,3,3,3,3,0,0,0,0,0,0,3,3,3,3,0,0,0,3,
        3,0,3,3,0,0,0,0,3,3,0,3,3,3,3,0,0,0,0,3,0,3,0,3,
        3,0,3,3,3,3,3,0,0,0,0,3,0,0,3,3,3,3,0,0,0,3,0,3,
        3,0,0,0,0,0,3,3,3,3,0,3,0,0,0,0,0,3,3,3,3,3,0,3,
        3,3,3,3,3,0,0,0,0,3,0,3,3,3,3,3,0,0,0,0,0,3,0,3,
        3,0,0,0,3,3,3,3,0,3,0,0,0,0,0,3,3,3,3,3,0,3,0,3,
        3,0,3,0,0,0,0,3,0,0,0,3,3,3,0,0,0,0,0,3,0,3,0,3,
        3,0,3,3,3,3,0,0,0,0,3,3,0,3,3,3,3,3,0,0,0,0,0,3,
        3,0,0,0,0,3,3,3,3,0,0,0,0,3,0,0,0,3,3,3,3,3,3,3,
        3,3,3,3,0,0,0,0,3,3,3,3,0,0,0,0,0,0,0,0,0,3,0,3,
        3,0,0,0,0,3,3,0,0,0,0,3,3,3,3,3,3,3,3,3,0,3,0,3,
        3,0,3,3,0,3,0,0,3,3,0,0,0,0,0,0,0,0,0,3,0,3,0,3,
        3,0,3,0,0,3,0,0,0,3,3,3,3,3,3,3,0,0,0,0,0,0,0,3,
        3,0,3,0,3,3,3,3,0,0,0,0,0,0,0,3,3,3,3,3,3,3,3,3,
        3,0,3,0,0,0,0,3,3,3,3,3,3,3,0,0,0,0,0,0,0,0,0,3,
        3,0,3,3,3,3,0,0,0,0,0,0,0,3,3,3,3,3,3,3,3,3,0,3,
        3,0,0,0,0,3,3,3,3,3,3,3,0,0,0,0,0,0,0,0,0,3,0,3,
        3,3,3,3,0,0,0,0,0,0,0,3,3,3,3,3,3,3,3,3,0,3,0,3,
        3,0,0,0,0,3,3,3,3,3,0,0,0,0,0,0,0,0,0,3,0,3,0,3,
        3,0,3,3,0,0,0,0,0,3,3,3,3,3,3,3,3,3,0,0,0,3,0,3,
        3,0,3,3,3,3,3,3,0,0,0,0,0,0,0,0,0,3,3,0,0,3,0,3,
        3,0,0,0,0,0,0,3,3,3,3,3,3,3,3,3,0,0,0,0,0,0,0,3,
        3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,
    ];
    SceneMap { data, width: 24, height: 24, encounter_rate: 10, encounter_enemies: &[] }
}
```

### 9.2.4 场景出入口连接

**文件**: `src/game/update.rs` — 在 `check_scene_boundaries()` 或等效场景边界检测函数中追加：

```rust
// ── 新增 Phase 9 场景连接 ──
match ctx.scene.current() {
    SceneId::WildForest => {
        // 左侧（x ≤ 0） → Bilibin 镇右侧
        if px <= 0.0 && py >= 8.0 && py <= 12.0 {
            ctx.request_scene_switch(SceneId::Bilibin);
            ctx.camera.x = 18.0; // Bilibin 右侧入口
            ctx.camera.y = py;
        }
    }
    SceneId::Bilibin => {
        // 右侧（x ≥ 19） → WildForest 左侧
        if px >= 19.0 && py >= 8.0 && py <= 12.0 {
            ctx.request_scene_switch(SceneId::WildForest);
            ctx.camera.x = 0.5;
            ctx.camera.y = py;
        }
        // 下方（y ≥ 19） → KolimaForest 上方
        if py >= 19.0 && px >= 8.0 && px <= 12.0 {
            ctx.request_scene_switch(SceneId::KolimaForest);
            ctx.camera.x = 10.0;
            ctx.camera.y = 1.0;
        }
    }
    SceneId::KolimaForest => {
        // 上方（y ≤ 0） → Bilibin 下方
        if py <= 0.0 && px >= 8.0 && px <= 12.0 {
            ctx.request_scene_switch(SceneId::Bilibin);
            ctx.camera.x = 10.0;
            ctx.camera.y = 18.0;
        }
    }
    _ => {}
}
```

### 9.2.5 新场景的敌人配置

**文件**: `src/data/mod.rs` — 在 `enemies_for_area()` 中追加：

```rust
"Bilibin" => vec![
    EnemyConfig { name: "Rat", level: 2 },
    EnemyConfig { name: "Spider", level: 3 },
],
"KolimaForest" => vec![
    EnemyConfig { name: "Wolf", level: 5 },
    EnemyConfig { name: "Treant", level: 6 },
    EnemyConfig { name: "Slime", level: 3 },
    EnemyConfig { name: "Mandrake", level: 7 },
    EnemyConfig { name: "Moth", level: 4 },
],
```

### 9.2.6 新场景的 NPC 配置

**文件**: `src/entity/mod.rs`

在 `create_npcs_for_scene()` 中追加：

```rust
SceneId::Bilibin => {
    vec![
        Entity::new(30, 4.0, 13.0, "bilibin_elder"),    // 比里宾长老
        Entity::new(31, 15.0, 10.0, "bilibin_merchant"), // 比里宾商人
        Entity::new(32, 10.0, 5.0, "bilibin_traveler"),  // 比里宾旅行者
        Entity::new(33, 8.0, 15.0, "bilibin_guard"),     // 比里宾守卫
    ]
}
SceneId::KolimaForest => {
    vec![
        Entity::new(40, 12.0, 17.0, "kolima_wanderer"),  // 柯利玛流浪者
        Entity::new(41, 4.0, 14.0, "kolima_sage"),       // 柯利玛贤者
    ]
}
```

### 9.2.7 新场景的 Djinn 位置

**文件**: `src/data/djinn.rs` — 在 `world_djinn()` 中追加 4 个新 Djinn：

```rust
// Bilibin
(Bilibin, DjinnId::Titania, 16.0, 8.0),   // Jupiter Djinn
(Bilibin, DjinnId::Laguna, 5.0, 3.0),     // Mars Djinn
// KolimaForest
(KolimaForest, DjinnId::Amduscia, 18.0, 6.0),  // Jupiter Djinn
(KolimaForest, DjinnId::Grendel, 9.0, 19.0),   // Mars Djinn
```

> **注意**：`world_djinn()` 的返回类型需要确认。如果当前是 `HashMap<SceneId, Vec<...>>` 或切片，在已有 8 个条目后追加 4 个，共 12 个地图可收集 Djinn。

---

## Phase 9.3：Bilibin 镇的 NPC 与故事内容（P1，~250 行）

### 9.3.1 Bilibin NPC 对话脚本

**文件**: `src/dialogue/script.rs` — 在 NPC_SCRIPTS 末尾追加 4 个新 NPC 脚本：

```rust
// ── Bilibin 镇 NPC ──
("bilibin_elder", &DialogueScript {
    pages: &[
        DialoguePage {
            lines: &[DialogueLine {
                text: "来自 Vale 的旅人？欢迎来到 Bilibin！\n我们这里虽然不大，但消息灵通。",
                actions: &[DialogueAction::SetFlag("met_bilibin_elder")],
            }, DialogueLine {
                text: "最近 Cave 那边怪事不断。\n有人说看到了会发光的石头和奇怪的生物。\n或许你该去看看。",
                actions: &[],
            }],
            choices: &[],
        },
        DialoguePage {
            lines: &[DialogueLine {
                text: "又见面了！Cave 的路很危险，\n记得准备好补给再出发。\n镇上的商人卖的东西很实用。",
                actions: &[],
            }],
            choices: &[],
        },
    ],
    start_flag: Some("talked_to_bilibin_elder"),
}),

("bilibin_merchant", &DialogueScript {
    pages: &[
        DialoguePage {
            lines: &[DialogueLine {
                text: "你好，冒险家！我的货虽然不多，\n但都是精品。\n如果你有好东西想卖，我也收！",
                actions: &[DialogueAction::SetFlag("met_bilibin_merchant")],
            }],
            choices: &[
                DialogueChoice {
                    label: "看看商品",
                    target_page: 5,
                    require_flag: Some("met_bilibin_merchant"),
                    require_affinity: None,
                    set_flag: Some("_open_shop_bilibin"),
                },
            ],
        },
        DialoguePage {
            lines: &[DialogueLine {
                text: "今天想买点什么？\n武器？防具？素材？\n好东西不等人！",
                actions: &[],
            }],
            choices: &[],
        },
    ],
    start_flag: Some("talked_to_bilibin_merchant"),
}),

("bilibin_traveler", &DialogueScript {
    pages: &[
        DialoguePage {
            lines: &[DialogueLine {
                text: "我刚从北边回来，那里的森林浓密得\n连阳光都透不进来…\n不过林子里据说有珍贵的 Djinn 出没。",
                actions: &[DialogueAction::SetFlag("met_bilibin_traveler")],
            }],
            choices: &[],
        },
        DialoguePage {
            lines: &[DialogueLine {
                text: "你真打算去 Kolima 森林？\n勇气可嘉！但最好带上强效回复药。",
                actions: &[],
            }],
            choices: &[],
        },
    ],
    start_flag: Some("talked_to_bilibin_traveler"),
}),

("bilibin_guard", &DialogueScript {
    pages: &[
        DialoguePage {
            lines: &[DialogueLine {
                text: "站住…啊，是旅行者。\n最近 Cave 方向有异动，\n我建议你提高警惕。",
                actions: &[DialogueAction::SetFlag("met_bilibin_guard")],
            }, DialogueLine {
                text: "Bilibin 往南是 Kolima 森林，\n那里的树木会移动——\n不过也许只是传说。",
                actions: &[],
            }],
            choices: &[],
        },
        DialoguePage {
            lines: &[DialogueLine {
                text: "又见面了！林子里的情况怎么样？\n祝你好运！",
                actions: &[],
            }],
            choices: &[],
        },
    ],
    start_flag: Some("talked_to_bilibin_guard"),
}),
```

### 9.3.2 Kolima Forest NPC 对话脚本

```rust
// ── Kolima Forest NPC ──
("kolima_wanderer", &DialogueScript {
    pages: &[
        DialoguePage {
            lines: &[DialogueLine {
                text: "这片森林比看起来大得多…\n不要被表面的小路迷惑。\n有些地方需要特殊的力量才能到达。",
                actions: &[DialogueAction::SetFlag("met_kolima_wanderer")],
            }],
            choices: &[],
        },
        DialoguePage {
            lines: &[DialogueLine {
                text: "如果你有 Grow 精灵力，\n可以种出通过某些沟壑的桥梁。\n试试在种子 tile 上使用它！",
                actions: &[DialogueAction::SetFlag("kolima_growth_hint")],
            }],
            choices: &[],
        },
    ],
    start_flag: Some("talked_to_kolima_wanderer"),
}),

("kolima_sage", &DialogueScript {
    pages: &[
        DialoguePage {
            lines: &[DialogueLine {
                text: "古老的树木在低语…\n它们见证过 Elemental Stars 的诞生。\n如果你能倾听，它们会告诉你很多。",
                actions: &[DialogueAction::SetFlag("met_kolima_sage")],
            }, DialogueLine {
                text: "森林深处藏着两个 Djinn。\n但只有真正懂得精灵力的人才能找到它们。",
                actions: &[],
            }],
            choices: &[],
        },
        DialoguePage {
            lines: &[DialogueLine {
                text: "Djinn 是大地的精灵，\n与你的灵魂共鸣。\n收集得越多，你的力量就越强大。",
                actions: &[],
            }],
            choices: &[],
        },
    ],
    start_flag: Some("talked_to_kolima_sage"),
}),
```

### 9.3.3 Bilibin 商店整合

**文件**: `src/game/mod.rs` — 在 `shop_inventory()` 或等效函数中为 Bilibin 新增商品列表：

如果系统是通过 `_open_shop` flag + 某个 tile 或 NPC 触发的多店铺机制，则需要区分商店 ID。如果是单一商店，改为 NPC 触发带不同列表的形式：

```rust
// 商店触发处理——根据 NPC 脚本的 flag 区分：
// "_open_shop" → Ivan 的 Vale 商店（目前已有）
// "_open_shop_bilibin" → Bilibin 商人商店
// 在 GameCtx 的 dialog_action 处理中：
fn handle_shop_trigger(ctx: &mut GameCtx) {
    if ctx.story_flags.get("_open_shop") {
        ctx.story_flags.clear("_open_shop");
        // 打开 Ivan 商店
        ctx.shop_equipment = vec![0, 1, 4, 5, 8]; // 短剑/铁剑/布甲/皮甲/守护戒指
        ctx.shop_items = vec![ItemType::Potion, ItemType::Ether];
        ctx.state = GameState::Shop { ... };
    }
    if ctx.story_flags.get("_open_shop_bilibin") {
        ctx.story_flags.clear("_open_shop_bilibin");
        // 打开 Bilibin 商店（更好装备）
        ctx.shop_equipment = vec![2, 3, 6, 7, 9, 10]; // 长剑/精灵之刃/锁子甲/精灵护甲/力量手环/精灵徽章
        ctx.shop_items = vec![ItemType::Potion, ItemType::Ether, ItemType::GoldRing];
        ctx.state = GameState::Shop { ... };
    }
}
```

> **注意**：此函数需要依据实际 Shop 状态结构设计。Phase 8 中 Shop 状态可能为 `GameState::Shop { ... }` 携带商品列表，也可能通过 GameCtx 字段传递。请查看现有代码调整。

### 9.3.4 Kolima Forest 的 Puzzle

**文件**: `src/game/update.rs` — Kolima Forest 特殊触发逻辑

Kolima Forest 的探索设计：
1. 在 KolimaForest 地图中放置几处 Seed tile（15），玩家使用 Growth 后现出新的路径
2. 特殊区域设置 HiddenChest（16），需要玩家靠近后按 Confirm 打开
3. Dusklia 区域在白天/夜晚不同（如夜间有发光精灵移动痕迹）

这些 puzzle 不需要硬编码到 update 中，因为它们用的是通用 Psynergy tile 交互机制。但如果需要更复杂的谜题（如多处种植激活整体事件），则需要增加检测：

```rust
// 在 update_world_map() 的 check_scene_events() 中追加：
match ctx.scene.current() {
    SceneId::KolimaForest => {
        // 检测 Kolima 核心区域（14, 18）是否所有必要种子已生长
        // 如果是，在特定位置生成一条新路（将 Wall tile 替换为 Grass）
        // 通过 modified_tiles 记录
    }
    _ => {}
}
```

---

## Phase 9.4：Vale 村 NPC 扩充（P2，~150 行）

### 9.4.1 Vale 村新增 NPC

**文件**: `src/entity/mod.rs` — 在 `create_vale_npcs()` 中追加 4 个新 NPC：

```rust
// 原 NPC 保持不变，追加以下 NPC：
Entity::new(6, 4.0, 5.0, "vale_child"),     // 村子里的小孩
Entity::new(7, 25.0, 10.0, "vale_farmer"),  // 农民
Entity::new(8, 12.0, 25.0, "vale_fisher"),  // 渔夫
Entity::new(9, 28.0, 28.0, "vale_old_woman"), // 老妇人
```

### 9.4.2 Vale 新增 NPC 对话脚本

**文件**: `src/dialogue/script.rs` — 在 NPC_SCRIPTS 追加：

```rust
// ── Vale 村新增 NPC ──
("vale_child", &DialogueScript {
    pages: &[
        DialoguePage {
            lines: &[DialogueLine {
                text: "Isaac 哥哥！你又要去冒险了吗？\n我以后也要像你一样！",
                actions: &[DialogueAction::SetFlag("met_vale_child")],
            }],
            choices: &[],
        },
        DialoguePage {
            lines: &[DialogueLine {
                text: "Garet 总是吹牛说他多厉害。\n不过 Isaac 哥哥肯定更厉害！",
                actions: &[],
            }],
            choices: &[],
        },
        DialoguePage {
            lines: &[DialogueLine {
                text: "听说山上有石像在动…\n好可怕… 但如果是 Isaac 的话就没问题！",
                actions: &[],
            }],
            choices: &[],
        },
    ],
    start_flag: Some("talked_to_vale_child"),
}),

("vale_farmer", &DialogueScript {
    pages: &[
        DialoguePage {
            lines: &[DialogueLine {
                text: "今年的收成不太好…\n山上的怪物吓跑了不少猎物。\n年轻人，你能帮忙清理一下吗？",
                actions: &[DialogueAction::SetFlag("met_vale_farmer")],
            }],
            choices: &[],
        },
        DialoguePage {
            lines: &[DialogueLine {
                text: "听说北边有座新镇子，\n叫 Bilibin 还是什么的。\n路不好走，但据说那里的人很友好。",
                actions: &[],
            }],
            choices: &[],
        },
        DialoguePage {
            lines: &[DialogueLine {
                text: "我已经听说你在圣殿的事了。\n守护 Vale 的责任就交给你了！",
                actions: &[],
            }],
            choices: &[],
        },
    ],
    start_flag: Some("talked_to_vale_farmer"),
}),

("vale_fisher", &DialogueScript {
    pages: &[
        DialoguePage {
            lines: &[DialogueLine {
                text: "今天的鱼都不上钩…\n池塘里有什么在发光。\n你看到了吗？就在那边。",
                actions: &[DialogueAction::SetFlag("met_vale_fisher")],
            }],
            choices: &[],
        },
        DialoguePage {
            lines: &[DialogueLine {
                text: "哦，就那条闪光的大鱼！\n它已经游了好几天了。\n说不定是和精灵有关？",
                actions: &[DialogueAction::SetFlag("vale_fisher_hint")],
            }],
            choices: &[],
        },
        DialoguePage {
            lines: &[DialogueLine {
                text: "哈哈哈，今天的鱼依然不听话。\n不过看到你我就放心了——村子有你在。",
                actions: &[],
            }],
            choices: &[],
        },
    ],
    start_flag: Some("talked_to_vale_fisher"),
}),

("vale_old_woman", &DialogueScript {
    pages: &[
        DialoguePage {
            lines: &[DialogueLine {
                text: "啊，Isaac 啊。\n你和你父亲越来越像了。\n他也曾经踏上过远方的旅程。",
                actions: &[DialogueAction::SetFlag("met_vale_old_woman")],
            }],
            choices: &[],
        },
        DialoguePage {
            lines: &[DialogueLine {
                text: "Sol Sanctum 的事我听说了。\n你做得对。有些封印，\n注定要被打破的。",
                actions: &[],
            }],
            choices: &[],
        },
        DialoguePage {
            lines: &[DialogueLine {
                text: "如果你在路上看到一种蓝色的小花，\n那是我年轻时最爱的品种。\n它叫 'Adepthil'，意为 '精灵之花'。",
                actions: &[DialogueAction::SetFlag("vale_flower_hint")],
            }],
            choices: &[],
        },
        DialoguePage {
            lines: &[DialogueLine {
                text: "去吧，年轻人。世界在呼唤你。\nVale 永远是你的家。",
                actions: &[],
            }],
            choices: &[],
        },
    ],
    start_flag: Some("talked_to_vale_old_woman"),
}),
```

### 9.4.3 修复缺失的 NPC 脚本

**文件**: `src/dialogue/script.rs`

修复 `forest_traveler` 和 `cave_sage` 两个 NPC 有实体定义但无对话脚本的问题。检查 NPC_SCRIPTS 是否已包含它们，如不包含则追加：

```rust
// ── 补充原有 NPC 的缺失脚本 ──
// forest_traveler (WildForest NPC ID 10, entity id "forest_traveler")
("forest_traveler", &DialogueScript {
    pages: &[
        DialoguePage {
            lines: &[DialogueLine {
                text: "你好！这条路通往 Cave。\n里面据说有强大的 Golem。\n你要小心啊！",
                actions: &[DialogueAction::SetFlag("met_forest_traveler")],
            }],
            choices: &[],
        },
        DialoguePage {
            lines: &[DialogueLine {
                text: "你还没去 Cave？\n那里的 RatKing 可不是好惹的。\n多准备些药水再去。",
                actions: &[],
            }],
            choices: &[],
        },
    ],
    start_flag: Some("talked_to_forest_traveler"),
}),

// cave_sage (Cave NPC ID 20, entity id "cave_sage")
("cave_sage", &DialogueScript {
    pages: &[
        DialoguePage {
            lines: &[DialogueLine {
                text: "欢迎来到 Cave…\n我已经在这里静修多年了。\n你身上有精灵的气息。",
                actions: &[DialogueAction::SetFlag("talked_to_cave_sage")],
            }, DialogueLine {
                text: "洞穴深处沉睡着远古的力量。\n如果你能通过考验，\n那些力量可能会选择你。",
                actions: &[],
            }],
            choices: &[],
        },
        DialoguePage {
            lines: &[DialogueLine {
                text: "你已经深入过了吗？\n很好。记住——真正的力量\n来源于内心，而非外物。",
                actions: &[],
            }],
            choices: &[],
        },
    ],
    start_flag: Some("talked_to_cave_sage"), // 注意：和 story_flags 中的 key 一致
}),
```

---

## Phase 9.5：装备与战斗系统扩展（P1，~180 行）

### 9.5.1 新增装备

**文件**: `src/game/mod.rs` — 在 `all_equipment()` 中追加 5 件新装备（共 16 件）：

```rust
// 追加在现有 11 件装备之后
// ── Phase 9 新装备 ──
Equipment::new("破甲剑", EquipmentSlot::Weapon, 12, 0, 0, 350, "专门用于破坏敌人防御的长剑"),
Equipment::new("战斗斧", EquipmentSlot::Weapon, 18, 0, 0, 650, "沉重的双手战斧，威力巨大"),
Equipment::new("铁甲", EquipmentSlot::Armor, 0, 12, 15, 350, "精铁打造的坚固护甲"),
Equipment::new("治愈戒指", EquipmentSlot::Accessory, 0, 3, 25, 500, "战斗中缓慢恢复 HP 的魔法戒指"),
Equipment::new("力量腰带", EquipmentSlot::Accessory, 8, 2, 5, 400, "激发身体潜能的腰带"),
```

### 9.5.2 战斗道具系统扩展

**文件**: `src/game/mod.rs` — 扩展 ItemType 和 item 效果

确认当前 `ItemType` 枚举的定义，如果只有 `Potion`, `Ether`, `GoldRing` 三种，则扩展：

```rust
// 在 ItemType 枚举中追加（或确认已存在）
pub enum ItemType {
    Potion,      // 回复 50 HP
    Ether,       // 回复 10 PP
    GoldRing,    // 出售用
    Elixir,      // ← 新增：完全回复 HP/PP
    Antidote,    // ← 新增：解毒（如果将来有中毒状态）
    Nut,         // ← 新增：永久提升 5 MaxHP
    // 副本类（战斗外使用）
}
```

**文件**: `src/game/mod.rs` — 在 Item 方法中实现效果：

```rust
impl Item {
    pub fn use_on_player(&self, stats: &mut PlayerStats) -> String {
        match self.item_type {
            ItemType::Potion => {
                stats.hp = (stats.hp + 50).min(stats.max_hp);
                format!("使用了 药水！HP 恢复了 50 点。")
            }
            ItemType::Ether => {
                // 需要访问 PP
                // stats.pp = (stats.pp + 10).min(stats.max_pp);
                format!("使用了 灵药！PP 恢复了 10 点。")
            }
            ItemType::Elixir => {
                stats.hp = stats.max_hp;
                // stats.pp = stats.max_pp;
                format!("使用了 圣灵药！体力完全恢复！")
            }
            ItemType::Nut => {
                stats.max_hp += 5;
                stats.hp = (stats.hp + 5).min(stats.max_hp);
                format!("使用了 坚果！最大 HP 上升 5 点。")
            }
            _ => "这个道具无法在此使用。".into(),
        }
    }
}
```

### 9.5.3 战斗内召唤 UI 集成

**文件**: `src/game/draw.rs` 和 `src/game/update.rs`

召唤系统（9 个召唤）当前存在于 `data::summon::all_summons()` 但战斗 UI 中可能不可用。确保以下集成完成：

**draw.rs** — 在战斗菜单中 "Summon" 选项有实际的绘制分支：
```rust
// 在战斗 Action 菜单中确认 "Summon" 存在
// 选中 Summon 后，显示可用的召唤列表：
// - 按元素颜色显示召唤名称
// - 灰色（不可用）：需要的 Standby Djinn 数不足
// - 白色（可用）：需要显示消耗 PP 和召唤名称
```

**update.rs** — 在战斗输入处理中确认 Summon 选择可执行：
```rust
// 在 battle player action 处理中：
// BattleAction::Summon(summon_idx) → 触发 execute_turn 中的召唤逻辑
// 需要确认 execute_turn() 的 Summon 分支已完整实现
```

> **验证方式**：进入战斗 → 选择 Summon → 查看召唤列表 → 试图使用一个召唤
> 如果召唤不可用，检查 Battle 中是否有 `collect_standby_djinn_count()` 和 `consume_standby_djinn()` 方法

### 9.5.4 战斗加速功能确认

**文件**: `src/game/update.rs`

检查 B 键加速功能是否已集成。如果 Battle 状态有 `turbo: bool` 字段，但不响应 B 键：

```rust
// 在 Battle update 中检测 B 键：
if let Some(battle) = &mut ctx.battle {
    if ctx.input_bus.has(InputEvent::Turbo) || ctx.input.boost { // 检测按住 B
        battle.turbo = true;
    }
}
```

> 检查 `InputBus` 是否有 `Turbo` 事件。没有则用 `ctx.input_bus.has(InputEvent::Action)` 的变通方案。

---

## Phase 9.6：Game Over 与标题增强（P2，~100 行）

### 9.6.1 Game Over 状态

**文件**: `src/engine/game_state.rs`

在 `GameState` 枚举中新增 `GameOver` 变体：

```rust
/// 全灭画面
GameOver {
    timer: f32,     // 显示时间
    retry: bool,    // 是否选择重试
},
```

**文件**: `src/game/update.rs` — 在 Battle 全灭检测中：

```rust
// 在 Battle update 中，检测全灭
if battle.phase == BattlePhase::Defeat || battle.all_party_dead() {
    ctx.state = GameState::GameOver { timer: 0.0, retry: false };
    ctx.battle = None;
}
```

**文件**: `src/game/update.rs` — Game Over 更新逻辑：

```rust
GameState::GameOver { ref mut timer, ref mut retry } => {
    *timer += ctx.time.delta;
    // 2 秒后可选择
    if *timer > 2.0 {
        if ctx.input_bus.consume(InputEvent::Confirm) {
            // 重新加载存档
            ctx.load_game().ok();
            if ctx.save_data.is_some() {
                // 恢复存档
                ctx.apply_save_data();
                ctx.state = GameState::WorldMap;
            } else {
                // 无存档 → 回到标题
                ctx.scene.request_switch(SceneId::Title);
                ctx.state = GameState::Title;
            }
        }
        if ctx.input_bus.consume(InputEvent::Cancel) {
            ctx.scene.request_switch(SceneId::Title);
            ctx.state = GameState::Title;
        }
    }
}
```

**文件**: `src/game/draw.rs` — Game Over 绘制：

```rust
fn draw_game_over(ctx: &GameCtx, timer: f32) {
    let screen_w = ctx.config.width as f32;
    let screen_h = ctx.config.height as f32;
    // 全屏暗红覆盖
    draw_rectangle(0.0, 0.0, screen_w, screen_h,
        Color { r: 0.3, g: 0.0, b: 0.0, a: 0.8 });
    // 显示 "GAME OVER"
    let text = "GAME OVER";
    let text_x = screen_w / 2.0 - 64.0;
    let text_y = screen_h / 2.0;
    draw_text(text, text_x, text_y, 32.0, WHITE);
    // 2 秒后显示提示
    if timer > 2.0 {
        let prompt = "Z: 重新读档    X: 返回标题";
        draw_text(prompt, screen_w / 2.0 - 80.0, text_y + 40.0, 16.0,
            Color { r: 0.8, g: 0.8, b: 0.8, a: 1.0 });
    }
}
```

### 9.6.2 标题画面增强

**文件**: `src/game/draw.rs` — 在 `draw_title_screen()` 中增强：

当前标题画面可能只有简单的文字。增强为：
- 背景：渐变色（深海蓝 → 深紫）
- 标题："Golden Sun" 大字，金色像素字体
- 副标题："— Rust Edition —" 
- 底部：闪烁的 "Press Z to Start"
- 如果检测到存档存在："X: 继续游戏"

```rust
// draw_title_screen 增强示例：
pub fn draw_title_screen(ctx: &GameCtx, has_save: bool) {
    let w = ctx.config.width as f32;
    let h = ctx.config.height as f32;
    
    // 渐变背景
    for y in 0..(h as i32) {
        let t = y as f32 / h;
        let r = 0.05 + t * 0.1;
        let g = 0.02 + t * 0.05;
        let b = 0.15 + t * 0.2;
        draw_rectangle(0.0, y as f32, w, 1.0, Color { r, g, b, a: 1.0 });
    }
    
    // 标题文字
    draw_text("Golden Sun", w / 2.0 - 100.0, h / 3.0, 36.0, GOLD);
    draw_text("— Rust Edition —", w / 2.0 - 80.0, h / 3.0 + 36.0, 16.0, LIGHT_GRAY);
    
    // 闪烁提示（使用 sin(time) 控制 alpha）
    let alpha = ((ctx.time.total * 3.0).sin() * 0.3 + 0.7).max(0.0).min(1.0);
    let blink = Color { r: 1.0, g: 1.0, b: 1.0, a: alpha };
    draw_text("Press Z to Start", w / 2.0 - 72.0, h * 0.7, 16.0, blink);
    
    if has_save {
        draw_text("X: Continue", w / 2.0 - 56.0, h * 0.7 + 24.0, 16.0, LIGHT_GRAY);
    }
    
    // 底部版本信息
    draw_text("v1.0", 8.0, h - 8.0, 12.0, DARK_GRAY);
}
```

---

## Phase 9.7：音频与视觉增强（P2，~120 行）

### 9.7.1 新增 BGM

**文件**: `src/audio/mod.rs` — 在 BgmPlayer 中新增场景专用 BGM：

当前有 `vale`, `battle`, `boss` 三首。追加 `bilibin`, `forest` 两首新 BGM：

```rust
// 在 BgmPlayer 结构体或现有 BGM 生成函数中追加：

/// Bilibin 镇 BGM — 轻松愉快的 E 大调
fn generate_bilibin_bgm() -> Vec<f32> {
    let sample_rate = 44100;
    let bpm = 90.0;
    let beat_len = 60.0 / bpm;
    let total_beats = 16; // 4 小节
    let len = (sample_rate as f32 * beat_len * total_beats as f32) as usize;
    let mut buf = vec![0.0f32; len];
    
    // 旋律：E 大调上行琶音 + 节奏
    let melody = [
        329.63, 392.00, 440.00, 523.25,  // E G# A B (E major)
        440.00, 392.00, 329.63, 293.66,  // A G# E D
        349.23, 440.00, 523.25, 587.33,  // F# A B C# 
        523.25, 440.00, 392.00, 329.63,  // B A G# E
    ];
    let chord = [
        329.63, 415.30, 523.25,  // E G# B (E major triad)
        293.66, 369.99, 440.00,  // D F# A (D major)
        349.23, 440.00, 523.25,  // F# A C# (F#m)
        261.63, 329.63, 392.00,  // C E G (C major)
    ];
    
    for (i, sample) in buf.iter_mut().enumerate() {
        let t = i as f32 / sample_rate as f32;
        let beat = (t / beat_len) as usize % melody.len();
        let chord_beat = (t / (beat_len * 4.0)) as usize % 4;
        
        let mut val = 0.0;
        // 旋律音
        let mel_freq = melody[beat % melody.len()];
        val += (t * mel_freq * 2.0 * std::f32::consts::PI).sin() * 0.15;
        // 和弦
        for ci in 0..3 {
            let ch_freq = chord[chord_beat * 3 + ci];
            val += (t * ch_freq * 2.0 * std::f32::consts::PI).sin() * 0.08;
        }
        // 节奏鼓点（每隔一拍强调）
        let beat_pos = t % beat_len;
        if beat_pos < 0.05 {
            val += 0.2;
        } else if (beat_pos - beat_len * 0.5).abs() < 0.05 {
            val += 0.1;
        }
        
        *sample = val;
    }
    buf
}

/// Kolima 森林 BGM — 神秘朦胧的 C 小调
fn generate_forest_bgm() -> Vec<f32> {
    let sample_rate = 44100;
    let bpm = 70.0;
    let beat_len = 60.0 / bpm;
    let total_beats = 16;
    let len = (sample_rate as f32 * beat_len * total_beats as f32) as usize;
    let mut buf = vec![0.0f32; len];
    
    // 氛围：长持续音 + 零星高音
    let drone = 130.81; // C3（低音持续）
    let melody = [
        261.63, 293.66, 311.13, 349.23,  // C D Eb F
        392.00, 349.23, 311.13, 293.66,  // G F Eb D
        311.13, 349.23, 392.00, 440.00,  // Eb F G A
        415.30, 392.00, 349.23, 311.13,  // Ab G F Eb
    ];
    
    for (i, sample) in buf.iter_mut().enumerate() {
        let t = i as f32 / sample_rate as f32;
        let beat = (t / beat_len) as usize % melody.len();
        
        let mut val = 0.0;
        // 持续低音
        val += (t * drone * 2.0 * std::f32::consts::PI).sin() * 0.2;
        // 旋律（淡入淡出，每隔两个拍出一次）
        let mel_phase = (t / (beat_len * 2.0)) % 1.0;
        if mel_phase < 0.5 {
            let mel_freq = melody[beat];
            let env = (mel_phase * 2.0).min(1.0); // fade in
            val += (t * mel_freq * 2.0 * std::f32::consts::PI).sin() * 0.12 * env;
        }
        // 环境噪声（微量）
        val += (t * 997.0).sin() * 0.02;
        
        *sample = val;
    }
    buf
}
```

在 `BgmPlayer` 的注册表或 `play_bgm()` 调度函数中注册新 BGM：
```rust
// 在 BGM 注册处追加：
self.register("bilibin", generate_bilibin_bgm());
self.register("forest", generate_forest_bgm());
```

**文件**: `src/game/mod.rs` — 在 `apply_scene_switch()` 中自动切换 BGM：

```rust
// 场景切换时自动选择合适的 BGM
fn apply_scene_switch(&mut self) {
    match self.scene.current() {
        SceneId::Vale | SceneId::SolSanctum => self.bgm_player.play("vale"),
        SceneId::WildForest | SceneId::KolimaForest => self.bgm_player.play("forest"),
        SceneId::Bilibin => self.bgm_player.play("bilibin"),
        SceneId::Cave => self.bgm_player.play("vale"), // Cave 暂时复用
        _ => {}
    }
}
```

### 9.7.2 新增 SFX

**文件**: `src/audio/mod.rs` — 在 SfxManager 中追加：

```rust
// 追加新 SFX
self.register("game_over", vec![
    // 低沉下降音（0.8 秒）
    // 从 440Hz 滑到 110Hz
]);
self.register("opening", vec![
    // 庄严的开幕音（1.0 秒）
    // 从 220Hz 上升到 880Hz 的和弦
]);
```

> **简化方案**：使用 `generate_square_wave()` 或等效辅助函数快速生成。例如：
> ```rust
> self.register("game_over", generate_descending_tone(440.0, 110.0, 0.8));
> ```

### 9.7.3 更多天气粒子效果

**文件**: `src/engine/particle.rs` 或 `src/game/draw.rs`

确认当前已有粒子系统。如果只有雨/雪两种，新增粒子种类用于氛围增强：

```rust
// 在 ParticleKind 枚举中追加：
pub enum ParticleKind {
    Rain,
    Snow,
    Sparkle,    // ← 新增：精灵光点（在 Djinn/Djinn 附近闪烁）
    Leaf,       // ← 新增：落叶（在 Kolima 森林使用）
}

// 实现 Leaf 粒子生成：
fn spawn_leaf(particles: &mut Vec<Particle>) {
    // 棕色椭圆粒子，横向飘落
    // 速度: (0.3~1.0, -0.5~0.5)
    // 可加微小旋转
}
```

> **提示**：如果当前 `ParticleKind` 不做复杂区分（只是颜色/速度变化），可以不新增枚举，而在现有粒子参数中微调。

---

## Phase 9.8：测试与验证（P2，~100 行）

### 9.8.1 对话脚本测试

**文件**: `tests/dialogue_bdd.rs`

在现有测试块中追加：

```rust
#[test]
fn bilibin_npcs_have_scripts() {
    for id in &["bilibin_elder", "bilibin_merchant", "bilibin_traveler", "bilibin_guard"] {
        let script = get_script(id);
        assert!(script.is_some(), "NPC {} 缺少对话脚本", id);
        let script = script.unwrap();
        assert!(script.pages.len() >= 1, "NPC {} 对话页数不足", id);
    }
}

#[test]
fn kolima_npcs_have_scripts() {
    for id in &["kolima_wanderer", "kolima_sage"] {
        let script = get_script(id);
        assert!(script.is_some(), "NPC {} 缺少对话脚本", id);
    }
}

#[test]
fn vale_new_npcs_have_scripts() {
    for id in &["vale_child", "vale_farmer", "vale_fisher", "vale_old_woman"] {
        let script = get_script(id);
        assert!(script.is_some(), "NPC {} 缺少对话脚本", id);
    }
}

#[test]
fn forest_traveler_and_cave_sage_have_scripts() {
    assert!(get_script("forest_traveler").is_some(), "forest_traveler 缺少对话脚本");
    assert!(get_script("cave_sage").is_some(), "cave_sage 缺少对话脚本");
}
```

### 9.8.2 场景测试

**文件**: `src/scene/mod.rs` 或 `src/game/mod.rs` — 在 `#[cfg(test)]` 块中：

```rust
#[test]
fn new_scenes_have_maps() {
    for scene in &[SceneId::Bilibin, SceneId::KolimaForest] {
        let map = get_scene_map(*scene);
        assert!(map.width > 0, "Scene {:?} 地图宽度为 0", scene);
        assert!(map.height > 0, "Scene {:?} 地图高度为 0", scene);
        assert!(!map.data.is_empty(), "Scene {:?} 地图数据为空", scene);
    }
}

#[test]
fn new_scenes_have_npcs() {
    let bilibin_npcs = create_npcs_for_scene(SceneId::Bilibin);
    assert_eq!(bilibin_npcs.len(), 4, "Bilibin 应有 4 个 NPC");
    
    let kolima_npcs = create_npcs_for_scene(SceneId::KolimaForest);
    assert_eq!(kolima_npcs.len(), 2, "KolimaForest 应有 2 个 NPC");
}
```

### 9.8.3 任务链测试

```rust
#[test]
fn quest_chain_has_15_entries() {
    let quests = default_quests();
    assert_eq!(quests.len(), 15, "任务链应有 15 个任务");
    
    // 检查任务 ID 唯一性
    let mut ids: Vec<&str> = quests.iter().map(|q| q.id.as_str()).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), 15, "任务 ID 不唯一");
}
```

### 9.8.4 Game Over 测试

```rust
#[test]
fn game_over_state_exists() {
    // 确认 GameState 有 GameOver 变体
    let game_over = GameState::GameOver { timer: 0.0, retry: false };
    match game_over {
        GameState::GameOver { .. } => {} // 编译通过即可
        _ => panic!("GameState 缺少 GameOver 变体"),
    }
}
```

### 9.8.5 装备测试

```rust
#[test]
fn all_equipment_is_valid() {
    let eqs = all_equipment();
    assert_eq!(eqs.len(), 16, "应有 16 件装备（原有 11 + 新增 5）");
    // 检查价格合理
    for eq in &eqs {
        assert!(eq.price > 0, "装备 {} 价格应为正数", eq.name);
        assert!(eq.atk_bonus > 0 || eq.def_bonus > 0 || eq.hp_bonus > 0,
            "装备 {} 应至少有一个属性加成", eq.name);
    }
}
```

### 9.8.6 Bilibin 敌人配置

```rust
#[test]
fn bilibin_has_enemies() {
    let enemies = enemies_for_area("Bilibin");
    assert!(!enemies.is_empty(), "Bilibin 应配置敌人");
}

#[test]
fn kolima_forest_has_enemies() {
    let enemies = enemies_for_area("KolimaForest");
    assert!(!enemies.is_empty(), "KolimaForest 应配置敌人");
}
```

---

## Phase 9.9：最终验证（必做）

### 执行顺序

1. **按 Phase 9.1 → 9.2 → 9.3 → 9.4 → 9.5 → 9.6 → 9.7 → 9.8 顺序逐一实现**
2. 每个子 Phase 实现后不要单独编译，全部改完后统一编译
3. 所有 match 表达式必须覆盖新变体（编译器会提示，逐一补全即可）

### 验证清单

```bash
# 1. 编译检查（必须零错误）
cargo check 2>&1

# 2. Clippy 检查（必须零警告）
cargo clippy --all-targets -- -D warnings 2>&1

# 3. 测试（必须全绿）
cargo test 2>&1
# 预期：260+ 测试全部通过

# 4. 手动验证（运行 cargo run）
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
# - [ ] 标题画面增强
# - [ ] Bilibin/KolimaForest BGM 自动切换
# - [ ] 新 Djinn 在地图上可收集（Bilibin 2 + Kolima 2）
# - [ ] 新敌人的战斗正常
# - [ ] 15 个任务全部可查看

# 5. 测试全部通过
cargo test --test '*' 2>&1

# 6. 提交
git add -A
git commit -m "Phase 9: 深度内容扩充 — 开场/Bilibin/Kolima/GameOver/12NPC/15个任务/16件装备/16Djinn"
```

---

## 常见风险与应对

| 风险 | 影响 | 缓解方案 |
|------|------|----------|
| SceneId 新增后大量 match 不完整 | 编译失败 | 编译器逐行报错，逐一补全即可 |
| NPC 脚本 ID 与实体 ID 不匹配 | 对话无法触发 | 确保 Entity 的 `dialogue_id` 与 NPC_SCRIPTS 中的 key 一致 |
| BGM 生成函数不兼容 | 运行时无声 | 复用已有 `generate_*_bgm()` 函数的模板 |
| world_djinn() 签名不同 | 编译错误 | 查看实际函数签名并调整新增条目格式 |
| GameOver 状态在 GameState match 中缺失 | 编译错误 | 补充所有 match 中的 `GameOver` 分支 |
| Shop 状态结构不同 | 编译/运行时错误 | 查看现有 ShopState 结构，按实际字段调整商店触发逻辑 |
| summon 在 battle UI 中不可用 | 功能缺失 | 确认 BattleAction::Summon 在 execute_turn 中已实现 |

---

## 全部任务总览

| Phase | 核心任务 | 文件数 | 预估行数 | 优先级 |
|-------|---------|--------|---------|--------|
| 9.1 开场与故事 | 开场过场 + 故事推进 + 15 任务链 | 4 | ~350 | P0 |
| 9.2 新场景 | Bilibin/KolimaForest 地图 + 场景连接 | 6 | ~550 | P1 |
| 9.3 Bilibin 内容 | 8 个 NPC 脚本 + 商店整合 | 3 | ~250 | P1 |
| 9.4 Vale NPC 扩充 | 4 个新 Vale NPC + 2 个修复脚本 | 2 | ~150 | P2 |
| 9.5 玩法扩展 | 5 件新装备 + 道具扩展 + 召唤集成 | 3 | ~180 | P1 |
| 9.6 Game Over | GameOver 状态 + 标题增强 | 3 | ~100 | P2 |
| 9.7 音频增强 | 2 首 BGM + SFX + 粒子 | 3 | ~120 | P2 |
| 9.8 测试验证 | 对话/场景/任务/装备/敌人测试 | 3 | ~100 | P2 |
| **总计** | | | **~1,800** | |
