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
    pub initiative: i32,
    pub inspiration: bool,
    pub level: i32,
    pub name: AttrValue,
    pub passives: Vec<Passive>,
    pub skills: Vec<Skill>,
    pub speed: Movement,
    pub spells: Vec<String>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct AC {
    pub base: i32,
    pub modifier: i32,
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
    pub count: i32,
    pub sides: i32,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Passive {
    pub name: String,
    pub value: i32,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Movement {
    pub base: i32,
    pub modifier: i32,
}

#[derive(Clone, Properties, PartialEq)]
pub struct DeathSaves {
    pub success: [bool; 3],
    pub failure: [bool; 3],
}

#[derive(Clone, Properties, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
    pub temp: i32,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Ability {
    pub name: AttrValue,
    pub value: i32,
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
    pub value: i32,
}

#[derive(Clone, Properties, PartialEq)]
pub struct LabeledValueCheckboxProps {
    pub label: AttrValue,
    pub value: i32,
    pub checked: bool,
}

#[derive(Clone, Properties, PartialEq)]
pub struct LabeledValueModiferBoxProps {
    pub label: AttrValue,
    pub value: Option<i32>,
    pub text: Option<AttrValue>,
    pub modifier: i32,
}
