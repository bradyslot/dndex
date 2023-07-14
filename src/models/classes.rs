#![allow(non_snake_case, non_camel_case_types, clippy::similar_names)]
use yew::prelude::Properties;
use serde::Deserialize;
use std::collections::HashMap;

// #[derive(Clone, Properties, PartialEq, Debug)]
// pub struct SRDClasses {
//     pub barbarian: SRDClass,
//     pub bard: SRDClass,
//     pub cleric: SRDClass,
//     pub druid: SRDClass,
//     pub fighter: SRDClass,
//     pub monk: SRDClass,
//     pub paladin: SRDClass,
//     pub ranger: SRDClass,
//     pub rogue: SRDClass,
//     pub sorcerer: SRDClass,
//     pub warlock: SRDClass,
//     pub wizard: SRDClass,
// }

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct SRDClass<T: PartialEq> {
    pub hit_points: SRDClassHitPoints,
    pub proficiencies: SRDClassProficiencies,
    pub equipment: SRDClassStartingEquipment,
    pub spellcasting: Option<SRDClassSpellcasting>,
    pub levels: SRDClassLevels<T>,
    pub features: HashMap<String, SRDClassFeatures>,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct SRDClassHitPoints {
    pub hit_dice: i32,
    pub static_option: i32,
    pub desc: String,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct SRDClassProficiencies {
    pub armor: Vec<SRDClassEquipmentItem>,
    pub weapons: Vec<SRDClassEquipmentItem>,
    pub tools: Vec<Vec<SRDClassEquipmentItem>>,
    pub saving_throws: Vec<String>,
    pub skills: SRDClassSkills,
    pub desc: String,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct SRDClassSkills {
    pub choices: i32,
    pub options: Vec<String>,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct SRDClassEquipmentItem {
    pub name: Option<String>,
    pub source: Option<String>,
    pub location: Option<String>,
    pub key: Option<String>,
    pub category: Option<String>,
    pub qty: Option<u8>,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct SRDClassStartingEquipment {
    pub choice_1: Vec<Vec<SRDClassEquipmentItem>>,
    pub choice_2: Vec<Vec<SRDClassEquipmentItem>>,
    pub choice_3: Vec<Vec<SRDClassEquipmentItem>>,
    pub choice_4: Vec<Vec<SRDClassEquipmentItem>>,
    pub defaults: Vec<SRDClassEquipmentItem>,
    pub desc: String,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct SRDClassSpellcasting {
    pub ability: String,
    pub desc: String,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct SRDClassLevelFeature {
    pub key: String,
    pub name: Option<String>, // if present, overrides the name of the feature
}

// #[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
// pub struct ClassLevelAttributes {
//     pub level: i32,
//     pub features: Vec<ClassLevelFeature>,
//     pub rages: Option<i32>,
//     pub rage_damage: Option<i32>,
//     pub cantrips_known: Option<i32>,
//     pub spells_known: Option<i32>,
//     pub spell_slots: Option<Vec<i32>>,
//     pub martial_arts: Option<i32>,
//     pub ki_points: Option<i32>,
//     pub unarmored_movement: Option<i32>,
//     pub sneak_attack: Option<i32>,
//     pub available_spell_slots: Option<i32>,
//     pub slot_level: Option<i32>,
//     pub invocations_known: Option<i32>,
// }

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct SRDBarbarianAttributes {
    pub level: i32,
    pub features: Vec<SRDClassLevelFeature>,
    pub rages: i32,
    pub rage_damage: i32,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct SRDBardAttributes {
    pub level: i32,
    pub features: Vec<SRDClassLevelFeature>,
    pub cantrips_known: i32,
    pub spells_known: i32,
    pub spell_slots: [i32; 9],
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
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

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct SRDClassFeatures {
    pub name: String,
    pub desc: String,
}
