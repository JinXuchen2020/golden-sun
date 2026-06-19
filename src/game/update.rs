use super::GameCtx;
use super::WaypointDef;

use golden_sun::battle::BattleAction;
use golden_sun::battle::BattlePhase;
use golden_sun::constants::{self, TILE_SIZE};
use golden_sun::dialogue::DialogueState;
use golden_sun::engine::{Camera, GameState, PsynergyAnim};
use golden_sun::replay;
use golden_sun::entity::sprite::AnimState;
use golden_sun::entity::{Direction, Entity, WalkPattern};
use golden_sun::map::{self, TileKind};
use golden_sun::psynergy::effects;
use golden_sun::{PsynergyType, InputEvent};
use macroquad::prelude::{is_key_down, is_key_pressed, KeyCode};

/// Debug 日志（release 模式编译消除）
macro_rules! dbg {
    ($($arg:tt)*) => { #[cfg(debug_assertions)] { eprintln!($($arg)*); } }
}

impl GameCtx {
    pub(super) fn update(&mut self) {
        self.time.poll();
        self.input.poll();
        self.input_bus.poll(&self.input);
        // 录制回放
        replay::record_frame(&self.input);

        debug_assert!(self.camera.validate(), "Camera 参数无效 — height={}, fov={}",
            self.camera.height, self.camera.fov);

        self.scene.commit_switch();

        // 场景过渡：推进计时器，解除借用后再改 state
        let transition_finished = if let GameState::Transition { ref mut timer, .. } = self.state {
            *timer += self.time.delta;
            *timer >= 1.0
        } else {
            false
        };
        if transition_finished {
            if self.encounter_pending {
                self.encounter_pending = false;
                self.start_random_battle();
                return;
            }
            // 处理场景切换
            if self.pending_scene.is_some() {
                self.apply_scene_switch();
            }
            self.state = GameState::WorldMap;
            return;
        }
        if self.state.is_transition() {
            return;
        }

        // 对话分支选择 — 特殊处理（需要 mutable + 借用 script）
        if matches!(self.state, GameState::DialogueChoices { .. }) {
            let choices = match &self.state {
                GameState::DialogueChoices { choices, .. } => *choices,
                _ => &[],
            };
            let num_choices = choices.len();
            if self.input_bus.consume(InputEvent::Up) {
                self.dialogue_choice_selection = self.dialogue_choice_selection.saturating_sub(1);
            }
            if self.input_bus.consume(InputEvent::Down) {
                self.dialogue_choice_selection = (self.dialogue_choice_selection + 1).min(num_choices - 1);
            }
            if self.input_bus.consume(InputEvent::Cancel) {
                self.play_sfx("cancel");
                self.dialogue = None;
                self.current_dialogue_script = None;
                self.state = GameState::WorldMap;
                return;
            }
            if self.input_bus.consume(InputEvent::Confirm) && num_choices > 0 {
                self.handle_choice_selection();
            }
            return;
        }

        match self.state {
            GameState::Title => {
                if self.input_bus.consume(InputEvent::Confirm)
                    || self.input_bus.consume(InputEvent::Menu)
                {
                    self.camera = Camera::new(super::PLAYER_START_X, super::PLAYER_START_Y);
                    self.player_entity = Entity::new_player(
                        Entity::tile_to_world(super::PLAYER_START_X, super::PLAYER_START_Y));
                    self.state = GameState::WorldMap;
                }
                // Secondary → 尝试读档
                if self.input_bus.consume(InputEvent::Secondary) && !self.load_game() {
                    #[cfg(debug_assertions)]
                    eprintln!("没有存档");
                }
            }
            GameState::WorldMap => {
                let moving = self.update_player();
                self.update_npcs();
                self.camera.update_lerp(self.time.delta);
                self.recover_pp(moving);
                self.trigger_random_encounter(moving);
                self.check_scene_boundaries();
                self.check_waypoints();
                self.check_djinn_pickup();
                self.game_time += self.time.delta;
                self.particles.spawn(self.time.delta, golden_sun::engine::particle::ParticleKind::Rain);
                self.particles.update(self.time.delta);
                self.track_quest_progress();

                if self.input_bus.consume(InputEvent::Confirm) {
                    if let Some(npc) = self.find_nearby_npc() {
                        if let Some(ref text) = npc.dialogue_id {
                            if let Some(s) = golden_sun::dialogue::script::get_script(text) {
                                if let Some(flag) = s.start_flag {
                                    self.story_flags.set(flag);
                                }
                                // 完成"初遇村民"任务
                                self.quest_log.complete("intro_talk");
                                let text = s.pages[0].lines[0].text.to_string();
                                self.dialogue = Some(DialogueState::new(text));
                            } else {
                                self.dialogue = Some(DialogueState::new(text.clone()));
                            }
                        }
                        self.state = GameState::Dialog;
                    }
                }

                // B/Secondary → 进入精灵力选择
                if self.input_bus.consume(InputEvent::Secondary) && self.unlocked_count > 0 {
                    self.play_sfx("cancel");
                    self.state = GameState::Psynergy;
                    self.selected_psynergy = 0;
                }

                if self.input_bus.consume(InputEvent::Menu) {
                    self.play_sfx("cancel");
                    self.state = GameState::Menu;
                }

                // Debug: R 开始/停止录制，P 开始回放
                if is_key_down(KeyCode::R) {
                    if !replay::has_recordings() {
                        replay::start_recording();
                    } else {
                        replay::stop_recording();
                    }
                }
                if is_key_pressed(KeyCode::P) && replay::has_recordings() {
                    replay::start_playback();
                }
                if replay::has_recordings() {
                    if let Some(frame) = replay::current_frame() {
                        self.input.up = frame.up;
                        self.input.down = frame.down;
                        self.input.left = frame.left;
                        self.input.right = frame.right;
                        self.input.a = frame.a;
                        self.input.b = frame.b;
                        self.input.start = frame.start;
                        replay::advance_playback();
                    }
                }
            }
            GameState::Dialog => {
                if let Some(ref mut d) = self.dialogue {
                    if !d.is_finished() {
                        d.advance(self.time.delta, constants::DIALOGUE_CHAR_SPEED);
                    }
                    if self.input_bus.consume(InputEvent::Confirm) {
                        if d.is_finished() {
                            self.play_sfx("confirm");
                            // 如果有选项，显示选择 UI
                            if let Some(ref script) = self.current_dialogue_script {
                                if !script.pages.is_empty() {
                                    let page = &script.pages[0];
                                    if !page.choices.is_empty() {
                                        self.state = GameState::DialogueChoices { choices: page.choices, script: script.clone() };
                                        self.dialogue_choice_selection = 0;
                                        return;
                                    }
                                }
                            }
                            self.dialogue = None;
                            self.state = GameState::WorldMap;
                        } else {
                            d.skip();
                        }
                    }
                    if self.input_bus.consume(InputEvent::Cancel) {
                        self.play_sfx("cancel");
                        self.dialogue = None;
                        self.state = GameState::WorldMap;
                    }
                }
            }
            GameState::Psynergy => {
                // 左/右切换精灵力
                if self.input_bus.consume(InputEvent::Left) {
                    self.selected_psynergy = (self.selected_psynergy + self.unlocked_count - 1) % self.unlocked_count;
                }
                if self.input_bus.consume(InputEvent::Right) {
                    self.selected_psynergy = (self.selected_psynergy + 1) % self.unlocked_count;
                }

                // A 确认使用
                if self.input_bus.consume(InputEvent::Confirm) {
                    self.try_use_selected_psynergy();
                }

                // B 取消
                if self.input_bus.consume(InputEvent::Cancel) {
                    self.play_sfx("cancel");
                    self.state = GameState::WorldMap;
                }
            }
            GameState::PsynergyAnim { anim } => {
                let mut anim_data = anim;
                // 动画期间锁定所有输入
                anim_data.timer += self.time.delta;
                if anim_data.is_finished() {
                    // 动画结束，执行 tile 修改
                    if let Some((psynergy, tx, ty)) = self.pending_psynergy.take() {
                        self.execute_psynergy_effect(psynergy, tx, ty);
                    }
                    self.state = GameState::WorldMap;
                } else {
                    self.state = GameState::PsynergyAnim { anim: anim_data };
                }
            }
            GameState::Travel { ref mut selection } => {
                let wp_count = self.activated_waypoints.len();
                if self.input_bus.consume(InputEvent::Up) {
                    *selection = selection.saturating_sub(1);
                }
                if self.input_bus.consume(InputEvent::Down) {
                    *selection = (*selection).min(wp_count - 1);
                }
                // 确认选择传送点
                if self.input_bus.consume(InputEvent::Confirm) && *selection > 0 && *selection <= wp_count {
                    let wp = &self.activated_waypoints[*selection - 1];
                    self.play_sfx("confirm");
                    self.camera = Camera::new(wp.x, wp.y);
                    self.player_entity = Entity::new_player(
                        Entity::tile_to_world(wp.x, wp.y));
                    self.state = GameState::Transition {
                        kind: golden_sun::engine::TransitionKind::FadeOut,
                        timer: 0.0,
                        from: "Travel",
                        to: "Teleport",
                    };
                    self.menu_page = 0;
                    self.menu_selection = 0;
                    return;
                }
                // Cancel 返回主菜单
                if self.input_bus.consume(InputEvent::Cancel) {
                    self.play_sfx("cancel");
                    self.state = GameState::Menu;
                    self.menu_selection = 5;
                    self.menu_page = 0;
                    return;
                }
                // 返回主菜单时也更新 MENU_ITEMS 索引
                if *selection > wp_count {
                    // 在 Travel 菜单中，超出传送点范围就是 Quit 位置
                    if self.input_bus.consume(InputEvent::Confirm) {
                        self.play_sfx("confirm");
                        self.camera = Camera::new(super::PLAYER_START_X, super::PLAYER_START_Y);
                        self.pp = constants::PP_INITIAL;
                        self.modified_tiles_current.clear();
                        self.pending_psynergy = None;
                        self.state = GameState::Title;
                        self.menu_page = 0;
                        self.menu_selection = 0;
                    }
                }
            }
            GameState::DjinnMenu { ref mut selection, ref mut page, ref mut character_select } => {
                if *page == 0 {
                    // Djinn 主列表
                    let djinn_count = self.collected_djinn.len();
                    if self.input_bus.consume(InputEvent::Up) {
                        *selection = selection.saturating_sub(1);
                    }
                    if self.input_bus.consume(InputEvent::Down) {
                        *selection = (*selection).min(djinn_count - 1);
                    }
                    // 切换角色选择
                    if self.input_bus.consume(InputEvent::Left) || self.input_bus.consume(InputEvent::Right) {
                        *character_select = if *character_select == 0 { 1 } else { 0 };
                    }
                    // 确认 → 切换装备
                    if self.input_bus.consume(InputEvent::Confirm) {
                        let idx = *selection;
                        if self.toggle_djinn_equip(idx) {
                            self.play_sfx("confirm");
                        } else {
                            self.play_sfx("cancel");
                        }
                    }
                    // Cancel 返回主菜单
                    if self.input_bus.consume(InputEvent::Cancel) {
                        self.play_sfx("cancel");
                        self.state = GameState::Menu;
                        self.menu_selection = 5;
                        self.menu_page = 0;
                    }
                }
            }
            GameState::Menu => {
                if self.menu_page == 0 {
                    const MENU_ITEMS: [&str; 8] = ["Continue", "Items", "Psynergy", "Status", "Save", "Djinn", "Travel", "Quit"];
                    // 主菜单导航
                    if self.input_bus.consume(InputEvent::Up) {
                        self.menu_selection = self.menu_selection.saturating_sub(1);
                    }
                    if self.input_bus.consume(InputEvent::Down) {
                        self.menu_selection = (self.menu_selection + 1).min(MENU_ITEMS.len() - 1);
                    }
                    if self.input_bus.consume(InputEvent::Confirm) {
                        match self.menu_selection {
                            0 => {
                                self.play_sfx("confirm");
                                self.state = GameState::WorldMap;   // Continue
                            }
                            1 => self.menu_page = 1,                  // Items
                            2 => self.menu_page = 2,                  // Psynergy
                            3 => self.menu_page = 3,                  // Status
                            4 => {
                                self.play_sfx("confirm");
                                self.save_game();                    // Save
                            }
                            5 => {
                                self.play_sfx("confirm");
                                // 有 Djinn 才显示 Djinn 菜单
                                if !self.collected_djinn.is_empty() {
                                    self.state = GameState::DjinnMenu {
                                        selection: 0, page: 0, character_select: 0,
                                    };
                                    self.djinn_menu_selection = 0;
                                    self.djinn_menu_page = 0;
                                    self.djinn_character_select = 0;
                                }
                                self.menu_selection = 0;
                            }
                            6 => {
                                self.play_sfx("confirm");
                                // 有传送点才显示 Travel 菜单
                                if !self.activated_waypoints.is_empty() {
                                    self.state = GameState::Travel { selection: 0 };
                                    self.menu_selection = 0;
                                }
                                self.menu_selection = 0;
                            }
                            7 => {
                                self.play_sfx("confirm");
                                self.camera = Camera::new(super::PLAYER_START_X, super::PLAYER_START_Y);
                                self.pp = constants::PP_INITIAL;
                                self.modified_tiles_current.clear();
                                self.pending_psynergy = None;
                                self.state = GameState::Title;
                            }
                            _ => {}
                        }
                        self.menu_selection = 0;
                    }
                } else {
                    // 子页面：Cancel 返回主菜单
                    if self.input_bus.consume(InputEvent::Cancel) {
                        self.play_sfx("cancel");
                        self.menu_page = 0;
                        self.menu_selection = 0;
                    }
                }
                if self.menu_page == 0 && self.input_bus.consume(InputEvent::Cancel) {
                    self.play_sfx("cancel");
                    self.state = GameState::WorldMap;
                }
            }
            GameState::Battle => {
                if let Some(ref mut battle) = self.battle {
                    // 衰减受击抖动
                    if battle.hit_shake > 0.0 {
                        battle.hit_shake -= self.time.delta;
                        if battle.hit_shake < 0.0 {
                            battle.hit_shake = 0.0;
                        }
                    }
                    // 更新伤害数字弹出
                    for popup in &mut battle.popups {
                        popup.timer += self.time.delta;
                        popup.y -= 0.5 * self.time.delta;
                    }
                    // 清理已完成的 popup
                    battle.popups.retain(|p| p.timer < 1.0);
                    // B键长按切换turbo加速
                    battle.turbo = is_key_down(KeyCode::Space);
                    match battle.phase {
                        BattlePhase::PlayerInput => {
                            let player_action = if self.input_bus.consume(InputEvent::Confirm) {
                                Some(BattleAction::Attack(0))
                            } else if self.input_bus.consume(InputEvent::Cancel) {
                                Some(BattleAction::Defend)
                            } else if self.input_bus.consume(InputEvent::Secondary) {
                                Some(BattleAction::Flee)
                            } else {
                                None
                            };
                            if let Some(action) = player_action {
                                battle.execute_turn(action);
                                loop {
                                    if matches!(battle.phase,
                                        BattlePhase::PlayerInput
                                        | BattlePhase::Victory
                                        | BattlePhase::Defeat
                                        | BattlePhase::FleeSuccess)
                                    {
                                        break;
                                    }
                                    let e_idx = battle.turn_order[battle.turn_index];
                                    if e_idx < battle.party.len() {
                                        break;
                                    }
                                    let action = battle.enemy_decision();
                                    battle.execute_turn(action);
                                }
                            }
                        }
                        BattlePhase::Victory => {
                            // 结算奖励（先取值避免借用冲突）
                            let coins = battle.total_coins;
                            let exp = battle.total_exp;
                            self.add_gold(coins);
                            self.add_exp(exp);
                            // 战斗胜利后召回所有 Djinn
                            self.recall_all_djinn();
                            if self.input_bus.consume(InputEvent::Confirm) {
                                self.battle = None;
                                self.state = GameState::WorldMap;
                            }
                        }
                        BattlePhase::Defeat | BattlePhase::FleeSuccess => {
                            if self.input_bus.consume(InputEvent::Confirm) {
                                self.battle = None;
                                self.state = GameState::WorldMap;
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    /// 尝试使用当前选中的精灵力 — 设置为施法动画状态
    fn try_use_selected_psynergy(&mut self) {
        debug_assert!(self.selected_psynergy < self.unlocked_count);
        let psynergy = self.unlocked_psynergies[self.selected_psynergy];
        if self.pp < psynergy.pp_cost() {
            dbg!("PP 不足！需要 {}，当前 {}", psynergy.pp_cost(), self.pp);
            return;
        }

        let tx = (self.camera.x + self.camera.rotation.cos()).floor() as i32;
        let ty = (self.camera.y + self.camera.rotation.sin()).floor() as i32;

        // 验证目标 tile 是否可以施放
        let can_cast = match psynergy {
            PsynergyType::Force => self.can_push_block(tx, ty),
            PsynergyType::Flash => self.can_apply_flash(tx, ty),
            PsynergyType::Reveal => self.can_apply_reveal(tx, ty),
            _ => {
                let tile = self.effective_tile(tx, ty);
                effects::apply_psynergy(tile, psynergy).is_some()
            }
        };

        if can_cast {
            self.pp -= psynergy.pp_cost();
            // 存储待执行的 tile 修改，动画结束后再应用
            self.pending_psynergy = Some((psynergy, tx, ty));
            self.state = GameState::PsynergyAnim {
                anim: PsynergyAnim { timer: 0.0, psynergy, tx, ty },
            };
        }
    }

    /// 检查是否可以推方块（只检测，不修改）
    fn can_push_block(&self, x: i32, y: i32) -> bool {
        if self.effective_tile(x, y) != TileKind::PushBlock {
            return false;
        }
        let facing = facing_from_angle(self.camera.rotation);
        let (dx, dy) = match facing {
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
            Direction::Down => (0, 1),
            Direction::Up => (0, -1),
        };
        self.effective_tile(x + dx, y + dy).is_walkable()
    }

    /// 检查是否可以照亮暗区（只检测，不修改）
    fn can_apply_flash(&self, cx: i32, cy: i32) -> bool {
        for dy in -1..=1 {
            for dx in -1..=1 {
                if self.effective_tile(cx + dx, cy + dy) == TileKind::DarkArea {
                    return true;
                }
            }
        }
        false
    }

    /// 检查是否可以揭示隐藏宝箱（只检测，不修改）
    fn can_apply_reveal(&self, cx: i32, cy: i32) -> bool {
        self.effective_tile(cx, cy) == TileKind::HiddenChest
    }

    /// 执行精灵力 tile 修改（动画结束后调用）
    fn execute_psynergy_effect(&mut self, psynergy: PsynergyType, tx: i32, ty: i32) {
        match psynergy {
            PsynergyType::Force => {
                let facing = facing_from_angle(self.camera.rotation);
                let (dx, dy) = match facing {
                    Direction::Right => (1, 0),
                    Direction::Left => (-1, 0),
                    Direction::Down => (0, 1),
                    Direction::Up => (0, -1),
                };
                self.modified_tiles_current.insert((tx + dx, ty + dy), TileKind::PushBlock);
                self.modified_tiles_current.insert((tx, ty), TileKind::Grass);
            }
            PsynergyType::Flash => {
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if self.effective_tile(tx + dx, ty + dy) == TileKind::DarkArea {
                            self.modified_tiles_current.insert((tx + dx, ty + dy), TileKind::Grass);
                        }
                    }
                }
            }
            PsynergyType::Reveal => {
                if self.effective_tile(tx, ty) == TileKind::HiddenChest {
                    self.modified_tiles_current.insert((tx, ty), TileKind::OpenedChest);
                }
            }
            _ => {
                let tile = self.effective_tile(tx, ty);
                if let Some(t) = effects::apply_psynergy(tile, psynergy) {
                    self.modified_tiles_current.insert((tx, ty), t);
                }
            }
        }
    }

    /// 获取带运行时覆盖的 tile（最快路径：未使用精灵力时不查 HashMap）
    pub(crate) fn effective_tile(&self, x: i32, y: i32) -> TileKind {
        if self.modified_tiles_current.is_empty() {
            let scene_map = map::tilemap::get_scene_map(self.scene.current());
            scene_map.get_tile(x, y)
        } else {
            self.modified_tiles_current.get(&(x, y))
                .copied()
                .unwrap_or_else(|| {
                    let scene_map = map::tilemap::get_scene_map(self.scene.current());
                    scene_map.get_tile(x, y)
                })
        }
    }

    /// 仅当玩家实际移动时累积 PP 恢复计时
    fn recover_pp(&mut self, moving: bool) {
        if self.pp >= self.max_pp || !moving { return; }
        self.pp_recover_timer += self.time.delta;
        if self.pp_recover_timer >= constants::PP_RECOVER_INTERVAL {
            self.pp = (self.pp + constants::PP_RECOVER_AMOUNT).min(self.max_pp);
            self.pp_recover_timer = 0.0;
            dbg!("PP 恢复至 {}", self.pp);
        }
    }

    /// 随机遇敌：行走在 Forest tile 上时概率触发
    fn trigger_random_encounter(&mut self, moving: bool) {
        if !moving || self.battle.is_some() { return; }

        let tx = self.camera.x.floor() as i32;
        let ty = self.camera.y.floor() as i32;
        let tile = self.effective_tile(tx, ty);

        // 只在 Forest 触发
        if tile != golden_sun::map::TileKind::Forest { return; }

        // 使用场景配置中的遇敌率
        let scene_map = map::tilemap::get_scene_map(self.scene.current());
        if self.encounter_step == 0 {
            self.encounter_step = quad_rand::gen_range(
                scene_map.encounter_rate,
                scene_map.encounter_rate + 7,
            );
        }
        self.encounter_step -= 1;

        if self.encounter_step == 0 {
            // 先闪光过渡，过渡完成后进入战斗
            self.encounter_pending = true;
            self.state = GameState::Transition {
                kind: golden_sun::engine::TransitionKind::FadeOut,
                timer: 0.0,
                from: "WorldMap",
                to: "Battle",
            };
        }
    }

    /// 返回玩家本帧是否移动
    fn update_player(&mut self) -> bool {
        let dt = self.time.delta;
        let speed = constants::PLAYER_SPEED
            * if self.input.a_held { constants::PLAYER_SPRINT_MULTIPLIER } else { 1.0 };

        let mut moved = false;

        if self.input_bus.consume(InputEvent::Up) {
            self.try_move(1.0, speed * dt);
            moved = true;
        }
        if self.input_bus.consume(InputEvent::Down) {
            self.try_move(-1.0, speed * dt);
            moved = true;
        }
        if self.input_bus.consume(InputEvent::Left) {
            self.try_rotate(-constants::PLAYER_TURN_SPEED * dt);
        }
        if self.input_bus.consume(InputEvent::Right) {
            self.try_rotate(constants::PLAYER_TURN_SPEED * dt);
        }

        let facing = facing_from_angle(self.camera.rotation);
        self.player_entity.facing = facing;

        if moved {
            self.player_entity.anim_state = AnimState::from_dir(facing, true);
            self.player_entity.anim_timer += dt;
        } else {
            self.player_entity.anim_state = AnimState::from_dir(facing, false);
            self.player_entity.anim_timer = 0.0;
        }

        moved
    }

    fn update_npcs(&mut self) {
        let dt = self.time.delta;
        // 提前获取场景地图引用，避免与 npc 可变借用冲突
        let scene_map = map::tilemap::get_scene_map(self.scene.current());
        for npc in &mut self.npcs {
            if let Some(WalkPattern::Patrol { waypoints, speed, index, pause }) = npc.walk_pattern.as_mut() {
                if *pause > 0.0 {
                    *pause -= dt;
                    if *pause <= 0.0 {
                        *index = (*index + 1) % waypoints.len();
                        npc.anim_timer = 0.0;
                    }
                    npc.anim_state = AnimState::from_dir(npc.facing, false);
                    continue;
                }
                let target = waypoints[*index];
                let dx = target.0 - npc.pos.0;
                let dy = target.1 - npc.pos.1;
                let dist_sq = dx * dx + dy * dy;
                if dist_sq < constants::NPC_PATROL_ARRIVE_SQ {
                    *pause = constants::NPC_PATROL_PAUSE_DURATION;
                    npc.anim_timer = 0.0;
                } else {
                    let dist = dist_sq.sqrt();
                    let step = *speed * dt * TILE_SIZE;
                    let npc_x = npc.pos.0 + dx / dist * step;
                    let npc_y = npc.pos.1 + dy / dist * step;
                    let ntx = (npc_x / TILE_SIZE).floor() as i32;
                    let nty = (npc_y / TILE_SIZE).floor() as i32;
                    if scene_map.is_walkable(ntx, nty) {
                        npc.pos.0 = npc_x;
                        npc.pos.1 = npc_y;
                    }
                    npc.facing = dir_from_delta(dx, dy);
                    npc.anim_state = AnimState::from_dir(npc.facing, true);
                    npc.anim_timer += dt;
                }
            }
        }
    }

    #[inline]
    fn find_nearby_npc(&self) -> Option<&Entity> {
        let px = self.camera.x;
        let py = self.camera.y;
        self.npcs.iter().find(|npc| {
            let nx = npc.pos.0 / TILE_SIZE;
            let ny = npc.pos.1 / TILE_SIZE;
            let dx = nx - px;
            let dy = ny - py;
            let range = npc.interact_radius.unwrap_or(constants::NPC_INTERACT_RANGE);
            dx * dx + dy * dy <= range * range
        })
    }

    #[inline]
    fn try_move(&mut self, sign: f32, distance: f32) {
        let new_x = self.camera.x + sign * distance * self.camera.rotation.cos();
        let new_y = self.camera.y + sign * distance * self.camera.rotation.sin();
        self.try_move_to(new_x, new_y);
    }

    #[inline]
    fn try_move_to(&mut self, new_x: f32, new_y: f32) {
        let tx = new_x.floor() as i32;
        let ty = new_y.floor() as i32;
        if self.effective_tile(tx, ty).is_walkable() {
            self.camera.x = new_x;
            self.camera.y = new_y;
            self.camera.target_x = new_x;
            self.camera.target_y = new_y;
        }
    }

    fn try_rotate(&mut self, radians: f32) {
        self.camera.rotate(radians);
    }

    /// 检测玩家是否到达地图边缘，触发场景切换
    fn check_scene_boundaries(&mut self) {
        let scene_map = map::tilemap::get_scene_map(self.scene.current());
        let (mw, mh) = scene_map.size();
        let px = self.camera.x;
        let py = self.camera.y;

        // 到达地图边缘 → 切换到默认下一个场景
        if px >= mw as f32 - 1.0 {
            let next = match self.scene.current() {
                golden_sun::SceneId::Vale => golden_sun::SceneId::WildForest,
                golden_sun::SceneId::WildForest => golden_sun::SceneId::Cave,
                golden_sun::SceneId::Cave => golden_sun::SceneId::Vale,
                _ => golden_sun::SceneId::Vale,
            };
            self.request_scene_switch(next);
        }
        if px <= 1.0 {
            let next = match self.scene.current() {
                golden_sun::SceneId::Vale => golden_sun::SceneId::Cave,
                golden_sun::SceneId::WildForest => golden_sun::SceneId::Vale,
                golden_sun::SceneId::Cave => golden_sun::SceneId::WildForest,
                _ => golden_sun::SceneId::Vale,
            };
            self.request_scene_switch(next);
        }
        if py >= mh as f32 - 1.0 {
            let next = match self.scene.current() {
                golden_sun::SceneId::Vale => golden_sun::SceneId::WildForest,
                golden_sun::SceneId::WildForest => golden_sun::SceneId::Cave,
                golden_sun::SceneId::Cave => golden_sun::SceneId::Vale,
                _ => golden_sun::SceneId::Vale,
            };
            self.request_scene_switch(next);
        }
        if py <= 1.0 {
            let next = match self.scene.current() {
                golden_sun::SceneId::Vale => golden_sun::SceneId::Cave,
                golden_sun::SceneId::WildForest => golden_sun::SceneId::Vale,
                golden_sun::SceneId::Cave => golden_sun::SceneId::WildForest,
                _ => golden_sun::SceneId::Vale,
            };
            self.request_scene_switch(next);
        }
    }

    /// 追踪任务进度 — 根据游戏事件推进 QuestLog
    fn track_quest_progress(&mut self) {
        // 解锁精灵力 → 完成"初次精灵力"任务
        if self.unlocked_count > 0 {
            self.quest_log.complete("first_psynergy");
        }
        // 离开 Vale 村 → 完成"探索密林"任务
        if self.scene.current() != golden_sun::SceneId::Vale {
            self.quest_log.complete("explore_forest");
        }
    }

    /// 检测玩家是否踩到传送点 tile
    fn check_waypoints(&mut self) {
        let tx = self.camera.x.floor() as i32;
        let ty = self.camera.y.floor() as i32;
        if self.effective_tile(tx, ty) == TileKind::Waypoint {
            let scene_name = match self.scene.current() {
                golden_sun::SceneId::Vale => "Vale",
                golden_sun::SceneId::WildForest => "WildForest",
                golden_sun::SceneId::Cave => "Cave",
                _ => "Vale",
            };
            let wp_name = format!("{scene_name}_waypoint_{tx}_{ty}");
            if !self.activated_waypoints.iter().any(|w| w.name == wp_name) {
                #[cfg(debug_assertions)]
                eprintln!("激活传送点: {wp_name}");
                self.activated_waypoints.push(WaypointDef {
                    name: wp_name,
                    scene: self.scene.current(),
                    x: self.camera.x,
                    y: self.camera.y,
                });
            }
        }
    }

    /// 处理对话分支选择
    fn handle_choice_selection(&mut self) {
        let GameState::DialogueChoices { choices, script } = &self.state else { return };
        let choice = &choices[self.dialogue_choice_selection];
        let script_clone = (*script).clone();
        let flag_ok = choice.require_flag.is_none_or(|f| self.story_flags.get(f));
        let aff_ok = choice.require_affinity.is_none_or(|req| {
            let npc_id = script.pages.first()
                .and_then(|p| p.lines.first())
                .and_then(|l| l.actions.iter().find_map(|a| {
                    if let golden_sun::dialogue::DialogueAction::SetFlag(fid) = a {
                        Some(*fid)
                    } else {
                        None
                    }
                }))
                .unwrap_or("default");
            let id_num: u32 = npc_id.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap_or(0);
            self.affinity.get(&id_num).copied().unwrap_or(0) >= req
        });
        if flag_ok && aff_ok {
            self.play_sfx("confirm");
            if let Some(set_flag) = choice.set_flag {
                self.story_flags.set(set_flag);
                if let Some(npc) = self.find_nearby_npc() {
                    if let Some(id_str) = &npc.dialogue_id {
                        let id_num: u32 = id_str.chars().filter(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap_or(0);
                        *self.affinity.entry(id_num).or_insert(0) += 1;
                    }
                }
            }
            let target_idx = choice.target_page.saturating_sub(1).min(script.pages.len().saturating_sub(1));
            let text = script_clone.pages[target_idx].lines[0].text.to_string();
            self.dialogue = Some(DialogueState::new(text));
            self.state = GameState::Dialog;
            self.current_dialogue_script = Some(script_clone);
        }
    }
}

fn facing_from_angle(rotation: f32) -> Direction {
    let r = rotation.rem_euclid(std::f32::consts::TAU);
    if r < std::f32::consts::FRAC_PI_4 * 1.0 {
        Direction::Right
    } else if r < std::f32::consts::FRAC_PI_4 * 3.0 {
        Direction::Down
    } else if r < std::f32::consts::FRAC_PI_4 * 5.0 {
        Direction::Left
    } else if r < std::f32::consts::FRAC_PI_4 * 7.0 {
        Direction::Up
    } else {
        Direction::Right
    }
}

fn dir_from_delta(dx: f32, dy: f32) -> Direction {
    if dx.abs() >= dy.abs() {
        if dx > 0.0 { Direction::Right } else { Direction::Left }
    } else if dy > 0.0 {
        Direction::Down
    } else {
        Direction::Up
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facing_from_angle_right_quadrant() {
        assert_eq!(facing_from_angle(0.0), Direction::Right);
        assert_eq!(facing_from_angle(std::f32::consts::FRAC_PI_4 * 0.5), Direction::Right);
    }

    #[test]
    fn facing_from_angle_down_quadrant() {
        assert_eq!(facing_from_angle(std::f32::consts::FRAC_PI_4 * 1.0), Direction::Down);
        assert_eq!(facing_from_angle(std::f32::consts::FRAC_PI_4 * 2.0), Direction::Down);
        assert_eq!(facing_from_angle(std::f32::consts::FRAC_PI_4 * 2.9), Direction::Down);
    }

    #[test]
    fn facing_from_angle_left_quadrant() {
        assert_eq!(facing_from_angle(std::f32::consts::FRAC_PI_4 * 3.0), Direction::Left);
        assert_eq!(facing_from_angle(std::f32::consts::FRAC_PI_4 * 4.0), Direction::Left);
        assert_eq!(facing_from_angle(std::f32::consts::FRAC_PI_4 * 4.9), Direction::Left);
    }

    #[test]
    fn facing_from_angle_up_quadrant() {
        assert_eq!(facing_from_angle(std::f32::consts::FRAC_PI_4 * 5.0), Direction::Up);
        assert_eq!(facing_from_angle(std::f32::consts::FRAC_PI_4 * 6.0), Direction::Up);
        assert_eq!(facing_from_angle(std::f32::consts::FRAC_PI_4 * 6.9), Direction::Up);
    }

    #[test]
    fn facing_from_angle_wraparound() {
        assert_eq!(facing_from_angle(std::f32::consts::FRAC_PI_4 * 7.0), Direction::Right);
        assert_eq!(facing_from_angle(std::f32::consts::TAU - 0.001), Direction::Right);
    }

    #[test]
    fn dir_from_delta_pure_horizontal() {
        assert_eq!(dir_from_delta(5.0, 0.0), Direction::Right);
        assert_eq!(dir_from_delta(-5.0, 0.0), Direction::Left);
    }

    #[test]
    fn dir_from_delta_pure_vertical() {
        assert_eq!(dir_from_delta(0.0, 5.0), Direction::Down);
        assert_eq!(dir_from_delta(0.0, -5.0), Direction::Up);
    }

    #[test]
    fn dir_from_delta_dominant_axis() {
        assert_eq!(dir_from_delta(3.0, 2.0), Direction::Right);
        assert_eq!(dir_from_delta(2.0, 3.0), Direction::Down);
        assert_eq!(dir_from_delta(-3.0, 2.0), Direction::Left);
        assert_eq!(dir_from_delta(2.0, -3.0), Direction::Up);
    }
}
