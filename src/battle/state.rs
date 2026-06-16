//! 战斗系统核心 — 回合制战斗状态机

use super::calculator;
use crate::Element;
use crate::PsynergyType;

/// 元素增益/减益状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusEffect {
    None,
    Delay,    // 下回合无法行动
    BuffAtk,  // 攻击力提升
    BuffDef,  // 防御力提升
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
        }
    }

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
    Flee,
}

/// 回合记录
#[derive(Debug, Clone)]
pub struct BattleTurn {
    pub actor_index: usize,
    pub action: BattleAction,
    pub log: String,
}

/// 单次攻击结果
#[derive(Debug, Clone)]
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
    Init,
    PlayerInput,
    EnemyTurn,
    Animating,
    Victory,
    Defeat,
    FleeSuccess,
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
    pub turn_actions: Vec<BattleTurn>,
    pub total_exp: u32,
    pub total_coins: u32,
}

impl Battle {
    pub fn new(party: Vec<Combatant>, enemies: Vec<Combatant>) -> Self {
        let total_exp = enemies.iter().map(|e| e.level * 3).sum();
        let total_coins = enemies.iter().map(|e| e.level * 2).sum();
        let turn_order = Self::compute_speed_order(&party, &enemies);
        Self {
            phase: BattlePhase::PlayerInput,
            party, enemies,
            turn_order,
            turn_index: 0,
            logs: Vec::new(),
            results: Vec::new(),
            turn_actions: Vec::new(),
            total_exp, total_coins,
        }
    }

    pub fn all_enemies_defeated(&self) -> bool {
        self.enemies.iter().all(|e| !e.is_alive())
    }

    pub fn all_party_defeated(&self) -> bool {
        self.party.iter().all(|p| !p.is_alive())
    }

    pub fn alive_count(&self, is_player: bool) -> usize {
        if is_player {
            self.party.iter().filter(|c| c.is_alive()).count()
        } else {
            self.enemies.iter().filter(|c| c.is_alive()).count()
        }
    }

    /// 执行一次行动（回合中的一步）
    pub fn execute_turn(&mut self, action: BattleAction) -> Vec<AttackResult> {
        let mut results = Vec::new();
        let actor_idx = self.turn_order[self.turn_index];
        let is_party_actor = actor_idx < self.party.len();

        // 提取 actor 信息（避免同一 Vec 的 immutable + mutable borrow）
        let actor_id: u32;
        let actor_name: &str;
        let actor_element: Element;
        let actor_level: u32;
        let actor_attack: u32;
        let actor_defense: u32;
        let actor_pp: u32;
        if is_party_actor {
            let a = &self.party[actor_idx];
            actor_id = a.id;
            actor_name = a.name;
            actor_element = a.element;
            actor_level = a.level;
            actor_attack = a.attack;
            actor_defense = a.defense;
            actor_pp = a.pp;
        } else {
            let a = &self.enemies[actor_idx - self.party.len()];
            actor_id = a.id;
            actor_name = a.name;
            actor_element = a.element;
            actor_level = a.level;
            actor_attack = a.attack;
            actor_defense = a.defense;
            actor_pp = a.pp;
        }

        match action {
            BattleAction::Attack(target) => {
                if target < self.enemies.len() && self.enemies[target].is_alive() {
                    let enemy_element = self.enemies[target].element;
                    let enemy_name = self.enemies[target].name;
                    let enemy_hp = self.enemies[target].hp;
                    let enemy_defense = self.enemies[target].defense;
                    let temp_atk = Combatant {
                        id: actor_id, name: actor_name, hp: 0, max_hp: 0, pp: 0, max_pp: 0,
                        attack: actor_attack, defense: actor_defense, speed: 0, level: actor_level,
                        element: actor_element, status: StatusEffect::None, is_player: is_party_actor,
                    };
                    let temp_def = Combatant {
                        id: self.enemies[target].id, name: enemy_name, hp: enemy_hp, max_hp: 0,
                        pp: 0, max_pp: 0, attack: 0, defense: enemy_defense, speed: 0, level: 0,
                        element: enemy_element, status: StatusEffect::None, is_player: false,
                    };
                    let dmg = calculator::calculate_physical_damage(&temp_atk, &temp_def);
                    let modifier = calculator::element_modifier(actor_element, enemy_element);
                    let killed = dmg >= self.enemies[target].hp;
                    self.enemies[target].take_damage(dmg);
                    results.push(AttackResult {
                        attacker: actor_id, target: self.enemies[target].id,
                        damage: dmg, element: actor_element, modifier, killed,
                    });
                    self.logs.push(format!("{actor_name} attacks {enemy_name} for {dmg} damage!"));
                }
            }
            BattleAction::Defend => {
                self.logs.push(format!("{actor_name} defends!"));
            }
            BattleAction::Psynergy(psynergy, target) => {
                if actor_pp >= psynergy.pp_cost() {
                    if target < self.enemies.len() && self.enemies[target].is_alive() {
                        let enemy_element = self.enemies[target].element;
                        let enemy_name = self.enemies[target].name;
                        let enemy_hp = self.enemies[target].hp;
                        let enemy_defense = self.enemies[target].defense;
                        let temp_atk = Combatant {
                            id: actor_id, name: actor_name, hp: 0, max_hp: 0, pp: 0, max_pp: 0,
                            attack: actor_attack, defense: actor_defense, speed: 0, level: actor_level,
                            element: actor_element, status: StatusEffect::None, is_player: is_party_actor,
                        };
                        let temp_def = Combatant {
                            id: self.enemies[target].id, name: enemy_name, hp: enemy_hp, max_hp: 0,
                            pp: 0, max_pp: 0, attack: 0, defense: enemy_defense, speed: 0, level: 0,
                            element: enemy_element, status: StatusEffect::None, is_player: false,
                        };
                        let dmg = calculator::calculate_psynergy_damage(&temp_atk, &temp_def, psynergy);
                        let modifier = calculator::element_modifier(psynergy.element(), enemy_element);
                        let killed = dmg >= self.enemies[target].hp;
                        self.enemies[target].take_damage(dmg);
                        results.push(AttackResult {
                            attacker: actor_id, target: self.enemies[target].id,
                            damage: dmg, element: psynergy.element(), modifier, killed,
                        });
                        self.logs.push(format!("{actor_name} uses {psynergy:?} on {enemy_name} for {dmg} damage!"));
                    }
                } else {
                    self.logs.push("Not enough PP!".to_string());
                }
            }
            BattleAction::Flee => {
                let party_speed: u32 = self.party.iter().map(|p| p.speed).sum();
                let enemy_speed: u32 = self.enemies.iter().map(|e| e.speed).sum();
                if party_speed > enemy_speed {
                    self.phase = BattlePhase::FleeSuccess;
                    self.logs.push("You fled successfully!".to_string());
                } else {
                    self.logs.push("Failed to flee!".to_string());
                }
            }
        }

        if let BattleAction::Psynergy(psynergy, _) = action {
            if is_party_actor {
                self.party[actor_idx].pp = self.party[actor_idx].pp.saturating_sub(psynergy.pp_cost());
            } else {
                let e_idx = actor_idx - self.party.len();
                self.enemies[e_idx].pp = self.enemies[e_idx].pp.saturating_sub(psynergy.pp_cost());
            }
        }

        self.turn_actions.push(BattleTurn {
            actor_index: actor_idx,
            action,
            log: self.logs.last().cloned().unwrap_or_default(),
        });

        self.results.extend(results.clone());
        self.advance_phase();
        results
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
        if self.phase == BattlePhase::Victory
            || self.phase == BattlePhase::Defeat
            || self.phase == BattlePhase::FleeSuccess
        {
            return;
        }
        if self.all_enemies_defeated() {
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
            self.turn_order = Self::compute_speed_order(&self.party, &self.enemies);

            let next = self.turn_order[0];
            if next >= self.party.len() {
                self.phase = BattlePhase::EnemyTurn;
            } else {
                self.phase = BattlePhase::PlayerInput;
            }
        } else {
            let next = self.turn_order[self.turn_index];
            if next >= self.party.len() {
                self.phase = BattlePhase::EnemyTurn;
            } else {
                self.phase = BattlePhase::PlayerInput;
            }
        }
    }

    /// 简单敌人 AI — 攻击第一个存活目标
    pub fn enemy_decision(&self, _enemy_index: usize) -> BattleAction {
        for (i, c) in self.party.iter().enumerate() {
            if c.is_alive() {
                return BattleAction::Attack(i);
            }
        }
        BattleAction::Defend
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
        if b.enemies[0].hp == 0 {
            b.enemies[0].hp = 0;
            b.advance_phase();
        }
        // Re-check after killing blows
        b.enemies[0].hp = 0;
        b.advance_phase();
        assert_eq!(b.phase, BattlePhase::Victory);
    }

    #[test]
    fn speed_order_has_all_combatants() {
        let order = Battle::compute_speed_order(&make_test_party(), &make_test_enemies());
        assert_eq!(order.len(), 4);
        // Speed: Isaac=15, Garet=15, Wolf=11, Bat=9
        assert_eq!(order[0], 0); // Isaac (speed=15)
        assert_eq!(order[1], 1); // Garet (speed=15)
        assert_eq!(order[2], 2); // Wolf  (speed=11)
        assert_eq!(order[3], 3); // Bat   (speed=9)
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
        let action = b.enemy_decision(0);
        assert!(matches!(action, BattleAction::Attack(_)));
    }
}
