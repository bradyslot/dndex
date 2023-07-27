#![allow(non_snake_case, non_camel_case_types, clippy::similar_names)]
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Class {
    pub hit_points: ClassHitPoints,
    pub proficiencies: ClassProficiencies,
    pub starting_equipment: ClassStartingEquipment,
    pub spellcasting: Option<ClassSpellcasting>,
    pub levels: Vec<ClassLevel>,
    pub features: Vec<ClassFeature>,
}

#[derive(Deserialize, Debug)]
pub struct ClassHitPoints {
    pub hit_dice: i32,
    pub static_option: i32,
    pub desc: String,
}

#[derive(Deserialize, Debug)]
pub struct Equipment {
    pub resource: String,
    pub key: Option<String>,
    pub category: Option<String>,
    pub qty: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct ClassProficiencies {
    pub armor: Vec<Equipment>,
    pub weapons: Vec<Equipment>,
    pub tools: Vec<Equipment>,
    pub saving_throws: Vec<String>,
    pub skills: ProficientSkills,
}

#[derive(Deserialize, Debug)]
pub struct ProficientSkills {
    pub choices: i32,
    pub options: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct ClassStartingEquipment {
    pub choice_1: Vec<Vec<Equipment>>,
    pub choice_2: Vec<Vec<Equipment>>,
    pub choice_3: Vec<Vec<Equipment>>,
    pub choice_4: Vec<Vec<Equipment>>,
    pub defaults: Vec<Equipment>,
}

#[derive(Deserialize, Debug)]
pub struct ClassSpellcasting {
    pub name: String,
    pub ability: String,
    pub at_level: i32,
    pub desc: String,
}

#[derive(Deserialize, Debug)]
pub struct ClassLevel {
    pub level: i32,
    pub features: Vec<LevelFeature>,
    pub rages: Option<i32>,
    pub rage_damage: Option<i32>,
    pub cantrips_known: Option<i32>,
    pub spells_known: Option<i32>,
    pub spell_slots: Option<Vec<i32>>,
    pub martial_arts: Option<i32>,
    pub ki_points: Option<i32>,
    pub unarmored_movement: Option<i32>,
    pub sneak_attack: Option<i32>,
    pub sorcery_points: Option<i32>,
    pub available_spell_slots: Option<i32>,
    pub invocations_known: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct LevelFeature {
    pub key: String,
    pub name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ClassFeature {
    pub key: String,
    pub name: String,
    pub desc: String,
}
