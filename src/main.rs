use golden_sun::engine::{Camera, FrameTime, GameState, InputState};
use golden_sun::{InputBus, InputEvent, ResourceManager, TextureCache, WindowConfig, create_storage};
use golden_sun::engine::storage::StorageBackend;
use golden_sun::constants::{RENDER_TARGET_W, RENDER_TARGET_H};
use golden_sun::GameResult;
use macroquad::prelude::*;

/// 全局游戏上下文
struct GameCtx {
    #[allow(dead_code)]
    config: WindowConfig,
    state: GameState,
    camera: Camera,
    input: InputState,
    input_bus: InputBus,
    time: FrameTime,
    _resources: ResourceManager,
    #[allow(dead_code)]
    textures: TextureCache,
    #[allow(dead_code)]
    storage: Box<dyn StorageBackend>,
}

impl GameCtx {
    fn new() -> Self {
        Self {
            config: WindowConfig::default(),
            state: GameState::Title,
            camera: Camera::new(0.0, 0.0),
            input: InputState::new(),
            input_bus: InputBus::new(),
            time: FrameTime::new(),
            _resources: ResourceManager::new(),
            textures: TextureCache::new(RENDER_TARGET_W, RENDER_TARGET_H),
            storage: create_storage(),
        }
    }

    /// 每帧更新
    fn update(&mut self) -> GameResult<()> {
        self.time.poll();
        self.input.poll();
        self.input_bus.poll(&self.input);

        // 场景过渡：所有输入锁定
        if self.state == GameState::Transition {
            return Ok(());
        }

        match self.state {
            GameState::Title => {
                if self.input_bus.consume(InputEvent::Confirm)
                    || self.input_bus.consume(InputEvent::Menu)
                {
                    self.state = GameState::WorldMap;
                }
            }
            GameState::WorldMap => {
                self.camera.update_lerp(self.time.delta);
                // Phase 1: 玩家移动 + Mode 7 渲染
                // Phase 3: B/Secondary → Psynergy 选择
                if self.input_bus.consume(InputEvent::Menu) {
                    self.state = GameState::Menu;
                }
            }
            GameState::Dialog => {
                // Phase 4: 对话引擎消费 Confirm/Cancel
            }
            GameState::Battle => {
                // Phase 5: 战斗状态机消费方向/Confirm/Cancel
            }
            GameState::Menu => {
                // Phase 6: 菜单系统
                if self.input_bus.consume(InputEvent::Cancel) {
                    self.state = GameState::WorldMap;
                }
            }
            GameState::Psynergy => {
                // Phase 3: 精灵力选择
                if self.input_bus.consume(InputEvent::Cancel) {
                    self.state = GameState::WorldMap;
                }
            }
            GameState::Transition => {
                // 过场动画期间不处理输入（已在顶部拦截）
            }
        }
        Ok(())
    }

    /// 每帧绘制
    fn draw(&self) {
        clear_background(Color::from_rgba(86, 130, 36, 255));

        match self.state {
            GameState::Title => {
                self.draw_title();
            }
            GameState::WorldMap => {
                self.draw_world_map();
                #[cfg(debug_assertions)]
                self.draw_debug();
            }
            GameState::Dialog | GameState::Battle | GameState::Menu
            | GameState::Psynergy | GameState::Transition => {
                draw_text(
                    "Phase pending...",
                    10.0,
                    30.0,
                    24.0,
                    Color::from_rgba(255, 200, 100, 255),
                );
                draw_text(
                    &format!("State: {:?}", self.state),
                    10.0,
                    60.0,
                    16.0,
                    LIGHTGRAY,
                );
                draw_text(
                    &format!("FPS: {}", get_fps()),
                    10.0,
                    80.0,
                    16.0,
                    LIGHTGRAY,
                );
            }
        }
    }

    fn draw_title(&self) {
        draw_text(
            "Golden Sun - Rust Edition",
            40.0,
            200.0,
            36.0,
            WHITE,
        );
        draw_text(
            "按 Z / Enter 开始",
            100.0,
            260.0,
            20.0,
            Color::from_rgba(200, 220, 255, 255),
        );
    }

    fn draw_world_map(&self) {
        // Phase 1: Mode 7 渲染流程
        // 1. 获取 pixels = self.textures.world_map_image_mut().get_image_data_mut()
        // 2. 逐行扫描写入像素 (mode7::render)
        // 3. 上传: self.textures.upload_world_map()
        // 4. 绘制: draw_texture(self.textures.world_map_texture(), 0, 0, WHITE)
        // 5. 叠加 NPC / 特效层
        //
        // 渲染层序: Sky → Terrain → EntitiesLow → Entities → Effects → Overlay → HUD
        draw_text("World Map (Phase 1)", 10.0, 30.0, 24.0, WHITE);
        draw_text(
            &format!("Camera: ({:.1}, {:.1})", self.camera.x, self.camera.y),
            10.0,
            60.0,
            16.0,
            LIGHTGRAY,
        );
    }

    #[cfg(debug_assertions)]
    fn draw_debug(&self) {
        let (wx, wy) = self.camera.world_pos();
        draw_text(
            &format!(
                "FPS: {} | Tile: {:?} | World: ({:.0}, {:.0}) | Rot: {:.2}",
                get_fps(),
                self.camera.tile_index(),
                wx,
                wy,
                self.camera.rotation
            ),
            10.0,
            self.config.height - 20.0,
            14.0,
            Color::from_rgba(255, 255, 100, 180),
        );
    }
}

#[macroquad::main("Golden Sun - Rust Edition")]
async fn main() -> GameResult<()> {
    let mut ctx = GameCtx::new();

    loop {
        ctx.update()?;
        ctx.draw();
        next_frame().await;
    }
}
