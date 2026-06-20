use super::{GameCtx, ItemType, SpriteAtlas};

use golden_sun::battle::BattlePhase;
use golden_sun::constants::{self, RENDER_TARGET_W, RENDER_TARGET_H, TILE_SIZE};
use golden_sun::data::djinn;
use golden_sun::engine::GameState;
use golden_sun::entity::sprite::AnimState;
use golden_sun::entity::Entity;
use golden_sun::map::mode7;
use golden_sun::Mode7Camera;
use golden_sun::SceneId;
use macroquad::prelude::*;

impl GameCtx {
    pub(super) fn draw(&mut self) {
        clear_background(constants::BG_COLOR);

        match self.state {
            GameState::Title => self.draw_title(),
            GameState::WorldMap => {
                self.draw_world_map();
                self.draw_hud();
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
            GameState::PsynergyAnim { anim } => {
                let anim_copy = anim;
                self.draw_world_map();
                self.draw_psynergy_effect_from_data(anim_copy);
                #[cfg(debug_assertions)]
                self.draw_debug();
            }
            GameState::Battle => self.draw_battle(),
            GameState::BattleMenu { selection: sel } => {
                self.draw_battle();
                self.draw_battle_menu(&sel);
            }
            GameState::BattleItemSelect { selection: sel } => {
                self.draw_battle();
                self.draw_battle_item_select(&sel);
            }
            GameState::BattleSummonSelect { selection: sel } => {
                self.draw_battle();
                self.draw_summon_submenu(&sel);
            }
            GameState::Menu => {
                self.draw_world_map();
                self.draw_menu();
                #[cfg(debug_assertions)]
                self.draw_debug();
            }
            GameState::Travel { .. } => {
                self.draw_world_map();
                self.draw_travel_menu();
                #[cfg(debug_assertions)]
                self.draw_debug();
            }
            GameState::DjinnMenu { selection, page, character_select } => {
                self.draw_world_map();
                self.draw_djinn_menu(selection, page, character_select);
                #[cfg(debug_assertions)]
                self.draw_debug();
            }
            GameState::LevelUp { old_level, new_level, timer } => {
                self.draw_level_up(old_level, new_level, timer);
            }
            GameState::Inn { cost, timer, restored } => {
                self.draw_inn(cost, timer, restored);
            }
            GameState::Shop { npc_id, ref equipment_for_sale, ref items_for_sale, selection, tab, sell_selection, ref message, .. } => {
                self.draw_shop(npc_id, equipment_for_sale, items_for_sale, selection, tab, sell_selection, message.as_str());
            }
            GameState::Transition { kind, timer, .. } => {
                self.draw_world_map();
                golden_sun::ui::draw_transition(timer, kind);
            }
            GameState::DjinnObtained { name, element_color, timer, .. } => {
                self.draw_world_map();
                let alpha = if timer < 0.3 { timer / 0.3 }
                            else if timer > 1.5 { (2.0 - timer) / 0.5 }
                            else { 1.0 };
                if alpha > 0.0 {
                    let color = Color::new(element_color.0, element_color.1, element_color.2, alpha);
                    draw_text(format!("获得了 Djinn：{name}！"),
                        RENDER_TARGET_W as f32 / 2.0 - 80.0, RENDER_TARGET_H as f32 / 2.0, 18.0, color);
                }
            }
            GameState::Cutscene { .. } => {
                self.draw_world_map();
            }
            GameState::SceneName { name, timer } => {
                self.draw_world_map();
                let alpha = (1.0 - (timer / 1.5).min(1.0)).max(0.0);
                if alpha > 0.0 {
                    let color = Color::new(1.0, 1.0, 0.0, alpha);
                    draw_text(name, RENDER_TARGET_W as f32 / 2.0 - 60.0, 100.0, 24.0, color);
                }
            }
            GameState::GameOver { timer, has_save } => {
                let alpha = (timer / 2.0).min(1.0);
                draw_rectangle(0.0, 0.0, RENDER_TARGET_W as f32, RENDER_TARGET_H as f32,
                    Color::from_rgba(80, 0, 0, (alpha * 200.0) as u8));

                if timer > 0.5 {
                    let text_alpha = ((timer - 0.5) / 1.5).min(1.0) * 255.0;
                    let color = Color::new(255.0, 50.0, 50.0, text_alpha / 255.0);
                    draw_text("GAME OVER", RENDER_TARGET_W as f32 / 2.0 - 60.0,
                        RENDER_TARGET_H as f32 / 2.0 - 20.0, 32.0, color);
                }

                if timer >= 2.0 {
                    let blink = (self.game_time * 3.0).sin().abs();
                    let prompt_alpha = (blink * 128.0 + 127.0) as u8;
                    let prompt_color = Color::new(200.0, 200.0, 200.0, prompt_alpha as f32 / 255.0);
                    if has_save {
                        draw_text("[A] 从存档继续  [B] 返回标题", RENDER_TARGET_W as f32 / 2.0 - 100.0,
                            RENDER_TARGET_H as f32 / 2.0 + 30.0, 14.0, prompt_color);
                    } else {
                        draw_text("[B] 返回标题", RENDER_TARGET_W as f32 / 2.0 - 40.0,
                            RENDER_TARGET_H as f32 / 2.0 + 30.0, 14.0, prompt_color);
                    }
                }
            }
            _ => self.draw_placeholder(),
        }
    }

    fn draw_title(&self) {
        let has_save = self._storage.exists("save");
        golden_sun::ui::draw_title_enhanced(self.game_time, has_save);
    }

    fn draw_placeholder(&self) {
        draw_text("Phase pending...", 10.0, 30.0, 24.0, constants::PLACEHOLDER_TEXT_COLOR);
        draw_text(format!("State: {:?}", self.state), 10.0, 60.0, 16.0, LIGHTGRAY);
        draw_text(format!("FPS: {}", get_fps()), 10.0, 80.0, 16.0, LIGHTGRAY);
    }

    fn draw_world_map(&mut self) {
        let scene_map = golden_sun::map::tilemap::get_scene_map(self.scene.current());
        if self.modified_tiles_current.is_empty() {
            mode7::render(&mut self.textures, &self.camera, |x, y| scene_map.get_tile(x, y), self.game_time);
        } else {
            let overlays = &self.modified_tiles_current;
            mode7::render(&mut self.textures, &self.camera, move |x, y| {
                overlays.get(&(x, y)).copied().unwrap_or_else(|| scene_map.get_tile(x, y))
            }, self.game_time);
        }
        self.textures.upload_world_map();
        draw_texture(self.textures.world_map_texture(), 0.0, 0.0, WHITE);

        // CRT 扫描线效果
        draw_crt_scanlines();

        // 天气粒子渲染
        self.particles.draw();

        // 森林场景落叶粒子
        if self.scene.current() == SceneId::KolimaForest
            && self.particles.count_by_kind(golden_sun::engine::particle::ParticleKind::Leaf) < 30
        {
            let leaves = golden_sun::engine::particle::generate_leaf_particles(3);
            for leaf in leaves {
                self.particles.push_particle(leaf);
            }
        }

        // Djinn 拾取附近的闪光粒子
        self.draw_djinn_hints();
        let px = self.camera.x.floor();
        let py = self.camera.y.floor();
        for entry in djinn::world_djinn() {
            let djinn_id = entry.0;
            let scene_name = entry.1;
            let tx = entry.2;
            let ty = entry.3;
            let current_name = match self.scene.current() {
                SceneId::Vale => "Vale",
                SceneId::WildForest => "WildForest",
                SceneId::Bilibin => "Bilibin",
                SceneId::KolimaForest => "KolimaForest",
                SceneId::Cave => "Cave",
                SceneId::SolSanctum => "SolSanctum",
                _ => "",
            };
            if scene_name != current_name { continue; }
            let collected = self.collected_djinn.iter().any(|d| d.djinn.id.as_str() == djinn_id.as_str());
            if collected { continue; }
            let dist_sq = (px - tx).powi(2) + (py - ty).powi(2);
            if dist_sq < 9.0
                && self.particles.count_by_kind(golden_sun::engine::particle::ParticleKind::Sparkle) < 10
            {
                let sparkles = golden_sun::engine::particle::generate_sparkle_particles(2);
                for mut sparkle in sparkles {
                    sparkle.x = (tx + 0.5) * constants::TILE_SIZE;
                    sparkle.y = (ty + 0.5) * constants::TILE_SIZE;
                    self.particles.push_particle(sparkle);
                }
            }
        }

        self.render_npcs();
        self.draw_djinn_hints();

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

            if d.is_finished() {
                draw_text("▼", self.config.width - 30.0, y + h - 10.0, 14.0, WHITE);
            }
        }
        // 绘制对话分支选择
        if let GameState::DialogueChoices { choices, .. } = self.state {
            let box_y = y - 40.0 - choices.len() as f32 * 22.0;
            let box_h = choices.len() as f32 * 22.0 + 10.0;
            draw_rectangle(10.0, box_y - 5.0, self.config.width - 20.0, box_h,
                Color::from_rgba(0, 0, 0, 220));
            draw_rectangle_lines(10.0, box_y - 5.0, self.config.width - 20.0, box_h,
                1.5, WHITE);
            for (i, choice) in choices.iter().enumerate() {
                let is_selected = i == self.dialogue_choice_selection;
                let color = if is_selected { YELLOW } else { LIGHTGRAY };
                let prefix = if is_selected { "▸ " } else { "  " };
                draw_text(format!("{prefix}{label}", prefix = prefix, label = choice.label),
                    20.0, box_y + i as f32 * 22.0, 16.0, color);
            }
            draw_text("Confirm to select / Cancel to close", 20.0, box_y - 10.0, 12.0, GRAY);
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

        let mut queue: Vec<(usize, NpcProj)> = Vec::with_capacity(self.npcs.len());
        for (npc_idx, npc) in self.npcs.iter().enumerate() {
            let Some((sx, sy, scale)) = Mode7Camera::world_to_screen_with(
                npc.pos.0, npc.pos.1, cam_x, cam_z, cos_r, sin_r, height, fov,
            ) else { continue; };
            if sx < 0.0 || sx > RENDER_TARGET_W as f32 || sy < 0.0 || sy > RENDER_TARGET_H as f32 {
                continue;
            }
            let frames = self.sprites.npc_tex(npc.anim_state);
            let frame_idx = npc.current_frame_index(frames.len());
            queue.push((npc_idx, NpcProj { sx, sy, scale, state: npc.anim_state, frame_idx }));
        }

        queue.sort_by(|a, b| a.1.scale.partial_cmp(&b.1.scale).unwrap_or(std::cmp::Ordering::Greater));

        // 检测哪些 NPC 在交互范围内，显示气泡
        let px = self.camera.x;
        let py = self.camera.y;
        let interactable: std::collections::HashSet<usize> = self.npcs.iter().enumerate().filter(|(_, npc)| {
            let nx = npc.pos.0 / TILE_SIZE;
            let ny = npc.pos.1 / TILE_SIZE;
            let dx = nx - px;
            let dy = ny - py;
            dx * dx + dy * dy <= constants::NPC_INTERACT_RANGE * constants::NPC_INTERACT_RANGE
        }).map(|(i, _)| i).collect();

        for (npc_idx, item) in &queue {
            let texes = self.sprites.npc_tex(item.state);
            let tex = &texes[item.frame_idx];
            let size = constants::SPRITE_SIZE as f32 * item.scale;
            draw_texture_ex(tex,
                item.sx - size * 0.5, item.sy - size, WHITE,
                DrawTextureParams { dest_size: Some(Vec2::new(size, size)), ..Default::default() },
            );
            // 在可交互 NPC 头顶绘制 "!" 气泡
            if interactable.contains(npc_idx) {
                let bubble_y = item.sy - size - 12.0 * item.scale;
                draw_text("!", item.sx - 4.0, bubble_y, 16.0, YELLOW);
            }
        }
    }

    fn draw_battle(&mut self) {
        let scene = self.scene.current();
        self.draw_battle_bg(scene);
        let Some(ref mut battle) = self.battle else { return };
        draw_text("⚔️ BATTLE ⚔️", 10.0, 20.0, 24.0, YELLOW);
        draw_text("━━━━━━━━━━━━━━━━━━━━━━━━━━━━", 10.0, 30.0, 14.0, GRAY);

        // 绘制战斗角色精灵
        let sprite_scale = 3.0;
        let sprite_w = constants::SPRITE_SIZE as f32 * sprite_scale;

        // 玩家精灵 — 左半屏
        for (i, combatant) in battle.party.iter().enumerate() {
            if combatant.is_alive() {
                let shake_offset = if battle.hit_shake > 0.0 && i == 0 {
                    (battle.hit_shake * 100.0).sin() * 6.0
                } else {
                    0.0
                };
                let sx = 60.0 + i as f32 * (sprite_w + 20.0) + shake_offset;
                let sy = 100.0;
                let tex = &self.battle_sprites[i];
                draw_texture_ex(tex, sx, sy, WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(sprite_w, sprite_w)),
                        ..Default::default()
                    });
                // HP 条
                let hp_ratio = combatant.hp as f32 / combatant.max_hp as f32;
                let bar_w = sprite_w;
                let bar_h = 6.0;
                let bar_y = sy + sprite_w + 4.0;
                draw_rectangle(sx, bar_y, bar_w, bar_h, DARKGRAY);
                let hp_color = if hp_ratio > 0.5 { GREEN } else if hp_ratio > 0.25 { YELLOW } else { RED };
                draw_rectangle(sx, bar_y, bar_w * hp_ratio, bar_h, hp_color);
                draw_text(combatant.name, sx, bar_y - 14.0, 14.0, WHITE);
            }
        }

        // 敌人精灵 — 右半屏
        for (i, enemy) in battle.enemies.iter().enumerate() {
            if enemy.is_alive() {
                let shake_offset = if battle.hit_shake > 0.0 && i == 0 {
                    (battle.hit_shake * 100.0).sin() * 6.0
                } else {
                    0.0
                };
                let col = i % 2;
                let row = i / 2;
                let sx = 400.0 + col as f32 * (sprite_w + 20.0) + shake_offset;
                let sy = 60.0 + row as f32 * (sprite_w + 10.0);
                let tex = &self.battle_sprites[2 + i];
                draw_texture_ex(tex, sx, sy, WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(sprite_w, sprite_w)),
                        ..Default::default()
                    });
                // HP 条
                let hp_ratio = enemy.hp as f32 / enemy.max_hp as f32;
                let bar_w = sprite_w;
                let bar_h = 6.0;
                let bar_y = sy + sprite_w + 4.0;
                draw_rectangle(sx, bar_y, bar_w, bar_h, DARKGRAY);
                let hp_color = if hp_ratio > 0.5 { GREEN } else if hp_ratio > 0.25 { YELLOW } else { RED };
                draw_rectangle(sx, bar_y, bar_w * hp_ratio, bar_h, hp_color);
                draw_text(enemy.name, sx, bar_y - 14.0, 14.0, WHITE);
            }
        }

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

        // 绘制伤害数字弹出
        for popup in &battle.popups {
            let elapsed = popup.timer;
            let alpha = if elapsed < 0.3 {
                elapsed / 0.3 * 255.0
            } else {
                (1.0 - (elapsed - 0.3) / 0.7) * 255.0
            }.clamp(0.0, 255.0);
            
            let color = if popup.modifier > 1.0 {
                // 克制 — 红色放大
                Color::new(1.0, 0.2, 0.2, alpha / 255.0)
            } else if popup.modifier < 1.0 {
                // 被克 — 黄色
                Color::new(1.0, 1.0, 0.2, alpha / 255.0)
            } else if popup.modifier == 0.0 {
                // 免疫 — 灰色
                Color::new(0.6, 0.6, 0.6, alpha / 255.0)
            } else {
                WHITE
            };
            
            let size = if popup.modifier > 1.0 { 24.0 } else { 18.0 };
            let label = if popup.modifier == 0.0 {
                "IMMUNE"
            } else if popup.modifier < 1.0 {
                "WEAK"
            } else {
                &popup.damage.to_string()
            };
            draw_text(label, popup.x, popup.y, size, color);
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
            // 半透明面板背景
            let panel_x = 120.0;
            let panel_y = 120.0;
            let panel_w = 400.0;
            let panel_h = 200.0;
            draw_rectangle(panel_x, panel_y, panel_w, panel_h, Color::from_rgba(0, 0, 0, 200));
            draw_rectangle_lines(panel_x, panel_y, panel_w, panel_h, 2.0, GREEN);

            let grade = Self::calculate_battle_grade(battle);
            draw_text("VICTORY!", 300.0, 140.0, 28.0, GREEN);
            draw_text(grade, 460.0, 140.0, 20.0, YELLOW);

            let reward = format!("EXP: {}  Coins: {}", battle.total_exp, battle.total_coins);
            draw_text(&reward, 200.0, 180.0, 18.0, LIGHTGRAY);

            // 每个队员的 EXP 和等级进度条
            let mut py = 210.0;
            for combatant in &battle.party {
                if combatant.is_alive() {
                    let exp_per = combatant.level * 3;
                    let max_exp = combatant.level * 20;
                    let progress = (exp_per as f32 / max_exp as f32).min(1.0);
                    draw_text(format!("{} Lv.{} EXP: {}/{}", combatant.name, combatant.level, exp_per, max_exp), 140.0, py, 14.0, WHITE);
                    draw_rectangle(140.0, py + 6.0, 200.0, 8.0, DARKGRAY);
                    draw_rectangle(140.0, py + 6.0, 200.0 * progress, 8.0, GREEN);
                    py += 30.0;
                }
            }

            draw_text("Press Confirm to continue", 250.0, 310.0, 14.0, GRAY);
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
        const MAIN_ITEMS: [&str; 7] = ["Continue", "Items", "Psynergy", "Status", "Save", "Travel", "Quit"];
        if self.menu_page == 0 {
            golden_sun::ui::draw_pause_menu(self.menu_selection, &MAIN_ITEMS);
        } else if self.menu_page == 1 {
            let item_names: Vec<String> = self.inventory.iter()
                .filter(|i| i.count > 0)
                .map(|i| format!("{} x{}", i.item_type.name(), i.count))
                .collect();
            if item_names.is_empty() {
                golden_sun::ui::draw_status_screen(self.pp, self.max_pp, self.gold, &["暂无道具"]);
            } else {
                let name_refs: Vec<&str> = item_names.iter().map(|s| s.as_str()).collect();
                golden_sun::ui::draw_status_screen(self.pp, self.max_pp, self.gold, &name_refs);
            }
        } else if self.menu_page == 2 || self.menu_page == 3 {
            let names: Vec<&str> = self.unlocked_psynergies[..self.unlocked_count]
                .iter().map(|p| match p {
                    golden_sun::PsynergyType::Whirlwind => "Whirlwind",
                    golden_sun::PsynergyType::Growth => "Growth",
                    golden_sun::PsynergyType::Freeze => "Freeze",
                    golden_sun::PsynergyType::Force => "Force",
                    golden_sun::PsynergyType::Catch => "Catch",
                    golden_sun::PsynergyType::Flash => "Flash",
                    golden_sun::PsynergyType::Reveal => "Reveal",
                }).collect();
            golden_sun::ui::draw_status_screen(self.pp, self.max_pp, self.gold, &names);
        }
        draw_text("Press Cancel to close", 10.0, self.config.height - 20.0, 14.0, GRAY);
    }

    /// 绘制传送菜单
    fn draw_travel_menu(&self) {
        if let GameState::Travel { selection } = &self.state {
            let wp_count = self.activated_waypoints.len();
            draw_rectangle(100.0, 80.0, 440.0, 320.0, Color::from_rgba(0, 0, 0, 200));
            draw_rectangle(100.0, 80.0, 440.0, 320.0, Color::from_rgba(60, 60, 60, 100));
            draw_rectangle_lines(100.0, 80.0, 440.0, 320.0, 2.0, WHITE);
            draw_text("== Travel ==", 150.0, 100.0, 22.0, YELLOW);
            for (i, wp) in self.activated_waypoints.iter().enumerate() {
                let y = 140.0 + i as f32 * 32.0;
                let color = if i + 1 == *selection { YELLOW } else { WHITE };
                draw_text(if i + 1 == *selection { "\u{25B8} " } else { "  " }, 150.0, y, 20.0, color);
                draw_text(&wp.name, 182.0, y, 20.0, color);
            }
            if wp_count == 0 {
                draw_text("No waypoints activated", 150.0, 160.0, 16.0, GRAY);
            }
            draw_text("Confirm to travel / Cancel to close", 100.0, 360.0, 12.0, GRAY);
        }
    }

    /// 绘制 Djinn 菜单
    fn draw_djinn_menu(&self, selection: usize, _page: usize, character_select: u32) {
        draw_rectangle(100.0, 80.0, 440.0, 320.0, Color::from_rgba(0, 0, 0, 200));
        draw_rectangle(100.0, 80.0, 440.0, 320.0, Color::from_rgba(60, 60, 60, 100));
        draw_rectangle_lines(100.0, 80.0, 440.0, 320.0, 2.0, WHITE);

        let char_name = if character_select == 0 { "Isaac" } else { "Garet" };
        draw_text(format!("== Djinn ({char_name}) ==",), 150.0, 100.0, 20.0, YELLOW);

        let djinn_count = self.collected_djinn.len();
        if djinn_count == 0 {
            draw_text("No Djinn collected yet", 150.0, 140.0, 16.0, GRAY);
        } else {
            for (i, od) in self.collected_djinn.iter().enumerate() {
                let y = 130.0 + i as f32 * 28.0;
                let is_selected = i == selection;
                let equipped = od.equipped;
                
                let prefix = if is_selected { "\u{25B8} " } else { "  " };
                let name_color = if equipped { GREEN } else { WHITE };
                let status = if equipped { " [EQUIPPED]" } else { "" };
                
                let element_color = match od.djinn.element() {
                    golden_sun::Element::Venus => Color::from_rgba(100, 200, 100, 255),
                    golden_sun::Element::Mercury => Color::from_rgba(100, 150, 255, 255),
                    golden_sun::Element::Mars => Color::from_rgba(255, 100, 100, 255),
                    golden_sun::Element::Jupiter => Color::from_rgba(200, 200, 100, 255),
                };
                
                draw_text(prefix, 150.0, y, 16.0, if is_selected { YELLOW } else { LIGHTGRAY });
                draw_text(format!("{}{}", od.djinn.name(), status), 170.0, y, 16.0, name_color);
                draw_text(format!("[{}]", od.djinn.element().as_str()), 350.0, y, 12.0, element_color);
            }
        }

        // 角色选择指示
        let sel_color = if character_select == 0 { GREEN } else { LIGHTGRAY };
        draw_text("[Isaac]", 150.0, 340.0, 14.0, sel_color);
        let sel_color2 = if character_select == 1 { GREEN } else { LIGHTGRAY };
        draw_text("[Garet]", 250.0, 340.0, 14.0, sel_color2);
        
        draw_text("Confirm: Toggle Equip / Cancel: Close", 100.0, 365.0, 12.0, GRAY);
    }

    /// 绘制精灵力施法特效（从 PsynergyAnim 数据）
    fn draw_psynergy_effect_from_data(&self, anim: golden_sun::PsynergyAnim) {
        use golden_sun::PsynergyType;
        let progress = anim.progress();

        // 将 tile 坐标转为屏幕坐标（Mode7 投影）
        let tile_px_x = anim.tx as f32 * constants::TILE_SIZE;
        let tile_px_y = anim.ty as f32 * constants::TILE_SIZE;
        let cam_x = self.camera.x * constants::TILE_SIZE;
        let cam_z = self.camera.y * constants::TILE_SIZE;
        let cos_r = self.camera.rotation.cos();
        let sin_r = self.camera.rotation.sin();

        let Some((sx, sy, scale)) = golden_sun::Mode7Camera::world_to_screen_with(
            tile_px_x, tile_px_y,
            cam_x, cam_z,
            cos_r, sin_r,
            self.camera.height, self.camera.fov,
        ) else { return; };

        let size = constants::SPRITE_SIZE as f32 * scale * 2.0;

        match anim.psynergy {
            PsynergyType::Whirlwind => {
                // 绿色螺旋粒子 — 随进度扩散
                for i in 0..8 {
                    let fi = i as f32;
                    let angle = (fi / 8.0) * std::f32::consts::TAU + progress * 6.0;
                    let radius = size * 0.3 * progress;
                    let px = sx + angle.cos() * radius;
                    let py = sy - size * 0.5 + angle.sin() * radius * 0.5;
                    let alpha = (1.0 - progress) * 200.0;
                    draw_circle(px, py, 3.0 * scale * (1.0 - progress * 0.5),
                        Color::new(0.0, 1.0, 0.0, alpha / 255.0));
                }
                // 中心白色闪光
                let flash_alpha = (1.0 - progress).powi(2) * 180.0;
                draw_circle(sx, sy - size * 0.3, size * 0.2,
                    Color::new(1.0, 1.0, 1.0, flash_alpha / 255.0));
            }
            PsynergyType::Growth => {
                // 棕色→绿色渐变，从地面向上生长
                let base_color = Color::new(
                    0.36 * (1.0 - progress) + 0.23 * progress,
                    0.24 * (1.0 - progress) + 0.63 * progress,
                    0.14 * (1.0 - progress) + 0.14 * progress,
                    progress * 220.0 / 255.0,
                );
                let grow_h = size * 0.8 * progress;
                draw_rectangle(sx - size * 0.1, sy - grow_h, size * 0.2, grow_h, base_color);
                // 顶部叶子
                let leaf_size = size * 0.15 * progress;
                draw_circle(sx, sy - grow_h, leaf_size,
                    Color::new(0.2, 0.8, 0.2, progress * 220.0 / 255.0));
            }
            PsynergyType::Freeze => {
                // 蓝色闪光 + 冰晶扩散
                let flash_alpha = (1.0 - progress).powi(2) * 200.0;
                draw_circle(sx, sy - size * 0.3, size * 0.5 * (0.5 + progress * 0.5),
                    Color::new(0.3, 0.6, 1.0, flash_alpha / 255.0));
                // 冰晶射线
                for i in 0..6 {
                    let angle = (i as f32 / 6.0) * std::f32::consts::TAU;
                    let ray_len = size * 0.4 * progress;
                    let end_x = sx + angle.cos() * ray_len;
                    let end_y = sy - size * 0.3 + angle.sin() * ray_len;
                    let alpha = (1.0 - progress * 0.5) * 180.0;
                    draw_line(sx, sy - size * 0.3, end_x, end_y, 2.0,
                        Color::new(0.5, 0.8, 1.0, alpha / 255.0));
                }
            }
            PsynergyType::Force => {
                // 屏幕震动效果 — 通过偏移整个画面模拟
                let shake = (1.0 - progress) * 8.0;
                let shake_x = (progress * 20.0).sin() * shake;
                let shake_y = (progress * 15.0).cos() * shake;
                // 橙色冲击波 — 用大圆圈模拟
                let wave_radius = size * 0.6 * progress;
                let alpha = (1.0 - progress) * 150.0;
                draw_circle(
                    sx + shake_x, sy - size * 0.3 + shake_y,
                    wave_radius,
                    Color::new(1.0, 0.5, 0.2, alpha / 255.0),
                );
            }
            PsynergyType::Catch => {
                // 金色星光闪烁
                for i in 0..5 {
                    let fi = i as f32;
                    let angle = (fi / 5.0) * std::f32::consts::TAU + progress * 3.0;
                    let radius = size * 0.25 * progress;
                    let px = sx + angle.cos() * radius;
                    let py = sy - size * 0.3 + angle.sin() * radius * 0.6;
                    let sparkle_size = 2.0 * scale * (1.0 - progress * 0.5 + 0.5);
                    let alpha = (1.0 - progress) * 255.0;
                    draw_circle(px, py, sparkle_size,
                        Color::new(1.0, 0.9, 0.3, alpha / 255.0));
                }
                // 中心向内的箭头效果
                let arrow_alpha = (1.0 - progress) * 120.0;
                draw_circle(sx, sy - size * 0.3, size * 0.15 * (1.0 + progress),
                    Color::new(1.0, 0.8, 0.2, arrow_alpha / 255.0));
            }
            PsynergyType::Flash => {
                // 全屏白色闪光，随进度衰减
                let screen_w = self.config.width;
                let screen_h = self.config.height;
                let flash_intensity = (1.0 - progress).powi(2);
                draw_rectangle(0.0, 0.0, screen_w, screen_h,
                    Color::new(1.0, 1.0, 1.0, flash_intensity));
            }
            PsynergyType::Reveal => {
                // 紫色光环向外扩散 — 用大圈模拟
                let ring_radius = size * 0.8 * progress;
                let alpha = (1.0 - progress) * 180.0;
                draw_circle(
                    sx, sy - size * 0.3,
                    ring_radius,
                    Color::new(0.7, 0.4, 0.9, alpha / 255.0),
                );
                // 内部发光
                let glow_alpha = (1.0 - progress) * 100.0;
                draw_circle(sx, sy - size * 0.3, size * 0.3 * (1.0 - progress * 0.5),
                    Color::new(0.8, 0.5, 1.0, glow_alpha / 255.0));
            }
        }
    }

    /// 游戏内 HUD
    fn draw_hud(&self) {
        golden_sun::ui::draw_hud(self.pp, self.max_pp, "Vale Village");
        // 右上角显示当前任务提示
        if let Some(hint) = self.quest_log.active_hint() {
            let loc_name = match self.scene.current() {
                golden_sun::SceneId::Vale => "Vale Village",
                golden_sun::SceneId::WildForest => "Wild Forest",
                golden_sun::SceneId::Bilibin => "Bilibin Town",
                golden_sun::SceneId::KolimaForest => "Kolima Forest",
                golden_sun::SceneId::Cave => "Dark Cave",
                golden_sun::SceneId::SolSanctum => "Sol Sanctum",
                _ => "Unknown",
            };
            golden_sun::ui::draw_hud(self.pp, self.max_pp, loc_name);
            draw_text(hint, self.config.width - 300.0, constants::HUD_Y + 18.0, 12.0, YELLOW);
        }
        // 右下角 NPC 小地图
        self.draw_minimap();
    }

    /// 右下角小地图 — 1:4 采样 + NPC/玩家标注
    fn draw_minimap(&self) {
        const MAP_W: i32 = 120;
        const MAP_H: i32 = 90;
        let map_x = self.config.width - MAP_W as f32 - 10.0;
        let map_y = self.config.height - MAP_H as f32 - 10.0;
        let scene_map = golden_sun::map::tilemap::get_scene_map(self.scene.current());
        let (mw, mh) = scene_map.size();
        let sample_x = (mw / (MAP_W / 4)).max(1);
        let sample_y = (mh / (MAP_H / 4)).max(1);

        // 半透明背景
        draw_rectangle(map_x - 2.0, map_y - 2.0, MAP_W as f32 + 4.0, MAP_H as f32 + 4.0,
            Color::from_rgba(0, 0, 0, 160));

        // 绘制缩略图（每4x4块采样一个tile）
        for my in 0..MAP_H {
            for mx in 0..MAP_W {
                let ty = my * sample_y / 4;
                let tx = mx * sample_x / 4;
                let tile = scene_map.get_tile(tx, ty);
                let c = tile.color();
                draw_rectangle(map_x + mx as f32, map_y + my as f32, 1.0, 1.0, c);
            }
        }

        // 玩家位置（红色点）
        let px = ((self.camera.x / mw as f32) * MAP_W as f32) as i32;
        let py = ((self.camera.y / mh as f32) * MAP_H as f32) as i32;
        if (0..MAP_W).contains(&px) && (0..MAP_H).contains(&py) {
            draw_rectangle(map_x + px as f32, map_y + py as f32, 2.0, 2.0, RED);
        }

        // NPC 位置（青色点）
        let cyan = Color::from_rgba(0, 255, 255, 255);
        for npc in &self.npcs {
            let nx = (npc.pos.0 / (mw as f32 * constants::TILE_SIZE)) * MAP_W as f32;
            let ny = (npc.pos.1 / (mh as f32 * constants::TILE_SIZE)) * MAP_H as f32;
            if nx >= 0.0 && nx < MAP_W as f32 && ny >= 0.0 && ny < MAP_H as f32 {
                draw_rectangle(map_x + nx, map_y + ny, 2.0, 2.0, cyan);
            }
        }

        // 边框
        draw_rectangle_lines(map_x - 2.0, map_y - 2.0, MAP_W as f32 + 4.0, MAP_H as f32 + 4.0,
            1.0, Color::from_rgba(200, 200, 200, 128));
    }

    #[allow(dead_code)]
    fn draw_battle_bg(&self, _scene: SceneId) {
        let scene = self.scene.current();
        let (r, g, b) = match scene {
            SceneId::Vale => (0.1, 0.3, 0.1),
            SceneId::WildForest => (0.05, 0.2, 0.05),
            SceneId::Bilibin => (0.3, 0.3, 0.2),
            SceneId::KolimaForest => (0.05, 0.25, 0.05),
            SceneId::Cave => (0.1, 0.1, 0.1),
            SceneId::SolSanctum => (0.2, 0.1, 0.3),
            _ => (0.1, 0.1, 0.2),
        };
        for y in (0..RENDER_TARGET_H as i32).step_by(2) {
            let t = y as f32 / RENDER_TARGET_H as f32;
            let alpha = t * 0.3 + 0.1;
            draw_line(0.0, y as f32, RENDER_TARGET_W as f32, y as f32, 1.0,
                Color::new(r + alpha, g + alpha, b + alpha, 1.0));
        }
        let ground_y = RENDER_TARGET_H as f32 * 0.6;
        draw_line(0.0, ground_y, RENDER_TARGET_W as f32, ground_y, 2.0, Color::new(0.3, 0.3, 0.3, 1.0));
    }

    fn draw_battle_menu(&self, selection: &usize) {
        const ACTIONS: &[&str] = &["Attack", "Defend", "Psynergy", "Summon", "Djinn", "Item", "Flee"];
        let menu_y = 340.0;
        draw_rectangle(0.0, menu_y, 300.0, 120.0, Color::from_rgba(0, 0, 0, 200));
        for (i, action) in ACTIONS.iter().enumerate() {
            let prefix = if i == *selection { "\u{25B8} " } else { "  " };
            let color = if i == *selection { YELLOW } else { WHITE };
            draw_text(format!("{prefix}{action}"), constants::BATTLE_MENU_X, menu_y + i as f32 * constants::BATTLE_MENU_LINE_H, 18.0, color);
        }
    }

    fn draw_battle_item_select(&self, selection: &usize) {
        let items: Vec<(&str, String)> = self.inventory.iter()
            .filter(|i| i.count > 0)
            .map(|i| (i.item_type.name(), format!("x{}", i.count)))
            .collect();
        if items.is_empty() {
            return;
        }
        let menu_y = 340.0;
        draw_rectangle(0.0, menu_y, 300.0, 120.0, Color::from_rgba(0, 0, 0, 200));
        draw_text("-- Items --", 20.0, menu_y + 2.0, 16.0, YELLOW);
        for (i, (name, count)) in items.iter().enumerate() {
            let prefix = if i == *selection { "\u{25B8} " } else { "  " };
            let color = if i == *selection { YELLOW } else { WHITE };
            draw_text(format!("{prefix}{name} {count}"), 20.0, menu_y + 22.0 + i as f32 * 20.0, 16.0, color);
        }
        draw_text("Cancel: Back", 20.0, menu_y + 100.0, 12.0, GRAY);
    }

    fn draw_summon_submenu(&self, selection: &usize) {
        use golden_sun::data::summon::all_summons;
        let summons = all_summons();
        let menu_y = 340.0;
        draw_rectangle(0.0, menu_y, 300.0, 140.0, Color::from_rgba(0, 0, 0, 200));
        draw_text("-- Summons --", 20.0, menu_y + 2.0, 16.0, YELLOW);

        let standby = if let Some(ref battle) = self.battle {
            battle.collect_standby_djinn_count(&self.collected_djinn)
        } else {
            0
        };

        for (i, summon) in summons.iter().enumerate() {
            let y = menu_y + 22.0 + i as f32 * 20.0;
            if i >= 7 { break; }
            let prefix = if i == *selection { "\u{25B8} " } else { "  " };
            let color = if i == *selection { YELLOW } else { WHITE };
            let can_use = standby >= summon.djinn_required as usize;
            let elem_color = match summon.element {
                golden_sun::Element::Venus => Color::from_rgba(100, 200, 100, 255),
                golden_sun::Element::Mercury => Color::from_rgba(100, 150, 255, 255),
                golden_sun::Element::Mars => Color::from_rgba(255, 100, 100, 255),
                golden_sun::Element::Jupiter => Color::from_rgba(200, 200, 100, 255),
            };
            let status = if can_use { "" } else { " [Need more Djinn]" };
            let display_color = if can_use { color } else { GRAY };
            draw_text(format!("{prefix}{}", summon.name), 20.0, y, 14.0, display_color);
            draw_text(format!("PP:{} Djinn:{}", summon.pp_cost, summon.djinn_required), 180.0, y, 10.0, elem_color);
            if !status.is_empty() && !can_use {
                draw_text(status, 320.0, y, 10.0, RED);
            }
        }
        draw_text(format!("Standby Djinn: {standby}"), 20.0, menu_y + 125.0, 10.0, LIGHTGRAY);
        draw_text("Cancel: Back", 20.0, menu_y + 130.0, 10.0, GRAY);
    }

    fn calculate_battle_grade(battle: &golden_sun::battle::Battle) -> &'static str {
        let turns = battle.turn_index.max(1) as u32;
        let total_damage: u32 = battle.results.iter().map(|r| r.damage).sum();
        let avg_damage = total_damage / turns.max(1);
        if turns <= 2 && avg_damage > 20 { "★★★" }
        else if turns <= 4 && avg_damage > 10 { "★★" }
        else { "★" }
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

    /// 绘制商店界面
    #[allow(clippy::too_many_arguments)]
    fn draw_shop(
        &self,
        _npc_id: u32,
        equipment_for_sale: &[usize],
        items_for_sale: &[ItemType],
        selection: usize,
        tab: usize,
        sell_selection: usize,
        message: &str,
    ) {
        let screen_w = RENDER_TARGET_W as f32;
        let screen_h = RENDER_TARGET_H as f32;

        // 半透明背景覆盖
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::from_rgba(0, 0, 0, 180));

        // 商店标题
        let title = match tab {
            0 => "━━ 商店 ━━ 购买装备 ━━",
            1 => "━━ 商店 ━━ 购买道具 ━━",
            2 => "━━ 商店 ━━ 出售物品 ━━",
            _ => "━━ 商店 ━━",
        };
        draw_text(title, screen_w / 2.0 - 80.0, 30.0, 16.0, YELLOW);

        // 标签页切换提示
        draw_text("[← →] 切换标签  [A] 购买/选择  [B] 返回", 20.0, screen_h - 20.0, 10.0, DARKGRAY);

        // 金币显示
        let gold_text = format!("金币: {}G", self.gold);
        draw_text(&gold_text, screen_w - 120.0, 30.0, 14.0, GOLD);

        // 商品列表
        let eqs = super::all_equipment();
        let start_y = 50.0;
        let line_h = 20.0;

        match tab {
            0 | 1 => {
                // 装备列表
                for (i, eq_idx) in equipment_for_sale.iter().enumerate() {
                    if *eq_idx >= eqs.len() { continue; }
                    let eq = &eqs[*eq_idx];
                    let y = start_y + i as f32 * line_h;
                    let selected = i == selection;

                    if selected {
                        draw_rectangle(30.0, y - 2.0, screen_w - 60.0, line_h, Color::from_rgba(80, 80, 150, 130));
                    }

                    let can_afford = self.gold >= eq.price;
                    let color = if !can_afford { DARKGRAY } else if selected { YELLOW } else { WHITE };
                    draw_text(eq.name, 40.0, y + 12.0, 12.0, color);

                    let bonus_str = format!("ATK+{} DEF+{} HP+{}", eq.atk_bonus, eq.def_bonus, eq.hp_bonus);
                    draw_text(&bonus_str, 200.0, y + 12.0, 10.0, LIGHTGRAY);

                    draw_text(format!("{}G", eq.price), screen_w - 100.0, y + 12.0, 12.0,
                        if can_afford { GOLD } else { RED });
                }

                // 道具列表
                let inv_offset = equipment_for_sale.len();
                for (i, item_type) in items_for_sale.iter().enumerate() {
                    let idx = inv_offset + i;
                    let y = start_y + idx as f32 * line_h;
                    let selected = idx == selection;

                    if selected {
                        draw_rectangle(30.0, y - 2.0, screen_w - 60.0, line_h, Color::from_rgba(80, 80, 150, 130));
                    }

                    let price = match item_type {
                        ItemType::Potion => 15,
                        ItemType::Ether => 20,
                        ItemType::GoldRing => 100,
                        ItemType::Elixir => 500,
                        ItemType::Antidote => 50,
                        ItemType::Nut => 100,
                    };
                    let color = if self.gold < price { DARKGRAY } else if selected { YELLOW } else { WHITE };
                    draw_text(item_type.name(), 40.0, y + 12.0, 12.0, color);
                    draw_text(format!("{price}G"), screen_w - 100.0, y + 12.0, 12.0, GOLD);
                }
            }
            2 => {
                // 出售模式：列出玩家物品栏
                let items_text: Vec<String> = self.inventory.iter()
                    .filter(|i| i.count > 0)
                    .map(|i| format!("{} x{}", i.item_type.name(), i.count))
                    .collect();

                for (i, text) in items_text.iter().enumerate() {
                    let y = start_y + i as f32 * line_h;
                    let selected = i == sell_selection;

                    if selected {
                        draw_rectangle(30.0, y - 2.0, screen_w - 60.0, line_h, Color::from_rgba(80, 80, 150, 130));
                    }

                    let color = if selected { YELLOW } else { WHITE };
                    draw_text(text, 40.0, y + 12.0, 12.0, color);
                    draw_text("出售", screen_w - 100.0, y + 12.0, 12.0, GOLD);
                }

                if items_text.is_empty() {
                    draw_text("没有可出售的物品", screen_w / 2.0 - 60.0, start_y + 20.0, 12.0, GRAY);
                }
            }
            _ => {}
        }

        // 选中商品的详细描述（底部信息栏）
        let desc_y = screen_h - 60.0;
        draw_rectangle(0.0, desc_y - 5.0, screen_w, 40.0, Color::from_rgba(0, 0, 0, 130));

        if !message.is_empty() {
            draw_text(message, screen_w / 2.0 - 80.0, screen_h / 2.0, 16.0, YELLOW);
        }
    }

    fn draw_level_up(&mut self, old_lv: u32, new_lv: u32, timer: f32) {
        if timer < 0.5 {
            let flash = (1.0 - timer / 0.5) * 0.7;
            draw_rectangle(0.0, 0.0,
                self.config.width, self.config.height,
                Color::new(1.0, 0.9, 0.3, flash));
        }

        if (0.3..2.0).contains(&timer) {
            let alpha = if timer < 0.5 {
                (timer - 0.3) / 0.2 * 255.0
            } else if timer > 1.8 {
                (2.0 - timer) / 0.2 * 255.0
            } else {
                255.0
            }.clamp(0.0, 255.0);

            let size = 32.0 + (timer - 0.3).sin() * 4.0;
            draw_text("LEVEL UP!",
                self.config.width / 2.0 - 80.0,
                180.0, size,
                Color::new(1.0, 0.9, 0.3, alpha / 255.0));

            draw_text(format!("Lv.{old_lv} → Lv.{new_lv}"),
                self.config.width / 2.0 - 60.0,
                230.0, 20.0,
                Color::new(1.0, 1.0, 0.8, alpha / 255.0));
        }

        if timer >= 1.5 {
            let alpha = (1.0 - (timer - 1.5) / 1.5).clamp(0.0, 1.0) * 255.0;
            let diff = new_lv - old_lv;
            let my = 270.0;
            draw_text(format!("HP +{}", diff * 8),
                self.config.width / 2.0 - 60.0, my, 16.0,
                Color::new(1.0, 0.3, 0.3, alpha / 255.0));
            draw_text(format!("ATK +{}", diff * 2),
                self.config.width / 2.0 - 60.0, my + 24.0, 16.0,
                Color::new(1.0, 0.5, 0.3, alpha / 255.0));
            draw_text(format!("DEF +{diff}"),
                self.config.width / 2.0 - 60.0, my + 48.0, 16.0,
                Color::new(0.3, 0.5, 1.0, alpha / 255.0));
        }

        if timer >= 3.0 {
            self.state = GameState::WorldMap;
        }
    }

    /// 绘制旅馆界面
    fn draw_inn(&self, cost: u32, timer: f32, restored: bool) {
        let screen_w = self.config.width;
        let screen_h = self.config.height;

        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::from_rgba(0, 0, 0, 153));

        if restored {
            draw_text("休息了一晚…", screen_w / 2.0 - 70.0, screen_h / 2.0 - 20.0, 16.0, WHITE);
            draw_text("HP 和 PP 完全恢复了！", screen_w / 2.0 - 90.0, screen_h / 2.0, 16.0, GREEN);
        } else {
            let progress = (timer / 0.5).min(1.0);
            let alpha = progress * 255.0;
            draw_text("住宿中…", screen_w / 2.0 - 40.0, screen_h / 2.0, 16.0,
                Color::new(1.0, 1.0, 1.0, alpha / 255.0));
        }

        draw_text(format!("- {cost}G"), screen_w / 2.0 - 20.0, screen_h / 2.0 + 30.0, 12.0, YELLOW);
    }

    fn draw_djinn_hints(&self) {
        let px = self.camera.x;
        let py = self.camera.y;
        let current = self.scene.current();

        for entry in djinn::world_djinn() {
            let djinn_id = entry.0;
            let scene_name = entry.1;
            let tx = entry.2;
            let ty = entry.3;
            let current_name = match current {
                SceneId::Vale => "Vale",
                SceneId::WildForest => "WildForest",
                SceneId::Bilibin => "Bilibin",
                SceneId::KolimaForest => "KolimaForest",
                SceneId::Cave => "Cave",
                SceneId::SolSanctum => "SolSanctum",
                _ => "",
            };
            if scene_name != current_name { continue; }

            let collected = self.collected_djinn.iter().any(|d| {
                d.djinn.id.as_str() == djinn_id.as_str()
            });
            if collected { continue; }

            let dist_sq = (px - tx).powi(2) + (py - ty).powi(2);
            if dist_sq < 9.0 {
                let blink = (self.game_time * 3.0).sin().abs();
                let (r, g, b) = match djinn_id.element() {
                    golden_sun::Element::Venus => (0.2, 1.0, 0.2),
                    golden_sun::Element::Mercury => (0.2, 0.5, 1.0),
                    golden_sun::Element::Mars => (1.0, 0.3, 0.2),
                    golden_sun::Element::Jupiter => (0.7, 0.2, 1.0),
                };
                let screen_x = RENDER_TARGET_W as f32 / 2.0;
                let screen_y = RENDER_TARGET_H as f32 * 0.5 - 50.0 - blink * 8.0;
                draw_circle(screen_x, screen_y, 3.0, Color::new(r, g, b, blink * 0.8));
            }
        }
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

/// CRT 扫描线滤镜 — 每偶数行覆盖半透明黑条
fn draw_crt_scanlines() {
    let h = constants::RENDER_TARGET_H as i32;
    for y in (0..h).step_by(2) {
        draw_rectangle(0.0, y as f32, constants::RENDER_TARGET_W as f32, 1.0,
            Color::from_rgba(0, 0, 0, 30));
    }
}
