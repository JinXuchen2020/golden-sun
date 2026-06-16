use super::{GameCtx, SpriteAtlas};

use golden_sun::battle::BattlePhase;
use golden_sun::constants::{self, RENDER_TARGET_W, RENDER_TARGET_H, TILE_SIZE};
use golden_sun::engine::GameState;
use golden_sun::entity::sprite::AnimState;
use golden_sun::entity::Entity;
use golden_sun::map::{mode7, tilemap};
use golden_sun::Mode7Camera;
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
            GameState::Dialog => {
                self.draw_world_map();
                self.draw_dialogue_box();
                #[cfg(debug_assertions)]
                self.draw_debug();
            }
            GameState::Psynergy => {
                self.draw_world_map();
                self.draw_psynergy_ui();
                #[cfg(debug_assertions)]
                self.draw_debug();
            }
            GameState::Battle => self.draw_battle(),
            GameState::Menu => {
                self.draw_world_map();
                self.draw_menu();
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
        if self.modified_tiles.is_empty() {
            mode7::render(&mut self.textures, &self.camera, tilemap::get_tile);
        } else {
            let overlays = &self.modified_tiles;
            mode7::render(&mut self.textures, &self.camera, |x, y|
                overlays.get(&(x, y)).copied().unwrap_or_else(|| tilemap::get_tile(x, y))
            );
        }
        self.textures.upload_world_map();
        draw_texture(self.textures.world_map_texture(), 0.0, 0.0, WHITE);

        self.render_npcs();

        let screen_x = self.config.width * 0.5;
        let screen_y = self.config.height - constants::SCREEN_MARGIN_BOTTOM;
        draw_player_sprite(&self.sprites, &self.player_entity, screen_x, screen_y);
    }

    /// 精灵力选择 UI：屏幕底部快捷栏
    fn draw_psynergy_ui(&self) {
        const BAR_H: f32 = 40.0;
        const BAR_MARGIN: f32 = 50.0;
        const ICON_SIZE: f32 = 28.0;
        const ICON_GAP: f32 = 40.0;
        const LABEL_OFFSET_X: f32 = 60.0;

        let bar_y = self.config.height - BAR_MARGIN;
        let mut bar_x = 10.0;

        draw_rectangle(0.0, bar_y, self.config.width, BAR_H, Color::from_rgba(0, 0, 0, 180));
        draw_text("精灵力", bar_x, bar_y - 8.0, 16.0, Color::from_rgba(200, 220, 255, 255));

        bar_x += LABEL_OFFSET_X;
        let unlocked = &self.unlocked_psynergies[..self.unlocked_count];
        for (i, psynergy) in unlocked.iter().enumerate() {
            let is_selected = i == self.selected_psynergy;
            let is_affordable = self.pp >= psynergy.pp_cost();

            if is_selected {
                draw_rectangle(bar_x - 4.0, bar_y + 4.0, ICON_SIZE + 8.0, ICON_SIZE + 4.0,
                    Color::from_rgba(255, 220, 60, 180));
            }

            let (r, g, b) = psynergy.icon_color();
            let icon_color = Color::from_rgba(r, g, b, 255);
            let text_color = if is_affordable { WHITE } else { GRAY };

            draw_rectangle(bar_x, bar_y + 8.0, ICON_SIZE, ICON_SIZE - 4.0, icon_color);
            draw_text(psynergy.name(), bar_x + 8.0, bar_y + 26.0, 16.0, text_color);
            draw_text(psynergy.pp_label(), bar_x, bar_y + 38.0, 10.0, text_color);

            bar_x += ICON_GAP;
        }

        let pp_text = format!("PP: {}/{}", self.pp, self.max_pp);
        draw_text(&pp_text, self.config.width - 120.0, bar_y + 26.0, 18.0,
            Color::from_rgba(100, 200, 255, 255));
    }

    fn draw_dialogue_box(&self) {
        let y = constants::DIALOGUE_BOX_Y;
        let h = constants::DIALOGUE_BOX_H;
        draw_rectangle(0.0, y, self.config.width, h, Color::from_rgba(0, 0, 0, 210));
        draw_rectangle_lines(0.0, y, self.config.width, h, 2.0, WHITE);

        if let Some(ref d) = self.dialogue {
            let display = d.visible_text();
            draw_text(display, constants::DIALOGUE_TEXT_X, constants::DIALOGUE_TEXT_Y,
                constants::DIALOGUE_TEXT_SIZE, WHITE);

            if d.finished {
                draw_text("▼", self.config.width - 30.0, y + h - 10.0, 14.0, WHITE);
            }
        }
    }

    fn render_npcs(&self) {
        struct NpcProj {
            sx: f32, sy: f32, scale: f32,
            state: AnimState,
            frame_idx: usize,
        }

        let cam_x = self.camera.x * TILE_SIZE;
        let cam_z = self.camera.y * TILE_SIZE;
        let cos_r = self.camera.rotation.cos();
        let sin_r = self.camera.rotation.sin();
        let height = self.camera.height;
        let fov = self.camera.fov;

        let mut queue: Vec<NpcProj> = Vec::with_capacity(self.npcs.len());
        for npc in &self.npcs {
            let Some((sx, sy, scale)) = Mode7Camera::world_to_screen_with(
                npc.pos.0, npc.pos.1, cam_x, cam_z, cos_r, sin_r, height, fov,
            ) else { continue; };
            if sx < 0.0 || sx > RENDER_TARGET_W as f32 || sy < 0.0 || sy > RENDER_TARGET_H as f32 {
                continue;
            }
            let frames = self.sprites.npc_tex(npc.anim_state);
            let frame_idx = npc.current_frame_index(frames.len());
            queue.push(NpcProj { sx, sy, scale, state: npc.anim_state, frame_idx });
        }

        queue.sort_by(|a, b| a.scale.partial_cmp(&b.scale).unwrap_or(std::cmp::Ordering::Greater));

        for item in &queue {
            let texes = self.sprites.npc_tex(item.state);
            let tex = &texes[item.frame_idx];
            let size = constants::SPRITE_SIZE as f32 * item.scale;
            draw_texture_ex(tex,
                item.sx - size * 0.5, item.sy - size, WHITE,
                DrawTextureParams { dest_size: Some(Vec2::new(size, size)), ..Default::default() },
            );
        }
    }

    #[cfg(debug_assertions)]
    fn draw_battle(&self) {
        let Some(ref battle) = self.battle else { return; };
        draw_text("⚔️ BATTLE ⚔️", 10.0, 20.0, 24.0, YELLOW);
        draw_text("━".repeat(28), 10.0, 30.0, 14.0, GRAY);

        let mut y = constants::BATTLE_ENEMY_NAME_Y;
        for e in &battle.enemies {
            let name = if e.is_alive() { e.name } else { "【DEAD】" };
            let hp_bar = "█".repeat((e.hp * 10 / e.max_hp.max(1)) as usize);
            let hp_empty = "░".repeat((10 - (e.hp * 10 / e.max_hp.max(1))) as usize);
            let hp_line = format!("{name}  HP:{hp}/{max_hp}  {hp_bar}{hp_empty}", hp = e.hp, max_hp = e.max_hp);
            draw_text(&hp_line, 30.0, y, 16.0, if e.is_alive() { RED } else { DARKGRAY });
            y += 22.0;
        }

        y = constants::BATTLE_LOG_Y;
        let start = battle.logs.len().saturating_sub(constants::BATTLE_LOG_MAX);
        for log in battle.logs.iter().skip(start) {
            draw_text(log, 10.0, y, constants::BATTLE_LOG_SIZE, WHITE);
            y += 18.0;
        }

        if battle.phase == BattlePhase::PlayerInput {
            draw_rectangle(0.0, 340.0, 300.0, 120.0, Color::from_rgba(0, 0, 0, 200));
            let actions = ["A: Attack", "D: Defend", "P: Psynergy", "F: Flee"];
            let mut my = constants::BATTLE_MENU_Y;
            for a in actions {
                draw_text(a, constants::BATTLE_MENU_X, my, 18.0, WHITE);
                my += constants::BATTLE_MENU_LINE_H;
            }
        }

        if battle.phase == BattlePhase::Victory {
            draw_text("VICTORY!", 200.0, 200.0, 36.0, GREEN);
            let reward = format!("EXP: {}  Coins: {}", battle.total_exp, battle.total_coins);
            draw_text(&reward, 200.0, 240.0, 20.0, LIGHTGRAY);
            draw_text("Press Confirm to continue", 200.0, 280.0, 16.0, GRAY);
        }
        if battle.phase == BattlePhase::Defeat {
            draw_text("DEFEATED...", 200.0, 200.0, 36.0, RED);
            draw_text("Press Confirm to continue", 200.0, 260.0, 16.0, GRAY);
        }
        if battle.phase == BattlePhase::FleeSuccess {
            draw_text("You fled!", 200.0, 200.0, 36.0, YELLOW);
            draw_text("Press Confirm to continue", 200.0, 260.0, 16.0, GRAY);
        }
    }

    fn draw_menu(&self) {
        draw_rectangle(0.0, 0.0, self.config.width, self.config.height,
            Color::from_rgba(0, 0, 0, 180));
        draw_text("MENU", 10.0, 30.0, 24.0, WHITE);
        let pp_line = format!("PP: {}/{}", self.pp, self.max_pp);
        draw_text(&pp_line, 10.0, 70.0, 18.0, LIGHTGRAY);
        draw_text("Press Cancel to close", 10.0, self.config.height - 20.0, 14.0, GRAY);
    }

    fn draw_debug(&self) {
        let (wx, wy) = self.camera.world_pos();
        let (tx, ty) = self.camera.tile_index();
        draw_text(
            format!("FPS: {} | Tile: ({},{}) | World: ({:.0},{:.0}) | Rot: {:.2} | PP: {}/{}",
                get_fps(), tx, ty, wx, wy, self.camera.rotation, self.pp, self.max_pp),
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
