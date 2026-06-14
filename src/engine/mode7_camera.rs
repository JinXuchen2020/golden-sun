//! Mode 7 投影相机 — 在 `Camera` 2D tile 坐标基础上添加透视投影方法
//!
//! ## 设计澄清
//! - `Camera`: 管理 2D 地图 tile 坐标 (`x`, `y`) 与插值移动
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

use crate::engine::constants::{self, HORIZON_RATIO, RENDER_TARGET_H, RENDER_TARGET_W, TILE_SIZE};
use crate::engine::Camera;

/// 投影常量（预计算，避免每次调用重算）
const SCREEN_W: f32 = RENDER_TARGET_W as f32;
const SCREEN_H: f32 = RENDER_TARGET_H as f32;
pub(crate) const HALF_SCREEN_W: f32 = SCREEN_W * 0.5;
const HORIZON_Y_PX: f32 = SCREEN_H * HORIZON_RATIO;

/// 单扫描线预计算数据 — 避免每像素重复计算
#[derive(Debug, Clone, Copy)]
pub struct ScanlineContext {
    pub cos_r: f32,
    pub sin_r: f32,
    pub world_x: f32,
    pub world_z: f32,
    pub scale: f32,
    pub z: f32,
    pub rz_sin: f32,
    pub rz_cos: f32,
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
    /// `cos_r`/`sin_r` 由调用方在帧级别预计算，避免重复 trig
    #[inline]
    #[must_use]
    pub fn prepare_scanline(&self, horizon_y: f32, sy: f32,
                            cos_r: f32, sin_r: f32) -> Option<ScanlineContext> {
        let dy = sy - horizon_y;
        if dy <= 0.0 {
            return None;
        }
        let z = self.inner.height / dy;
        Some(ScanlineContext {
            cos_r,
            sin_r,
            world_x: self.world_x(),
            world_z: self.world_z(),
            scale: z / self.inner.fov,
            z,
            rz_sin: -z * sin_r,
            rz_cos: z * cos_r,
        })
    }

    /// 在已准备好的扫描线上下文中投影单个像素（每像素调用一次）
    /// 调用方已内联此逻辑到 render 循环以消除 `sx as f32` + `sx - HALF_SCREEN_W`
    #[inline]
    #[must_use]
    pub fn project_pixel(ctx: &ScanlineContext, sx: f32) -> (f32, f32) {
        let rel_x = (sx - HALF_SCREEN_W) * ctx.scale;
        let wx = ctx.world_x + rel_x * ctx.cos_r + ctx.rz_sin;
        let wz = ctx.world_z + rel_x * ctx.sin_r + ctx.rz_cos;
        (wx, wz)
    }

    /// 计算雾化系数（0.0 = 全雾, 1.0 = 无雾）
    #[inline]
    #[must_use]
    pub fn fog_factor(&self, depth: f32) -> f32 {
        let fog = 1.0 - depth * constants::INV_FOG_END;
        fog.clamp(constants::FOG_MIN_ALPHA, 1.0)
    }

    /// 静态逆投影 — 不依赖 Mode7Camera，预计算 trig/camera 值后调用
    ///
    /// 当多个 NPC 共享同一相机时，将 `cos_r`/`sin_r`/`cam_x`/`cam_z` 算好传一次即可。
    #[inline]
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn world_to_screen_with(
        wx: f32, wz: f32,
        cam_x: f32, cam_z: f32,
        cos_r: f32, sin_r: f32,
        height: f32, fov: f32,
    ) -> Option<(f32, f32, f32)> {
        let dx = wx - cam_x;
        let dz = wz - cam_z;
        let rel_x = dx * cos_r + dz * sin_r;
        let rel_z = -dx * sin_r + dz * cos_r;

        if rel_z <= 0.0 { return None; }

        let inv_z = 1.0 / rel_z;
        let sy = HORIZON_Y_PX + height * inv_z;
        let sx = HALF_SCREEN_W + rel_x * fov * inv_z;
        let scale = height * inv_z;

        if !(0.0..=SCREEN_H).contains(&sy) || !(0.0..=SCREEN_W).contains(&sx) { return None; }

        Some((sx, sy, scale))
    }
}
