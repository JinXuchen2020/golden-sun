mod update;
mod draw;

use std::collections::HashMap;

use golden_sun::battle::{Battle, Combatant};
use golden_sun::engine::{Camera, FrameTime, GameState, InputState};
use golden_sun::{InputBus, TextureCache, WindowConfig};
use golden_sun::constants::{RENDER_TARGET_W, RENDER_TARGET_H, SPRITE_SIZE, PP_INITIAL, PP_MAX};
use golden_sun::{SceneId, SceneRegistry, PsynergyType};
use golden_sun::entity::{create_vale_npcs, Entity};
use golden_sun::dialogue::{DialogueState, StoryFlags};
use golden_sun::entity::sprite::{self, AnimState};
use golden_sun::map::TileKind;
use macroquad::prelude::*;

const PLAYER_START_X: f32 = 15.0;
const PLAYER_START_Y: f32 = 16.0;
const ANIM_COUNT: usize = 8;
const HAT_COLOR: (u8, u8, u8) = (60, 180, 60);
const BODY_COLOR: (u8, u8, u8) = (100, 150, 200);

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
    // ── Phase 3: 精灵力 ──
    pp: u32,
    max_pp: u32,
    unlocked_psynergies: [PsynergyType; 7],
    unlocked_count: usize,
    selected_psynergy: usize,
    pp_recover_timer: f32,
    /// 运行时 tile 覆盖（精灵力修改的地图格）
    modified_tiles: HashMap<(i32, i32), TileKind>,
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
    /// 随机遇敌步数计数器（每步递减）
    encounter_step: u32,
    /// StorageBackend（存档用）
    _storage: Box<dyn golden_sun::engine::storage::StorageBackend>,
}

impl GameCtx {
    #[must_use]
    pub fn new() -> Self {
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
            pp: PP_INITIAL,
            max_pp: PP_MAX,
            unlocked_psynergies: PsynergyType::ALL,
            unlocked_count: 4,
            selected_psynergy: 0,
            pp_recover_timer: 0.0,
            modified_tiles: HashMap::new(),
            dialogue: None,
            story_flags: StoryFlags::new(),
            battle: None,
            menu_selection: 0,
            menu_page: 0,
            gold: 0,
            encounter_step: 0,
            _storage: golden_sun::engine::storage::create_storage(),
        }
    }

    /// 启动随机遭遇战
    pub fn start_random_battle(&mut self) {
        let party = vec![
            Combatant::new(1, "Isaac", 5, golden_sun::Element::Venus, true),
            Combatant::new(2, "Garet", 5, golden_sun::Element::Mars, true),
        ];
        let enemies = vec![
            Combatant::new(10, "Wolf", 3, golden_sun::Element::Jupiter, false),
            Combatant::new(11, "Bat", 2, golden_sun::Element::Mercury, false),
        ];
        self.battle = Some(Battle::new(party, enemies));
        self.state = GameState::Battle;
        #[cfg(debug_assertions)]
        eprintln!("遭遇战开始");
    }

    /// 保存游戏到 StorageBackend
    pub fn save_game(&mut self) {
        use golden_sun::SaveData;
        let psynergies: Vec<String> = self.unlocked_psynergies[..self.unlocked_count]
            .iter().map(|p| format!("{p:?}")).collect();
        let data = SaveData {
            scene: "Vale".into(),
            player_x: self.camera.x,
            player_y: self.camera.y,
            player_rotation: self.camera.rotation,
            flags: std::collections::HashMap::new(),
            inventory: Vec::new(),
            psynergies,
            gold: 0,
            player_hp: 100,
            player_pp: self.pp,
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
                        self.state = GameState::WorldMap;
                        #[cfg(debug_assertions)]
                        eprintln!("游戏已读档 ({:.1}, {:.1})", data.player_x, data.player_y);
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

    pub fn step(&mut self) {
        self.update();
        self.draw();
    }
}
