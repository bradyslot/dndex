#![allow(non_snake_case, non_camel_case_types, clippy::similar_names)]
use yew::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct Open5eEndpoint<T: PartialEq> {
    pub count: i32,
    // don't care how many pages of results there are
    // pub next: Option<i32>,
    // pub previous: Option<i32>,
    pub results: Vec<T>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct Open5eFeat {
    pub slug: String,
    pub name: String,
    pub desc: String,
    pub prerequisite: Option<String>,
    pub effects_desc: Vec<String>,
    #[serde(flatten)]
    pub document: Open5eDocument,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct Open5eArchetype {
    pub name: String,
    pub slug: String,
    pub desc: String,
    #[serde(flatten)]
    pub document: Open5eDocument,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct Open5eASI {
    pub attributes: Vec<String>,
    pub value: u8,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct Open5eSpeed {
    pub walk: i32
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct Open5eSubrace {
    pub name: String,
    pub slug: String,
    pub desc: String,
    pub asi: Vec<Open5eASI>,
    pub traits: String,
    pub asi_desc: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct Open5eRace {
    pub name: String,
    pub slug: String,
    pub desc: String,
    pub asi_desc: String,
    pub asi: Vec<Open5eASI>,
    pub age: String,
    pub alignment: String,
    pub size: String,
    pub speed: Open5eSpeed,
    pub speed_desc: String,
    pub languages: String,
    pub vision: String,
    pub traits: String,
    pub subraces: Vec<Open5eSubrace>,
    #[serde(flatten)]
    pub document: Open5eDocument,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq, Debug)]
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
