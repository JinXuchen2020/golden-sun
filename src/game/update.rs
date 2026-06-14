use super::GameCtx;

use golden_sun::constants::{self, TILE_SIZE};
use golden_sun::engine::{Camera, GameState};
use golden_sun::entity::sprite::AnimState;
use golden_sun::entity::{Direction, Entity, WalkPattern};
use golden_sun::map::tilemap;
use golden_sun::InputEvent;

impl GameCtx {
    pub(super) fn update(&mut self) {
        self.time.poll();
        self.input.poll();
        self.input_bus.poll(&self.input);

        debug_assert!(self.camera.validate(), "Camera 参数无效 — height={}, fov={}",
            self.camera.height, self.camera.fov);

        self.scene.commit_switch();

        if self.state.is_transition() {
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
            }
            GameState::WorldMap => {
                self.update_player();
                self.update_npcs();
                self.camera.update_lerp(self.time.delta);

                if self.input_bus.consume(InputEvent::Confirm) {
                    if let Some(npc) = self.find_nearby_npc() {
                        eprintln!("交互 NPC {}: {:?}", npc.id, npc.dialogue_id);
                        self.state = GameState::Dialog;
                    }
                }

                if self.input_bus.consume(InputEvent::Menu) {
                    self.state = GameState::Menu;
                }
            }
            GameState::Dialog => {
                if self.input_bus.consume(InputEvent::Confirm)
                    || self.input_bus.consume(InputEvent::Cancel)
                {
                    self.state = GameState::WorldMap;
                }
            }
            GameState::Menu => {
                if self.input_bus.consume(InputEvent::Cancel) {
                    self.state = GameState::WorldMap;
                }
            }
            _ => {}
        }
    }

    /// 玩家移动：Up/Down 沿 camera.rotation 移动，Left/Right 旋转视角。
    /// facing 从 camera.rotation 推算（不再硬编码键盘键）。
    fn update_player(&mut self) {
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

        // facing 完全由 camera.rotation 决定，不依赖按键
        let facing = facing_from_angle(self.camera.rotation);
        self.player_entity.facing = facing;

        if moved {
            self.player_entity.anim_state = AnimState::from_dir(facing, true);
            self.player_entity.anim_timer += dt;
        } else {
            self.player_entity.anim_state = AnimState::from_dir(facing, false);
            self.player_entity.anim_timer = 0.0;
        }
    }

    fn update_npcs(&mut self) {
        let dt = self.time.delta;
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
                    // 碰撞保护：前进方向不可通行则停住
                    if tilemap::is_walkable(
                        (npc_x / TILE_SIZE).floor() as i32,
                        (npc_y / TILE_SIZE).floor() as i32,
                    ) {
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
        if tilemap::is_walkable(new_x.floor() as i32, new_y.floor() as i32) {
            self.camera.x = new_x;
            self.camera.y = new_y;
            self.camera.target_x = new_x;
            self.camera.target_y = new_y;
        }
    }

    fn try_rotate(&mut self, radians: f32) {
        self.camera.rotate(radians);
    }
}

/// 从相机旋转角推算面朝方向（8 象限，每 45° 一档）
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
