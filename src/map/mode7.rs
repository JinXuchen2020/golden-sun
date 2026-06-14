//! Mode 7 逐行扫描渲染器

use crate::engine::constants::{self, HORIZON_RATIO, MAP_HEIGHT, MAP_WIDTH, RENDER_TARGET_H, RENDER_TARGET_W};
use crate::engine::mode7_camera::Mode7Camera;
use crate::engine::Camera;
use crate::engine::texture::TextureCache;
use crate::map::tilemap;
use macroquad::prelude::*;

/// 渲染 Mode 7 画面到 TextureCache
pub fn render(textures: &mut TextureCache, camera: &Camera) {
    let w = RENDER_TARGET_W as usize;
    let h = RENDER_TARGET_H as usize;
    let horizon_y = (h as f32 * HORIZON_RATIO) as usize;

    let image = textures.world_map_image_mut();
    // macroquad Image::bytes 是 Vec<u8>，RGBA 排列
    let pixels = &mut image.bytes;

    // 1. 天空渐变
    render_sky(pixels, w, h, horizon_y);

    // 2. 预计算 tile 颜色查找表（避免逐像素 get_tile + color 两次 match）
    let mut color_map = [[Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }; MAP_WIDTH as usize]; MAP_HEIGHT as usize];
    for (y, row) in color_map.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            *cell = tilemap::get_tile(x as i32, y as i32).color();
        }
    }

    // 3. Mode 7 地面
    let m7 = Mode7Camera::new(camera);
    let top_r = constants::SKY_COLOR_HORIZON.r * 255.0;
    let top_g = constants::SKY_COLOR_HORIZON.g * 255.0;
    let top_b = constants::SKY_COLOR_HORIZON.b * 255.0;
    let mw = MAP_WIDTH as i32;
    let mh = MAP_HEIGHT as i32;

    for sy in horizon_y..h {
        let sy_f = sy as f32;
        let ctx = match m7.prepare_scanline(sy_f, w as f32, sy_f) {
            Some(c) => c,
            None => continue,
        };

        let fog = m7.fog_factor(ctx.z);
        let fog_scale = fog * 255.0;
        let inv = 1.0 - fog;
        let tr = top_r * inv;
        let tg = top_g * inv;
        let tb = top_b * inv;
        let row_start = sy * w * 4;

        for sx in 0..w {
            let sx_f = sx as f32;
            let (world_x, world_z) = Mode7Camera::project_pixel(&ctx, sx_f);

            let tx = super::world_to_tile_index(world_x);
            let tz = super::world_to_tile_index(world_z);
            let tc = if tx >= 0 && tx < mw && tz >= 0 && tz < mh {
                color_map[tz as usize][tx as usize]
            } else {
                Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
            };

            let idx = row_start + sx * 4;
            pixels[idx]     = (tc.r * fog_scale + tr) as u8;
            pixels[idx + 1] = (tc.g * fog_scale + tg) as u8;
            pixels[idx + 2] = (tc.b * fog_scale + tb) as u8;
            pixels[idx + 3] = 255;
        }
    }
}

/// 填充天空渐变
fn render_sky(pixels: &mut [u8], w: usize, h: usize, horizon_y: usize) {
    let top = constants::SKY_COLOR_TOP;
    let hoz = constants::SKY_COLOR_HORIZON;

    for y in 0..horizon_y.min(h) {
        let t = y as f32 / horizon_y.max(1) as f32;
        let r = ((top.r + (hoz.r - top.r) * t) * 255.0) as u8;
        let g = ((top.g + (hoz.g - top.g) * t) * 255.0) as u8;
        let b = ((top.b + (hoz.b - top.b) * t) * 255.0) as u8;

        let row_start = y * w * 4;
        let row = &mut pixels[row_start..row_start + w * 4];
        for px in row.chunks_exact_mut(4) {
            px[0] = r;
            px[1] = g;
            px[2] = b;
            px[3] = 255;
        }
    }
}
