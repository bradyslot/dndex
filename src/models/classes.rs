#![allow(non_snake_case, non_camel_case_types, clippy::similar_names)]
use yew::prelude::*;
use serde::Deserialize;
// use std::collections::HashMap;

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct DnDClass {
    class_hit_points: ClassHitPoints,
    class_proficiencies: ClassProficiencies,
    class_equipment: ClassEquipment,
    class_spellcasting: ClassSpellcasting,
    class_levels: ClassLevels,
    class_features: ClassFeatures,
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct ClassHitPoints {
    hit_dice: u8,
    static_option: u8,
    desc: String,
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct ClassProficiencies {
    armor: Vec<ClassEquipmentItem>,
    weapons: Vec<ClassEquipmentItem>,
    tools: Vec<Vec<ClassEquipmentItem>>,
    saving_throws: Vec<String>,
    skills: ClassSkillChoices,
    desc: String,
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct ClassSkillChoices {
    choices: u8,
    options: Vec<String>,
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct ClassEquipmentItem {
    name: Option<String>,
    source: Option<String>,
    location: Option<String>,
    key: Option<String>,
    category: Option<String>,
    qty: Option<u8>,
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct ClassEquipment {
    choice_1: Vec<Vec<ClassEquipmentItem>>,
    choice_2: Vec<Vec<ClassEquipmentItem>>,
    choice_3: Vec<Vec<ClassEquipmentItem>>,
    choice_4: Vec<Vec<ClassEquipmentItem>>,
    defaults: Vec<ClassEquipmentItem>,
    desc: String,
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct ClassSpellcasting {
    ability: String,
    desc: String,
}

#[derive(Deserialize, Clone, PartialEq)]
pub enum ClassLevels {
    Barbarian(BarbarianLevel),
    Bard(BardLevel),
    Cleric(ClericLevel),
    Druid(DruidLevel),
    Fighter(FighterLevel),
    Monk(MonkLevel),
    Paladin(PaladinLevel),
    Ranger(RangerLevel),
    Rogue(RogueLevel),
    Sorcerer(SorcererLevel),
    Warlock(WarlockLevel),
    Wizard(WizardLevel),
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct ClassLevelFeature {
    key: String,
    name: Option<String>, // if present, overrides the name of the feature
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct BarbarianLevel {
    features: Vec<ClassLevelFeature>,
    rages: u8,
    rage_damage: u8,
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct BardLevel {
    features: Vec<ClassLevelFeature>,
    cantrips_known: u8,
    spells_known: u8,
    spell_slots: Vec<u8>,
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct ClericLevel {
    features: Vec<ClassLevelFeature>,
    cantrips_known: u8,
    spell_slots: Vec<u8>,
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct DruidLevel {
    features: Vec<ClassLevelFeature>,
    cantrips_known: u8,
    spell_slots: Vec<u8>,
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct FighterLevel {
    features: Vec<ClassLevelFeature>,
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct MonkLevel {
    features: Vec<ClassLevelFeature>,
    martial_arts: u8,
    ki_points: u8,
    unarmored_movement: u8,
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct PaladinLevel {
    features: Vec<ClassLevelFeature>,
    spells_known: u8,
    spell_slots: Vec<u8>,
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct RangerLevel {
    features: Vec<ClassLevelFeature>,
    spells_known: u8,
    spell_slots: Vec<u8>,
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct RogueLevel {
    features: Vec<ClassLevelFeature>,
    sneak_attack: u8,
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct SorcererLevel {
    features: Vec<ClassLevelFeature>,
    sorcery_points: u8,
    cantrips_known: u8,
    spells_known: u8,
    spell_slots: Vec<u8>,
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct WarlockLevel {
    features: Vec<ClassLevelFeature>,
    cantrips_known: u8,
    spells_known: u8,
    spell_slots: u8,
    slot_level: u8,
    invocations_known: u8,
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct WizardLevel {
    features: Vec<ClassLevelFeature>,
    cantrips_known: u8,
    spell_slots: Vec<u8>,
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct ClassFeatures {
    name: String,
    desc: String,
}
