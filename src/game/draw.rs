use super::{GameCtx, SpriteAtlas};

use golden_sun::constants::{self, RENDER_TARGET_W, RENDER_TARGET_H, SPRITE_SIZE, TILE_SIZE};
use golden_sun::engine::GameState;
use golden_sun::entity::Entity;
use macroquad::prelude::*;

impl GameCtx {
    pub(super) fn draw(&mut self) {
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
        draw_text("按 Z / Enter 开始", 100.0, 260.0, 20.0, constants::TITLE_TEXT_COLOR);
    }

    fn draw_placeholder(&self) {
        draw_text("Phase pending...", 10.0, 30.0, 24.0, constants::PLACEHOLDER_TEXT_COLOR);
        draw_text(format!("State: {:?}", self.state), 10.0, 60.0, 16.0, LIGHTGRAY);
        draw_text(format!("FPS: {}", get_fps()), 10.0, 80.0, 16.0, LIGHTGRAY);
    }

    fn draw_world_map(&mut self) {
        golden_sun::map::mode7::render(&mut self.textures, &self.camera);

        let cam_x = self.camera.x * TILE_SIZE;
        let cam_z = self.camera.y * TILE_SIZE;
        let cos_r = self.camera.rotation.cos();
        let sin_r = self.camera.rotation.sin();
        let height = self.camera.height;
        let fov = self.camera.fov;
        let image = self.textures.world_map_image_mut();
        let pixels = &mut image.bytes;
        let w = RENDER_TARGET_W as usize;
        let h = RENDER_TARGET_H as usize;

        struct NpcRender<'a> {
            sx: f32,
            sy: f32,
            scale: f32,
            frames: &'a Vec<Vec<u8>>,
            frame_idx: usize,
        }

        let mut render_queue: Vec<NpcRender> = Vec::with_capacity(self.npcs.len());

        for npc in &self.npcs {
            if let Some((sx, sy, scale)) = golden_sun::Mode7Camera::world_to_screen_with(
                npc.pos.0, npc.pos.1, cam_x, cam_z, cos_r, sin_r, height, fov,
            ) {
                let sx_i = sx as usize;
                let sy_i = sy as usize;
                if sx_i >= w || sy_i >= h { continue; }

                let frames = self.sprites.npc_frame(npc.anim_state);
                let frame_idx = npc.current_frame_index(frames.len());
                render_queue.push(NpcRender {
                    sx, sy, scale,
                    frames,
                    frame_idx,
                });
            }
        }

        render_queue.sort_by(|a, b| a.scale.partial_cmp(&b.scale).unwrap_or(std::cmp::Ordering::Greater));

        for item in &render_queue {
            let raw = &item.frames[item.frame_idx];

            let draw_h = (SPRITE_SIZE as f32 * item.scale).max(4.0) as usize;
            let draw_w = (SPRITE_SIZE as f32 * item.scale).max(4.0) as usize;
            let ratio_x = (constants::SPRITE_SIZE as f32) / draw_w as f32;
            let ratio_y = (constants::SPRITE_SIZE as f32) / draw_h as f32;

            let step_x = ratio_x * 4.0;
            let step_y = ratio_y * (constants::SPRITE_SIZE as f32) * 4.0;
            let sx_i = item.sx as usize;
            let sy_i = item.sy as usize;

            for dy in 0..draw_h.min(h - sy_i) {
                let src_off = (dy as f32 * step_y) as usize;
                if src_off + 4 > raw.len() { break; }
                let src_row = &raw[src_off..];
                let mut dst_i = ((sy_i + dy) * w + sx_i) * 4;
                for dx in 0..draw_w.min(w - sx_i) {
                    let src_i = (dx as f32 * step_x) as usize;
                    // 安全边界：src_i + 2 必须在 src_row 范围内
                    if src_i + 2 >= src_row.len() { break; }
                    pixels[dst_i]     = src_row[src_i];
                    pixels[dst_i + 1] = src_row[src_i + 1];
                    pixels[dst_i + 2] = src_row[src_i + 2];
                    pixels[dst_i + 3] = 255;
                    dst_i += 4;
                }
            }
        }

         self.textures.upload_world_map();
        draw_texture(self.textures.world_map_texture(), 0.0, 0.0, WHITE);

        let screen_x = self.config.width * 0.5;
        let screen_y = self.config.height - constants::SCREEN_MARGIN_BOTTOM;
        draw_player_sprite(&self.sprites, &self.player_entity, screen_x, screen_y);
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

fn draw_player_sprite(atlas: &SpriteAtlas, entity: &Entity, x: f32, y: f32) {
    let texes = atlas.player_tex(entity.anim_state);
    let frame_idx = entity.current_frame_index(texes.len());
    let tex = &texes[frame_idx];
    let s = constants::SPRITE_SIZE as f32;
    draw_texture_ex(tex, x - s * 0.5, y - s, WHITE, DrawTextureParams {
        dest_size: Some(Vec2::new(s, s)),
        ..Default::default()
    });
}
