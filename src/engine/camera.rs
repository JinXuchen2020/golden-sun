//! 相机系统 — 支持 Mode 7 透视投影和 lerp 插值跟随
//!
//! ## 坐标系统约定
//! - **Tile 单位** (`x`, `y`): 逻辑坐标，`(0.0, 0.0)` = 地图左上角
//! - **世界坐标（像素）**: `tile × TILE_SIZE`
//! - 转换: `world = tile * TILE_SIZE`, `tile = world / TILE_SIZE`

use crate::engine::constants;

/// 相机状态
///
/// ## 字段说明
/// - `x`, `y`: 2D 地图 tile 坐标（用于 Mode 7 时 y 映射为 3D 深度 Z）
/// - `height`: 相机高度（决定 Mode 7 透视强度），对应 3D 空间的 Y 轴
/// - `fov`: 视野角度
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub height: f32,
    pub rotation: f32,
    pub(crate) target_x: f32,
    pub(crate) target_y: f32,
    pub fov: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            height: constants::CAMERA_DEFAULT_HEIGHT,
            rotation: 0.0,
            target_x: x,
            target_y: y,
            fov: constants::CAMERA_DEFAULT_FOV,
        }
    }

    /// tile 坐标 → 世界像素坐标
    #[inline]
    #[must_use]
    pub fn tile_to_world(tile: f32) -> f32 {
        tile * constants::TILE_SIZE
    }

    /// 世界像素坐标 → tile 坐标
    #[inline]
    #[must_use]
    pub fn world_to_tile(world: f32) -> f32 {
        world * constants::WORLD_TO_TILE
    }

    /// 当前相机在 tile 网格中的索引
    #[inline]
    #[must_use]
    pub fn tile_index(&self) -> (i32, i32) {
        (self.x.floor() as i32, self.y.floor() as i32)
    }

    /// 当前相机在世界坐标中的位置
    #[inline]
    #[must_use]
    pub fn world_pos(&self) -> (f32, f32) {
        (Self::tile_to_world(self.x), Self::tile_to_world(self.y))
    }

    /// 更新 lerp 插值（每帧调用）
    pub fn update_lerp(&mut self, dt: f32) {
        let speed = constants::CAMERA_LERP_SPEED;
        self.x += (self.target_x - self.x) * (speed * dt).min(1.0);
        self.y += (self.target_y - self.y) * (speed * dt).min(1.0);
    }

    pub fn set_target(&mut self, x: f32, y: f32) {
        self.target_x = x;
        self.target_y = y;
    }

    pub fn snap_to_target(&mut self) {
        self.x = self.target_x;
        self.y = self.target_y;
    }

    /// 沿当前朝向移动（tile 单位）
    pub fn move_forward(&mut self, distance: f32) {
        self.x += distance * self.rotation.cos();
        self.y += distance * self.rotation.sin();
    }

    pub fn move_backward(&mut self, distance: f32) {
        self.move_forward(-distance);
    }

    /// 横向平移（tile 单位）
    pub fn strafe(&mut self, distance: f32) {
        let angle = self.rotation + std::f32::consts::FRAC_PI_2;
        self.x += distance * angle.cos();
        self.y += distance * angle.sin();
    }

    /// 旋转视角（弧度），使用 `rem_euclid` 确保结果在 [0, 2π) 内
    pub fn rotate(&mut self, radians: f32) {
        self.rotation = (self.rotation + radians).rem_euclid(std::f32::consts::TAU);
    }

    /// 确保 height > 0（防止 Mode 7 除零），fov/rotation 有效
    #[must_use]
    pub fn validate(&self) -> bool {
        self.height > 0.0 && self.fov > 0.0 && self.rotation.is_finite()
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}
