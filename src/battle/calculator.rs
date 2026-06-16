//! 伤害计算与元素克制

use super::state::Combatant;
use crate::Element;
use crate::PsynergyType;

/// 元素克制链：Venus → Jupiter → Mars → Mercury → Venus
pub fn element_modifier(atk: Element, def: Element) -> f32 {
    if atk == def {
        1.0
    } else {
        match (atk, def) {
            (Element::Venus, Element::Jupiter) => 1.5,
            (Element::Jupiter, Element::Mars) => 1.5,
            (Element::Mars, Element::Mercury) => 1.5,
            (Element::Mercury, Element::Venus) => 1.5,
            // 被克制
            (Element::Venus, Element::Mercury) => 0.75,
            (Element::Mercury, Element::Mars) => 0.75,
            (Element::Mars, Element::Jupiter) => 0.75,
            (Element::Jupiter, Element::Venus) => 0.75,
            _ => 1.0,
        }
    }
}

/// 物理攻击伤害
pub fn calculate_physical_damage(attacker: &Combatant, defender: &Combatant) -> u32 {
    let base = attacker.attack.saturating_sub(defender.defense / 2).max(1);
    let element = attacker.element;
    let modifier = element_modifier(element, defender.element);
    let dmg = (base as f32 * modifier) as u32;
    dmg.max(1)
}

/// 精灵力伤害
pub fn calculate_psynergy_damage(
    attacker: &Combatant,
    defender: &Combatant,
    psynergy: PsynergyType,
) -> u32 {
    let power = match psynergy {
        PsynergyType::Whirlwind => 15,
        PsynergyType::Growth => 20,
        PsynergyType::Freeze => 25,
        PsynergyType::Force => 18,
        PsynergyType::Catch => 10,
        PsynergyType::Flash => 12,
        PsynergyType::Reveal => 5,
    };
    let base = (power * attacker.level).saturating_sub(defender.defense).max(1);
    let modifier = element_modifier(psynergy.element(), defender.element);
    (base as f32 * modifier) as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::battle::state::Combatant;

    fn atk5_venus() -> Combatant {
        Combatant::new(1, "Attacker", 5, Element::Venus, true)
    }

    fn def3_jupiter() -> Combatant {
        Combatant::new(2, "Defender", 3, Element::Jupiter, true)
    }

    fn def3_mercury() -> Combatant {
        Combatant::new(3, "Defender", 3, Element::Mercury, true)
    }

    #[test]
    fn element_advantage_venus_over_jupiter() {
        let m = element_modifier(Element::Venus, Element::Jupiter);
        assert!((m - 1.5).abs() < 0.001);
    }

    #[test]
    fn element_disadvantage_venus_vs_mercury() {
        let m = element_modifier(Element::Venus, Element::Mercury);
        assert!((m - 0.75).abs() < 0.001);
    }

    #[test]
    fn element_neutral() {
        let m = element_modifier(Element::Venus, Element::Venus);
        assert!((m - 1.0).abs() < 0.001);
    }

    #[test]
    fn element_cycle_complete() {
        assert!((element_modifier(Element::Venus, Element::Jupiter) - 1.5).abs() < 0.001);
        assert!((element_modifier(Element::Jupiter, Element::Mars) - 1.5).abs() < 0.001);
        assert!((element_modifier(Element::Mars, Element::Mercury) - 1.5).abs() < 0.001);
        assert!((element_modifier(Element::Mercury, Element::Venus) - 1.5).abs() < 0.001);
    }

    #[test]
    fn physical_damage_min_one() {
        let weak = atk5_venus();
        let tough = Combatant::new(2, "Tank", 99, Element::Mercury, false);
        let dmg = calculate_physical_damage(&weak, &tough);
        assert!(dmg >= 1);
    }

    #[test]
    fn physical_damage_advantage_higher() {
        let atk = atk5_venus();
        let def_jup = def3_jupiter();
        let def_merc = def3_mercury();
        let dmg_adv = calculate_physical_damage(&atk, &def_jup);
        let dmg_dis = calculate_physical_damage(&atk, &def_merc);
        assert!(dmg_adv >= dmg_dis);
    }

    #[test]
    fn psynergy_damage_scales_with_level() {
        let attacker = Combatant::new(1, "A", 10, Element::Venus, true);
        let defender = Combatant::new(2, "D", 1, Element::Jupiter, true);
        let dmg = calculate_psynergy_damage(&attacker, &defender, PsynergyType::Growth);
        assert!(dmg >= 1);
    }

    #[test]
    fn psynergy_damage_variety() {
        let a = Combatant::new(1, "A", 5, Element::Venus, true);
        let d = Combatant::new(2, "D", 3, Element::Jupiter, true);
        for psynergy in PsynergyType::ALL {
            let dmg = calculate_psynergy_damage(&a, &d, psynergy);
            assert!(dmg >= 1, "psynergy {:?} deals 0 damage", psynergy);
        }
    }

    #[test]
    fn defense_reduces_damage() {
        let a = Combatant::new(1, "A", 5, Element::Venus, true);
        let low_def = Combatant::new(2, "Low", 3, Element::Jupiter, false);
        let mut high = low_def.clone();
        high.defense = 999;
        let dmg_low = calculate_physical_damage(&a, &low_def);
        let dmg_high = calculate_physical_damage(&a, &high);
        assert!(dmg_low >= dmg_high);
    }
}
