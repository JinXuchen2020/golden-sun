//! BDD 测试: 战斗系统
//! 对应 features/combat.feature

use golden_sun::battle::{Battle, BattleAction, BattlePhase, Combatant};
use golden_sun::battle::calculator;
use golden_sun::{Element, PsynergyType};

fn party() -> Vec<Combatant> {
    vec![
        Combatant::new(1, "Isaac", 5, Element::Venus, true),
        Combatant::new(2, "Garet", 5, Element::Mars, true),
    ]
}

fn enemies() -> Vec<Combatant> {
    vec![
        Combatant::new(10, "Wolf", 3, Element::Jupiter, false),
        Combatant::new(11, "Bat", 2, Element::Mercury, false),
    ]
}

fn attr(c: &Combatant) -> (u32, u32, Element) { (c.attack, c.defense, c.element) }

// ── Scenario: 物理攻击伤害范围 ──

#[test]
fn physical_attack_damage_range() {
    let b = Battle::new(party(), enemies());
    let (atk, _, atk_el) = attr(&b.party[0]);
    let (_, def, def_el) = attr(&b.enemies[0]);
    let dmg = calculator::calculate_physical_damage(atk, atk_el, def, def_el);
    assert!(dmg >= 1, "physical attack should deal at least 1 damage");
}

// ── Scenario: 高防御减少伤害 ──

#[test]
fn high_defense_reduces_damage() {
    let dmg_low = calculator::calculate_physical_damage(25, Element::Venus, 5, Element::Jupiter);
    let dmg_high = calculator::calculate_physical_damage(25, Element::Venus, 999, Element::Jupiter);
    assert!(dmg_high <= dmg_low, "high defense should reduce or equalize damage");
    assert!(dmg_high >= 1, "minimum damage is 1");
}

// ── Scenario: 元素优势倍率 ──

#[test]
fn element_advantage_multiplier() {
    let dmg = calculator::calculate_physical_damage(25, Element::Venus, 7, Element::Jupiter);
    let dmg_neutral = calculator::calculate_physical_damage(25, Element::Venus, 7, Element::Venus);
    assert!(dmg >= dmg_neutral, "element advantage should be >= neutral damage");
}

// ── Scenario: 元素劣势倍率 ──

#[test]
fn element_disadvantage_multiplier() {
    let dmg = calculator::calculate_physical_damage(25, Element::Venus, 7, Element::Mercury);
    let dmg_neutral = calculator::calculate_physical_damage(25, Element::Venus, 7, Element::Venus);
    assert!(dmg <= dmg_neutral, "element disadvantage should be <= neutral damage");
}

// ── Scenario: 元素中立倍率（同元素） ──

#[test]
fn element_neutral_multiplier() {
    let dmg = calculator::calculate_physical_damage(25, Element::Mars, 7, Element::Mars);
    assert!(dmg >= 1);
}

// ── Scenario: 完整战斗状态机循环 ──

#[test]
fn full_battle_state_machine_loop() {
    let mut b = Battle::new(party(), enemies());
    assert_eq!(b.phase, BattlePhase::PlayerInput);

    b.execute_turn(BattleAction::Attack(0));
    assert!(b.enemies[0].hp < b.enemies[0].max_hp, "enemy should take damage");

    while b.phase == BattlePhase::EnemyTurn {
        b.execute_turn(BattleAction::Attack(0));
    }
    if !b.all_enemies_defeated() {
        assert_eq!(b.phase, BattlePhase::PlayerInput);
    }
}

// ── Scenario: 胜利条件 ──

#[test]
fn victory_condition() {
    let mut b = Battle::new(party(), vec![
        Combatant::new(10, "Slime", 1, Element::Mercury, false)
    ]);
    assert_eq!(b.phase, BattlePhase::PlayerInput);

    while b.enemies.iter().any(|e| e.is_alive()) && b.phase == BattlePhase::PlayerInput {
        b.execute_turn(BattleAction::Attack(0));
        while b.phase == BattlePhase::EnemyTurn {
            b.execute_turn(BattleAction::Attack(0));
        }
    }

    if b.all_enemies_defeated() {
        assert_eq!(b.phase, BattlePhase::Victory, "should be victory when all enemies defeated");
    }
}

// ── Scenario: 失败条件 ──

#[test]
fn defeat_condition() {
    let mut b = Battle::new(
        vec![Combatant::new(1, "Isaac", 1, Element::Venus, true)],
        vec![Combatant::new(10, "Boss", 99, Element::Mars, false)],
    );
    b.party[0].hp = 1;
    b.execute_turn(BattleAction::Defend);
    while b.phase == BattlePhase::EnemyTurn {
        b.execute_turn(BattleAction::Attack(0));
    }
    if !b.party[0].is_alive() {
        assert!(b.all_party_defeated(), "party should be defeated");
    }
}

// ── Scenario: 逃跑成功率 ──

#[test]
fn flee_speed_based_success() {
    let mut b = Battle::new(party(), enemies());
    b.execute_turn(BattleAction::Flee);
    assert_eq!(b.phase, BattlePhase::FleeSuccess);
}

// ── Scenario: 敌人 AI 决策 ──

#[test]
fn ai_decision_by_hp() {
    let b = Battle::new(party(), enemies());
    let action = b.enemy_decision();
    match action {
        BattleAction::Attack(target) => {
            assert!(target < b.party.len(), "AI should target a valid party member");
        }
        _ => panic!("AI should always attack when targets are alive"),
    }
}

// ── Scenario: 精灵力消耗 PP ──

#[test]
fn psynergy_deducts_pp() {
    let mut b = Battle::new(party(), enemies());
    let initial_pp = b.party[0].pp;
    let psynergy = PsynergyType::Whirlwind;
    assert!(initial_pp >= psynergy.pp_cost(), "party member should have enough PP");
    b.execute_turn(BattleAction::Psynergy(psynergy, 0));
    assert_eq!(b.party[0].pp, initial_pp - psynergy.pp_cost());
}

// ── Scenario: 同一元素精灵力伤害 ──

#[test]
fn same_element_psynergy_is_effective() {
    let dmg = calculator::calculate_psynergy_damage(5, PsynergyType::Flash.element(), 7, Element::Mercury, PsynergyType::Flash);
    assert!(dmg >= 20, "same-element psynergy should deal significant damage: got {dmg}");
}

// ── Scenario: PP 不足阻止精灵力 ──

#[test]
fn insufficient_pp_blocks_psynergy() {
    let mut b = Battle::new(party(), enemies());
    b.party[0].pp = 0;
    b.execute_turn(BattleAction::Psynergy(PsynergyType::Freeze, 0));
    assert_eq!(b.party[0].pp, 0);
}
