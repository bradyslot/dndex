#![allow(non_snake_case, non_camel_case_types, clippy::similar_names)]
use yew::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Properties, PartialEq)]
pub struct Character {
    pub abilities: Vec<Ability>,
    pub ac: AC,
    pub class: Class,
    pub deathsaves: DeathSaves,
    pub hp: Health,
    pub initiative: i8,
    pub inspiration: bool,
    pub level: u8,
    pub name: AttrValue,
    pub passives: Vec<Passive>,
    pub skills: Vec<Skill>,
    pub speed: Movement,
    pub spells: Vec<String>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct AC {
    pub base: u8,
    pub modifier: i8,
}

// this only applies to WOTC 5e SRD 5.1
// #[derive(Deserialize, Clone, Properties, PartialEq)]
// pub struct Open5eSpellList {
//     pub slug: String, // spellcasters
//     pub name: String,
//     // pub desc: String, // always empty
//     pub spells: Vec<String>,
// }

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct Open5eFeats {
    pub slug: String,
    pub name: String,
    pub desc: String,
    pub prerequisite: Option<String>, // possibly null
    pub effects_desc: Vec<String>,
}


#[derive(Deserialize, Clone, Properties, PartialEq)]
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

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct Open5eASI {
    pub attributes: Vec<String>,
    pub value: u8,
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct Open5eSpeed {
    pub speed: HashMap<String, u8>,
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct Open5eSubrace {
    pub name: String,
    pub slug: String,
    pub desc: String,
    pub asi: Vec<Open5eASI>,
    pub traits: String, // Markdown
    pub asi_desc: String, // Markdown
}

#[derive(Deserialize, Clone, Properties, PartialEq)]
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

#[derive(Deserialize, Clone, Properties, PartialEq)]
pub struct Open5eArchetype {
    pub name: String,
    pub slug: String,
    pub desc: String,
}

#[derive(Deserialize, Debug, Clone, Properties, PartialEq)]
pub struct Open5eSpell {
    pub slug: String,
    pub name: String,
    pub desc: String, // Markdown sometimes
    pub higher_level: String, // Markdown sometimes
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

#[derive(Clone, Properties, PartialEq)]
pub struct Class {
    pub name: AttrValue,
    pub subclass: AttrValue,
    pub primary: Ability,
    pub saves: Vec<Ability>,
    pub hitdice: Dice,
    // pub features
    // pub archetype
}

#[derive(Clone, Properties, PartialEq)]
pub struct Dice {
    pub count: u8,
    pub sides: u8,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Passive {
    pub name: String,
    pub value: i8,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Movement {
    pub base: u8,
    pub modifier: i8,
}

#[derive(Clone, Properties, PartialEq)]
pub struct DeathSaves {
    pub success: [bool; 3],
    pub failure: [bool; 3],
}

#[derive(Clone, Properties, PartialEq)]
pub struct Health {
    pub current: i8,
    pub max: i8,
    pub temp: i8,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Ability {
    pub name: AttrValue,
    pub value: u8,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Skill {
    pub name: AttrValue,
    pub proficiency: bool,
    pub primary: Ability,
}

// generic component props
#[derive(Clone, Properties, PartialEq)]
pub struct Child {
    pub children: Option<Children>,
    pub label: Option<AttrValue>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Label {
    pub text: Option<AttrValue>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct LabeledValueProps {
    pub label: AttrValue,
    pub value: i8,
}

#[derive(Clone, Properties, PartialEq)]
pub struct LabeledValueCheckboxProps {
    pub label: AttrValue,
    pub value: i8,
    pub checked: bool,
}

#[derive(Clone, Properties, PartialEq)]
pub struct LabeledValueModiferBoxProps {
    pub label: AttrValue,
    pub value: Option<u8>,
    pub text: Option<AttrValue>,
    pub modifier: i8,
}
