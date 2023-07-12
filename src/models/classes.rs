#![allow(non_snake_case, non_camel_case_types, clippy::similar_names)]
use yew::prelude::Properties;
use serde::Deserialize;
use std::collections::HashMap;


#[derive(Clone, Properties, PartialEq, Debug)]
pub struct SRDClasses {
    pub barbarian: SRDClass,
    pub bard: SRDClass,
    pub cleric: SRDClass,
    pub druid: SRDClass,
    pub fighter: SRDClass,
    pub monk: SRDClass,
    pub paladin: SRDClass,
    pub ranger: SRDClass,
    pub rogue: SRDClass,
    pub sorcerer: SRDClass,
    pub warlock: SRDClass,
    pub wizard: SRDClass,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct SRDClass {
    pub hit_points: ClassHitPoints,
    pub proficiencies: ClassProficiencies,
    pub equipment: ClassEquipment,
    pub spellcasting: ClassSpellcasting,
    pub levels: Vec<ClassLevelAttributes>,
    pub features: HashMap<String, ClassFeatures>,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct ClassHitPoints {
    pub hit_dice: u8,
    pub static_option: u8,
    pub desc: String,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct ClassProficiencies {
    pub armor: Vec<ClassEquipmentItem>,
    pub weapons: Vec<ClassEquipmentItem>,
    pub tools: Vec<Vec<ClassEquipmentItem>>,
    pub saving_throws: Vec<String>,
    pub skills: ClassSkillChoices,
    pub desc: String,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct ClassSkillChoices {
    pub choices: u8,
    pub options: Vec<String>,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct ClassEquipmentItem {
    pub name: Option<String>,
    pub source: Option<String>,
    pub location: Option<String>,
    pub key: Option<String>,
    pub category: Option<String>,
    pub qty: Option<u8>,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct ClassEquipment {
    pub choice_1: Vec<Vec<ClassEquipmentItem>>,
    pub choice_2: Vec<Vec<ClassEquipmentItem>>,
    pub choice_3: Vec<Vec<ClassEquipmentItem>>,
    pub choice_4: Vec<Vec<ClassEquipmentItem>>,
    pub defaults: Vec<ClassEquipmentItem>,
    pub desc: String,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct ClassSpellcasting {
    pub ability: Option<String>,
    pub desc: Option<String>,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct ClassLevelFeature {
    pub key: String,
    pub name: Option<String>, // if present, overrides the name of the feature
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct ClassLevelAttributes {
    pub level: u8,
    pub features: Vec<ClassLevelFeature>,
    pub rages: Option<u8>,
    pub rage_damage: Option<u8>,
    pub cantrips_known: Option<u8>,
    pub spells_known: Option<u8>,
    pub spell_slots: Option<Vec<u8>>,
    pub martial_arts: Option<u8>,
    pub ki_points: Option<u8>,
    pub unarmored_movement: Option<u8>,
    pub sneak_attack: Option<u8>,
    pub available_spell_slots: Option<u8>,
    pub slot_level: Option<u8>,
    pub invocations_known: Option<u8>,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct ClassFeatures {
    pub name: String,
    pub desc: String,
}
