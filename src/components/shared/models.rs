use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Character {
    pub level: u8,
    pub hp: Health,
    pub abilities: Abilities,
    pub skills: AllSkills,
    pub saves: Saves,
    pub inspiration: bool,
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
pub struct Abilities {
    pub strength: Ability,
    pub dexterity: Ability,
    pub constitution: Ability,
    pub intelligence: Ability,
    pub wisdom: Ability,
    pub charisma: Ability,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Skill {
    pub name: AttrValue,
    pub proficiency: bool,
    pub primary: Ability,
}

#[derive(Clone, Properties, PartialEq)]
pub struct AllSkills {
    pub acrobatics: Skill,
    pub animalhandling: Skill,
    pub arcana: Skill,
    pub athletics: Skill,
    pub deception: Skill,
    pub history: Skill,
    pub insight: Skill,
    pub intimidation: Skill,
    pub investigation: Skill,
    pub medicine: Skill,
    pub nature: Skill,
    pub perception: Skill,
    pub performance: Skill,
    pub persuasion: Skill,
    pub religion: Skill,
    pub sleightofhand: Skill,
    pub stealth: Skill,
    pub survival: Skill,
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
