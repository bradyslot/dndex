#![allow(non_snake_case, non_camel_case_types, clippy::similar_names)]
use yew::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct Open5eFeats {
    pub slug: String,
    pub name: String,
    pub desc: String,
    pub prerequisite: Option<String>, // possibly null
    pub effects_desc: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct Open5eClass {
    pub name: String,
    pub slug: String,
    pub desc: String, // Markdown
    pub hit_dice: String,
    pub hp_at_1st_level: String,
    pub hp_at_higher_levels: String,
    pub prof_armor: String,
    pub prof_weapons: String,
    pub prof_tools: String,
    pub prof_saving_throws: String,
    pub prof_skills: String,
    pub equipment: String, // Markdown
    pub table: String, // Markdown
    pub spellcasting_ability: String,
    pub subtypes_name: String,
    pub archetypes: Vec<Open5eArchetype>,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct Open5eArchetype {
    pub name: String,
    pub slug: String,
    pub desc: String,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct Open5eASI {
    pub attributes: Vec<String>,
    pub value: u8,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct Open5eSpeed {
    pub speed: HashMap<String, u8>,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct Open5eSubrace {
    pub name: String,
    pub slug: String,
    pub desc: String,
    pub asi: Vec<Open5eASI>,
    pub traits: String, // Markdown
    pub asi_desc: String, // Markdown
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct Open5eRaceList {
    pub count: i32,
    pub next: Value,
    pub previous: Value,
    pub results: Vec<Open5eRace>,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct Open5eRace {
    pub name: String,
    pub slug: String,
    pub desc: String, // Markdown
    pub asi_desc: String, // Markdown
    pub asi: Vec<Open5eASI>,
    pub age: String, // Markdown
    pub alignment: String, // Markdown
    pub size: String, // Markdown
    pub speed: Open5eSpeed,
    pub speed_desc: String, // Markdown
    pub languages: String, // Markdown
    pub vision: String, // Markdown
    pub traits: String, // Markdown
    pub subraces: Vec<Open5eSubrace>,
}

#[derive(Serialize, Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct Open5eSpell {
    pub slug: String,
    pub name: String,
    pub desc: String, // Markdown
    pub higher_level: String, // Markdown
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
}
