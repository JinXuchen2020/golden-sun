//! Mode 7 投影相机 — 在 `Camera` 2D tile 坐标基础上添加透视投影方法
//!
//! ## 设计澄清
//! - `Camera` (engine/mod.rs): 管理 2D 地图 tile 坐标 (`x`, `y`) 与插值移动
//! - `Mode7Camera`: 封装 Camera，提供 Mode 7 逐行扫描投影所需的世界坐标计算
//!   - `x` → 相机在世界空间 X 轴位置（像素）
//!   - `y` → 地图 tile 纵坐标（2D 平面），投影时映射为深度 Z
//!   - `z` → 相机高度（决定透视强度）
//!
//! ## 使用
//! ```ignore
//! let cam = Camera::new(16.0, 16.0);
//! let m7 = Mode7Camera::new(&cam);
//! for screen_y in horizon..height {
//!     let (world_x, world_z) = m7.project(screen_y, screen_x);
//!     // ...
//! }
//! ```

use crate::engine::constants::{self, TILE_SIZE};
use crate::engine::Camera;

/// 单扫描线预计算数据 — 避免每像素重复计算 cos/sin
#[derive(Debug, Clone, Copy)]
pub struct ScanlineContext {
    pub cos_r: f32,
    pub sin_r: f32,
    pub world_x: f32,
    pub world_z: f32,
    pub fov: f32,
    pub z: f32,
    pub screen_w: f32,
    pub horizon_y: f32,
}

/// Mode 7 投影相机 — 将 Camera 的 tile 坐标投影到伪 3D 空间
#[derive(Debug)]
pub struct Mode7Camera<'a> {
    inner: &'a Camera,
}

impl<'a> Mode7Camera<'a> {
    pub fn new(camera: &'a Camera) -> Self {
        Self { inner: camera }
    }

    /// 相机在世界空间 X 轴位置（像素）
    #[must_use]
    pub fn world_x(&self) -> f32 {
        self.inner.x * TILE_SIZE
    }

    /// 相机在世界空间 Z 轴位置（像素）— 来自 Camera.y
    #[must_use]
    pub fn world_z(&self) -> f32 {
        self.inner.y * TILE_SIZE
    }

    /// 相机高度
    #[must_use]
    pub fn height(&self) -> f32 {
        self.inner.height
    }

    /// 水平旋转角
    #[must_use]
    pub fn rotation(&self) -> f32 {
        self.inner.rotation
    }

    /// 视野角度
    #[must_use]
    pub fn fov(&self) -> f32 {
        self.inner.fov
    }

    /// 准备一条扫描线的投影参数（每扫描线调用一次）
    #[inline]
    #[must_use]
    pub fn prepare_scanline(&self, horizon_y: f32, screen_w: f32, sy: f32) -> Option<ScanlineContext> {
        let dy = sy - horizon_y;
        if dy <= 0.0 {
            return None;
        }
        let z = self.inner.height / dy;
        Some(ScanlineContext {
            cos_r: self.inner.rotation.cos(),
            sin_r: self.inner.rotation.sin(),
            world_x: self.world_x(),
            world_z: self.world_z(),
            fov: self.inner.fov,
            z,
            screen_w,
            horizon_y,
        })
    }

    /// 在已准备好的扫描线上下文中投影单个像素（每像素调用一次）
    #[inline]
    #[must_use]
    pub fn project_pixel(ctx: &ScanlineContext, sx: f32) -> (f32, f32) {
        let scale = ctx.z / ctx.fov;
        let rel_x = (sx - ctx.screen_w * 0.5) * scale;
        let rel_z = ctx.z;

        let wx = ctx.world_x + rel_x * ctx.cos_r - rel_z * ctx.sin_r;
        let wz = ctx.world_z + rel_x * ctx.sin_r + rel_z * ctx.cos_r;
        (wx, wz)
    }

    /// 计算透视投影：给定屏幕坐标 (sx, sy)，返回世界坐标 (wx, wz)
    ///
    /// 地平线上方返回 `None`，与 `prepare_scanline` 行为一致。
    /// 逐像素循环时建议使用 `prepare_scanline` + `project_pixel` 避免重复计算。
    #[inline]
    #[must_use]
    pub fn project(&self, horizon_y: f32, screen_w: f32, sy: f32, sx: f32) -> Option<(f32, f32)> {
        let dy = sy - horizon_y;
        if dy <= 0.0 {
            return None;
        }

        let z = self.inner.height / dy;
        let scale = z / self.inner.fov;

        let rel_x = (sx - screen_w * 0.5) * scale;
        let rel_z = z;

        let cos_r = self.inner.rotation.cos();
        let sin_r = self.inner.rotation.sin();

        let wx = self.world_x() + rel_x * cos_r - rel_z * sin_r;
        let wz = self.world_z() + rel_x * sin_r + rel_z * cos_r;

        Some((wx, wz))
    }

    /// 计算雾化系数（0.0 = 全雾, 1.0 = 无雾）
    #[inline]
    #[must_use]
    pub fn fog_factor(&self, depth: f32) -> f32 {
        let fog = 1.0 - depth / constants::FOG_END;
        fog.clamp(constants::FOG_MIN_ALPHA, 1.0)
    }
}
