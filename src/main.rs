use golden_sun::engine::{Camera, FrameTime, GameState, InputState};
use golden_sun::{InputBus, InputEvent, ResourceManager, TextureCache, WindowConfig, create_storage};
use golden_sun::engine::storage::StorageBackend;
use golden_sun::constants::{self, RENDER_TARGET_W, RENDER_TARGET_H};
use golden_sun::{GameResult, SceneId, SceneRegistry};
use golden_sun::map::tilemap;
use macroquad::prelude::*;

/// 玩家起始 tile 坐标（Vale 村中心偏左）
const PLAYER_START_X: f32 = 15.0;
const PLAYER_START_Y: f32 = 16.0;

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
    textures: TextureCache,
    _storage: Box<dyn StorageBackend>,
}

impl GameCtx {
    fn new() -> Self {
        Self {
            config: WindowConfig::default(),
            state: GameState::Title,
            scene: SceneRegistry::new(SceneId::Title),
            camera: Camera::new(PLAYER_START_X, PLAYER_START_Y),
            input: InputState::new(),
            input_bus: InputBus::new(),
            time: FrameTime::new(),
            _resources: ResourceManager::new(),
            textures: TextureCache::new(RENDER_TARGET_W, RENDER_TARGET_H),
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

        self.scene.commit_switch();

        // 过渡：所有输入锁定
        if self.state.is_transition() {
            return Ok(());
        }

        match self.state {
            GameState::Title => {
                if self.input_bus.consume(InputEvent::Confirm)
                    || self.input_bus.consume(InputEvent::Menu)
                {
                    // 切到 WorldMap 时重置玩家位置
                    self.camera = Camera::new(PLAYER_START_X, PLAYER_START_Y);
                    self.state = GameState::WorldMap;
                }
            }
            GameState::WorldMap => {
                self.update_player_movement();
                self.camera.update_lerp(self.time.delta);

                // 菜单（Phase 6）
                if self.input_bus.consume(InputEvent::Menu) {
                    self.state = GameState::Menu;
                }
            }
            GameState::Dialog => {}
            GameState::Battle => {}
            GameState::Menu => {
                if self.input_bus.consume(InputEvent::Cancel) {
                    self.state = GameState::WorldMap;
                }
            }
            GameState::Psynergy => {
                if self.input_bus.consume(InputEvent::Cancel) {
                    self.state = GameState::WorldMap;
                }
            }
            GameState::Transition { .. } => {}
            _ => {}
        }
        Ok(())
    }

    /// 玩家移动 + 碰撞检测
    fn update_player_movement(&mut self) {
        let dt = self.time.delta;

        // 基础速度 + 加速
        let speed = constants::PLAYER_SPEED
            * if self.input.a { constants::PLAYER_SPRINT_MULTIPLIER } else { 1.0 };

        if self.input_bus.consume(InputEvent::Up) {
            self.try_move(1.0, speed * dt);
        }
        if self.input_bus.consume(InputEvent::Down) {
            self.try_move(-1.0, speed * dt);
        }
        if self.input_bus.consume(InputEvent::Left) {
            self.try_rotate(-constants::PLAYER_TURN_SPEED * dt);
        }
        if self.input_bus.consume(InputEvent::Right) {
            self.try_rotate(constants::PLAYER_TURN_SPEED * dt);
        }
    }

    /// 尝试沿当前朝向移动（sign=1 前，sign=-1 后），含碰撞检测
    fn try_move(&mut self, sign: f32, distance: f32) {
        let new_x = self.camera.x + sign * distance * self.camera.rotation.cos();
        let new_y = self.camera.y + sign * distance * self.camera.rotation.sin();
        self.try_move_to(new_x, new_y);
    }

    /// 尝试移动到目标 tile（碰撞检测 + 边界保护）
    fn try_move_to(&mut self, new_x: f32, new_y: f32) {
        let tx = new_x.floor() as i32;
        let ty = new_y.floor() as i32;

        if tilemap::is_walkable(tx, ty) {
            self.camera.x = new_x;
            self.camera.y = new_y;
        }
        // 碰撞失败：不移动
    }

    fn try_rotate(&mut self, radians: f32) {
        self.camera.rotate(radians);
    }

    // ═══════════════ 绘制 ═══════════════

    fn draw(&mut self) {
        clear_background(constants::BG_COLOR);

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
        draw_text("Golden Sun - Rust Edition", 40.0, 200.0, 36.0, WHITE);
        draw_text(
            "按 Z / Enter 开始",
            100.0,
            260.0,
            20.0,
            constants::TITLE_TEXT_COLOR,
        );
    }

    fn draw_placeholder(&self) {
        draw_text("Phase pending...", 10.0, 30.0, 24.0, constants::PLACEHOLDER_TEXT_COLOR);
        draw_text(
            format!("State: {:?}", self.state),
            10.0, 60.0, 16.0, LIGHTGRAY,
        );
        draw_text(format!("FPS: {}", get_fps()), 10.0, 80.0, 16.0, LIGHTGRAY);
    }

    /// WorldMap 渲染：Mode 7 地面 → 玩家标记
    fn draw_world_map(&mut self) {
        // 1. Mode 7 渲染到纹理
        golden_sun::map::mode7::render(&mut self.textures, &self.camera);
        self.textures.upload_world_map();

        draw_texture(self.textures.world_map_texture(), 0.0, 0.0, WHITE);

        let screen_x = self.config.width * 0.5;
        let screen_y = self.config.height - constants::SCREEN_MARGIN_BOTTOM;
        draw_player_marker(screen_x, screen_y);
    }

    #[cfg(debug_assertions)]
    fn draw_debug(&self) {
        let (wx, wy) = self.camera.world_pos();
        let (tx, ty) = self.camera.tile_index();
        draw_text(
            format!("FPS: {} | Tile: ({},{}) | World: ({:.0},{:.0}) | Rot: {:.2}",
                get_fps(), tx, ty, wx, wy, self.camera.rotation),
            10.0, self.config.height - constants::SCREEN_MARGIN_BOTTOM, 14.0,
            constants::DEBUG_TEXT_COLOR,
        );
    }
}

/// 绘制玩家脚底菱形标记
fn draw_player_marker(x: f32, y: f32) {
    let size = constants::PLAYER_MARKER_SIZE;
    let color = constants::PLAYER_MARKER_COLOR;

    // 菱形：4 个三角形顶点
    draw_triangle(
        Vec2::new(x, y - size),         // 上
        Vec2::new(x - size * 0.7, y),    // 左
        Vec2::new(x, y + size),          // 下
        color,
    );
    draw_triangle(
        Vec2::new(x, y - size),         // 上
        Vec2::new(x + size * 0.7, y),    // 右
        Vec2::new(x, y + size),          // 下
        color,
    );
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
