#![allow(non_snake_case, non_camel_case_types, clippy::similar_names)]
use std::collections::HashMap;
use crate::api::open5e::*;
use crate::models::open5e::*;
use crate::models::equipment::*;
use wasm_bindgen_futures::spawn_local;

#[derive(PartialEq, Debug)]
pub struct SRDClass<T: PartialEq> {
    pub hit_points: SRDClassHitPoints,
    pub proficiencies: SRDClassProficiencies,
    pub starting_equipment: SRDClassStartingEquipment,
    pub spellcasting: Option<SRDClassSpellcasting>,
    pub levels: SRDClassLevels<T>,
    pub features: HashMap<&'static str, SRDClassFeature>,
}

#[derive(PartialEq, Debug)]
pub struct SRDClassHitPoints {
    pub hit_dice: i32,
    pub static_option: i32,
    pub desc: &'static str,
}

#[derive(PartialEq, Debug)]
pub struct SRDClassProficiencies {
    pub armor: Vec<Equipment>,
    pub weapons: Vec<Equipment>,
    pub tools: Vec<Vec<Equipment>>,
    pub saving_throws: Vec<&'static str>,
    pub skills: SRDClassProficientSkills,
    pub desc: &'static str,
}

#[derive(PartialEq, Debug)]
pub struct SRDClassProficientSkills {
    pub choices: i32,
    pub options: Vec<&'static str>,
}

#[derive(PartialEq, Debug)]
pub struct SRDClassStartingEquipment {
    pub choice_1: Vec<Vec<Equipment>>,
    pub choice_2: Vec<Vec<Equipment>>,
    pub choice_3: Vec<Vec<Equipment>>,
    pub choice_4: Vec<Vec<Equipment>>,
    pub defaults: Vec<Equipment>,
    pub desc: &'static str,
}

#[derive(PartialEq, Debug)]
pub struct SRDClassSpellcasting {
    pub name: &'static str,
    pub ability: &'static str,
    pub desc: &'static str,
    pub at_level: i32,
}

#[derive(PartialEq, Debug)]
pub struct SRDClassLevelFeature {
    pub key: &'static str,
    pub name: Option<&'static str>, // if present, overrides the name of the feature
}

#[derive(PartialEq, Debug)]
pub struct SRDBarbarianAttributes {
    pub level: i32,
    pub features: Vec<SRDClassLevelFeature>,
    pub rages: i32,
    pub rage_damage: i32,
}

#[derive(PartialEq, Debug)]
pub struct SRDBardAttributes {
    pub level: i32,
    pub features: Vec<SRDClassLevelFeature>,
    pub cantrips_known: i32,
    pub spells_known: i32,
    pub spell_slots: [i32; 9],
}

#[derive(PartialEq, Debug)]
pub struct SRDClericAttributes {
    pub level: i32,
    pub features: Vec<SRDClassLevelFeature>,
    pub cantrips_known: i32,
    pub spell_slots: [i32; 9],
}

#[derive(PartialEq, Debug)]
pub struct SRDDruidAttributes {
    pub level: i32,
    pub features: Vec<SRDClassLevelFeature>,
    pub cantrips_known: i32,
    pub spell_slots: [i32; 9],
}

#[derive(PartialEq, Debug)]
pub struct SRDFighterAttributes {
    pub level: i32,
    pub features: Vec<SRDClassLevelFeature>,
}

#[derive(PartialEq, Debug)]
pub struct SRDMonkAttributes {
    pub level: i32,
    pub features: Vec<SRDClassLevelFeature>,
    pub martial_arts: i32,
    pub ki_points: i32,
    pub unarmored_movement: i32,
}

#[derive(PartialEq, Debug)]
pub struct SRDPaladinAttributes {
    pub level: i32,
    pub features: Vec<SRDClassLevelFeature>,
    pub spells_known: i32,
    pub spell_slots: [i32; 9],
}

#[derive(PartialEq, Debug)]
pub struct SRDRangerAttributes {
    pub level: i32,
    pub features: Vec<SRDClassLevelFeature>,
    pub spells_known: i32,
    pub spell_slots: [i32; 9],
}

#[derive(PartialEq, Debug)]
pub struct SRDRogueAttributes {
    pub level: i32,
    pub features: Vec<SRDClassLevelFeature>,
    pub sneak_attack: i32,
}

#[derive(PartialEq, Debug)]
pub struct SRDSorcererAttributes {
    pub level: i32,
    pub features: Vec<SRDClassLevelFeature>,
    pub sorcery_points: i32,
    pub cantrips_known: i32,
    pub spells_known: i32,
    pub spell_slots: [i32; 9],
}

#[derive(PartialEq, Debug)]
pub struct SRDWarlockAttributes {
    pub level: i32,
    pub features: Vec<SRDClassLevelFeature>,
    pub cantrips_known: i32,
    pub spells_known: i32,
    pub available_spell_slots: i32,
    pub slot_level: i32,
    pub invocations_known: i32,
}

#[derive(PartialEq, Debug)]
pub struct SRDWizardAttributes {
    pub level: i32,
    pub features: Vec<SRDClassLevelFeature>,
    pub cantrips_known: i32,
    pub spell_slots: [i32; 9],
}

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
pub struct SRDClassFeature {
    pub name: &'static str,
    pub desc: &'static str,
}
