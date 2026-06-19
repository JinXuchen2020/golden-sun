//! 召唤系统 — 黄金太阳经典召唤技能

use crate::Element;

/// 召唤技能
#[derive(Debug, Clone)]
pub struct Summon {
    pub name: &'static str,
    pub element: Element,
    pub power: u32,
    pub pp_cost: u32,
    pub djinn_required: u32,
    pub description: &'static str,
}

impl Summon {
    pub fn base_damage(&self, level: u32) -> u32 {
        self.power * level / 2 + self.power * 3
    }
}

/// 所有召唤列表
pub fn all_summons() -> &'static [Summon] {
    static SUMMONS: &[Summon] = &[
        Summon { name: "Venus",  element: Element::Venus,   power: 30, pp_cost: 8,  djinn_required: 2, description: "召唤大地精灵 Venus" },
        Summon { name: "Ramses", element: Element::Venus,   power: 50, pp_cost: 15, djinn_required: 3, description: "召唤巨像 Ramses" },
        Summon { name: "Cybele", element: Element::Venus,   power: 80, pp_cost: 25, djinn_required: 4, description: "召唤大地之母 Cybele" },
        Summon { name: "Neptune",element: Element::Mercury, power: 30, pp_cost: 8,  djinn_required: 2, description: "召唤水之精灵 Neptune" },
        Summon { name: "Boreas", element: Element::Mercury, power: 60, pp_cost: 18, djinn_required: 3, description: "召唤极寒之力 Boreas" },
        Summon { name: "Mars",   element: Element::Mars,    power: 35, pp_cost: 9,  djinn_required: 2, description: "召唤火之精灵 Mars" },
        Summon { name: "Meteor", element: Element::Mars,    power: 65, pp_cost: 20, djinn_required: 3, description: "召唤陨石 Meteor" },
        Summon { name: "Jupiter",element: Element::Jupiter, power: 30, pp_cost: 8,  djinn_required: 2, description: "召唤风之精灵 Jupiter" },
        Summon { name: "Atlas",  element: Element::Jupiter, power: 70, pp_cost: 22, djinn_required: 4, description: "召唤巨人 Atlas" },
    ];
    SUMMONS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_summons_returns_9() {
        assert_eq!(all_summons().len(), 9);
    }

    #[test]
    fn summon_requires_djinn() {
        for s in all_summons() {
            assert!(s.djinn_required >= 1);
            assert!(s.djinn_required <= 4);
            assert!(s.pp_cost > 0);
        }
    }

    #[test]
    fn summon_base_damage_scales_with_level() {
        let venus = &all_summons()[0];
        let dmg1 = venus.base_damage(1);
        let dmg5 = venus.base_damage(5);
        assert!(dmg5 > dmg1);
    }

    #[test]
    fn summon_elements_cover_all_four() {
        let elements: Vec<_> = all_summons().iter().map(|s| s.element.as_str()).collect();
        assert!(elements.contains(&"Venus"));
        assert!(elements.contains(&"Mercury"));
        assert!(elements.contains(&"Mars"));
        assert!(elements.contains(&"Jupiter"));
    }
}
