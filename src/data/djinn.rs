//! Djinn 精灵系统 — 收集、装备、释放/召回、属性加成、职业切换
//!
//! Djinn 是黄金太阳的核心系统：
//! - 地图上散布可收集的 Djinn
//! - 装备 Djinn 永久提升角色属性
//! - 释放 Djinn 在战斗中临时大幅提升属性
//! - 召回 Djinn 恢复基础属性
//! - 同元素 Djinn 成对装备触发 Set Bonus
//! - 特定 Djinn 组合解锁新精灵力

use crate::Element;

/// Djinn ID — 唯一标识每个精灵
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum DjinnId {
    // Venus (Earth) set — 12 个
    Fafnir,     // +Atk
    Belian,     // +Def
    Alupam,     // +Atk +Def
    Gnome,      // +HP
    // Mercury (Water) set
    Vermin,     // +Speed
    Malina,     // +PP
    Kelpie,     // +Atk +PP
    Undine,     // +Def +Speed
    // Mars (Fire) set
    Laguna,     // +Atk
    Betsy,      // +Def
    Grendel,    // +HP +Def
    Naga,       // +Atk +Def
    // Jupiter (Wind) set
    Titania,    // +Speed
    Siren,      // +PP
    Amduscia,   // +Atk +PP
    Marilith,   // +Atk +Def +Speed
}

impl DjinnId {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            DjinnId::Fafnir => "Fafnir",
            DjinnId::Belian => "Belian",
            DjinnId::Alupam => "Alupam",
            DjinnId::Gnome => "Gnome",
            DjinnId::Vermin => "Vermin",
            DjinnId::Malina => "Malina",
            DjinnId::Kelpie => "Kelpie",
            DjinnId::Undine => "Undine",
            DjinnId::Laguna => "Laguna",
            DjinnId::Betsy => "Betsy",
            DjinnId::Grendel => "Grendel",
            DjinnId::Naga => "Naga",
            DjinnId::Titania => "Titania",
            DjinnId::Siren => "Siren",
            DjinnId::Amduscia => "Amduscia",
            DjinnId::Marilith => "Marilith",
        }
    }

    #[must_use]
    pub fn element(self) -> Element {
        match self {
            DjinnId::Fafnir | DjinnId::Belian | DjinnId::Alupam | DjinnId::Gnome => Element::Venus,
            DjinnId::Vermin | DjinnId::Malina | DjinnId::Kelpie | DjinnId::Undine => Element::Mercury,
            DjinnId::Laguna | DjinnId::Betsy | DjinnId::Grendel | DjinnId::Naga => Element::Mars,
            DjinnId::Titania | DjinnId::Siren | DjinnId::Amduscia | DjinnId::Marilith => Element::Jupiter,
        }
    }

    #[must_use]
    pub const fn id(self) -> u32 {
        match self {
            DjinnId::Fafnir => 1,
            DjinnId::Belian => 2,
            DjinnId::Alupam => 3,
            DjinnId::Gnome => 4,
            DjinnId::Vermin => 5,
            DjinnId::Malina => 6,
            DjinnId::Kelpie => 7,
            DjinnId::Undine => 8,
            DjinnId::Laguna => 9,
            DjinnId::Betsy => 10,
            DjinnId::Grendel => 11,
            DjinnId::Naga => 12,
            DjinnId::Titania => 13,
            DjinnId::Siren => 14,
            DjinnId::Amduscia => 15,
            DjinnId::Marilith => 16,
        }
    }
}

/// Djinn 数据 — 装备时永久提供的属性加成
#[derive(Debug, Clone)]
pub struct Djinn {
    pub id: DjinnId,
    /// 攻击加成
    pub atk_bonus: u32,
    /// 防御加成
    pub def_bonus: u32,
    /// HP 加成
    pub hp_bonus: u32,
    /// PP 加成
    pub pp_bonus: u32,
    /// 速度加成
    pub speed_bonus: u32,
    /// 释放时的战斗临时加成倍率
    pub release_atk_mult: u32,
    pub release_def_mult: u32,
    pub release_hp_mult: u32,
    pub release_pp_mult: u32,
    pub release_speed_mult: u32,
}

impl Djinn {
    #[must_use]
    pub fn name(&self) -> &'static str {
        self.id.as_str()
    }

    #[must_use]
    pub fn element(&self) -> Element {
        self.id.element()
    }

    /// 获取装备时的总属性加成
    #[must_use]
    pub fn total_atk(&self) -> u32 { self.atk_bonus }

    #[must_use]
    pub fn total_def(&self) -> u32 { self.def_bonus }

    #[must_use]
    pub fn total_hp(&self) -> u32 { self.hp_bonus }

    #[must_use]
    pub fn total_pp(&self) -> u32 { self.pp_bonus }

    #[must_use]
    pub fn total_speed(&self) -> u32 { self.speed_bonus }
}

/// 已收集的 Djinn — 包含装备状态
#[derive(Debug, Clone)]
pub struct OwnedDjinn {
    pub djinn: Djinn,
    /// 是否已装备
    pub equipped: bool,
    /// 当前装备给哪个角色（0 = Isaac, 1 = Garet）
    pub equipped_to: Option<u32>,
    /// 是否在战斗中释放了（战斗中临时不可用）
    pub released: bool,
}

impl OwnedDjinn {
    #[must_use]
    pub fn new(djinn: Djinn) -> Self {
        Self {
            djinn,
            equipped: false,
            equipped_to: None,
            released: false,
        }
    }
}

/// Djinn 套装加成 — 当同元素 Djinn 成对装备时触发
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetBonus {
    None,
    /// 生命之环：装备 2+ 个同元素 Djinn 时 HP +20%
    LifeRing,
    /// 魔力之环：装备 2+ 个同元素 Djinn 时 PP +20%
    ManaRing,
    /// 力量之环：装备 2+ 个同元素 Djinn 时 ATK +20%
    PowerRing,
    /// 守护之环：装备 2+ 个同元素 Djinn 时 DEF +20%
    GuardRing,
    /// 敏捷之环：装备 2+ 个同元素 Djinn 时 SPD +20%
    SwiftRing,
    /// 贤者之戒：装备 4+ 个同元素 Djinn 时全属性 +15%
    SageRing,
}

impl SetBonus {
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            SetBonus::None => "No Bonus",
            SetBonus::LifeRing => "Life Ring (+20% HP)",
            SetBonus::ManaRing => "Mana Ring (+20% PP)",
            SetBonus::PowerRing => "Power Ring (+20% ATK)",
            SetBonus::GuardRing => "Guard Ring (+20% DEF)",
            SetBonus::SwiftRing => "Swift Ring (+20% SPD)",
            SetBonus::SageRing => "Sage Ring (+15% All)",
        }
    }

    #[must_use]
    pub fn element(&self) -> Option<Element> {
        match self {
            SetBonus::LifeRing | SetBonus::GuardRing | SetBonus::SageRing => Some(Element::Venus),
            SetBonus::ManaRing => Some(Element::Mercury),
            SetBonus::SwiftRing => Some(Element::Jupiter),
            SetBonus::PowerRing => Some(Element::Mars),
            _ => None,
        }
    }
}

/// 职业 — Djinn 装备组合决定的角色职业
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    Adept,      // 初心者 — 初始职业
    Psychokineticist, // 念动士 — Venus + Mercury
    Climber,    // 登山者 — Venus + Mars
    Dreamseer,  // 预知者 — Venus + Jupiter
    Warrior,    // 战士 — Mars + Mercury
    Dawnmist,   // 晨曦术士 — Mars + Jupiter
    Shadowreader, // 读心者 — Mercury + Jupiter
   _TRANSMOGGRIFIER, // 转化者 — 全部四个元素
}

impl Class {
    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            Class::Adept => "Adept",
            Class::Psychokineticist => "Psychokineticist",
            Class::Climber => "Climber",
            Class::Dreamseer => "Dreamseer",
            Class::Warrior => "Warrior",
            Class::Dawnmist => "Dawnmist",
            Class::Shadowreader => "Shadowreader",
            Class::_TRANSMOGGRIFIER => "Transmoggrifier",
        }
    }

    /// 根据装备的 Djinn 元素组合计算职业
    #[must_use]
    pub fn from_elements(elements: &[Element]) -> Self {
        let has_venus = elements.contains(&Element::Venus);
        let has_mercury = elements.contains(&Element::Mercury);
        let has_mars = elements.contains(&Element::Mars);
        let has_jupiter = elements.contains(&Element::Jupiter);

        let element_count = [has_venus, has_mercury, has_mars, has_jupiter]
            .iter().filter(|&&b| b).count();

        match element_count {
            0 => Class::Adept,
            1 => Class::Adept,
            2 => {
                if has_venus && has_mercury { Class::Psychokineticist }
                else if has_venus && has_mars { Class::Climber }
                else if has_venus && has_jupiter { Class::Dreamseer }
                else if has_mercury && has_mars { Class::Warrior }
                else if has_mercury && has_jupiter { Class::Shadowreader }
                else if has_mars && has_jupiter { Class::Dawnmist }
                else { Class::Adept }
            }
            3 => Class::_TRANSMOGGRIFIER,
            4 => Class::_TRANSMOGGRIFIER,
            _ => Class::Adept,
        }
    }

    /// 职业解锁的精灵力
    #[must_use]
    pub fn unlocked_psynergies(&self) -> Vec<crate::PsynergyType> {
        match self {
            Class::Adept => vec![
                crate::PsynergyType::Whirlwind,
                crate::PsynergyType::Growth,
                crate::PsynergyType::Freeze,
                crate::PsynergyType::Force,
            ],
            Class::Psychokineticist => vec![
                crate::PsynergyType::Whirlwind,
                crate::PsynergyType::Growth,
                crate::PsynergyType::Freeze,
                crate::PsynergyType::Force,
                crate::PsynergyType::Catch,
                crate::PsynergyType::Reveal,
            ],
            Class::Climber => vec![
                crate::PsynergyType::Whirlwind,
                crate::PsynergyType::Growth,
                crate::PsynergyType::Freeze,
                crate::PsynergyType::Force,
                crate::PsynergyType::Flash,
            ],
            Class::Dreamseer => vec![
                crate::PsynergyType::Whirlwind,
                crate::PsynergyType::Growth,
                crate::PsynergyType::Freeze,
                crate::PsynergyType::Force,
                crate::PsynergyType::Flash,
                crate::PsynergyType::Reveal,
            ],
            Class::Warrior => vec![
                crate::PsynergyType::Whirlwind,
                crate::PsynergyType::Growth,
                crate::PsynergyType::Freeze,
                crate::PsynergyType::Force,
                crate::PsynergyType::Flash,
                crate::PsynergyType::Catch,
            ],
            Class::Dawnmist => vec![
                crate::PsynergyType::Whirlwind,
                crate::PsynergyType::Growth,
                crate::PsynergyType::Freeze,
                crate::PsynergyType::Force,
                crate::PsynergyType::Catch,
                crate::PsynergyType::Reveal,
            ],
            Class::Shadowreader => vec![
                crate::PsynergyType::Whirlwind,
                crate::PsynergyType::Growth,
                crate::PsynergyType::Freeze,
                crate::PsynergyType::Force,
                crate::PsynergyType::Flash,
                crate::PsynergyType::Catch,
                crate::PsynergyType::Reveal,
            ],
            Class::_TRANSMOGGRIFIER => crate::PsynergyType::ALL.to_vec(),
        }
    }
}

/// 获取所有 Djinn 的数据表
#[must_use]
pub fn all_djinn_data() -> Vec<Djinn> {
    vec![
        // Venus set
        Djinn { id: DjinnId::Fafnir, atk_bonus: 3, def_bonus: 0, hp_bonus: 0, pp_bonus: 0, speed_bonus: 0, release_atk_mult: 2, release_def_mult: 1, release_hp_mult: 1, release_pp_mult: 1, release_speed_mult: 1 },
        Djinn { id: DjinnId::Belian, atk_bonus: 0, def_bonus: 3, hp_bonus: 0, pp_bonus: 0, speed_bonus: 0, release_atk_mult: 1, release_def_mult: 2, release_hp_mult: 1, release_pp_mult: 1, release_speed_mult: 1 },
        Djinn { id: DjinnId::Alupam, atk_bonus: 2, def_bonus: 2, hp_bonus: 0, pp_bonus: 0, speed_bonus: 0, release_atk_mult: 2, release_def_mult: 2, release_hp_mult: 1, release_pp_mult: 1, release_speed_mult: 1 },
        Djinn { id: DjinnId::Gnome, atk_bonus: 0, def_bonus: 0, hp_bonus: 15, pp_bonus: 0, speed_bonus: 0, release_atk_mult: 1, release_def_mult: 1, release_hp_mult: 2, release_pp_mult: 1, release_speed_mult: 1 },
        // Mercury set
        Djinn { id: DjinnId::Vermin, atk_bonus: 0, def_bonus: 0, hp_bonus: 0, pp_bonus: 0, speed_bonus: 3, release_atk_mult: 1, release_def_mult: 1, release_hp_mult: 1, release_pp_mult: 1, release_speed_mult: 2 },
        Djinn { id: DjinnId::Malina, atk_bonus: 0, def_bonus: 0, hp_bonus: 0, pp_bonus: 8, speed_bonus: 0, release_atk_mult: 1, release_def_mult: 1, release_hp_mult: 1, release_pp_mult: 2, release_speed_mult: 1 },
        Djinn { id: DjinnId::Kelpie, atk_bonus: 2, def_bonus: 0, hp_bonus: 0, pp_bonus: 4, speed_bonus: 0, release_atk_mult: 2, release_def_mult: 1, release_hp_mult: 1, release_pp_mult: 2, release_speed_mult: 1 },
        Djinn { id: DjinnId::Undine, atk_bonus: 0, def_bonus: 2, hp_bonus: 0, pp_bonus: 0, speed_bonus: 2, release_atk_mult: 1, release_def_mult: 2, release_hp_mult: 1, release_pp_mult: 1, release_speed_mult: 2 },
        // Mars set
        Djinn { id: DjinnId::Laguna, atk_bonus: 4, def_bonus: 0, hp_bonus: 0, pp_bonus: 0, speed_bonus: 0, release_atk_mult: 2, release_def_mult: 1, release_hp_mult: 1, release_pp_mult: 1, release_speed_mult: 1 },
        Djinn { id: DjinnId::Betsy, atk_bonus: 0, def_bonus: 4, hp_bonus: 0, pp_bonus: 0, speed_bonus: 0, release_atk_mult: 1, release_def_mult: 2, release_hp_mult: 1, release_pp_mult: 1, release_speed_mult: 1 },
        Djinn { id: DjinnId::Grendel, atk_bonus: 0, def_bonus: 2, hp_bonus: 10, pp_bonus: 0, speed_bonus: 0, release_atk_mult: 1, release_def_mult: 2, release_hp_mult: 2, release_pp_mult: 1, release_speed_mult: 1 },
        Djinn { id: DjinnId::Naga, atk_bonus: 2, def_bonus: 2, hp_bonus: 0, pp_bonus: 0, speed_bonus: 0, release_atk_mult: 2, release_def_mult: 2, release_hp_mult: 1, release_pp_mult: 1, release_speed_mult: 1 },
        // Jupiter set
        Djinn { id: DjinnId::Titania, atk_bonus: 0, def_bonus: 0, hp_bonus: 0, pp_bonus: 0, speed_bonus: 4, release_atk_mult: 1, release_def_mult: 1, release_hp_mult: 1, release_pp_mult: 1, release_speed_mult: 2 },
        Djinn { id: DjinnId::Siren, atk_bonus: 0, def_bonus: 0, hp_bonus: 0, pp_bonus: 6, speed_bonus: 0, release_atk_mult: 1, release_def_mult: 1, release_hp_mult: 1, release_pp_mult: 2, release_speed_mult: 1 },
        Djinn { id: DjinnId::Amduscia, atk_bonus: 2, def_bonus: 0, hp_bonus: 0, pp_bonus: 4, speed_bonus: 0, release_atk_mult: 2, release_def_mult: 1, release_hp_mult: 1, release_pp_mult: 2, release_speed_mult: 1 },
        Djinn { id: DjinnId::Marilith, atk_bonus: 2, def_bonus: 2, hp_bonus: 0, pp_bonus: 0, speed_bonus: 2, release_atk_mult: 2, release_def_mult: 2, release_hp_mult: 1, release_pp_mult: 1, release_speed_mult: 2 },
    ]
}

/// 获取可放置在世界地图上的 Djinn 列表（用于收集）
#[must_use]
pub fn world_djinn() -> Vec<(DjinnId, &'static str, f32, f32)> {
    // 返回 (DjinnId, 场景名, tile_x, tile_y)
    vec![
        // Vale 村 — Fafnir (Venus, ATK+)
        (DjinnId::Fafnir, "Vale", 8.0, 10.0),
        // WildForest — Malina (Mercury, PP+)
        (DjinnId::Malina, "WildForest", 12.0, 15.0),
        // Cave — Betsy (Mars, DEF+)
        (DjinnId::Betsy, "Cave", 6.0, 8.0),
    ]
}

/// 计算装备 Djinn 后的套装加成
#[must_use]
pub fn calculate_set_bonus(equipped_djinn: &[&OwnedDjinn]) -> Vec<SetBonus> {
    let mut elements: Vec<(Element, Vec<&OwnedDjinn>)> = Vec::new();
    
    for od in equipped_djinn {
        if od.equipped {
            let elem = od.djinn.element();
            if let Some(entry) = elements.iter_mut().find(|(e, _)| *e == elem) {
                entry.1.push(od);
            } else {
                elements.push((elem, vec![od]));
            }
        }
    }

    let mut bonuses = Vec::new();
    for (_elem, group) in &elements {
        let count = group.len();
        if count >= 4 {
            bonuses.push(SetBonus::SageRing);
        } else if count >= 2 {
            // 根据元素选择对应的套装加成
            bonuses.push(match _elem {
                Element::Venus => SetBonus::LifeRing,
                Element::Mercury => SetBonus::ManaRing,
                Element::Mars => SetBonus::PowerRing,
                Element::Jupiter => SetBonus::SwiftRing,
            });
        }
    }
    bonuses
}

/// 计算装备 Djinn + 套装加成的总属性提升
#[must_use]
pub fn calculate_total_bonuses(equipped_djinn: &[&OwnedDjinn]) -> (u32, u32, u32, u32, u32) {
    let mut atk = 0u32;
    let mut def = 0u32;
    let mut hp = 0u32;
    let mut pp = 0u32;
    let mut speed = 0u32;

    // 累加所有装备 Djinn 的属性
    for od in equipped_djinn {
        if od.equipped {
            atk += od.djinn.atk_bonus;
            def += od.djinn.def_bonus;
            hp += od.djinn.hp_bonus;
            pp += od.djinn.pp_bonus;
            speed += od.djinn.speed_bonus;
        }
    }

    // 应用套装加成
    let bonuses = calculate_set_bonus(equipped_djinn);
    for bonus in &bonuses {
        match bonus {
            SetBonus::LifeRing => { hp = (hp as f32 * 1.2) as u32; }
            SetBonus::ManaRing => { pp = (pp as f32 * 1.2) as u32; }
            SetBonus::PowerRing => { atk = (atk as f32 * 1.2) as u32; }
            SetBonus::GuardRing => { def = (def as f32 * 1.2) as u32; }
            SetBonus::SwiftRing => { speed = (speed as f32 * 1.2) as u32; }
            SetBonus::SageRing => {
                atk = (atk as f32 * 1.15) as u32;
                def = (def as f32 * 1.15) as u32;
                hp = (hp as f32 * 1.15) as u32;
                pp = (pp as f32 * 1.15) as u32;
                speed = (speed as f32 * 1.15) as u32;
            }
            SetBonus::None => {}
        }
    }

    (atk, def, hp, pp, speed)
}

/// 检查某个 Djinn 是否已经被收集
#[must_use]
pub fn is_djinn_collected(collected: &[DjinnId], id: DjinnId) -> bool {
    collected.contains(&id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_djinn_have_correct_elements() {
        let data = all_djinn_data();
        assert_eq!(data.len(), 16);
        
        // Venus set
        assert_eq!(data[0].element(), Element::Venus);
        assert_eq!(data[1].element(), Element::Venus);
        assert_eq!(data[2].element(), Element::Venus);
        assert_eq!(data[3].element(), Element::Venus);
        // Mercury set
        assert_eq!(data[4].element(), Element::Mercury);
        assert_eq!(data[5].element(), Element::Mercury);
        assert_eq!(data[6].element(), Element::Mercury);
        assert_eq!(data[7].element(), Element::Mercury);
        // Mars set
        assert_eq!(data[8].element(), Element::Mars);
        assert_eq!(data[9].element(), Element::Mars);
        assert_eq!(data[10].element(), Element::Mars);
        assert_eq!(data[11].element(), Element::Mars);
        // Jupiter set
        assert_eq!(data[12].element(), Element::Jupiter);
        assert_eq!(data[13].element(), Element::Jupiter);
        assert_eq!(data[14].element(), Element::Jupiter);
        assert_eq!(data[15].element(), Element::Jupiter);
    }

    #[test]
    fn djinn_id_to_str_roundtrip() {
        let data = all_djinn_data();
        for djinn in &data {
            assert_eq!(djinn.id.as_str(), djinn.name());
        }
    }

    #[test]
    fn djinn_ids_are_unique() {
        let data = all_djinn_data();
        let mut ids = data.iter().map(|d| d.id).collect::<Vec<_>>();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), 16);
    }

    #[test]
    fn release_multipliers_exist() {
        let data = all_djinn_data();
        for djinn in &data {
            assert!(djinn.release_atk_mult >= 1);
            assert!(djinn.release_def_mult >= 1);
            assert!(djinn.release_hp_mult >= 1);
            assert!(djinn.release_pp_mult >= 1);
            assert!(djinn.release_speed_mult >= 1);
            // At least one multiplier > 1
            assert!(djinn.release_atk_mult + djinn.release_def_mult 
                  + djinn.release_hp_mult + djinn.release_pp_mult 
                  + djinn.release_speed_mult > 5);
        }
    }

    #[test]
    fn set_bonus_name_nonempty() {
        assert!(!SetBonus::None.name().is_empty());
        assert!(!SetBonus::LifeRing.name().is_empty());
        assert!(!SetBonus::ManaRing.name().is_empty());
        assert!(!SetBonus::PowerRing.name().is_empty());
        assert!(!SetBonus::GuardRing.name().is_empty());
        assert!(!SetBonus::SwiftRing.name().is_empty());
        assert!(!SetBonus::SageRing.name().is_empty());
    }

    #[test]
    fn class_from_elements_adept_when_empty() {
        let classes = [Class::from_elements(&[])];
        assert_eq!(classes[0], Class::Adept);
    }

    #[test]
    fn class_from_elements_single_element_is_adept() {
        let cls = Class::from_elements(&[Element::Venus]);
        assert_eq!(cls, Class::Adept);
    }

    #[test]
    fn class_from_elements_two_elements() {
        assert_eq!(Class::from_elements(&[Element::Venus, Element::Mercury]), 
                   Class::Psychokineticist);
        assert_eq!(Class::from_elements(&[Element::Venus, Element::Mars]), 
                   Class::Climber);
        assert_eq!(Class::from_elements(&[Element::Venus, Element::Jupiter]), 
                   Class::Dreamseer);
        assert_eq!(Class::from_elements(&[Element::Mercury, Element::Mars]), 
                   Class::Warrior);
        assert_eq!(Class::from_elements(&[Element::Jupiter, Element::Mars]), 
                   Class::Dawnmist);
        assert_eq!(Class::from_elements(&[Element::Jupiter, Element::Mercury]), 
                   Class::Shadowreader);
    }

    #[test]
    fn class_from_elements_three_elements() {
        assert_eq!(Class::from_elements(&[Element::Venus, Element::Mercury, Element::Mars]), 
                   Class::_TRANSMOGGRIFIER);
    }

    #[test]
    fn class_from_elements_all_elements() {
        assert_eq!(Class::from_elements(&[Element::Venus, Element::Mercury, Element::Mars, Element::Jupiter]), 
                   Class::_TRANSMOGGRIFIER);
    }

    #[test]
    fn adept_unlocks_four_psynergies() {
        let psy = Class::Adept.unlocked_psynergies();
        assert_eq!(psy.len(), 4);
    }

    #[test]
    fn transmoggrifier_unlocks_all_psynergies() {
        let psy = Class::_TRANSMOGGRIFIER.unlocked_psynergies();
        assert_eq!(psy.len(), 7);
    }

    #[test]
    fn calculate_set_bonus_empty() {
        let bonuses = calculate_set_bonus(&[]);
        assert!(bonuses.is_empty());
    }

    #[test]
    fn calculate_total_bonuses_empty() {
        let (atk, def, hp, pp, spd) = calculate_total_bonuses(&[]);
        assert_eq!(atk, 0);
        assert_eq!(def, 0);
        assert_eq!(hp, 0);
        assert_eq!(pp, 0);
        assert_eq!(spd, 0);
    }

    #[test]
    fn calculate_total_bonuses_with_equipped() {
        let data = all_djinn_data();
        let od1 = OwnedDjinn::new(data[0].clone()); // Fafnir: +3 ATK
        let od2 = OwnedDjinn::new(data[1].clone()); // Belian: +3 DEF
        
        let mut equipped = [od1, od2];
        equipped[0].equipped = true;
        equipped[0].equipped_to = Some(0);
        equipped[1].equipped = true;
        equipped[1].equipped_to = Some(0);
        
        let refs: Vec<&OwnedDjinn> = equipped.iter().collect();
        let (atk, def, _, _, _) = calculate_total_bonuses(&refs);
        assert_eq!(atk, 3);
        assert_eq!(def, 3);
    }

    #[test]
    fn world_djinn_returns_three_locations() {
        let djinn_locs = world_djinn();
        assert_eq!(djinn_locs.len(), 3);
    }

    #[test]
    fn djinn_id_order_is_consistent() {
        let ids = [
            DjinnId::Fafnir, DjinnId::Belian, DjinnId::Alupam, DjinnId::Gnome,
            DjinnId::Vermin, DjinnId::Malina, DjinnId::Kelpie, DjinnId::Undine,
            DjinnId::Laguna, DjinnId::Betsy, DjinnId::Grendel, DjinnId::Naga,
            DjinnId::Titania, DjinnId::Siren, DjinnId::Amduscia, DjinnId::Marilith,
        ];
        assert_eq!(ids.len(), 16);
        for (i, id) in ids.iter().enumerate() {
            assert_eq!(id.id(), (i as u32) + 1);
        }
    }

    #[test]
    fn is_djinn_collected_works() {
        let collected = vec![DjinnId::Fafnir, DjinnId::Belian];
        assert!(is_djinn_collected(&collected, DjinnId::Fafnir));
        assert!(is_djinn_collected(&collected, DjinnId::Belian));
        assert!(!is_djinn_collected(&collected, DjinnId::Laguna));
    }

    #[test]
    fn set_bonus_element_mapping() {
        assert_eq!(SetBonus::LifeRing.element(), Some(Element::Venus));
        assert_eq!(SetBonus::ManaRing.element(), Some(Element::Mercury));
        assert_eq!(SetBonus::PowerRing.element(), Some(Element::Mars));
        assert_eq!(SetBonus::SwiftRing.element(), Some(Element::Jupiter));
        assert_eq!(SetBonus::SageRing.element(), Some(Element::Venus));
        assert_eq!(SetBonus::None.element(), None);
    }
}
