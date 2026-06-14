//! 纹理缓存 — 复用 GPU 纹理句柄，避免每帧重新分配
//!
//! ## 设计原则
//! - Phase 1: Mode 7 渲染目标 (640×480) — 每帧覆盖像素 → 上传到 GPU
//! - Phase 2: 精灵图集 — 多帧程序化纹理打包
//! - Phase 6: 字体位图 — 8×8 字符纹理
//!
//! 关键是 **复用 Texture2D 句柄**，而非每帧 `Texture2D::from_image()` 创建新的。

use macroquad::prelude::*;

/// GPU 纹理缓存 — 每个渲染阶段一个槽位
pub struct TextureCache {
    /// Phase 1: Mode 7 渲染目标（每帧覆盖后复用上传）
    world_map_texture: Texture2D,
    /// Phase 1: CPU 端像素缓冲区（直接写 RGBA 字节）
    world_map_image: Image,
    /// Phase 2: 精灵图集（预留）
    sprite_atlas: Option<Texture2D>,
    /// Phase 6: 字体位图（预留）
    font_texture: Option<Texture2D>,
}

impl TextureCache {
    /// 创建纹理缓存，初始化 Mode 7 渲染目标
    ///
    /// 纹理滤镜设为 `FilterMode::Nearest` — GBA 像素风格
    pub fn new(render_w: u32, render_h: u32) -> Self {
        let image = Image::gen_image_color(
            render_w as u16,
            render_h as u16,
            Color::from_rgba(0, 0, 0, 255),
        );
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest);

        Self {
            world_map_texture: texture,
            world_map_image: image,
            sprite_atlas: None,
            font_texture: None,
        }
    }

    // ── Phase 1: Mode 7 渲染目标 ──

    /// 获取 CPU 端像素缓冲区的可变引用（写入 Mode 7 像素）
    pub fn world_map_image_mut(&mut self) -> &mut Image {
        &mut self.world_map_image
    }

    /// 获取 CPU 端像素缓冲区的不可变引用
    pub fn world_map_image(&self) -> &Image {
        &self.world_map_image
    }

    /// 将 CPU 端像素上传到 GPU 纹理（每帧渲染完成后调用一次）
    pub fn upload_world_map(&mut self) {
        self.world_map_texture = Texture2D::from_image(&self.world_map_image);
        self.world_map_texture.set_filter(FilterMode::Nearest);
    }

    /// 获取 Mode 7 材质（用于 `draw_texture()`）
    pub fn world_map_texture(&self) -> &Texture2D {
        &self.world_map_texture
    }

    /// 调整 Mode 7 渲染目标尺寸（窗口 resize / 分辨率切换时）
    pub fn resize_world_map(&mut self, w: u32, h: u32) {
        self.world_map_image = Image::gen_image_color(w as u16, h as u16, Color::from_rgba(0, 0, 0, 255));
        self.world_map_texture = Texture2D::from_image(&self.world_map_image);
        self.world_map_texture.set_filter(FilterMode::Nearest);
    }

    // ── Phase 2: 精灵图集 ──

    /// 注册精灵图集纹理
    pub fn set_sprite_atlas(&mut self, image: &Image) {
        let texture = Texture2D::from_image(image);
        texture.set_filter(FilterMode::Nearest);
        self.sprite_atlas = Some(texture);
    }

    /// 获取精灵图集
    pub fn sprite_atlas(&self) -> Option<&Texture2D> {
        self.sprite_atlas.as_ref()
    }

    // ── Phase 6: 字体位图 ──

    /// 注册字体纹理
    pub fn set_font_texture(&mut self, image: &Image) {
        let texture = Texture2D::from_image(image);
        texture.set_filter(FilterMode::Nearest);
        self.font_texture = Some(texture);
    }

    /// 获取字体纹理
    pub fn font_texture(&self) -> Option<&Texture2D> {
        self.font_texture.as_ref()
    }

    // ── 清理 ──

    /// 释放所有非必要纹理（场景切换时调用）
    pub fn clear_optional(&mut self) {
        self.sprite_atlas = None;
        self.font_texture = None;
    }
}
