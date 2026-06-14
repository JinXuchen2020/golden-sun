//! 核心引擎模块 — 所有 Phase 共享的基础类型与接口

pub mod constants;
pub mod error;
pub mod input;
pub mod resources;
pub mod storage;
pub mod texture;

use macroquad::prelude::*;

// ── 窗口配置 ──

/// 窗口配置 — 一处修改全局生效
#[derive(Debug, Clone)]
pub struct WindowConfig {
    /// 逻辑宽度（像素）
    pub width: f32,
    /// 逻辑高度（像素）
    pub height: f32,
    /// 全屏模式
    pub fullscreen: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: constants::WINDOW_WIDTH,
            height: constants::WINDOW_HEIGHT,
            fullscreen: false,
        }
    }
}

// ── 渲染层序定义 ──

/// 渲染层序 — 定义 WorldMap 状态下各层的绘制顺序
///
/// 各 Phase 的渲染模块按此枚举顺序依次调用 draw():
/// ```text
/// Sky(天空渐变) → Terrain(Mode7地面) → EntitiesLow(远NPC) → Entities(玩家/近NPC)
/// → Effects(粒子/精灵力特效) → Overlay(精灵力UI) → HUD(HP/PP) → Debug(FPS等)
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RenderPhase {
    /// 天空渐变（Phase 1）
    Sky = 0,
    /// Mode 7 地面渲染（Phase 1）
    Terrain = 1,
    /// 远距离实体（Phase 2）
    EntitiesLow = 2,
    /// 玩家 + 近 NPC（Phase 2）
    Entities = 3,
    /// 粒子/精灵力特效（Phase 3）
    Effects = 4,
    /// 精灵力选择 UI（Phase 3）
    Overlay = 5,
    /// 底部 HUD（Phase 6）
    HUD = 6,
    /// 调试信息（全 Phase，`cargo build --release` 禁用）
    Debug = 7,
}

// ── 全局游戏状态机 ──

/// 全局游戏状态机
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    /// Title: 标题画面（Phase 0/6）
    Title,
    /// WorldMap: Mode 7 世界地图漫游（Phase 1）
    WorldMap,
    /// Dialog: 对话中（Phase 4）
    Dialog,
    /// Battle: 战斗中（Phase 5）
    Battle,
    /// Menu: 菜单/道具界面（Phase 6）
    Menu,
    /// Psynergy: 精灵力选择（Phase 3）
    Psynergy,
    /// Transition: 场景切换过渡（Phase 6）
    Transition,
}

impl GameState {
    /// 是否允许世界地图更新（玩家移动等）
    pub fn allows_world_update(&self) -> bool {
        matches!(self, GameState::WorldMap)
    }

    /// 是否允许玩家输入
    pub fn accepts_input(&self) -> bool {
        matches!(self, GameState::WorldMap | GameState::Title | GameState::Menu)
    }
}

// ── 相机 ──

/// 相机状态 — 支持 Mode 7 透视投影和 lerp 插值跟随
///
/// ## 坐标系统约定
/// - **Tile 单位** (`x`, `y`): Phase 0 Camera 使用的逻辑坐标，`(0.0, 0.0)` = 地图左上角
/// - **世界坐标（像素）**: Phase 1 Mode7Camera 使用的渲染坐标，`tile × TILE_SIZE`
/// - 转换: `world = tile * TILE_SIZE`, `tile = world / TILE_SIZE`
#[derive(Debug, Clone)]
pub struct Camera {
    /// 当前 X（tile 单位）
    pub x: f32,
    /// 当前 Y（tile 单位）
    pub y: f32,
    /// 相机高度（影响透视强度）
    pub z: f32,
    /// 水平旋转角度（弧度）
    pub rotation: f32,
    /// 目标 X（lerp 插值用）
    pub target_x: f32,
    /// 目标 Y（lerp 插值用）
    pub target_y: f32,
    /// Mode 7 视野角度（弧度）
    pub fov: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            z: constants::CAMERA_DEFAULT_Z,
            rotation: 0.0,
            target_x: x,
            target_y: y,
            fov: constants::CAMERA_DEFAULT_FOV,
        }
    }

    // ── 坐标转换（消除 tile/像素两套系统的混乱） ──

    /// tile 坐标 → 世界像素坐标
    #[inline]
    pub fn tile_to_world(tile: f32) -> f32 {
        tile * constants::TILE_SIZE
    }

    /// 世界像素坐标 → tile 坐标
    #[inline]
    pub fn world_to_tile(world: f32) -> f32 {
        world / constants::TILE_SIZE
    }

    /// 获取当前相机在 tile 网格中的索引
    #[inline]
    pub fn tile_index(&self) -> (i32, i32) {
        (self.x.floor() as i32, self.y.floor() as i32)
    }

    /// 获取当前相机在世界坐标中的位置
    #[inline]
    pub fn world_pos(&self) -> (f32, f32) {
        (Self::tile_to_world(self.x), Self::tile_to_world(self.y))
    }

    // ── 移动 ──

    /// 更新 lerp 插值（每帧调用）
    pub fn update_lerp(&mut self, dt: f32) {
        let speed = constants::CAMERA_LERP_SPEED;
        self.x += (self.target_x - self.x) * (speed * dt).min(1.0);
        self.y += (self.target_y - self.y) * (speed * dt).min(1.0);
    }

    /// 设置插值目标
    pub fn set_target(&mut self, x: f32, y: f32) {
        self.target_x = x;
        self.target_y = y;
    }

    /// 立即跳转到目标（取消插值）
    pub fn snap_to_target(&mut self) {
        self.x = self.target_x;
        self.y = self.target_y;
    }

    /// 沿当前朝向移动（tile 单位）
    pub fn move_forward(&mut self, distance: f32) {
        self.x += distance * self.rotation.cos();
        self.y += distance * self.rotation.sin();
    }

    /// 沿当前朝向后退（tile 单位）
    pub fn move_backward(&mut self, distance: f32) {
        self.move_forward(-distance);
    }

    /// 横向平移（tile 单位）
    pub fn strafe(&mut self, distance: f32) {
        let angle = self.rotation + std::f32::consts::FRAC_PI_2;
        self.x += distance * angle.cos();
        self.y += distance * angle.sin();
    }

    /// 旋转视角（弧度）
    pub fn rotate(&mut self, radians: f32) {
        self.rotation = (self.rotation + radians) % (std::f32::consts::TAU);
    }
}

// ── 输入状态 ──

/// 当前帧的输入状态（裸按键读值，供 InputBus 消费）
#[derive(Debug, Clone)]
pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            a: false,
            b: false,
            start: false,
            select: false,
        }
    }

    /// 从 macroquad 按键状态刷新
    pub fn poll(&mut self) {
        self.up = is_key_down(KeyCode::Up) || is_key_down(KeyCode::W);
        self.down = is_key_down(KeyCode::Down) || is_key_down(KeyCode::S);
        self.left = is_key_down(KeyCode::Left) || is_key_down(KeyCode::A);
        self.right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D);
        self.a = is_key_pressed(KeyCode::Z) || is_key_pressed(KeyCode::Enter);
        self.b = is_key_pressed(KeyCode::X) || is_key_pressed(KeyCode::Escape);
        self.start = is_key_pressed(KeyCode::Space);
        self.select = is_key_pressed(KeyCode::LeftShift);
    }
}

// ── 帧时序 ──

/// 帧时序信息
#[derive(Debug, Clone)]
pub struct FrameTime {
    /// 上一帧的 delta 时间（秒）
    pub delta: f32,
    /// 程序启动以来的总时间（秒）
    pub elapsed: f32,
}

impl FrameTime {
    pub fn new() -> Self {
        Self {
            delta: 0.0,
            elapsed: 0.0,
        }
    }

    /// 从 macroquad 帧状态刷新（带 delta 裁剪保护）
    pub fn poll(&mut self) {
        let raw = get_frame_time();
        // 裁剪 delta 防止帧卡顿飞越下界 + 无 vsync 零除上界
        self.delta = raw.clamp(constants::DELTA_MIN, constants::DELTA_MAX);
        self.elapsed += self.delta;
    }
}
