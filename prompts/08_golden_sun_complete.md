# Phase 8: 黄金太阳完整化 — 商店/旅馆/过场/战斗增强/内容补完

> 目标：在 Phase 0-7 完整引擎基础上，补齐最后的关键功能与内容，使游戏在故事叙事和玩法深度上真正逼近 GBA 原版《黄金太阳》
> 执行方式：单 Agent 一次性完成所有任务，按顺序执行各 Phase
> 预估新增行数：~1,200 行，修改 18+ 个文件

---

## 项目现状（仅供 Agent 上下文理解）

**已有架构**：40+ 源文件，5300+ 行 Rust，176+ 测试零 Clippy 警告，desktop/wasm32 双端。

**已有功能**：
- Mode 7 伪 3D 渲染（5 张地图：Vale 32×32, WildForest 20×20, Cave 16×16, SolSanctum 16×16）
- 23 种 TileKind（含交互型：Vine/Seed/PushBlock/DarkArea/HiddenChest/Waypoint 等）
- 7 种精灵力（Whirlwind/Growth/Freeze/Force/Catch/Flash/Reveal），含施法动画特效
- 回合制战斗系统（攻击/防御/精灵力/逃跑/Djinn 释放召回/召唤），含元素克制(4×4)、伤害数字弹出
- 对话系统（打字机效果 + 分支选择 + DialogueAction 副作用机制）
- Djinn 精灵系统（16 个 Djinn，4 元素各 4 个，含装备/释放/召回/职业切换）
- 任务日志系统（QuestLog）
- BGM 合成器（vale/battle 两首）+ SFX 音效（confirm/cancel/hurt/heal/psynergy）
- 存档系统（bincode 序列化 + StorageBackend trait + Save 菜单）
- 传送系统（Waypoint tile → Travel 菜单）
- 天气粒子系统 + CRT 扫描线滤镜 + 小地图 + HUD
- 输入录制/回放（debug 模式 R/P 键）
- 4 个对话 NPC（Ivan/Mia/Garsmin/Garet），Garsmin 有分支选择
- 装备系统（11 件装备，武/防/饰三槽）
- 升级动画（金色闪光 + LEVEL UP 文字 + 属性增长，3 秒自动结束）
- 召唤系统（9 个召唤，4 元素各 2-3 个，Djinn Standby 消耗机制）
- 过场过渡（FadeIn/FadeOut/Wipe 三种效果）
- Boss 机制（MythrilGolem, AncientGuard，特殊 AI + 阶段切换 + 双倍奖励）

**最大缺失**（Phase 8 将补全）：
- 商店/购买系统 — Ivan 的 "看看商品" 选项无实际功能
- 旅馆/恢复系统 — 无法恢复 HP/PP
- 游戏开场动画 — 无故事背景介绍
- 场景名称弹出 — 进入新场景时无地名显示
- 战斗增强 — 无战斗背景、道具使用、角色切换
- Djinn 获取 — 仅 3/16 可收集，其他无处获取
- 音频扩展 — 缺少升级/商店/战斗胜利音效
- 故事补完 — 缺少结局/离开 Vale 的完整叙事

---

## 共享类型引用

```rust
// 引擎核心
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

// 地图与场景
use golden_sun::map::{TileKind, SceneMap, get_scene_map, tile_center, TileData};
use golden_sun::scene::{SceneId, SceneRegistry};

// 实体与精灵
use golden_sun::entity::{Entity, create_npcs_for_scene, create_vale_npcs};
use golden_sun::entity::sprite::AnimState;

// 精灵力
use golden_sun::psynergy::{PsynergyType, Element, apply_psynergy};

// 对话
use golden_sun::dialogue::{DialogueState, StoryFlags, DialogueAction};
use golden_sun::dialogue::script::{DialogueScript, DialoguePage, DialogueLine, DialogueChoice, get_script, NPC_SCRIPTS};

// 战斗
use golden_sun::battle::{Battle, Combatant, BattlePhase, AttackResult, DamagePopup, BattleAction};
use golden_sun::battle::calculator::{calculate_physical_damage, calculate_psynergy_damage};

// 数据
use golden_sun::data::{SaveData, EnemyConfig, enemies_for_area};
use golden_sun::data::quest::{QuestLog, QuestEntry};
use golden_sun::data::djinn::{self, DjinnId, OwnedDjinn, SetBonus, Class, world_djinn, all_djinn_data};
use golden_sun::data::summon::{Summon, all_summons};
use golden_sun::data::loader::{load_map_data, load_npc_data};

// UI / 音频
use golden_sun::ui::{draw_hud, draw_pause_menu, draw_title_screen, draw_status_screen, draw_transition};
use golden_sun::audio::{SfxManager, BgmPlayer};

// 项目内
use crate::game::{GameCtx, Item, ItemType, PlayerStats, SpriteAtlas, EquipmentSlot, Equipment, all_equipment, WaypointDef};
```

---

## Phase 8.1：商店系统（P0，~200 行）

### 8.1.1 新增 ShopState

**文件**: `src/engine/game_state.rs`

在 `GameState` 枚举末尾新增变体（位于 `LevelUp` 之后）：

```rust
/// 商店界面
Shop {
    /// NPC ID（用于好感度）
    npc_id: u32,
    /// 在售装备索引列表（all_equipment() 的索引）
    equipment_for_sale: Vec<usize>,
    /// 在售道具类型列表
    items_for_sale: Vec<ItemType>,
    /// 当前选中索引
    selection: usize,
    /// 当前标签页: 0=装备, 1=道具, 2=出售
    tab: usize,
    /// 出售时的玩家物品选择索引
    sell_selection: usize,
    /// 系统消息（如 "购买成功！"）
    message: String,
    /// 消息剩余显示时间
    message_timer: f32,
}
```

### 8.1.2 商店数据配置

**文件**: `src/game/mod.rs`

在 `all_equipment()` 函数附近添加商店配置函数：

```rust
/// 商店库存配置 — 返回 (装备索引列表, 道具列表)
/// npc_id 用于区分不同NPC商店
pub fn shop_inventory(npc_id: u32) -> (Vec<usize>, Vec<ItemType>) {
    match npc_id {
        // Ivan 的武器/防具店
        0 => (
            vec![0, 1, 2, 3, 4, 5, 6, 7], // 所有武器+防具
            vec![ItemType::Potion, ItemType::Ether],
        ),
        // 通用杂货店
        _ => (
            vec![8, 9, 10], // 饰品
            vec![ItemType::Potion, ItemType::Ether],
        ),
    }
}

/// 出售价格（原价的一半）
pub fn sell_price(equipment_idx: usize) -> u32 {
    let eqs = all_equipment();
    if equipment_idx < eqs.len() {
        eqs[equipment_idx].price / 2
    } else {
        0
    }
}
```

### 8.1.3 商店 UI 绘制

**文件**: `src/game/draw.rs`

新增 `draw_shop()` 函数，在 `draw()` 的 `_ =>` placeholder 之前注册：

```rust
/// 绘制商店界面
fn draw_shop(&self, shop: &ShopState) {
    let screen_w = RENDER_TARGET_W as f32;
    let screen_h = RENDER_TARGET_H as f32;

    // 1. 半透明背景覆盖
    draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.7));

    // 2. 商店标题
    let title = match shop.tab {
        0 => "—  商店 — 购买装备  —",
        1 => "—  商店 — 购买道具  —",
        2 => "—  商店 — 出售物品  —",
        _ => "—  商店  —",
    };
    draw_text(title, screen_w / 2.0 - 80.0, 30.0, 16.0, GOLD);

    // 3. 标签页切换提示（上方小字）
    draw_text("[← →] 切换标签  [A] 购买/选择  [B] 返回", 20.0, screen_h - 20.0, 10.0, LIGHTGRAY);

    // 4. 金币显示（右上角）
    let gold_text = format!("金币: {}G", self.gold);
    draw_text(&gold_text, screen_w - 120.0, 30.0, 14.0, YELLOW);

    // 5. 商品列表
    let eqs = all_equipment();
    let start_y = 50.0;
    let line_h = 20.0;

    match shop.tab {
        0 | 1 => {
            // 购买模式：列出商品
            let items = if shop.tab == 0 { &shop.equipment_for_sale } else { &[] };
            let inv_items = if shop.tab == 1 { &shop.items_for_sale } else { &[] };

            // 装备列表
            for (i, eq_idx) in items.iter().enumerate() {
                if *eq_idx >= eqs.len() { continue; }
                let eq = &eqs[*eq_idx];
                let y = start_y + i as f32 * line_h;
                let selected = i == shop.selection;

                // 选中高亮
                if selected {
                    draw_rectangle(30.0, y - 2.0, screen_w - 60.0, line_h, Color::new(0.3, 0.3, 0.6, 0.5));
                }

                // 装备名称（根据能否购买上色）
                let can_afford = self.gold >= eq.price;
                let color = if !can_afford { DARKGRAY } else if selected { YELLOW } else { WHITE };
                draw_text(eq.name, 40.0, y + 12.0, 12.0, color);

                // 属性加成
                let bonus_str = format!("ATK+{} DEF+{} HP+{}", eq.atk_bonus, eq.def_bonus, eq.hp_bonus);
                draw_text(&bonus_str, 200.0, y + 12.0, 10.0, LIGHTGRAY);

                // 价格
                draw_text(&format!("{}G", eq.price), screen_w - 100.0, y + 12.0, 12.0,
                    if can_afford { GOLD } else { RED });
            }

            // 道具列表
            let inv_offset = shop.equipment_for_sale.len();
            for (i, item_type) in inv_items.iter().enumerate() {
                let idx = inv_offset + i;
                let y = start_y + idx as f32 * line_h;
                let selected = idx == shop.selection;

                if selected {
                    draw_rectangle(30.0, y - 2.0, screen_w - 60.0, line_h, Color::new(0.3, 0.3, 0.6, 0.5));
                }

                let price = match item_type {
                    ItemType::Potion => 15,
                    ItemType::Ether => 20,
                    ItemType::GoldRing => 100,
                };
                let name = item_type.name();
                let color = if self.gold < price { DARKGRAY } else if selected { YELLOW } else { WHITE };
                draw_text(name, 40.0, y + 12.0, 12.0, color);
                draw_text(&format!("{}G", price), screen_w - 100.0, y + 12.0, 12.0, GOLD);
            }
        }
        2 => {
            // 出售模式：列出玩家物品栏
            let items_text: Vec<String> = self.inventory.iter()
                .map(|i| format!("{} x{}", i.item_type.name(), i.count))
                .collect();

            for (i, text) in items_text.iter().enumerate() {
                let y = start_y + i as f32 * line_h;
                let selected = i == shop.sell_selection;

                if selected {
                    draw_rectangle(30.0, y - 2.0, screen_w - 60.0, line_h, Color::new(0.3, 0.3, 0.6, 0.5));
                }

                let color = if selected { YELLOW } else { WHITE };
                draw_text(text, 40.0, y + 12.0, 12.0, color);
                draw_text("出售", screen_w - 100.0, y + 12.0, 12.0, GOLD);
            }

            if items_text.is_empty() {
                draw_text("没有可出售的物品", screen_w / 2.0 - 60.0, start_y + 20.0, 12.0, GRAY);
            }
        }
        _ => {}
    }

    // 6. 选中商品的详细描述（底部信息栏）
    let desc_y = screen_h - 60.0;
    draw_rectangle(0.0, desc_y - 5.0, screen_w, 40.0, Color::new(0.0, 0.0, 0.0, 0.5));

    // 显示消息
    if !shop.message.is_empty() {
        draw_text(&shop.message, screen_w / 2.0 - 80.0, screen_h / 2.0, 16.0, YELLOW);
    }
}
```

### 8.1.4 商店交互逻辑

**文件**: `src/game/update.rs`

在 `update()` 函数的 `GameState::Shop` 分支中处理输入：

```rust
GameState::Shop { ref mut npc_id, ref equipment_for_sale, ref items_for_sale, ref mut selection,
    ref mut tab, ref mut sell_selection, ref mut message, ref mut message_timer } =>
{
    *message_timer = message_timer.saturating_sub(self.time.delta);
    if *message_timer <= 0.0 {
        message.clear();
    }

    // 标签页切换
    if self.input_bus.consume(InputEvent::Left) {
        *tab = if *tab == 0 { 2 } else { *tab - 1 };
        *selection = 0;
        *sell_selection = 0;
        self.play_sfx("cancel");
    }
    if self.input_bus.consume(InputEvent::Right) {
        *tab = if *tab == 2 { 0 } else { *tab + 1 };
        *selection = 0;
        *sell_selection = 0;
        self.play_sfx("cancel");
    }

    // 上下选择
    if self.input_bus.consume(InputEvent::Up) {
        if *tab < 2 {
            *selection = selection.saturating_sub(1);
        } else {
            *sell_selection = sell_selection.saturating_sub(1);
        }
        self.play_sfx("cancel");
    }
    if self.input_bus.consume(InputEvent::Down) {
        if *tab < 2 {
            let max_items = equipment_for_sale.len() + items_for_sale.len();
            *selection = (*selection + 1).min(max_items.saturating_sub(1));
        } else {
            *sell_selection = (*sell_selection + 1).min(self.inventory.len().saturating_sub(1));
        }
        self.play_sfx("cancel");
    }

    // A 确认 — 购买/出售
    if self.input_bus.consume(InputEvent::Confirm) {
        match *tab {
            0 | 1 => {
                // 购买
                let eqs = all_equipment();
                let eq_count = equipment_for_sale.len();

                if *tab == 0 && *selection < eq_count {
                    let eq_idx = equipment_for_sale[*selection];
                    if eq_idx < eqs.len() {
                        let price = eqs[eq_idx].price;
                        if self.gold >= price {
                            self.gold -= price;
                            self.equip_item(eqs[eq_idx].slot, eq_idx);
                            *message = "购买成功！".into();
                            *message_timer = 2.0;
                            self.play_sfx("confirm");
                        } else {
                            *message = "金币不足！".into();
                            *message_timer = 2.0;
                            self.play_sfx("cancel");
                        }
                    }
                } else if *tab == 1 {
                    // 购买道具
                    let item_idx = *selection - eq_count;
                    if item_idx < items_for_sale.len() {
                        let item_type = items_for_sale[item_idx];
                        let price = match item_type {
                            ItemType::Potion => 15,
                            ItemType::Ether => 20,
                            ItemType::GoldRing => 100,
                        };
                        if self.gold >= price {
                            self.gold -= price;
                            // 添加到物品栏
                            if let Some(existing) = self.inventory.iter_mut().find(|i| i.item_type == item_type) {
                                existing.count += 1;
                            } else {
                                self.inventory.push(Item::new(item_type));
                            }
                            *message = "购买成功！".into();
                            *message_timer = 2.0;
                            self.play_sfx("confirm");
                        } else {
                            *message = "金币不足！".into();
                            *message_timer = 2.0;
                            self.play_sfx("cancel");
                        }
                    }
                }
            }
            2 => {
                // 出售
                if *sell_selection < self.inventory.len() {
                    let item = &self.inventory[*sell_selection];
                    let price = match item.item_type {
                        ItemType::Potion => 7,
                        ItemType::Ether => 10,
                        ItemType::GoldRing => 50,
                    };
                    self.gold += price;
                    self.inventory[*sell_selection].count -= 1;
                    if self.inventory[*sell_selection].count == 0 {
                        self.inventory.remove(*sell_selection);
                        *sell_selection = sell_selection.saturating_sub(1);
                    }
                    *message = "出售成功！".into();
                    *message_timer = 2.0;
                    self.play_sfx("confirm");
                }
            }
            _ => {}
        }
    }

    // B 取消 — 关闭商店
    if self.input_bus.consume(InputEvent::Cancel) {
        self.play_sfx("cancel");
        self.state = GameState::WorldMap;
    }
}
```

### 8.1.5 对话触发商店

**文件**: `src/dialogue/mod.rs`

在 `DialogueAction` 枚举中新增：

```rust
/// 打开商店界面（参数：NPC ID）
OpenShop(u32),
```

**文件**: `src/dialogue/script.rs`

在 Ivan 的对话（NPC_SCRIPTS）中，找到 "看看商品" 这个选择所在行，将原有的 `target_page: 1` 保持（因为第一页是 Ivan 的首页），同时需要从 DialogueChoice 处触发商店。修改方式：

Ivan 的对话不能通过 target_page 来触发商店，而是需要在 DialogueAction 中处理。由于 DialogueAction 不支持外部状态跳转，改为在 update.rs 中 `handle_choice_selection()` 处理成功后检测选择对应的动作。

更好的方案：在 `src/game/update.rs` 的 `handle_choice_selection()` 末尾，在设置完新对话框后，检查被选中的 choice 是否应该触发商店。但最简单的实施方案是：

在 Ivan 的对话脚本中，将 "看看商品" 选项的 `set_flag` 设置为一个特殊标记 `"_open_shop"`。然后在 `update()` 的 Dialog 状态中，当对话关闭时检查这个标记：

```rust
// 在 GameState::Dialog 分支中，对话结束后：
if self.story_flags.get("_open_shop") {
    self.story_flags.clear("_open_shop");
    let npc_id = 0; // Ivan 的 NPC ID
    let (eq_items, inv_items) = shop_inventory(npc_id);
    self.state = GameState::Shop {
        npc_id,
        equipment_for_sale: eq_items,
        items_for_sale: inv_items,
        selection: 0,
        tab: 0,
        sell_selection: 0,
        message: String::new(),
        message_timer: 0.0,
    };
    return;
}
```

简化实现：在 Ivan 的对话脚本中，将 "看看商品" choice 的 `set_flag` 设为 `Some("_open_shop")`。然后在 `update()` 的 Dialog 分支中，对话结束时检查该标记并跳转到商店状态即可。

**Ivan 脚本修改**：在 ivan 的 `pages[0].choices` 中找到 "看看商品"（如果不存在则添加）：

```rust
DialogueChoice {
    label: "看看商品",
    target_page: 0,  // 不跳转页面
    require_flag: None,
    require_affinity: None,
    set_flag: Some("_open_shop"),
},
```

---

## Phase 8.2：旅馆/恢复系统（P0，~100 行）

### 8.2.1 新增 Inn 状态

**文件**: `src/engine/game_state.rs`

```rust
/// 旅馆/恢复界面
Inn {
    /// 住宿费用
    cost: u32,
    /// 恢复动画计时器
    timer: f32,
    /// 是否已完成恢复
    restored: bool,
}
```

### 8.2.2 旅馆触发方式

在 Vale 村地图上，将一个房间 tile 替换为特殊 tile（在 `src/map/mod.rs` 的 TileKind 枚举中新增 `Inn` 变体）。或者更简单地，通过 NPC 对话触发。

**NPC 方案（推荐）**：在 NPC_SCRIPTS 中新增一个旅馆老板娘 NPC，对话带有选项 "住宿休息"（需支付金币）。

```rust
// 在 NPC_SCRIPTS 数组中追加
("innkeeper", &DialogueScript {
    pages: &[
        DialoguePage {
            lines: &[DialogueLine {
                text: "欢迎来到旅馆！一晚10金币，要休息一下吗？",
                actions: &[DialogueAction::SetFlag("met_innkeeper")],
            }],
            choices: &[
                DialogueChoice { label: "住宿 (10G)", target_page: 0, require_flag: None, require_affinity: None, set_flag: Some("_rest_at_inn") },
                DialogueChoice { label: "不用了", target_page: 1, require_flag: None, require_affinity: None, set_flag: None },
            ],
        },
        DialoguePage {
            lines: &[DialogueLine { text: "好的，注意安全！", actions: &[] }],
            choices: &[],
        },
    ],
    start_flag: Some("talked_to_innkeeper"),
})
```

在 Vale 村地图 `src/map/tilemap.rs` 的 Vale NPC 布局位置，在合适位置新增一个 Innkeeper NPC (使用新 ID，如 npc_id=5)。

### 8.2.3 旅馆逻辑 & UI

**文件**: `src/game/update.rs`

在 `update()` 函数中，检查对话关闭时的 `_rest_at_inn` flag（与商店类似方式）：

```rust
// 在 Dialog 对话结束后，检查旅馆触发
if self.story_flags.get("_rest_at_inn") {
    self.story_flags.clear("_rest_at_inn");
    let cost = 10;
    if self.gold >= cost {
        self.gold -= cost;
        self.state = GameState::Inn { cost, timer: 0.0, restored: false };
    }
    // 金币不足时显示提示消息 — 通过对话完成
    return;
}
```

**旅馆更新逻辑**：

```rust
GameState::Inn { cost: _, ref mut timer, restored } => {
    if !*restored {
        *timer += self.time.delta;
        // 0.5 秒时恢复 HP/PP
        if *timer >= 0.5 && !*restored {
            *restored = true;
            self.player_stats.hp = self.player_stats.max_hp;
            self.pp = self.max_pp;
            self.play_sfx("heal");
        }
        // 2 秒后自动回到 WorldMap
        if *timer >= 2.0 {
            self.state = GameState::WorldMap;
        }
    }
}
```

**旅馆 UI 绘制** (`src/game/draw.rs`)：

```rust
fn draw_inn(&self, inn: &InnState) {
    let screen_w = RENDER_TARGET_W as f32;
    let screen_h = RENDER_TARGET_H as f32;

    draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0, 0.0, 0.0, 0.6));

    if inn.restored {
        // 恢复完成
        let text = "休息了一晚…\nHP 和 PP 完全恢复了！";
        // 居中显示（分行处理）
        draw_text("休息了一晚…", screen_w / 2.0 - 70.0, screen_h / 2.0 - 20.0, 16.0, WHITE);
        draw_text("HP 和 PP 完全恢复了！", screen_w / 2.0 - 90.0, screen_h / 2.0, 16.0, GREEN);
    } else {
        // 恢复动画
        let progress = (inn.timer / 0.5).min(1.0);
        let alpha = progress;
        draw_text("住宿中…", screen_w / 2.0 - 40.0, screen_h / 2.0, 16.0,
            Color::new(1.0, 1.0, 1.0, alpha));
    }

    draw_text(&format!("- {}G", inn.cost), screen_w / 2.0 - 20.0, screen_h / 2.0 + 30.0, 12.0, GOLD);
}
```

---

## Phase 8.3：过场引擎与开场动画（P1，~250 行）

### 8.3.1 新增 Cutscene 状态

**文件**: `src/engine/game_state.rs`

```rust
/// 过场动画 — 脚本化的叙事序列
Cutscene {
    /// 过场 ID
    id: &'static str,
    /// 已执行的步骤索引
    step: usize,
    /// 总步骤数
    total_steps: usize,
    /// 当前步骤的内部计时器
    timer: f32,
}
```

### 8.3.2 过场脚本数据结构

**新文件**: `src/data/cutscene.rs`

```rust
//! 过场动画系统 — 脚本化叙事序列

/// 过场动画指令
#[derive(Debug, Clone)]
pub enum CutsceneCmd {
    /// 设置剧情 flag
    SetFlag(&'static str),
    /// 显示对话框文字（自动推进，无需玩家按 A）
    AutoDialog(&'static str),
    /// 等待玩家按 A 继续
    WaitForConfirm,
    /// 等待指定秒数
    Wait(f32),
    /// 全屏闪光
    Flash(f32),  // 持续时间
    /// 渐黑
    FadeToBlack(f32),
    /// 从渐黑恢复
    FadeFromBlack(f32),
    /// 移动 NPC（npc_id, target_x, target_y, speed）
    MoveNpc(usize, f32, f32, f32),
    /// 设置摄像机位置
    SetCamera(f32, f32),
    /// 触发战斗
    StartBattle,
    /// 切换场景
    SwitchScene(SceneId),
    /// 播放音效
    PlaySfx(&'static str),
    /// 播放 BGM
    PlayBgm(&'static str),
    /// 给玩家道具
    GiveItem(ItemType, u32),
    /// 触发升级
    LevelUp(u32),
}

/// 过场动画定义
#[derive(Debug, Clone)]
pub struct Cutscene {
    pub id: &'static str,
    pub commands: &'static [CutsceneCmd],
}

/// 所有过场动画
pub fn all_cutscenes() -> &'static [Cutscene] {
    &[
        // 开场动画
        &OPENING_CUTSCENE,
        // Sol Sanctum Boss 战后
        &AFTER_SANCTUM,
        // 离开 Vale 场景
        &LEAVE_VALE,
    ]
}
```

**开场动画 — 游戏开头的故事叙述**：

```rust
pub const OPENING_CUTSCENE: Cutscene = Cutscene {
    id: "opening",
    commands: &[
        CutsceneCmd::FadeToBlack(1.0),
        CutsceneCmd::AutoDialog("远古时代，炼金术掌控着世界的力量。"),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::AutoDialog("四种元素 — 地、水、火、风 — 被引导者操控，\n他们被称为「精灵使」(Adept)。"),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::AutoDialog("但炼金术的力量太过强大，几乎毁灭了世界。"),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::AutoDialog("为了拯救世界，古代的贤者们将炼金术封印在\n圣祭坛(Sol Sanctum)深处…"),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::AutoDialog("…并创造 Elemental Stars 作为钥匙，\n将这股力量永远封锁。"),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::AutoDialog("如今，数百年过去了…\nVale 村——一个被群山环抱的宁静村庄——"),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::AutoDialog("村民过着与世隔绝的生活，\n古老的传说渐渐被遗忘…"),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::AutoDialog("但封印正在减弱，\n山上的怪物变得越来越多…"),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::FadeFromBlack(1.0),
        CutsceneCmd::SetFlag("opening_seen"),
    ],
};
```

**Sol Sanctum Boss 战后**：

```rust
pub const AFTER_SANCTUM: Cutscene = Cutscene {
    id: "after_sanctum",
    commands: &[
        CutsceneCmd::AutoDialog("石像崩塌了，Elemental Star 落在你手中。"),
        CutsceneCmd::AutoDialog("你能感受到它蕴含的无穷力量。"),
        CutsceneCmd::AutoDialog("远处传来古老的声音：\n「Elemental Star 的守护者已被击败…」"),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::AutoDialog("「…但真正的旅程才刚刚开始。\n寻找其他的 Elemental Stars，\n否则世界的平衡将永远被打破。」"),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::SetFlag("completed_sol_sanctum"),
    ],
};
```

**离开 Vale**：

```rust
pub const LEAVE_VALE: Cutscene = Cutscene {
    id: "leave_vale",
    commands: &[
        CutsceneCmd::AutoDialog("Isaac 和 Garet 站在村口，\n回望他们长大的 Vale 村。"),
        CutsceneCmd::AutoDialog("Garet: 「准备好了吗，Isaac？\n前方的世界在等着我们。」"),
        CutsceneCmd::Wait(1.0),
        CutsceneCmd::AutoDialog("带着 Elemental Star 和长老的嘱托，\n两位年轻的 Adept 踏上了旅程…"),
        CutsceneCmd::Wait(1.5),
        CutsceneCmd::SetFlag("left_vale"),
    ],
};
```

### 8.3.3 过场驱动逻辑

**文件**: `src/game/update.rs`

```rust
// 在主 update 的 match 中添加
GameState::Cutscene { id, ref mut step, total_steps, ref mut timer } => {
    *timer += self.time.delta;
    let all = all_cutscenes();
    if let Some(cutscene) = all.iter().find(|c| c.id == id) {
        if *step < cutscene.commands.len() {
            // 执行当前步骤
            self.execute_cutscene_step(&cutscene.commands[*step], step, timer, total_steps);
        } else {
            // 所有步骤完成，返回 WorldMap
            self.state = GameState::WorldMap;
        }
    } else {
        self.state = GameState::WorldMap;
    }
}

fn execute_cutscene_step(&mut self, cmd: &CutsceneCmd, step: &mut usize, timer: &mut f32, _total: usize) {
    match cmd {
        CutsceneCmd::SetFlag(flag) => {
            self.story_flags.set(flag);
            *step += 1;
            *timer = 0.0;
        }
        CutsceneCmd::AutoDialog(text) => {
            // 显示自动对话框
            if !self.dialogue.is_some() {
                self.dialogue = Some(DialogueState::new_full(text.to_string(), true));
                // new_full: 新增的构造，带 auto_close 标记
            }
            if let Some(ref d) = self.dialogue {
                if d.is_finished() && *timer > 0.5 {
                    // 自动关闭并推进到下一步
                    self.dialogue = None;
                    *step += 1;
                    *timer = 0.0;
                }
            }
        }
        CutsceneCmd::WaitForConfirm => {
            if self.input_bus.consume(InputEvent::Confirm) {
                *step += 1;
                *timer = 0.0;
            }
        }
        CutsceneCmd::Wait(duration) => {
            if *timer >= *duration {
                *step += 1;
                *timer = 0.0;
            }
        }
        CutsceneCmd::Flash(duration) => {
            self.state = GameState::Transition {
                kind: TransitionKind::Flash,
                timer: *timer,
                from: "",
                to: "",
            };
            if *timer >= *duration {
                *step += 1;
                *timer = 0.0;
            }
        }
        CutsceneCmd::FadeToBlack(duration) => {
            self.state = GameState::Transition {
                kind: TransitionKind::FadeOut,
                timer: *timer,
                from: "",
                to: "",
            };
            if *timer >= *duration {
                *step += 1;
                *timer = 0.0;
            }
        }
        CutsceneCmd::FadeFromBlack(duration) => {
            self.state = GameState::Transition {
                kind: TransitionKind::FadeIn,
                timer: *timer,
                from: "",
                to: "",
            };
            if *timer >= *duration {
                *step += 1;
                *timer = 0.0;
            }
        }
        CutsceneCmd::PlaySfx(name) => {
            self.play_sfx(name);
            *step += 1;
            *timer = 0.0;
        }
        CutsceneCmd::SwitchScene(target) => {
            self.request_scene_switch(*target);
            *step += 1;
            *timer = 0.0;
        }
        _ => {
            *step += 1;
            *timer = 0.0;
        }
    }
}
```

### 8.3.4 DialogueState 支持自动关闭

**文件**: `src/dialogue/mod.rs`

为 DialogueState 增加 `auto_close` 字段：

```rust
pub struct DialogueState {
    // ... 现有字段 ...
    pub auto_close: bool,  // true = 过场用自动推进对话框，无需按 A
}

impl DialogueState {
    pub fn new(text: String) -> Self {
        Self { text, ..Self::default() }
    }

    pub fn new_full(text: String, auto_close: bool) -> Self {
        Self { text, auto_close, ..Self::default() }
    }
}
```

`advance()` 方法中：当 `auto_close = true` 且 `is_finished()` 时，不再阻塞等待 A 键——由外部 cutscene 逻辑控制时间。

### 8.3.5 开场触发

**文件**: `src/game/update.rs`

在 `Title` 状态按 Start 进入游戏时，检查是否已看过开场：

```rust
// Title → 按 Start
GameState::Title => {
    if self.input_bus.consume(InputEvent::Confirm) {
        if !self.story_flags.get("opening_seen") {
            // 播放开场过场
            self.state = GameState::Cutscene {
                id: "opening",
                step: 0,
                total_steps: OPENING_CUTSCENE.commands.len() + 1,
                timer: 0.0,
            };
        } else {
            // 直接进入游戏
            self.start_new_game();
        }
    }
}
```

并在 `memory/mod.rs` 的存档中持久化 `story_flags`（当前故事 flag 未存档）。

### 8.3.6 过场 UI 绘制

在 `draw()` 函数中注册 `Cutscene`：

```rust
GameState::Cutscene { .. } => {
    self.draw_world_map();  // 如果有世界地图背景
    // 过场中对话框由 DialogueState 自动绘制
    // cutscene 特有的全屏效果（渐黑/闪光等）由 Transition 绘制
}
```

在过场期间的 update 中，需要调用 `self.dialogue` 的 `advance()` 进行打字机推进。

---

## Phase 8.4：场景名称弹窗（P1，~50 行）

### 8.4.1 场景切换时显示地名

**文件**: `src/game/update.rs`

在 `apply_scene_switch()` 末尾，设置场景名称弹窗：

```rust
pub fn apply_scene_switch(&mut self) {
    if let Some(target) = self.pending_scene.take() {
        self.scene.commit_switch();
        self.npcs = create_npcs_for_scene(target);
        self.modified_tiles_current.clear();
        self.modified_tiles.clear();

        // 触发场景名称弹窗
        self.state = GameState::SceneName {
            name: target.display_name(),
            timer: 0.0,
        };

        #[cfg(debug_assertions)]
        eprintln!("场景切换至: {target:?}");
    }
}
```

### 8.4.2 新状态

**文件**: `src/engine/game_state.rs`

```rust
/// 场景名称弹窗 — 进入新场景时显示
SceneName {
    name: &'static str,
    timer: f32,
}
```

### 8.4.3 更新逻辑 & UI

**update**：`SceneName` 状态中 timer 累加，1.5 秒后回到 WorldMap。

**draw**：在 `draw()` 函数中注册：

```rust
GameState::SceneName { name, timer } => {
    self.draw_world_map();
    let alpha = (1.0 - (timer / 1.5).min(1.0)).max(0.0); // 渐隐
    if alpha > 0.0 {
        let color = Color::new(1.0, 1.0, 0.0, alpha); // 金色
        draw_text(name, RENDER_TARGET_W as f32 / 2.0 - 60.0, 100.0, 24.0, color);
    }
}
```

### 8.4.4 SceneId 新增 display_name()

**文件**: `src/scene/mod.rs`

```rust
impl SceneId {
    pub fn display_name(&self) -> &'static str {
        match self {
            SceneId::Title => "标题画面",
            SceneId::Vale => "Vale 村",
            SceneId::WildForest => "密林",
            SceneId::Cave => "洞穴",
            SceneId::SolSanctum => "Sol Sanctum",
        }
    }
}
```

---

## Phase 8.5：战斗增强（P1，~200 行）

### 8.5.1 战斗背景根据场景变化

**文件**: `src/game/draw.rs`

在 `draw_battle()` 中，根据当前场景绘制不同背景：

```rust
fn draw_battle_bg(scene: SceneId) {
    let (r, g, b) = match scene {
        SceneId::Vale => (0.1, 0.3, 0.1),         // 森林绿
        SceneId::WildForest => (0.05, 0.2, 0.05), // 深林
        SceneId::Cave => (0.1, 0.1, 0.1),         // 洞穴黑
        SceneId::SolSanctum => (0.2, 0.1, 0.3),   // 紫色神圣
        _ => (0.1, 0.1, 0.2),
    };

    // 背景渐变
    for y in 0..=RENDER_TARGET_H {
        let t = y as f32 / RENDER_TARGET_H as f32;
        let alpha = t * 0.3 + 0.1;  // 从下到上略微变亮
        draw_line(0.0, y as f32, RENDER_TARGET_W as f32, y as f32, 1.0,
            Color::new(r + alpha, g + alpha, b + alpha, 1.0));
    }

    // 地面线
    let ground_y = RENDER_TARGET_H as f32 * 0.6;
    draw_line(0.0, ground_y, RENDER_TARGET_W as f32, ground_y, 2.0, Color::new(0.3, 0.3, 0.3, 1.0));
}
```

### 8.5.2 战斗中使用道具

在战斗菜单中新增 "Item"（道具）选项：

**文件**: `src/game/draw.rs` — 在 `draw_battle()` 的战斗菜单列表中添加：

```rust
// 增加 Item 和 Summon 的入口
const BATTLE_ACTIONS: &[&str] = &[
    "Attack", "Defend", "Psynergy", "Summon", "Djinn", "Item", "Flee"
];

// 战斗动作选择索引
// 在 update 的 Battle::PlayerInput 分支中，允许左右选择
```

**文件**: `src/game/update.rs` — 在 `Battle::PlayerInput` 分支中添加道具选择子状态：

```rust
// 需要新增战斗子状态表用记：battle_menu_selection: usize
// 在 GameCtx 中新增字段 battle_menu_selection，初始化为 0
// PlayerInput 阶段：左右切换菜单项，确认选择

// 简化方案（不使用子菜单）：按 Left/Right 切换战斗行动
// 当前战斗菜单仅通过 Confirm/Cancel/Secondary 控制三个选项
// 改为：按 Left/Right 在 BATTLE_ACTIONS 中循环
```

**更简单的实现**：保持当前三个快捷键（A=攻击, B=防御, X=精灵力/特殊），新增：

```rust
// 在 Battle::PlayerInput 分支中：
// 当 player_action 为 None 时，检测 Menu 键打开战斗菜单选择
if self.input_bus.consume(InputEvent::Menu) {
    self.battle_menu_selection = 0;
    self.state = GameState::BattleMenu { selection: 0 };
}
```

**战斗菜单状态**（新增）：

```rust
/// 战斗动作选择子菜单
BattleMenu { selection: usize },
```

在 `update()` 中处理 `BattleMenu` 状态，按方向键选择，A 确认执行对应 `BattleAction`。

**道具使用逻辑**：选中 Item 后，列出玩家的恢复道具列表。选择 Potion → 使用后 HP 恢复 30；选择 Ether → PP 恢复 10。

### 8.5.3 战斗后 EXP 面板增强

**文件**: `src/game/draw.rs` — 在胜利画面添加更多信息：

```rust
// 在 Victory 面板中增加：
// - 每个 party 成员获得的 EXP（当前只有总计）
// - 掉落的金币显示
// - 按等级/经验显示升级进度条
// - 每个战斗用 Djinn 的状态变化
```

### 8.5.4 属性增强时的战斗评级星级

```rust
/// 根据战斗表现计算评级
fn calculate_battle_grade(battle: &Battle) -> &'static str {
    let turns = battle.turn_index.max(1);
    let total_damage: u32 = battle.results.iter().map(|r| r.damage).sum();
    let avg_damage = total_damage / turns;

    if turns <= 2 && avg_damage > 20 { "★★★" }
    else if turns <= 4 && avg_damage > 10 { "★★" }
    else { "★" }
}
```

---

## Phase 8.6：Djinn 收集增强（P2，~150 行）

### 8.6.1 更多 Djinn 地图放置

**文件**: `src/data/djinn.rs`

当前 `world_djinn()` 只返回 3 个 Djinn，扩展到 8 个：

```rust
pub fn world_djinn() -> &'static [(DjinnId, &'static str, f32, f32)] {
    &[
        (DjinnId::Fafnir,  "Vale",        8.0,  11.0),
        (DjinnId::Belian,  "Vale",        24.0, 20.0),
        (DjinnId::Gnome,   "WildForest",  5.0,  5.0),
        (DjinnId::Vermin,  "WildForest",  15.0, 10.0),
        (DjinnId::Malina,  "Cave",        10.0, 12.0),
        (DjinnId::Betsy,   "Cave",        3.0,  3.0),
        (DjinnId::Laguna,  "SolSanctum",  4.0,  3.0),
        (DjinnId::Undine,  "SolSanctum",  12.0, 10.0),
    ]
}
```

### 8.6.2 Djinn 收集视觉效果

**文件**: `src/game/draw.rs` — 在 `draw_world_map()` 的 HUD 层之上绘制 Djinn 光点：

```rust
fn draw_djinn_pickups(&self) {
    let px = self.camera.x;
    let py = self.camera.y;

    for (djinn_id, scene_name, tx, ty) in djinn::world_djinn() {
        let current_name = match self.scene.current() {
            SceneId::Vale => "Vale",
            SceneId::WildForest => "WildForest",
            SceneId::Cave => "Cave",
            SceneId::SolSanctum => "SolSanctum",
            _ => "Vale",
        };
        if scene_name != current_name { continue; }
        if self.collected_djinn.iter().any(|d| d.djinn.id == *djinn_id) { continue; }

        // 将世界坐标转换为屏幕坐标
        let sx = RENDER_TARGET_W as f32 / 2.0;
        let sy = RENDER_TARGET_H as f32 * 0.5;

        // 在 3D 空间中的近似位置 — 使用简单距离判断
        let dist_sq = (px - tx).powi(2) + (py - ty).powi(2);
        if dist_sq < 4.0 { // 在附近约 2 tile 内
            let blink = (self.game_time * 4.0).sin().abs() * 0.5 + 0.5;
            // 在地面上绘制跳动光点
            let djinn_color = match djinn_id.element() {
                Element::Venus => GREEN,
                Element::Mercury => BLUE,
                Element::Mars => RED,
                Element::Jupiter => Color::new(0.5, 0.0, 1.0, 1.0),
            };
            draw_circle(sx, sy - 40.0 - blink * 5.0, 4.0, Color::new(
                djinn_color.r, djinn_color.g, djinn_color.b, blink * 0.8
            ));
        }
    }
}
```

### 8.6.3 Djinn 收集动画

在 `collect_djinn()` 调用时，显示收集反馈：

```rust
// 在 update() 的 check_djinn_pickup() 中收集成功后：
if collected {
    // 弹出 Djinn 名称
    self.state = GameState::DjinnObtained {
        djinn_id,
        timer: 0.0,
    };
}

// 在 draw() 中 DjinnObtained 状态显示：
// 居中大字显示 "获得 Djinn: Fafnir！"
// 元素颜色作为文字色
// 2 秒后自动关闭
```

---

## Phase 8.7：音频扩展（P2，~100 行）

### 8.7.1 新增音效

**文件**: `src/audio/mod.rs`

在 `SfxManager::new()` 中增加以下音效生成：

```rust
// 新增音效（在 existing 5 种之后追加）
sounds.insert("levelup",
    load_audio(generate_sfx_sound(880.0, 0.3, 0.5)).await);  // 升级
sounds.insert("shop_buy",
    load_audio(generate_sfx_sound(660.0, 0.15, 0.4)).await);  // 购买
sounds.insert("victory",
    load_audio(generate_sfx_sound(440.0, 0.5, 0.6)).await);   // 胜利
sounds.insert("djinn",
    load_audio(generate_sfx_sound(550.0, 0.25, 0.5)).await);  // Djinn
sounds.insert("summon",
    load_audio(generate_sfx_sound(330.0, 0.5, 0.7)).await);   // 召唤
```

### 8.7.2 Boss 战斗 BGM

在 BgmPlayer 中新增 Boss 主题：

```rust
// 在 BgmPlayer::new() 中：
let boss_pcm = generate_boss_bgm();
sounds.insert("boss", load_audio(boss_pcm).await);

// generate_boss_bgm() 函数：
fn generate_boss_bgm() -> AudioSample {
    let sample_rate = 44100;
    let duration = 8.0; // 8 秒循环
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut data = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        // 低音延续 + 高音交替 = 威胁感
        let bass = (t * 110.0 * std::f32::consts::TAU).sin() * 0.4;
        let alt = (t * 220.0 * std::f32::consts::TAU).sin() * 0.3;
        let staccato = if (t * 4.0).fract() < 0.3 { 1.0 } else { 0.0 };
        let sample = (bass + alt * staccato) * 0.5;
        data.push(sample);
    }

    AudioSample { data, sample_rate }
}
```

### 8.7.3 场景 BGM 自动切换

在 `apply_scene_switch()` 中，根据新场景自动播放对应 BGM：

```rust
fn apply_scene_switch(&mut self) {
    // ... 现有场景切换逻辑 ...

    // 自动切换 BGM
    match target {
        SceneId::Vale => self.play_bgm("vale"),
        SceneId::WildForest => self.play_bgm("battle"), // 暂用战斗主题
        SceneId::Cave => self.play_bgm("battle"),
        SceneId::SolSanctum => self.play_bgm("boss"),
        _ => {}
    }
}
```

---

## Phase 8.8：故事补完（P2，~150 行）

### 8.8.1 主线对话增强

**文件**: `src/dialogue/script.rs`

在 Garsmin 的对话末尾新增"告别"页（当前 7 页后追加）：

```
Page 8 (新增): require "garsmin_complete_1"
  "拿到 Elemental Star 后，
   世界上的其他封印也会开始松动。
   去吧，未来的路还很长。"
```

在 Garet 的对话末尾新增出发页（当前 3 页后追加）：

```
Page 4: require "garsmin_farewell"
  "长老说我们应该先去 Bilibin 镇。
   据说那里最近有神秘的事件发生。
   现在就出发吧！"
```

### 8.8.2 完整任务链

**文件**: `src/data/quest.rs`

替换 `default_quests()` 为连贯的主线任务链：

```rust
pub fn default_quests() -> Vec<QuestEntry> {
    vec![
        QuestEntry::new("talk_to_villagers", "与村民交谈",
            "和 Vale 村的 Ivan、Mia、Garsmin 打招呼"),
        QuestEntry::new("learn_psynergy", "学会精灵力",
            "使用一次精灵力，感受 Adept 的力量"),
        QuestEntry::new("explore_sanctum", "探索 Sol Sanctum",
            "前往村后山上的 Sol Sanctum"),
        QuestEntry::new("defeat_guardian", "击败守护者",
            "在 Sol Sanctum 深处击败 MythrilGolem"),
        QuestEntry::new("leave_vale", "踏上旅程",
            "带上 Elemental Star，和 Garet 一起出发"),
        QuestEntry::new("collect_djinn", "收集精灵",
            "在旅途中找到并收集散落的 Djinn"),
        QuestEntry::new("collect_all_psynergy", "精灵力大师",
            "解锁全部 7 种精灵力"),
        QuestEntry::new("become_adept", "成为真正的 Adept",
            "收集 5 个 Djinn，激活高级职业"),
    ]
}
```

### 8.8.3 任务自动推进

**文件**: `src/game/update.rs` — 在 `update_story_progression()` 中补全：

```rust
fn update_story_progression(&mut self) {
    // 与所有村民对话 → 完成 talk_to_villagers
    if self.story_flags.get("met_ivan") && self.story_flags.get("met_mia")
        && self.story_flags.get("met_garsmin")
        && !self.story_flags.get("villagers_all_met")
    {
        self.story_flags.set("villagers_all_met");
        self.quest_log.complete("talk_to_villagers");
        self.quest_log.unlock("learn_psynergy");
    }

    // 使用精灵力 → 完成 learn_psynergy
    if self.unlocked_count > 0 && self.story_flags.get("villagers_all_met")
        && !self.story_flags.get("psynergy_used")
    {
        // 在执行精灵力效果后设置
    }

    // Garet 就绪 → 解锁 explore_sanctum
    if self.story_flags.get("met_garet")
        && self.story_flags.get("garsmin_sent_to_sanctum")
        && !self.story_flags.get("party_ready")
    {
        self.story_flags.set("party_ready");
        self.quest_log.complete("learn_psynergy");
        self.quest_log.unlock("explore_sanctum");
    }

    // Sol Sanctum 完成 → 解锁 leave_vale
    if self.story_flags.get("completed_sol_sanctum")
        && !self.story_flags.get("sanctum_complete_logged")
    {
        self.story_flags.set("sanctum_complete_logged");
        self.quest_log.complete("defeat_guardian");
        self.quest_log.unlock("leave_vale");
    }

    // 进入 WildForest → 离开 Vale
    if self.scene.current() != SceneId::Vale
        && !self.story_flags.get("left_vale_logged")
    {
        self.story_flags.set("left_vale_logged");
        self.quest_log.complete("leave_vale");
        self.quest_log.unlock("collect_djinn");
    }

    // 5+ Djinn → 完成 become_adept
    if self.collected_djinn.len() >= 5 && !self.story_flags.get("became_adept") {
        self.story_flags.set("became_adept");
        self.quest_log.complete("become_adept");
    }

    // 7 种精灵力 → 完成 collect_all_psynergy
    if self.unlocked_count >= 7 && !self.story_flags.get("all_psynergy_unlocked") {
        self.story_flags.set("all_psynergy_unlocked");
        self.quest_log.complete("collect_all_psynergy");
    }
}
```

---

## Phase 8.9：最终验证（必做）

### 执行顺序

1. **按 Phase 8.1 → 8.2 → 8.3 → 8.4 → 8.5 → 8.6 → 8.7 → 8.8 顺序逐个实现**
2. 每个子 Phase 完成后可以 `cargo check` 验证编译，但不要修改后续 Phase 的前置依赖
3. **不要跳过任何步骤** — 每个 Phase 都彼此独立

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
# - [ ] 标题画面按 Start 播放开场过场（首次启动）
# - [ ] 开场动画文字自动推进
# - [ ] 开场后进入 Vale 村，场景名称 "Vale 村" 金色弹窗显示
# - [ ] 与 Ivan 对话有 "看看商品" 选项
# - [ ] 选择 "看看商品" 进入商店界面
# - [ ] 商店可购买装备/道具，金币不足时提示
# - [ ] 商店可出售物品
# - [ ] 旅馆老板娘对话可选 "住宿 (10G)"
# - [ ] 住宿后 HP/PP 全恢复，金币-10
# - [ ] Sol Sanctum 中心触发 Boss 战，Boss 有独特 BGM
# - [ ] 战斗胜利显示评级 + 战利品面板
# - [ ] 战斗中可使用 Item 恢复 HP/PP
# - [ ] Djinn 在 Vale 地图上有光点闪烁
# - [ ] 收集 Djinn 时弹出名称动画
# - [ ] 不同场景进入时自动切换 BGM
# - [ ] 任务链按进度自动推进
# - [ ] 存档功能正常（包含所有新状态）

# 5. 测试全部通过
cargo test --test '*'
```

### 提交

```bash
# 确认一切正常后：
git add -A
git commit -m "Phase 8: 商店/旅馆/过场引擎/战斗增强/Djinn收集/音频扩展/故事补完"

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
- 所有 `#[non_exhaustive]` 枚举（GameState, SceneId, DialogueAction）新增变体后必须更新所有 match

### SceneId 变更警告
在 `SceneId` 中新增变体后，必须遍历以下全部位置确保处理：

| 文件 | 匹配位置 |
|------|----------|
| `src/scene/mod.rs` | `display_name()`, `SceneRegistry::new()` |
| `src/map/tilemap.rs` | `get_scene_map()`, `map_size()` |
| `src/game/mod.rs` | `start_random_battle()`, `save_game()`, `check_djinn_pickup()` |
| `src/game/update.rs` | `update_world_map()`, `check_scene_triggers()`, 边界检查 |
| `src/game/draw.rs` | `draw_hud()` 中的地点名 |
| `src/entity/mod.rs` | `create_npcs_for_scene()` |
| `src/data/mod.rs` | `enemies_for_area()` |

### GameState 变更警告
新增 `GameState` 变体后，必须在以下位置添加处理：

| 文件 | 需要处理的位置 |
|------|---------------|
| `src/game/update.rs` | `update()` 的 match |
| `src/game/draw.rs` | `draw()` 的 match |
| `src/engine/game_state.rs` | `is_transition()`, `is_psynergy_anim()` |

### 对话脚本位置
- NPC 对话定义在 `src/dialogue/script.rs` 的 `NPC_SCRIPTS` 数组中
- 系统事件脚本（以 `_` 开头）也放在同一个数组
- `get_script()` 函数通过线性查找匹配 ID
- DialogueAction 新增变体必须在 update 中处理

### 测试
- 测试文件在 `tests/` 目录
- BDD 测试：`.feature` 文件在 `tests/features/`
- 新增对话单元测试直接在 `src/dialogue/script.rs` 的 `#[cfg(test)]` 中添加
- 商店测试在 `src/game/mod.rs` 的 `#[cfg(test)]` 中添加

---

## 全部任务总览

| 阶段 | 核心任务 | 文件数 | 预估行数 | 优先级 |
|------|---------|--------|---------|--------|
| 8.1 商店系统 | 商店 UI + 购买/出售 + 对话触发 | 4 | ~200 | P0 |
| 8.2 旅馆系统 | 旅馆 UI + HP/PP 恢复 + 对话触发 | 3 | ~100 | P0 |
| 8.3 过场引擎 | Cutscene 状态机 + 开场动画 + 剧情序列 | 4 | ~250 | P1 |
| 8.4 场景名称弹窗 | SceneName 状态 + 金色弹窗 + 渐隐 | 3 | ~50 | P1 |
| 8.5 战斗增强 | 场景背景 + 道具使用 + 战斗评级 | 3 | ~200 | P1 |
| 8.6 Djinn 收集增强 | 更多地图 Djinn + 光点显示 + 收集动画 | 3 | ~150 | P2 |
| 8.7 音频扩展 | 新音效 + Boss BGM + 场景 BGM 自动切换 | 2 | ~100 | P2 |
| 8.8 故事补完 | 对话扩展 + 任务链 + 自动推进 | 2 | ~150 | P2 |
| **总计** | | | **~1,200** | |

> **Agent 执行顺序**：按 8.1 → 8.2 → 8.3 → 8.4 → 8.5 → 8.6 → 8.7 → 8.8 → 8.9 逐段实现
> 每完成一个子 Phase，用 `cargo check` 验证无编译错误。全部完成后执行完整验证。
