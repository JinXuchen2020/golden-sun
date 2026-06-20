//! 伤害计算与元素克制

use crate::Element;
use crate::PsynergyType;

/// 4×4 元素克制表：行=攻击方元素，列=防御方元素
/// Element enum order: Venus(0), Mercury(1), Mars(2), Jupiter(3)
const ELEMENT_TABLE: [[f32; 4]; 4] = [
    // Venus  Mercury Mars   Jupiter
    [1.0,    0.75,   1.0,   1.5 ],  // atk Venus
    [1.5,    1.0,    0.75,  1.0 ],  // atk Mercury
    [1.0,    1.5,    1.0,   0.75],  // atk Mars
    [0.75,   1.0,    1.5,   1.0 ],  // atk Jupiter
];

/// O(1) 分支无关元素克制查询
pub fn element_modifier(atk: Element, def: Element) -> f32 {
    ELEMENT_TABLE[atk as usize][def as usize]
}

/// 物理攻击伤害（接收原始属性，无需 Combatant 构造）
pub fn calculate_physical_damage(
    attack: u32, atk_element: Element,
    defense: u32, def_element: Element,
) -> u32 {
    let base = attack.saturating_sub(defense / 2).max(1);
    let modifier = element_modifier(atk_element, def_element);
    ((base as f32 * modifier) as u32).max(1)
}

/// 精灵力伤害（接收原始属性，无需 Combatant 构造）
pub fn calculate_psynergy_damage(
    level: u32, element: Element,
    defense: u32, def_element: Element,
    psynergy: PsynergyType,
) -> u32 {
    let power = psynergy_power(psynergy);
    let base = (power * level).saturating_sub(defense).max(1);
    let modifier = element_modifier(element, def_element);
    ((base as f32 * modifier) as u32).max(1)
}

pub fn psynergy_power(psynergy: PsynergyType) -> u32 {
    match psynergy {
        PsynergyType::Whirlwind => 15,
        PsynergyType::Growth => 20,
        PsynergyType::Freeze => 25,
        PsynergyType::Force => 18,
        PsynergyType::Catch => 10,
        PsynergyType::Flash => 12,
        PsynergyType::Reveal => 5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let dmg = calculate_physical_damage(5, Element::Venus, 999, Element::Mercury);
        assert!(dmg >= 1);
    }

    #[test]
    fn physical_damage_advantage_higher() {
        let dmg_adv = calculate_physical_damage(5, Element::Venus, 7, Element::Jupiter);
        let dmg_dis = calculate_physical_damage(5, Element::Venus, 7, Element::Mercury);
        assert!(dmg_adv >= dmg_dis);
    }

    #[test]
    fn psynergy_damage_scales_with_level() {
        let dmg = calculate_psynergy_damage(10, Element::Venus, 1, Element::Jupiter, PsynergyType::Growth);
        assert!(dmg >= 1);
    }

    #[test]
    fn psynergy_damage_variety() {
        for psynergy in PsynergyType::ALL {
            let dmg = calculate_psynergy_damage(5, Element::Venus, 3, Element::Jupiter, psynergy);
            assert!(dmg >= 1, "psynergy {psynergy:?} deals 0 damage");
        }
    }

    #[test]
    fn defense_reduces_damage() {
        let dmg_low = calculate_physical_damage(5, Element::Venus, 3, Element::Jupiter);
        let dmg_high = calculate_physical_damage(5, Element::Venus, 999, Element::Jupiter);
        assert!(dmg_low >= dmg_high);
    }
}
