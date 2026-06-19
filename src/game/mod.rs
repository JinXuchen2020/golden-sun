mod update;
mod draw;

use std::collections::HashMap;

use golden_sun::battle::{Battle, Combatant};
use golden_sun::engine::{Camera, FrameTime, GameState, InputState};
use golden_sun::{InputBus, TextureCache, WindowConfig};
use golden_sun::constants::{RENDER_TARGET_W, RENDER_TARGET_H, SPRITE_SIZE, PP_INITIAL, PP_MAX};
use golden_sun::{SceneId, SceneRegistry, PsynergyType};
use golden_sun::entity::{create_npcs_for_scene, create_vale_npcs, Entity};
use golden_sun::dialogue::{DialogueState, StoryFlags};
use golden_sun::entity::sprite::{self, AnimState};
use golden_sun::map::TileKind;
use golden_sun::data::quest::QuestLog;
use golden_sun::data::djinn::{self, DjinnId, OwnedDjinn, SetBonus, Class};
use macroquad::prelude::*;

/// 道具类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemType {
    Potion,
    Ether,
    GoldRing,
}

impl ItemType {
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            ItemType::Potion => "Potion",
            ItemType::Ether => "Ether",
            ItemType::GoldRing => "Gold Ring",
        }
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            ItemType::Potion => "Restores 30 HP",
            ItemType::Ether => "Restores 10 PP",
            ItemType::GoldRing => "Increases gold drop",
        }
    }
}

/// 道具实例
#[derive(Debug, Clone)]
pub struct Item {
    pub item_type: ItemType,
    pub count: u32,
}

impl Item {
    #[must_use]
    pub fn new(item_type: ItemType) -> Self {
        Self { item_type, count: 1 }
    }
}

/// 玩家角色属性（含等级/EXP）
#[derive(Debug, Clone)]
pub struct PlayerStats {
    pub level: u32,
    pub exp: u32,
    pub exp_to_next: u32,
    pub hp: u32,
    pub max_hp: u32,
    pub attack: u32,
    pub defense: u32,
}

impl PlayerStats {
    pub fn new() -> Self {
        Self {
            level: 1,
            exp: 0,
            exp_to_next: 20,
            hp: 50,
            max_hp: 50,
            attack: 10,
            defense: 8,
        }
    }

    pub fn add_exp(&mut self, amount: u32) {
        self.exp += amount;
        while self.exp >= self.exp_to_next {
            self.exp -= self.exp_to_next;
            self.level += 1;
            self.max_hp += 8;
            self.hp = self.max_hp;
            self.attack += 2;
            self.defense += 1;
            self.exp_to_next = self.level * 20;
        }
    }
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self::new()
    }
}

const PLAYER_START_X: f32 = 15.0;
const PLAYER_START_Y: f32 = 16.0;
const ANIM_COUNT: usize = 8;
const HAT_COLOR: (u8, u8, u8) = (60, 180, 60);
const BODY_COLOR: (u8, u8, u8) = (100, 150, 200);

/// 传送点定义
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct WaypointDef {
    pub name: String,
    pub scene: SceneId,
    pub x: f32,
    pub y: f32,
}

/// 精灵图集 — 统一管理玩家和 NPC 的 GPU 纹理
struct SpriteAtlas {
    player: [Vec<Texture2D>; ANIM_COUNT],
    npc: [Vec<Texture2D>; ANIM_COUNT],
}

impl SpriteAtlas {
    fn new() -> Self {
        let mut player: [Vec<Texture2D>; ANIM_COUNT] = Default::default();
        for (state, anim) in sprite::generate_player_animations() {
            let texes: Vec<Texture2D> = anim.frames.into_iter().map(|f| {
                let img = Image { width: SPRITE_SIZE as u16, height: SPRITE_SIZE as u16, bytes: f.pixels };
                let t = Texture2D::from_image(&img);
                t.set_filter(FilterMode::Nearest);
                t
            }).collect();
            player[state.index()] = texes;
        }

        let mut npc: [Vec<Texture2D>; ANIM_COUNT] = Default::default();
        for (state, anim) in sprite::generate_npc_animations(HAT_COLOR, BODY_COLOR) {
            let texes: Vec<Texture2D> = anim.frames.into_iter().map(|f| {
                let img = Image { width: SPRITE_SIZE as u16, height: SPRITE_SIZE as u16, bytes: f.pixels };
                let t = Texture2D::from_image(&img);
                t.set_filter(FilterMode::Nearest);
                t
            }).collect();
            npc[state.index()] = texes;
        }

        Self { player, npc }
    }

    #[inline]
    fn player_tex(&self, state: AnimState) -> &[Texture2D] {
        &self.player[state.index()]
    }

    #[inline]
    fn npc_tex(&self, state: AnimState) -> &[Texture2D] {
        &self.npc[state.index()]
    }
}

pub struct GameCtx {
    config: WindowConfig,
    state: GameState,
    scene: SceneRegistry,
    camera: Camera,
    input: InputState,
    input_bus: InputBus,
    time: FrameTime,
    textures: TextureCache,
    player_entity: Entity,
    npcs: Vec<Entity>,
    sprites: SpriteAtlas,
    /// 战斗角色纹理（玩家1/2 + 敌人1/2）
    battle_sprites: [Texture2D; 4],
    // ── Phase 3: 精灵力 ──
    pp: u32,
    max_pp: u32,
    unlocked_psynergies: [PsynergyType; 7],
    unlocked_count: usize,
    selected_psynergy: usize,
    pp_recover_timer: f32,
    /// 运行时 tile 覆盖（精灵力修改的地图格）
    modified_tiles: HashMap<(i32, i32), TileKind>,
    /// 待执行的精灵力效果（psynergy, tx, ty），动画结束后应用
    pending_psynergy: Option<(PsynergyType, i32, i32)>,
    // ── Phase 4: 对话 ──
    dialogue: Option<DialogueState>,
    story_flags: StoryFlags,
    // ── Phase 5: 战斗 ──
    battle: Option<Battle>,
    // ── Phase 6: 菜单 ──
    menu_selection: usize,
    menu_page: usize,  // 0=main, 1=items, 2=psynergy, 3=status
    /// 金币
    gold: u32,
    /// 玩家属性（等级/EXP/HP等）
    player_stats: PlayerStats,
    /// 道具栏
    inventory: Vec<Item>,
    /// 随机遇敌步数计数器（每步递减）
    encounter_step: u32,
    /// 遇敌待处理标志（过渡完成后进入战斗）
    encounter_pending: bool,
    /// StorageBackend（存档用）
    _storage: Box<dyn golden_sun::engine::storage::StorageBackend>,
    /// SFX 管理器（菜单音效）
    sfx_manager: Option<golden_sun::audio::SfxManager>,
    /// 当前场景的 modified tiles 覆盖
    modified_tiles_current: std::collections::HashMap<(i32, i32), TileKind>,
    /// 场景切换目标
    pending_scene: Option<golden_sun::SceneId>,
    /// 已激活的传送点
    activated_waypoints: Vec<WaypointDef>,
    /// 游戏内时间（秒），用于昼夜循环
    game_time: f32,
    /// 天气粒子系统
    particles: golden_sun::engine::particle::ParticleSystem,
    /// 任务日志
    quest_log: QuestLog,
    /// NPC 好感度 (npc_id → affinity)
    affinity: HashMap<u32, i32>,
    /// 当前对话选择的索引
    dialogue_choice_selection: usize,
    /// 当前对话脚本（用于分支选择）
    current_dialogue_script: Option<golden_sun::dialogue::script::DialogueScript>,
    // ── Phase 6.9: Djinn 精灵系统 ──
    /// 已收集的 Djinn
    collected_djinn: Vec<OwnedDjinn>,
    /// 当前职业
    current_class: Class,
    /// Djinn 菜单页面 (0=main, 1=equip, 2=release)
    djinn_menu_page: usize,
    /// Djinn 菜单选中索引
    djinn_menu_selection: usize,
    /// 角色选择 (0=Isaac, 1=Garet)
    djinn_character_select: usize,
}

impl GameCtx {
    #[must_use]
    pub async fn new() -> Self {
        Self {
            config: WindowConfig::default(),
            state: GameState::Title,
            scene: SceneRegistry::new(SceneId::Title),
            camera: Camera::new(PLAYER_START_X, PLAYER_START_Y),
            input: InputState::new(),
            input_bus: InputBus::new(),
            time: FrameTime::new(),
            textures: TextureCache::new(RENDER_TARGET_W, RENDER_TARGET_H),
            player_entity: Entity::new_player(Entity::tile_to_world(PLAYER_START_X, PLAYER_START_Y)),
            npcs: create_vale_npcs(),
            sprites: SpriteAtlas::new(),
            battle_sprites: {
                let frames = sprite::generate_battle_sprites();
                let mut sprites: [Texture2D; 4] = [
                    Texture2D::empty(), Texture2D::empty(),
                    Texture2D::empty(), Texture2D::empty(),
                ];
                for (i, frame_list) in frames.into_iter().enumerate() {
                    if !frame_list.is_empty() {
                        let img = Image {
                            width: SPRITE_SIZE as u16,
                            height: SPRITE_SIZE as u16,
                            bytes: frame_list[0].pixels.clone(),
                        };
                        sprites[i] = Texture2D::from_image(&img);
                        sprites[i].set_filter(FilterMode::Nearest);
                    }
                }
                sprites
            },
            pp: PP_INITIAL,
            max_pp: PP_MAX,
            unlocked_psynergies: PsynergyType::ALL,
            unlocked_count: 4,
            selected_psynergy: 0,
            pp_recover_timer: 0.0,
            modified_tiles: HashMap::new(),
            pending_psynergy: None,
            dialogue: None,
            story_flags: StoryFlags::new(),
            battle: None,
            menu_selection: 0,
            menu_page: 0,
            gold: 0,
            player_stats: PlayerStats::new(),
            inventory: vec![
                Item::new(ItemType::Potion),
                Item::new(ItemType::Potion),
                Item::new(ItemType::Ether),
            ],
            encounter_step: 0,
            encounter_pending: false,
            _storage: golden_sun::engine::storage::create_storage(),
            sfx_manager: Some(golden_sun::audio::SfxManager::new().await),
            modified_tiles_current: HashMap::new(),
            pending_scene: None,
            activated_waypoints: vec![
                WaypointDef { name: "Vale 村口".into(), scene: SceneId::Vale, x: PLAYER_START_X, y: PLAYER_START_Y },
            ],
            game_time: 0.0,
            particles: golden_sun::engine::particle::ParticleSystem::new(200),
            quest_log: {
                let mut ql = QuestLog::new();
                for q in golden_sun::data::quest::default_quests() {
                    ql.add(q);
                }
                ql
            },
            affinity: HashMap::new(),
            dialogue_choice_selection: 0,
            current_dialogue_script: None,
            // ── Phase 6.9: Djinn 初始化 ──
            collected_djinn: Vec::new(),
            current_class: Class::Adept,
            djinn_menu_page: 0,
            djinn_menu_selection: 0,
            djinn_character_select: 0,
        }
    }

    /// 启动随机遭遇战
    pub fn start_random_battle(&mut self) {
        let party = vec![
            Combatant::new(1, "Isaac", self.player_stats.level, golden_sun::Element::Venus, true),
            Combatant::new(2, "Garet", self.player_stats.level, golden_sun::Element::Mars, true),
        ];

        let scene_name = match self.scene.current() {
            SceneId::Vale => "Vale",
            SceneId::WildForest => "WildForest",
            SceneId::Cave => "Cave",
            _ => "Vale",
        };

        let configs = golden_sun::data::enemies_for_area(scene_name);
        let num_enemies = configs.len().clamp(1, 2);
        let mut enemies = Vec::with_capacity(num_enemies);
        for i in 0..num_enemies {
            let cfg = &configs[i % configs.len()];
            let elem = if i % 2 == 0 {
                golden_sun::Element::Jupiter
            } else {
                golden_sun::Element::Mercury
            };
            enemies.push(Combatant::new(
                10 + i as u32,
                cfg.name,
                cfg.level,
                elem,
                false,
            ));
        }

        self.battle = Some(Battle::new(party, enemies));
        self.state = GameState::Battle;
        #[cfg(debug_assertions)]
        eprintln!("遭遇战开始 (区域={scene_name})");
    }

    /// 保存游戏到 StorageBackend
    pub fn save_game(&mut self) {
        use golden_sun::SaveData;
        let psynergies: Vec<String> = self.unlocked_psynergies[..self.unlocked_count]
            .iter().map(|p| format!("{p:?}")).collect();
        let collected: Vec<String> = self.collected_djinn.iter().map(|d| d.djinn.id.as_str().to_string()).collect();
        let equipped: Vec<(String, u32)> = self.collected_djinn.iter()
            .filter(|d| d.equipped)
            .map(|d| (d.djinn.id.as_str().to_string(), d.equipped_to.unwrap_or(0)))
            .collect();
        let data = SaveData {
            scene: match self.scene.current() {
                golden_sun::SceneId::Vale => "Vale",
                golden_sun::SceneId::WildForest => "WildForest",
                golden_sun::SceneId::Cave => "Cave",
                _ => "Vale",
            }.into(),
            player_x: self.camera.x,
            player_y: self.camera.y,
            player_rotation: self.camera.rotation,
            flags: std::collections::HashMap::new(),
            inventory: Vec::new(),
            psynergies,
            gold: 0,
            player_hp: self.player_stats.hp,
            player_pp: self.pp,
            player_level: self.player_stats.level,
            player_attack: self.player_stats.attack,
            player_defense: self.player_stats.defense,
            collected_djinn: collected,
            equipped_djinn: equipped,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        };
        if let Ok(bytes) = bincode::serialize(&data) {
            if self._storage.save("save", &bytes).is_ok() {
                #[cfg(debug_assertions)]
                eprintln!("游戏已保存 ({:.1}, {:.1})", self.camera.x, self.camera.y);
            }
        }
    }

    /// 从 StorageBackend 读档
    pub fn load_game(&mut self) -> bool {
        use golden_sun::SaveData;
        match self._storage.load("save") {
            Ok(Some(bytes)) => {
                match bincode::deserialize::<SaveData>(&bytes) {
                    Ok(data) => {
                        self.camera = Camera::new(data.player_x, data.player_y);
                        self.camera.rotation = data.player_rotation;
                        self.pp = data.player_pp;
                        
                        // 恢复 Djinn 收集状态
                        let all_djinn_data = djinn::all_djinn_data();
                        for saved_id in &data.collected_djinn {
                            if let Some(template) = all_djinn_data.iter().find(|d| d.id.as_str() == saved_id.as_str()) {
                                let owned = OwnedDjinn::new(template.clone());
                                self.collected_djinn.push(owned);
                            }
                        }
                        
                        // 恢复装备状态
                        for (saved_id, slot) in &data.equipped_djinn {
                            if let Some(od) = self.collected_djinn.iter_mut().find(|d| d.djinn.id.as_str() == saved_id.as_str()) {
                                od.equipped = true;
                                od.equipped_to = Some(*slot);
                            }
                        }
                        
                        if !self.collected_djinn.is_empty() {
                            self.apply_djinn_bonuses();
                        }
                        
                        self.state = GameState::WorldMap;
                        #[cfg(debug_assertions)]
                        eprintln!("游戏已读档 ({:.1}, {:.1}), Djinn: {}", data.player_x, data.player_y, self.collected_djinn.len());
                        true
                    }
                    Err(_) => false,
                }
            }
            _ => false,
        }
    }

    /// 触发场景过渡动画
    #[allow(dead_code)]
    pub fn start_transition(&mut self, kind: golden_sun::engine::TransitionKind) {
        self.state = GameState::Transition {
            kind,
            timer: 0.0,
            from: "",
            to: "",
        };
    }

    /// 请求切换到新场景（带过渡动画）
    pub fn request_scene_switch(&mut self, target: golden_sun::SceneId) {
        self.pending_scene = Some(target);
        self.start_transition(golden_sun::engine::TransitionKind::Wipe);
    }

    /// 执行场景切换（过渡动画完成后调用）
    pub fn apply_scene_switch(&mut self) {
        if let Some(target) = self.pending_scene.take() {
            self.scene.commit_switch();
            self.npcs = create_npcs_for_scene(target);
            self.modified_tiles_current.clear();
            self.modified_tiles.clear();
            #[cfg(debug_assertions)]
            eprintln!("场景切换至: {target:?}");
        }
    }

    /// 使用药水恢复 HP
    #[allow(dead_code)]
    pub fn use_potion(&mut self) -> bool {
        if let Some(item) = self.inventory.iter_mut().find(|i| i.item_type == ItemType::Potion && i.count > 0) {
            item.count -= 1;
            if item.count == 0 {
                self.inventory.retain(|i| i.count > 0);
            }
            let heal = 30u32;
            self.player_stats.hp = (self.player_stats.hp + heal).min(self.player_stats.max_hp);
            true
        } else {
            false
        }
    }

    /// 使用以太恢复 PP
    #[allow(dead_code)]
    pub fn use_ether(&mut self) -> bool {
        if let Some(item) = self.inventory.iter_mut().find(|i| i.item_type == ItemType::Ether && i.count > 0) {
            item.count -= 1;
            if item.count == 0 {
                self.inventory.retain(|i| i.count > 0);
            }
            let recover = 10u32;
            self.pp = (self.pp + recover).min(self.max_pp);
            true
        } else {
            false
        }
    }

    /// 添加金币（战斗奖励）
    pub fn add_gold(&mut self, amount: u32) {
        let multiplier = if self.has_item(ItemType::GoldRing) { 2 } else { 1 };
        self.gold += amount * multiplier;
    }

    /// 检查是否有某类道具
    pub fn has_item(&self, item_type: ItemType) -> bool {
        self.inventory.iter().any(|i| i.item_type == item_type && i.count > 0)
    }

    /// 添加经验值并升级
    pub fn add_exp(&mut self, amount: u32) {
        self.player_stats.add_exp(amount);
    }

    /// 播放 SFX 音效
    fn play_sfx(&self, name: &str) {
        if let Some(ref sfx) = self.sfx_manager {
            sfx.play(name);
        }
    }

    pub fn step(&mut self) {
        self.update();
        self.draw();
    }

    // ── Phase 6.9: Djinn 方法 ──

    /// 获取已装备的 Djinn 列表（用于计算属性加成）
    #[allow(dead_code)]
    fn equipped_djinn_list(&self) -> Vec<&OwnedDjinn> {
        self.collected_djinn.iter().filter(|d| d.equipped).collect()
    }

    /// 获取角色装备的 Djinn
    fn character_djinn(&self, char_idx: u32) -> Vec<&OwnedDjinn> {
        self.collected_djinn.iter()
            .filter(|d| d.equipped_to == Some(char_idx))
            .collect()
    }

    /// 应用 Djinn 装备属性加成到玩家
    fn apply_djinn_bonuses(&mut self) {
        let mut atk_bonus = 0u32;
        let mut def_bonus = 0u32;
        let mut hp_bonus = 0u32;
        let mut pp_bonus = 0u32;

        for od in &self.collected_djinn {
            if od.equipped {
                atk_bonus += od.djinn.atk_bonus;
                def_bonus += od.djinn.def_bonus;
                hp_bonus += od.djinn.hp_bonus;
                pp_bonus += od.djinn.pp_bonus;
            }
        }

        // 计算套装加成
        let equipped: Vec<&OwnedDjinn> = self.collected_djinn.iter().filter(|d| d.equipped).collect();
        let bonuses = djinn::calculate_set_bonus(&equipped);
        for bonus in &bonuses {
            match bonus {
                SetBonus::LifeRing => { hp_bonus = (hp_bonus as f32 * 0.2) as u32; }
                SetBonus::ManaRing => { pp_bonus = (pp_bonus as f32 * 0.2) as u32; }
                SetBonus::PowerRing => { atk_bonus = (atk_bonus as f32 * 0.2) as u32; }
                SetBonus::GuardRing => { def_bonus = (def_bonus as f32 * 0.2) as u32; }
                SetBonus::SwiftRing => { /* speed handled separately */ }
                SetBonus::SageRing => {
                    atk_bonus = (atk_bonus as f32 * 0.15) as u32;
                    def_bonus = (def_bonus as f32 * 0.15) as u32;
                    hp_bonus = (hp_bonus as f32 * 0.15) as u32;
                    pp_bonus = (pp_bonus as f32 * 0.15) as u32;
                }
                SetBonus::None => {}
            }
        }

        self.player_stats.attack += atk_bonus;
        self.player_stats.defense += def_bonus;
        self.player_stats.max_hp += hp_bonus;
        self.player_stats.hp = self.player_stats.hp.saturating_add(hp_bonus).min(self.player_stats.max_hp);
        self.max_pp += pp_bonus;
        self.pp = self.pp.min(self.max_pp);

        // 根据职业更新可用精灵力
        self.update_class_psynergies();
    }

    /// 根据当前职业更新可用精灵力
    fn update_class_psynergies(&mut self) {
        let elements: Vec<golden_sun::Element> = self.character_djinn(0)
            .iter().map(|d| d.djinn.element()).collect();
        let elements2: Vec<golden_sun::Element> = self.character_djinn(1)
            .iter().map(|d| d.djinn.element()).collect();
        let mut all_elements = elements;
        all_elements.extend(elements2);
        all_elements.sort_by_key(|e| *e as u8);
        all_elements.dedup();

        let new_class = Class::from_elements(&all_elements);
        
        if new_class != self.current_class {
            #[cfg(debug_assertions)]
            eprintln!("职业变化: {} → {}", self.current_class.name(), new_class.name());
            self.current_class = new_class;
            
            // 更新精灵力解锁
            let unlocked = new_class.unlocked_psynergies();
            self.unlocked_count = unlocked.len().min(PsynergyType::ALL.len());
            for (i, psy) in unlocked.iter().enumerate() {
                if i < PsynergyType::ALL.len() {
                    self.unlocked_psynergies[i] = *psy;
                }
            }
        }
    }

    /// 收集 Djinn（在地图上找到时调用）
    pub fn collect_djinn(&mut self, djinn_id: DjinnId) -> bool {
        let data = djinn::all_djinn_data();
        if let Some(djinn_template) = data.iter().find(|d| d.id == djinn_id) {
            // 检查是否已拥有
            if self.collected_djinn.iter().any(|d| d.djinn.id == djinn_id) {
                return false;
            }
            let owned = OwnedDjinn::new(djinn_template.clone());
            self.collected_djinn.push(owned);
            
            // 自动装备到第一个有空位的角色
            self.auto_equip_djinn(djinn_id);
            
            // 应用属性加成
            self.apply_djinn_bonuses();
            
            // 标记任务
            self.quest_log.complete("first_djinn");
            
            #[cfg(debug_assertions)]
            eprintln!("收集到 Djinn: {}", djinn_id.as_str());
            true
        } else {
            false
        }
    }

    /// 自动装备 Djinn 到第一个有空位的角色
    fn auto_equip_djinn(&mut self, djinn_id: DjinnId) {
        // 查找哪个角色槽位 Djinn 较少
        let count_0 = self.collected_djinn.iter().filter(|d| d.equipped_to == Some(0)).count();
        let count_1 = self.collected_djinn.iter().filter(|d| d.equipped_to == Some(1)).count();
        let target = if count_0 <= count_1 { 0u32 } else { 1 };
        
        if let Some(od) = self.collected_djinn.iter_mut().find(|d| d.djinn.id == djinn_id) {
            od.equipped = true;
            od.equipped_to = Some(target);
        }
    }

    /// 切换 Djinn 装备状态
    fn toggle_djinn_equip(&mut self, idx: usize) -> bool {
        if idx >= self.collected_djinn.len() {
            return false;
        }
        let is_equipped = self.collected_djinn[idx].equipped;
        let char_idx = self.djinn_character_select;
        
        // 先检查当前角色已装备的 Djinn 数量
        let current_count = self.collected_djinn.iter()
            .filter(|d| d.equipped_to == Some(char_idx as u32))
            .count();
        
        if is_equipped {
            // 卸下
            self.collected_djinn[idx].equipped = false;
            self.collected_djinn[idx].equipped_to = None;
        } else {
            // 装备 — 限制每个角色最多装备 4 个 Djinn
            if current_count >= 4 {
                return false;
            }
            self.collected_djinn[idx].equipped = true;
            self.collected_djinn[idx].equipped_to = Some(char_idx as u32);
        }
        
        self.apply_djinn_bonuses();
        true
    }

    /// 在战斗中释放 Djinn
    #[allow(dead_code)]
    pub fn release_djinn_in_battle(&mut self, djinn_id: DjinnId) -> bool {
        if let Some(od) = self.collected_djinn.iter_mut().find(|d| d.djinn.id == djinn_id) {
            if od.equipped && !od.released {
                od.released = true;
                #[cfg(debug_assertions)]
                eprintln!("释放 Djinn: {} — 战斗后恢复", djinn_id.as_str());
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// 在战斗结束后召回所有已释放的 Djinn
    pub fn recall_all_djinn(&mut self) {
        let mut changed = false;
        for od in &mut self.collected_djinn {
            if od.released {
                od.released = false;
                changed = true;
            }
        }
        if changed {
            self.apply_djinn_bonuses();
            #[cfg(debug_assertions)]
            eprintln!("所有 Djinn 已召回");
        }
    }

    /// 检查玩家是否站在 Djinn 位置上
    fn check_djinn_pickup(&mut self) {
        let px = self.camera.x.floor();
        let py = self.camera.y.floor();
        
        for (djinn_id, scene_name, tx, ty) in djinn::world_djinn() {
            if scene_name == match self.scene.current() {
                golden_sun::SceneId::Vale => "Vale",
                golden_sun::SceneId::WildForest => "WildForest",
                golden_sun::SceneId::Cave => "Cave",
                _ => "Vale",
            } {
                let dist_sq = (px - tx).powi(2) + (py - ty).powi(2);
                if dist_sq < 2.0 {
                    // 附近有 Djinn，按 A 收集
                    if self.input_bus.consume(golden_sun::InputEvent::Confirm)
                        && !self.collected_djinn.iter().any(|d| d.djinn.id == djinn_id)
                    {
                        self.collect_djinn(djinn_id);
                    }
                }
            }
        }
    }
}
