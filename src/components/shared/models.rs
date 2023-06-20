use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Character {
    pub level: u8,
    pub hp: Health,
    pub abilities: Vec<Ability>,
    pub skills: Vec<Skill>,
    pub saves: Saves,
    pub inspiration: bool,
    pub speed: Movement,
    pub passives: Vec<Passive>,
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
pub struct Saves {
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
}

#[derive(Clone, Properties, PartialEq)]
pub struct Label {
    pub text: AttrValue,
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
