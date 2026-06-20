//! 战斗系统核心 — 回合制战斗状态机

use super::calculator;
use crate::Element;
use crate::PsynergyType;
use crate::data::djinn::DjinnId;

/// 元素增益/减益状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusEffect {
    None,
    Delay,
    BuffAtk,
    BuffDef,
}

/// 战斗参与者
#[derive(Debug, Clone)]
pub struct Combatant {
    pub id: u32,
    pub name: &'static str,
    pub hp: u32,
    pub max_hp: u32,
    pub pp: u32,
    pub max_pp: u32,
    pub attack: u32,
    pub defense: u32,
    pub speed: u32,
    pub level: u32,
    pub element: Element,
    pub status: StatusEffect,
    pub is_player: bool,
    pub is_boss: bool,
}

impl Combatant {
    pub fn new(id: u32, name: &'static str, level: u32, element: Element, is_player: bool) -> Self {
        let base = 10 + level * 3;
        Self {
            id, name,
            hp: base * 2, max_hp: base * 2,
            pp: level * 2, max_pp: level * 2,
            attack: base, defense: base,
            speed: 5 + level * 2,
            level, element,
            status: StatusEffect::None,
            is_player,
            is_boss: false,
        }
    }

    pub fn new_boss(id: u32, name: &'static str, level: u32, element: Element) -> Self {
        let base = 10 + level * 3;
        Self {
            id, name,
            hp: base * 3, max_hp: base * 3,
            pp: level * 3, max_pp: level * 3,
            attack: (base as f32 * 1.5) as u32,
            defense: (base as f32 * 1.5) as u32,
            speed: 5 + level * 2,
            level, element,
            status: StatusEffect::None,
            is_player: false,
            is_boss: true,
        }
    }

    #[inline]
    pub fn is_alive(&self) -> bool { self.hp > 0 }

    pub fn take_damage(&mut self, dmg: u32) {
        self.hp = self.hp.saturating_sub(dmg);
    }
}

/// 战斗指令
#[derive(Debug, Clone, Copy)]
pub enum BattleAction {
    Attack(usize),
    Defend,
    Psynergy(PsynergyType, usize),
    ReleaseDjinn(DjinnId),
    RecallDjinn(DjinnId),
    Summon(usize),
    Flee,
    UseItem(crate::engine::ItemType, usize),
}

impl BattleAction {
    pub fn pp_cost(&self) -> u32 {
        match self {
            BattleAction::Psynergy(p, _) => p.pp_cost(),
            _ => 0,
        }
    }
}

/// 单次攻击结果
#[derive(Debug, Clone, Copy)]
pub struct AttackResult {
    pub attacker: u32,
    pub target: u32,
    pub damage: u32,
    pub element: Element,
    pub modifier: f32,
    pub killed: bool,
}

/// 战斗阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattlePhase {
    PlayerInput,
    EnemyTurn,
    Victory,
    Defeat,
    FleeSuccess,
}

/// 伤害数字弹出效果
#[derive(Debug, Clone)]
pub struct DamagePopup {
    pub damage: u32,
    pub x: f32,
    pub y: f32,
    pub timer: f32,
    pub element: Element,
    pub modifier: f32,
}

/// 战斗实例
#[derive(Debug, Clone)]
pub struct Battle {
    pub phase: BattlePhase,
    pub party: Vec<Combatant>,
    pub enemies: Vec<Combatant>,
    pub turn_order: Vec<usize>,
    pub turn_index: usize,
    pub logs: Vec<String>,
    pub results: Vec<AttackResult>,
    pub total_exp: u32,
    pub total_coins: u32,
    party_speed: u32,
    enemy_speed: u32,
    /// 受击抖动计时器（秒），>0 时目标闪烁
    pub hit_shake: f32,
    /// 加速模式（按住B键时3倍速）
    pub turbo: bool,
    /// 伤害数字弹出列表
    pub popups: Vec<DamagePopup>,
    /// 是否已使用过召唤
    pub summon_used: bool,
    /// 待命 Djinn 数量（已装备但未释放的 Djinn 数）
    pub standby_djinn_count: usize,
}

/// 战斗统计
#[derive(Debug, Clone, Default)]
pub struct BattleStats {
    pub turns: u32,
    pub damage_dealt: u32,
    pub damage_taken: u32,
    pub items_used: u32,
    pub djinn_released: u32,
}

/// 从 party/enemies 中读取的当前行动者快照
struct ActorInfo {
    id: u32,
    name: &'static str,
    element: Element,
    level: u32,
    attack: u32,
    pp: u32,
}

/// 从 party/enemies 中读取的目标快照
struct TargetInfo {
    defense: u32,
    element: Element,
    hp: u32,
    id: u32,
    name: &'static str,
}

impl Battle {
    #[must_use]
    pub fn get_stats(&self) -> BattleStats {
        let damage_dealt: u32 = self.results.iter().map(|r| r.damage).sum();
        BattleStats {
            turns: self.turn_index as u32,
            damage_dealt,
            damage_taken: 0,
            items_used: 0,
            djinn_released: 0,
        }
    }

    pub fn new(party: Vec<Combatant>, enemies: Vec<Combatant>) -> Self {
        let total_exp = enemies.iter().map(|e| e.level * 3).sum();
        let total_coins = enemies.iter().map(|e| e.level * 2).sum();
        let turn_order = Self::compute_speed_order(&party, &enemies);
        let party_speed = party.iter().map(|p| p.speed).sum();
        let enemy_speed = enemies.iter().map(|e| e.speed).sum();
        Self {
            phase: BattlePhase::PlayerInput,
            party, enemies,
            turn_order,
            turn_index: 0,
            logs: Vec::new(),
            results: Vec::new(),
            total_exp, total_coins,
            party_speed, enemy_speed,
            hit_shake: 0.0,
            turbo: false,
            popups: Vec::new(),
            summon_used: false,
            standby_djinn_count: 0,
        }
    }

    pub fn all_enemies_defeated(&self) -> bool {
        self.enemies.iter().all(|e| !e.is_alive())
    }

    pub fn all_party_defeated(&self) -> bool {
        self.party.iter().all(|p| !p.is_alive())
    }

    /// 执行一次行动（回合中的一步）
    pub fn execute_turn(&mut self, action: BattleAction) {
        let actor_idx = self.turn_order[self.turn_index];
        let is_party_actor = actor_idx < self.party.len();
        let Some(actor) = self.actor_info(actor_idx) else { return; };

        match action {
            BattleAction::Attack(target) => {
                let is_party_target = !is_party_actor;
                if let Some(t) = self.target_info(target, is_party_target) {
                    let dmg = calculator::calculate_physical_damage(
                        actor.attack, actor.element, t.defense, t.element,
                    );
                    let modifier = calculator::element_modifier(actor.element, t.element);
                    let killed = dmg >= t.hp;
                    self.apply_damage(target, is_party_target, dmg);
                    self.hit_shake = 0.15;
                    let target_x = if is_party_target { 440.0 } else { 100.0 + target as f32 * 80.0 };
                    let target_y = if is_party_target { 80.0 } else { 100.0 };
                    self.popups.push(DamagePopup {
                        damage: dmg, x: target_x, y: target_y, timer: 0.0,
                        element: actor.element, modifier,
                    });
                    self.results.push(AttackResult {
                        attacker: actor.id, target: t.id,
                        damage: dmg, element: actor.element, modifier, killed,
                    });
                    self.logs.push(format!("{} attacks {} for {dmg} damage!",
                        actor.name, t.name));
                }
            }
            BattleAction::Defend => {
                self.logs.push(format!("{} defends!", actor.name));
            }
            BattleAction::Psynergy(psynergy, target) => {
                if actor.pp < psynergy.pp_cost() {
                    self.logs.push("Not enough PP!".to_string());
                } else {
                    let is_party_target = !is_party_actor;
                    if let Some(t) = self.target_info(target, is_party_target) {
                        let dmg = calculator::calculate_psynergy_damage(
                            actor.level, psynergy.element(), t.defense, t.element, psynergy,
                        );
                        let modifier = calculator::element_modifier(psynergy.element(), t.element);
                        let killed = dmg >= t.hp;
                        self.apply_damage(target, is_party_target, dmg);
                        self.hit_shake = 0.15;
                        let target_x = if is_party_target { 440.0 } else { 100.0 + target as f32 * 80.0 };
                        let target_y = if is_party_target { 80.0 } else { 100.0 };
                        self.popups.push(DamagePopup {
                            damage: dmg, x: target_x, y: target_y, timer: 0.0,
                            element: psynergy.element(), modifier,
                        });
                        self.results.push(AttackResult {
                            attacker: actor.id, target: t.id,
                            damage: dmg, element: psynergy.element(), modifier, killed,
                        });
                        self.logs.push(format!("{} uses {:?} on {} for {dmg} damage!",
                            actor.name, psynergy, t.name));
                    }
                }
            }
            BattleAction::Flee => {
                if self.party_speed > self.enemy_speed {
                    self.phase = BattlePhase::FleeSuccess;
                    self.logs.push("You fled successfully!".to_string());
                } else {
                    self.logs.push("Failed to flee!".to_string());
                }
            }
            BattleAction::ReleaseDjinn(djinn_id) => {
                if is_party_actor {
                    // 查找对应角色的 Djinn 并应用释放加成
                    let djinn_data = crate::data::djinn::all_djinn_data();
                    if let Some(djinn) = djinn_data.iter().find(|d| d.id == djinn_id) {
                        // 在 party 中找第一个角色（Isaac=0）应用加成
                        if let Some(party_member) = self.party.get_mut(0) {
                            let orig_atk = party_member.attack;
                            let orig_def = party_member.defense;
                            let orig_hp = party_member.max_hp;
                            let orig_pp = party_member.max_pp;
                            let orig_speed = party_member.speed;
                            
                            // 应用释放倍率
                            party_member.attack = (orig_atk as f32 * djinn.release_atk_mult as f32) as u32;
                            party_member.defense = (orig_def as f32 * djinn.release_def_mult as f32) as u32;
                            party_member.max_hp = (orig_hp as f32 * djinn.release_hp_mult as f32) as u32;
                            party_member.hp = party_member.hp.min(party_member.max_hp);
                            party_member.max_pp = (orig_pp as f32 * djinn.release_pp_mult as f32) as u32;
                            party_member.pp = party_member.pp.min(party_member.max_pp);
                            party_member.speed = (orig_speed as f32 * djinn.release_speed_mult as f32) as u32;
                            
                            self.logs.push(format!("{} releases {}! Stats boosted!",
                                party_member.name, djinn.name()));
                        }
                    } else {
                        self.logs.push(format!("Don't have {} equipped!", djinn_id.as_str()));
                    }
                } else {
                    self.logs.push("Enemies can't release Djinn!".to_string());
                }
            }
            BattleAction::RecallDjinn(djinn_id) => {
                if is_party_actor {
                    let djinn_data = crate::data::djinn::all_djinn_data();
                    if let Some(djinn) = djinn_data.iter().find(|d| d.id == djinn_id) {
                        if let Some(party_member) = self.party.get_mut(0) {
                            // 恢复为基础值（level * 3 = base）
                            let base = 10 + party_member.level * 3;
                            party_member.attack = base;
                            party_member.defense = base;
                            party_member.max_hp = base * 2;
                            party_member.hp = party_member.hp.min(party_member.max_hp);
                            party_member.max_pp = party_member.level * 2;
                            party_member.pp = party_member.pp.min(party_member.max_pp);
                            party_member.speed = 5 + party_member.level * 2;

                            self.logs.push(format!("{} recalls {}! Stats restored.",
                                party_member.name, djinn.name()));
                        }
                    } else {
                        self.logs.push(format!("Don't have {} equipped!", djinn_id.as_str()));
                    }
                } else {
                    self.logs.push("Enemies can't recall Djinn!".to_string());
                }
            }
            BattleAction::Summon(summon_idx) => {
                let summons = crate::data::summon::all_summons();
                if let Some(summon) = summons.get(summon_idx) {
                    self.summon_used = true;
                    let base_dmg = summon.base_damage(actor.level);
                    let mut dmg_results: Vec<(usize, u32)> = Vec::new();
                    for (ei, enemy) in self.enemies.iter().enumerate() {
                        if enemy.is_alive() {
                            let dmg = ((base_dmg as f32) * (1.0 - enemy.defense as f32 / 100.0).max(0.05)) as u32;
                            dmg_results.push((ei, dmg));
                        }
                    }
                    for (ei, dmg) in dmg_results {
                        self.apply_damage(ei + self.party.len(), false, dmg);
                        self.hit_shake = 0.15;
                        let enemy = &self.enemies[ei];
                        self.popups.push(DamagePopup {
                            damage: dmg, x: 440.0 + ei as f32 * 80.0, y: 80.0, timer: 0.0,
                            element: summon.element, modifier: 1.5,
                        });
                        self.results.push(AttackResult {
                            attacker: actor.id, target: enemy.id,
                            damage: dmg, element: summon.element, modifier: 1.5, killed: dmg >= enemy.hp,
                        });
                    }
                    self.deduct_pp(actor_idx, true, summon.pp_cost);
                    self.logs.push(format!("{} 召唤了 {}！", actor.name, summon.name));
                }
            }
            BattleAction::UseItem(item_type, target_idx) => {
                if target_idx >= self.party.len() {
                    self.logs.push("Invalid target!".to_string());
                } else {
                    let target = &mut self.party[target_idx];
                    match item_type {
                        crate::engine::ItemType::Potion => {
                            let heal = 30u32;
                            target.hp = (target.hp + heal).min(target.max_hp);
                            self.logs.push(format!("{} 使用了药水，恢复了 {} HP！", target.name, heal));
                        }
                        crate::engine::ItemType::Ether => {
                            let recover = 10u32;
                            target.pp = (target.pp + recover).min(target.max_pp);
                            self.logs.push(format!("{} 使用了以太，恢复了 {} PP！", target.name, recover));
                        }
                        _ => {
                            self.logs.push("这个道具不能在战斗中使用".to_string());
                        }
                    }
                }
            }
        }

        let pp_cost = action.pp_cost();
        if pp_cost > 0 {
            self.deduct_pp(actor_idx, is_party_actor, pp_cost);
        }

        self.advance_phase();
    }

    /// 计算速度排序（降序）
    fn compute_speed_order(party: &[Combatant], enemies: &[Combatant]) -> Vec<usize> {
        let mut indexed: Vec<(usize, u32)> = party.iter().enumerate()
            .map(|(i, c)| (i, c.speed))
            .chain(enemies.iter().enumerate().map(|(i, c)| (i + party.len(), c.speed)))
            .collect();
        indexed.sort_by(|a, b| b.1.cmp(&a.1));
        indexed.into_iter().map(|(i, _)| i).collect()
    }

    fn advance_phase(&mut self) {
        if matches!(self.phase, BattlePhase::Victory | BattlePhase::Defeat | BattlePhase::FleeSuccess) {
            return;
        }
        if self.all_enemies_defeated() {
            let boss_mult = if self.enemies.iter().any(|e| e.is_boss) { 2u32 } else { 1u32 };
            self.total_exp *= boss_mult;
            self.total_coins *= boss_mult;
            self.phase = BattlePhase::Victory;
            return;
        }
        if self.all_party_defeated() {
            self.phase = BattlePhase::Defeat;
            return;
        }

        self.turn_index += 1;
        if self.turn_index >= self.turn_order.len() {
            self.turn_index = 0;
        }
        if self.turn_order[self.turn_index] >= self.party.len() {
            self.phase = BattlePhase::EnemyTurn;
        } else {
            self.phase = BattlePhase::PlayerInput;
        }
    }

    /// 敌人 AI — 考虑 Boss 行为
    pub fn enemy_decision(&self) -> BattleAction {
        let enemy_idx = self.enemies.iter().position(|e| e.is_alive());
        let Some(ei) = enemy_idx else { return BattleAction::Defend; };
        let enemy = &self.enemies[ei];

        if enemy.is_boss {
            let hash = (ei as u32) ^ (self.turn_index as u32) ^ (self.logs.len() as u32);
            let roll = hash % 100;
            match roll {
                0..=49 => {
                    if let Some(pi) = self.party.iter().position(|p| p.is_alive()) {
                        BattleAction::Attack(pi)
                    } else {
                        BattleAction::Defend
                    }
                }
                50..=79 => {
                    if let Some(pi) = self.party.iter().position(|p| p.is_alive() && p.pp >= 3) {
                        BattleAction::Psynergy(PsynergyType::Force, pi)
                    } else {
                        BattleAction::Attack(0)
                    }
                }
                _ => BattleAction::Defend,
            }
        } else {
            let hash = (ei as u32) ^ (self.turn_index as u32);
            let roll = hash % 100;
            if roll < 70 {
                if let Some(pi) = self.party.iter().position(|p| p.is_alive()) {
                    BattleAction::Attack(pi)
                } else {
                    BattleAction::Defend
                }
            } else {
                BattleAction::Defend
            }
        }
    }

    fn actor_info(&self, idx: usize) -> Option<ActorInfo> {
        if idx < self.party.len() {
            self.party.get(idx).map(|a| ActorInfo {
                id: a.id, name: a.name, element: a.element, level: a.level, attack: a.attack, pp: a.pp,
            })
        } else {
            self.enemies.get(idx.saturating_sub(self.party.len())).map(|a| ActorInfo {
                id: a.id, name: a.name, element: a.element, level: a.level, attack: a.attack, pp: a.pp,
            })
        }
    }

    fn target_info(&self, target: usize, is_party_target: bool) -> Option<TargetInfo> {
        let arr = if is_party_target { &self.party } else { &self.enemies };
        arr.get(target).filter(|c| c.is_alive()).map(|c| TargetInfo {
            defense: c.defense, element: c.element, hp: c.hp, id: c.id, name: c.name,
        })
    }

    fn apply_damage(&mut self, target: usize, is_party_target: bool, dmg: u32) {
        let arr = if is_party_target { &mut self.party } else { &mut self.enemies };
        if let Some(c) = arr.get_mut(target) {
            c.take_damage(dmg);
        }
    }

    fn deduct_pp(&mut self, actor_idx: usize, is_party: bool, cost: u32) {
        let idx = if is_party { actor_idx } else { actor_idx.saturating_sub(self.party.len()) };
        let arr = if is_party { &mut self.party } else { &mut self.enemies };
        if let Some(c) = arr.get_mut(idx) {
            c.pp = c.pp.saturating_sub(cost);
        }
    }

    /// 统计待命 Djinn 数量（已装备但未释放的 Djinn）
    pub fn collect_standby_djinn_count(&self, collected: &[crate::data::djinn::OwnedDjinn]) -> usize {
        collected.iter().filter(|d| d.equipped && !d.released).count()
    }

    /// 消耗待命 Djinn 数量（标记为已释放）
    pub fn consume_standby_djinn(&mut self, count: usize) {
        self.standby_djinn_count = self.standby_djinn_count.saturating_sub(count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_party() -> Vec<Combatant> {
        vec![
            Combatant::new(1, "Isaac", 5, Element::Venus, true),
            Combatant::new(2, "Garet", 5, Element::Mars, true),
        ]
    }

    fn make_test_enemies() -> Vec<Combatant> {
        vec![
            Combatant::new(10, "Wolf", 3, Element::Jupiter, false),
            Combatant::new(11, "Bat", 2, Element::Mercury, false),
        ]
    }

    #[test]
    fn battle_init_phase() {
        let b = Battle::new(make_test_party(), make_test_enemies());
        assert_eq!(b.phase, BattlePhase::PlayerInput);
        assert_eq!(b.party.len(), 2);
        assert_eq!(b.enemies.len(), 2);
    }

    #[test]
    fn combatant_take_damage() {
        let mut c = Combatant::new(1, "Test", 5, Element::Venus, true);
        c.take_damage(10);
        assert_eq!(c.hp, c.max_hp - 10);
    }

    #[test]
    fn combatant_death() {
        let mut c = Combatant::new(1, "Test", 5, Element::Venus, true);
        c.take_damage(999);
        assert!(!c.is_alive());
    }

    #[test]
    fn attack_reduces_enemy_hp() {
        let mut b = Battle::new(make_test_party(), make_test_enemies());
        let initial_hp = b.enemies[0].hp;
        b.execute_turn(BattleAction::Attack(0));
        assert!(b.enemies[0].hp < initial_hp);
    }

    #[test]
    fn defend_does_no_damage() {
        let mut b = Battle::new(make_test_party(), make_test_enemies());
        let initial_hp = b.enemies[0].hp;
        b.execute_turn(BattleAction::Defend);
        assert_eq!(b.enemies[0].hp, initial_hp);
    }

    #[test]
    fn all_enemies_defeated_detection() {
        let mut b = Battle::new(make_test_party(), make_test_enemies());
        b.enemies[0].hp = 0;
        b.enemies[1].hp = 0;
        assert!(b.all_enemies_defeated());
    }

    #[test]
    fn all_party_defeated_detection() {
        let mut b = Battle::new(make_test_party(), make_test_enemies());
        b.party[0].hp = 0;
        b.party[1].hp = 0;
        assert!(b.all_party_defeated());
    }

    #[test]
    fn victory_phase_on_all_enemies_down() {
        let mut b = Battle::new(make_test_party(), vec![
            Combatant::new(10, "Slime", 1, Element::Mercury, false)
        ]);
        b.execute_turn(BattleAction::Attack(0));
        b.enemies[0].hp = 0;
        b.advance_phase();
        assert_eq!(b.phase, BattlePhase::Victory);
    }

    #[test]
    fn speed_order_has_all_combatants() {
        let order = Battle::compute_speed_order(&make_test_party(), &make_test_enemies());
        assert_eq!(order.len(), 4);
        assert_eq!(order[0], 0);
        assert_eq!(order[1], 1);
        assert_eq!(order[2], 2);
        assert_eq!(order[3], 3);
    }

    #[test]
    fn total_exp_and_coins() {
        let b = Battle::new(make_test_party(), make_test_enemies());
        assert_eq!(b.total_exp, (3 + 2) * 3);
        assert_eq!(b.total_coins, (3 + 2) * 2);
    }

    #[test]
    fn enemy_ai_returns_attack() {
        let b = Battle::new(make_test_party(), make_test_enemies());
        let action = b.enemy_decision();
        assert!(matches!(action, BattleAction::Attack(_)));
    }
}
