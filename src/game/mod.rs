mod update;
mod draw;

use golden_sun::engine::{Camera, FrameTime, GameState, InputState};
use golden_sun::{InputBus, TextureCache, WindowConfig};
use golden_sun::constants::{RENDER_TARGET_W, RENDER_TARGET_H};
use golden_sun::{SceneId, SceneRegistry};
use golden_sun::entity::{create_vale_npcs, Entity};
use golden_sun::entity::sprite::{self, AnimState};
use golden_sun::constants::SPRITE_SIZE;
use macroquad::prelude::*;

const PLAYER_START_X: f32 = 15.0;
const PLAYER_START_Y: f32 = 16.0;
const ANIM_COUNT: usize = 8;

const NPC_HAT: (u8, u8, u8) = (60, 180, 60);
const NPC_BODY: (u8, u8, u8) = (100, 150, 200);

struct SpriteAtlas {
    player: [Vec<Texture2D>; ANIM_COUNT],
    npc_sprites: [Vec<Vec<u8>>; ANIM_COUNT],
}

impl SpriteAtlas {
    fn new() -> Self {
        let mut player: [Vec<Texture2D>; ANIM_COUNT] = Default::default();
        for (state, anim) in sprite::generate_player_animations() {
            let texes: Vec<Texture2D> = anim.frames.into_iter().map(|f| {
                let img = Image {
                    width: SPRITE_SIZE as u16,
                    height: SPRITE_SIZE as u16,
                    bytes: f.pixels,
                };
                let t = Texture2D::from_image(&img);
                t.set_filter(FilterMode::Nearest);
                t
            }).collect();
            player[state.index()] = texes;
        }

        let mut npc_sprites: [Vec<Vec<u8>>; ANIM_COUNT] = Default::default();
        for (state, anim) in sprite::generate_npc_animations(NPC_HAT, NPC_BODY) {
            let frames: Vec<Vec<u8>> = anim.frames.into_iter().map(|f| f.pixels).collect();
            npc_sprites[state.index()] = frames;
        }

        Self { player, npc_sprites }
    }

    #[inline]
    fn player_tex(&self, state: AnimState) -> &Vec<Texture2D> {
        &self.player[state.index()]
    }

    #[inline]
    fn npc_frame(&self, state: AnimState) -> &Vec<Vec<u8>> {
        &self.npc_sprites[state.index()]
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
        }
    }

    pub fn step(&mut self) {
        self.update();
        self.draw();
    }
}
