#![allow(non_snake_case, non_camel_case_types, clippy::similar_names)]
use std::collections::HashMap;
use yew::prelude::*;
use serde::{Deserialize, Deserializer};
use serde_with::{serde_as, DefaultOnError};

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct Open5eResults<T: PartialEq> {
    pub results: Vec<T>,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug, Default)]
pub struct Open5eDocument {
    #[serde(rename = "document__slug")]
    pub slug: String,
    #[serde(rename = "document__title")]
    pub title: String,
    #[serde(rename = "document__license_url")]
    pub license_url: String,
    #[serde(rename = "document__url")]
    pub url: String,
}

#[serde_as]
#[derive(Deserialize, Clone, Properties, PartialEq, Debug, Default)]
pub struct Open5eMonster {
    pub slug: String,
    pub desc: String,
    pub name: String,
    pub size: String,
    #[serde(rename = "type")] // "type" is a reserved keyword in Rust
    pub mtype: String,
    pub subtype: String,
    pub group: Option<String>,
    pub alignment: String,
    pub armor_class: i32,
    pub armor_desc: Option<String>,
    pub hit_points: i32,
    pub hit_dice: String,
    pub speed: HashMap<String, i32>,
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
    pub strength_save: Option<i32>,
    pub dexterity_save: Option<i32>,
    pub constitution_save: Option<i32>,
    pub intelligence_save: Option<i32>,
    pub wisdom_save: Option<i32>,
    pub charisma_save: Option<i32>,
    pub perception: Option<i32>,
    pub skills: HashMap<String, i32>,
    pub damage_vulnerabilities: String,
    pub damage_resistances: String,
    pub damage_immunities: String,
    pub condition_immunities: String,
    pub senses: String,
    pub languages: String,
    pub challenge_rating: String,
    pub cr: f32,
    #[serde_as(as = "DefaultOnError")]
    pub actions: Vec<Open5eMonsterAction>,
    #[serde_as(as = "DefaultOnError")]
    pub reactions: Vec<Open5eMonsterAction>,
    pub legendary_desc: String,
    #[serde_as(as = "DefaultOnError")]
    pub legendary_actions: Vec<Open5eMonsterAction>,
    #[serde_as(as = "DefaultOnError")]
    pub special_abilities: Vec<Open5eMonsterSpecialAbility>,
    pub spell_list: Vec<String>,
    pub page_no: i32,
    pub environments: Vec<String>,
    pub img_main: Option<String>,
    #[serde(flatten)]
    pub document: Open5eDocument,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug, Default)]
pub struct Open5eMonsterSpecialAbility {
    name: String,
    desc: String,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug, Default)]
pub struct Open5eMonsterAction {
    name: String,
    desc: String,
    attack_bonus: Option<i32>,
    damage_dice: Option<String>,
    damage_bonus: Option<i32>,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug, Default)]
pub struct Open5eArmor {
    name: String,
    slug: String,
    category: String,
    #[serde(flatten)]
    document: Open5eDocument,
    base_ac: i32,
    plus_dex_mod: bool,
    plus_con_mod: bool,
    plus_wis_mod: bool,
    plus_flat_mod: i32,
    plus_max: i32,
    ac_string: String,
    strength_requirement: Option<i32>,
    cost: String,
    weight: String,
    stealth_disadvantage: bool,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug, Default)]
pub struct Open5eWeapon {
    pub name: String,
    pub slug: String,
    pub category: String,
    #[serde(flatten)]
    pub document: Open5eDocument,
    pub cost: String,
    pub damage_dice: String,
    pub damage_type: String,
    pub weight: String,
    pub properties: Vec<String>
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug, Default)]
pub struct Open5eBackground {
    pub name: String,
    pub desc: String,
    pub slug: String,
    pub skill_proficiencies: String,
    pub tool_proficiencies: Option<String>,
    pub languages: String,
    pub equipment: String,
    pub feature: String,
    pub feature_desc: String,
    pub suggested_characteristics: String,
    #[serde(flatten)]
    pub document: Open5eDocument,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug, Default)]
pub struct Open5eFeat {
    pub slug: String,
    pub name: String,
    pub desc: String,
    pub prerequisite: Option<String>,
    pub effects_desc: Vec<String>,
    #[serde(flatten)]
    pub document: Open5eDocument,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug, Default)]
pub struct Open5eClass {
    pub name: String,
    pub slug: String,
    pub desc: String,
    pub hit_dice: String,
    pub hp_at_1st_level: String,
    pub hp_at_higher_levels: String,
    pub prof_armor: String,
    pub prof_weapons: String,
    pub prof_tools: String,
    pub prof_saving_throws: String,
    pub prof_skills: String,
    pub equipment: String,
    pub table: String,
    pub spellcasting_ability: String,
    pub subtypes_name: String,
    pub archetypes: Vec<Open5eArchetype>,
    #[serde(flatten)]
    pub document: Open5eDocument,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct Open5eArchetype {
    pub name: String,
    pub slug: String,
    pub desc: String,
    #[serde(flatten)]
    pub document: Open5eDocument,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug, Default)]
pub struct Open5eASI {
    pub attributes: Vec<String>,
    pub value: u8,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug, Default)]
pub struct Open5eSubrace {
    pub name: String,
    pub slug: String,
    pub desc: String,
    pub asi: Vec<Open5eASI>,
    pub traits: String,
    pub asi_desc: String,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug, Default)]
pub struct Open5eRace {
    pub name: String,
    pub slug: String,
    pub desc: String,
    pub asi_desc: String,
    pub asi: Vec<Open5eASI>,
    pub age: String,
    pub alignment: String,
    pub size: String,
    pub speed: HashMap<String, i32>,
    pub speed_desc: String,
    pub languages: String,
    pub vision: String,
    pub traits: String,
    pub subraces: Vec<Open5eSubrace>,
    #[serde(flatten)]
    pub document: Open5eDocument,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug, Default)]
pub struct Open5eSpell {
    pub slug: String,
    pub name: String,
    pub desc: String,
    pub higher_level: String,
    pub page: String,
    pub range: String,
    pub target_range_sort: u8,
    pub components: String,
    pub requires_verbal_components: bool,
    pub requires_somatic_components: bool,
    pub requires_material_components: bool,
    pub material: String,
    pub can_be_cast_as_ritual: bool,
    pub ritual: String,
    pub duration: String,
    pub concentration: String,
    pub requires_concentration: bool,
    pub casting_time: String,
    pub level: String,
    pub level_int: u8,
    pub spell_level: u8,
    pub school: String,
    pub dnd_class: String,
    pub spell_lists: Vec<String>,
    pub archetype: String,
    pub circles: String,
    #[serde(flatten)]
    pub document: Open5eDocument,
}
