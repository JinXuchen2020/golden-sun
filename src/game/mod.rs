mod update;
mod draw;

use golden_sun::engine::{Camera, FrameTime, GameState, InputState};
use golden_sun::{InputBus, TextureCache, WindowConfig};
use golden_sun::constants::{RENDER_TARGET_W, RENDER_TARGET_H, SPRITE_SIZE, PP_INITIAL, PP_MAX};
use golden_sun::{SceneId, SceneRegistry, PsynergyType};
use golden_sun::entity::{create_vale_npcs, Entity};
use golden_sun::entity::sprite::{self, AnimState};
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
    modified_tiles: std::collections::HashMap<(i32, i32), golden_sun::map::TileKind>,
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
            modified_tiles: std::collections::HashMap::new(),
        }
    }

    pub fn step(&mut self) {
        self.update();
        self.draw();
    }
}
