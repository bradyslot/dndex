#![allow(non_snake_case, non_camel_case_types, clippy::similar_names)]
use yew::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct DnDClass {
    class_hit_points: ClassHitPoints,
    class_proficiencies: ClassProficiencies,
    class_equipment: ClassEquipment,
    class_spellcasting: ClassSpellcasting,
    class_levels: Vec<ClassLevelAttributes>,
    class_features: HashMap<String, ClassFeatures>,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct ClassHitPoints {
    hit_dice: u8,
    static_option: u8,
    desc: String,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct ClassProficiencies {
    armor: Vec<ClassEquipmentItem>,
    weapons: Vec<ClassEquipmentItem>,
    tools: Vec<Vec<ClassEquipmentItem>>,
    saving_throws: Vec<String>,
    skills: ClassSkillChoices,
    desc: String,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct ClassSkillChoices {
    choices: u8,
    options: Vec<String>,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct ClassEquipmentItem {
    name: Option<String>,
    source: Option<String>,
    location: Option<String>,
    key: Option<String>,
    category: Option<String>,
    qty: Option<u8>,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct ClassEquipment {
    choice_1: Vec<Vec<ClassEquipmentItem>>,
    choice_2: Vec<Vec<ClassEquipmentItem>>,
    choice_3: Vec<Vec<ClassEquipmentItem>>,
    choice_4: Vec<Vec<ClassEquipmentItem>>,
    defaults: Vec<ClassEquipmentItem>,
    desc: String,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct ClassSpellcasting {
    ability: Option<String>,
    desc: Option<String>,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct ClassLevelFeature {
    key: String,
    name: Option<String>, // if present, overrides the name of the feature
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct ClassLevelAttributes {
    level: u8,
    features: Vec<ClassLevelFeature>,
    rages: Option<u8>,
    rage_damage: Option<u8>,
    cantrips_known: Option<u8>,
    spells_known: Option<u8>,
    spell_slots: Option<Vec<u8>>,
    martial_arts: Option<u8>,
    ki_points: Option<u8>,
    unarmored_movement: Option<u8>,
    sneak_attack: Option<u8>,
    available_spell_slots: Option<u8>,
    slot_level: Option<u8>,
    invocations_known: Option<u8>,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct ClassFeatures {
    name: String,
    desc: String,
}
