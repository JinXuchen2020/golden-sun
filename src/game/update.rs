use super::GameCtx;

use golden_sun::constants::{self, TILE_SIZE};
use golden_sun::engine::{Camera, GameState};
use golden_sun::entity::sprite::AnimState;
use golden_sun::entity::{Direction, Entity, WalkPattern};
use golden_sun::map::{tilemap, TileKind};
use golden_sun::psynergy::effects;
use golden_sun::{PsynergyType, InputEvent};

/// Debug 日志（release 模式编译消除）
macro_rules! dbg {
    ($($arg:tt)*) => { #[cfg(debug_assertions)] { eprintln!($($arg)*); } }
}

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
                let moving = self.update_player();
                self.update_npcs();
                self.camera.update_lerp(self.time.delta);
                self.recover_pp(moving);

                if self.input_bus.consume(InputEvent::Confirm) {
                    if let Some(npc) = self.find_nearby_npc() {
                        dbg!("交互 NPC {}: {:?}", npc.id, npc.dialogue_id);
                        self.state = GameState::Dialog;
                    }
                }

                // B/Secondary → 进入精灵力选择
                if self.input_bus.consume(InputEvent::Secondary) && self.unlocked_count > 0 {
                    self.state = GameState::Psynergy;
                    self.selected_psynergy = 0;
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

    /// 尝试使用当前选中的精灵力
    fn try_use_selected_psynergy(&mut self) {
        debug_assert!(self.selected_psynergy < self.unlocked_count);
        let psynergy = self.unlocked_psynergies[self.selected_psynergy];
        if self.pp < psynergy.pp_cost() {
            dbg!("PP 不足！需要 {}，当前 {}", psynergy.pp_cost(), self.pp);
            return;
        }

        let tx = (self.camera.x + self.camera.rotation.cos()).floor() as i32;
        let ty = (self.camera.y + self.camera.rotation.sin()).floor() as i32;

        let succeeded = match psynergy {
            PsynergyType::Force => self.try_push_block(tx, ty),
            PsynergyType::Flash => self.apply_flash(tx, ty),
            PsynergyType::Reveal => self.apply_reveal(tx, ty),
            _ => {
                let tile = self.effective_tile(tx, ty);
                if let Some(t) = effects::apply_psynergy(tile, psynergy) {
                    self.modified_tiles.insert((tx, ty), t);
                    true
                } else {
                    false
                }
            }
        };

        if succeeded {
            self.pp -= psynergy.pp_cost();
        }

        self.state = GameState::WorldMap;
    }

    /// 获取带运行时覆盖的 tile（最快路径：未使用精灵力时不查 HashMap）
    pub(crate) fn effective_tile(&self, x: i32, y: i32) -> TileKind {
        if self.modified_tiles.is_empty() {
            tilemap::get_tile(x, y)
        } else {
            self.modified_tiles.get(&(x, y)).copied().unwrap_or_else(|| tilemap::get_tile(x, y))
        }
    }

    /// Force：朝玩家 facing 方向推 PushBlock 一格，返回是否成功
    fn try_push_block(&mut self, x: i32, y: i32) -> bool {
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
        let target_x = x + dx;
        let target_y = y + dy;

        if self.effective_tile(target_x, target_y).is_walkable() {
            self.modified_tiles.insert((target_x, target_y), TileKind::PushBlock);
            self.modified_tiles.insert((x, y), TileKind::Grass);
            true
        } else {
            false
        }
    }

    /// Flash：照亮前方 3×3 暗区，返回是否至少照亮一处
    fn apply_flash(&mut self, cx: i32, cy: i32) -> bool {
        let mut affected = false;
        for dy in -1..=1 {
            for dx in -1..=1 {
                let tx = cx + dx;
                let ty = cy + dy;
                if self.effective_tile(tx, ty) == TileKind::DarkArea {
                    self.modified_tiles.insert((tx, ty), TileKind::Grass);
                    affected = true;
                }
            }
        }
        affected
    }

    /// Reveal：显示前方隐藏宝箱，返回是否发现
    fn apply_reveal(&mut self, cx: i32, cy: i32) -> bool {
        if self.effective_tile(cx, cy) == TileKind::HiddenChest {
            self.modified_tiles.insert((cx, cy), TileKind::OpenedChest);
            true
        } else {
            false
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
