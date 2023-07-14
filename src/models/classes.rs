#![allow(non_snake_case, non_camel_case_types, clippy::similar_names)]
use yew::prelude::Properties;
use std::collections::HashMap;

#[derive(Properties, PartialEq)]
pub struct SRDClass<T: PartialEq> {
    pub hit_points: SRDClassHitPoints,
    pub proficiencies: SRDClassProficiencies,
    pub equipment: SRDClassStartingEquipment,
    pub spellcasting: Option<SRDClassSpellcasting>,
    pub levels: SRDClassLevels<T>,
    pub features: HashMap<&'static str, SRDClassFeatures>,
}

#[derive(Properties, PartialEq)]
pub struct SRDClassHitPoints {
    pub hit_dice: i32,
    pub static_option: i32,
    pub desc: &'static str,
}

#[derive(Properties, PartialEq)]
pub struct SRDClassProficiencies {
    pub armor: Vec<SRDClassEquipmentItem>,
    pub weapons: Vec<SRDClassEquipmentItem>,
    pub tools: Vec<Vec<SRDClassEquipmentItem>>,
    pub saving_throws: Vec<&'static str>,
    pub skills: SRDClassSkills,
    pub desc: &'static str,
}

#[derive(Properties, PartialEq)]
pub struct SRDClassSkills {
    pub choices: i32,
    pub options: Vec<&'static str>,
}

#[derive(Properties, PartialEq)]
pub struct SRDClassEquipmentItem {
    pub name: Option<&'static str>,
    pub source: Option<&'static str>,
    pub location: Option<&'static str>,
    pub key: Option<&'static str>,
    pub category: Option<&'static str>,
    pub qty: Option<u8>,
}

#[derive(Properties, PartialEq)]
pub struct SRDClassStartingEquipment {
    pub choice_1: Option<Vec<Vec<SRDClassEquipmentItem>>>,
    pub choice_2: Option<Vec<Vec<SRDClassEquipmentItem>>>,
    pub choice_3: Option<Vec<Vec<SRDClassEquipmentItem>>>,
    pub choice_4: Option<Vec<Vec<SRDClassEquipmentItem>>>,
    pub defaults: Vec<SRDClassEquipmentItem>,
    pub desc: &'static str,
}

#[derive(Properties, PartialEq)]
pub struct SRDClassSpellcasting {
    pub ability: &'static str,
    pub desc: &'static str,
}

#[derive(Properties, PartialEq)]
pub struct SRDClassLevelFeature {
    pub key: &'static str,
    pub name: Option<&'static str>, // if present, overrides the name of the feature
}

#[derive(Properties, PartialEq)]
pub struct SRDBarbarianAttributes {
    pub level: i32,
    pub features: Vec<SRDClassLevelFeature>,
    pub rages: i32,
    pub rage_damage: i32,
}

#[derive(Properties, PartialEq)]
pub struct SRDBardAttributes {
    pub level: i32,
    pub features: Vec<SRDClassLevelFeature>,
    pub cantrips_known: i32,
    pub spells_known: i32,
    pub spell_slots: [i32; 9],
}

#[derive(Properties, PartialEq)]
pub struct SRDClassLevels<T: PartialEq> {
    pub level_1: T,
    pub level_2: T,
    pub level_3: T,
    pub level_4: T,
    pub level_5: T,
    pub level_6: T,
    pub level_7: T,
    pub level_8: T,
    pub level_9: T,
    pub level_10: T,
    pub level_11: T,
    pub level_12: T,
    pub level_13: T,
    pub level_14: T,
    pub level_15: T,
    pub level_16: T,
    pub level_17: T,
    pub level_18: T,
    pub level_19: T,
    pub level_20: T,
}

#[derive(Properties, PartialEq)]
pub struct SRDClassFeatures {
    pub name: &'static str,
    pub desc: &'static str,
}
