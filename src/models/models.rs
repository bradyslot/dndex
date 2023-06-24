use yew::prelude::*;
use serde::Deserialize;

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

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone, Properties, PartialEq)]
pub struct Spell {
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
    pub document__slug: String,
    pub document__title: String,
    pub document__license_url: String,
    pub document__url: String,
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
