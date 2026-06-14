use golden_sun::engine::{Camera, FrameTime, GameState, InputState};
use golden_sun::{InputBus, InputEvent, ResourceManager, TextureCache, WindowConfig, create_storage};
use golden_sun::engine::storage::StorageBackend;
use golden_sun::constants::{RENDER_TARGET_W, RENDER_TARGET_H};
use golden_sun::{GameResult, SceneId, SceneRegistry};
use macroquad::prelude::*;

/// 全局游戏上下文
struct GameCtx {
    config: WindowConfig,
    state: GameState,
    scene: SceneRegistry,
    camera: Camera,
    input: InputState,
    input_bus: InputBus,
    time: FrameTime,
    _resources: ResourceManager,
    _textures: TextureCache,
    _storage: Box<dyn StorageBackend>,
}

impl GameCtx {
    fn new() -> Self {
        Self {
            config: WindowConfig::default(),
            state: GameState::Title,
            scene: SceneRegistry::new(SceneId::Title),
            camera: Camera::new(0.0, 0.0),
            input: InputState::new(),
            input_bus: InputBus::new(),
            time: FrameTime::new(),
            _resources: ResourceManager::new(),
            _textures: TextureCache::new(RENDER_TARGET_W, RENDER_TARGET_H),
            _storage: create_storage(),
        }
    }

    /// 每帧更新
    fn update(&mut self) -> GameResult<()> {
        self.time.poll();
        self.input.poll();
        self.input_bus.poll(&self.input);

        debug_assert!(self.camera.validate(), "Camera 参数无效 — height={}, fov={}",
            self.camera.height, self.camera.fov);

        // 提交待处理的场景切换
        self.scene.commit_switch();

        // 场景过渡：所有输入锁定
        if self.state.is_transition() {
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
            GameState::Transition { .. } => {
                // 过场动画期间不处理输入（已在顶部拦截）
            }
            // 未来 Phase 新增状态时的安全 fallback
            _ => {}
        }
        Ok(())
    }

    /// 每帧绘制
    fn draw(&self) {
        clear_background(golden_sun::constants::BG_COLOR);

        match self.state {
            GameState::Title => self.draw_title(),
            GameState::WorldMap => {
                self.draw_world_map();
                #[cfg(debug_assertions)]
                self.draw_debug();
            }
            _ => self.draw_placeholder(),
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
            golden_sun::constants::TITLE_TEXT_COLOR,
        );
    }

    fn draw_placeholder(&self) {
        draw_text(
            "Phase pending...",
            10.0, 30.0, 24.0,
            golden_sun::constants::PLACEHOLDER_TEXT_COLOR,
        );
        draw_text(
            format!("State: {:?}", self.state),
            10.0, 60.0, 16.0, LIGHTGRAY,
        );
        draw_text(
            format!("FPS: {}", get_fps()),
            10.0, 80.0, 16.0, LIGHTGRAY,
        );
    }

    fn draw_world_map(&self) {
        draw_text("World Map (Phase 1)", 10.0, 30.0, 24.0, WHITE);
        draw_text(
            format!("Camera: ({:.1}, {:.1})", self.camera.x, self.camera.y),
            10.0, 60.0, 16.0, LIGHTGRAY,
        );
    }

    #[cfg(debug_assertions)]
    fn draw_debug(&self) {
        let (wx, wy) = self.camera.world_pos();
        draw_text(
            format!(
                "FPS: {} | Tile: {:?} | World: ({:.0}, {:.0}) | Rot: {:.2}",
                get_fps(),
                self.camera.tile_index(),
                wx, wy, self.camera.rotation
            ),
            10.0, self.config.height - 20.0, 14.0,
            golden_sun::constants::DEBUG_TEXT_COLOR,
        );
    }
}

#[macroquad::main("Golden Sun - Rust Edition")]
async fn main() -> GameResult<()> {
    // vsync 已默认启用；macroquad 0.4 set_target_fps 不可用，Phase 1 如需帧率限制再处理
    let mut ctx = GameCtx::new();

    loop {
        ctx.update()?;
        ctx.draw();
        next_frame().await;
    }
}
