use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Character {
    pub abilities: Vec<Ability>,
    pub deathsaves: DeathSaves,
    pub hp: Health,
    pub initiative: i8,
    pub inspiration: bool,
    pub level: u8,
    pub passives: Vec<Passive>,
    pub skills: Vec<Skill>,
    pub speed: Movement,
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
    pub saving: bool,
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
    pub children: Children,
    pub label: Option<AttrValue>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Label {
    pub text: Option<AttrValue>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct LabeledValueProps {
    pub value: i8,
    pub label: AttrValue,
}

#[derive(Clone, Properties, PartialEq)]
pub struct LabeledValueCheckboxProps {
    pub value: i8,
    pub label: AttrValue,
    pub checked: bool,
}

#[derive(Clone, Properties, PartialEq)]
pub struct LabeledValueModiferBoxProps {
    pub label: AttrValue,
    pub value: u8,
    pub modifier: i8,
}
