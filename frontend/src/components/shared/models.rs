use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct HitPointProps {
    pub current: i8,
    pub max: i8,
    pub temp: i8,
    pub inspiration: bool,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Label {
    pub text: AttrValue,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Dimensions {
    pub height: AttrValue,
    pub width: AttrValue,
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
pub struct AbilityScoreProps {
    pub name: AttrValue,
    pub value: u8,
}

#[derive(Clone, Properties, PartialEq)]
pub struct PrimaryAbilitiesProps {
    pub strength: AbilityScoreProps,
    pub dexterity: AbilityScoreProps,
    pub constitution: AbilityScoreProps,
    pub intelligence: AbilityScoreProps,
    pub wisdom: AbilityScoreProps,
    pub charisma: AbilityScoreProps,
}

#[derive(Clone, Properties, PartialEq)]
pub struct CharacterProps {
    pub level: u8,
}

#[derive(Clone, Properties, PartialEq)]
pub struct SkillProps {
    pub name: AttrValue,
    pub proficiency: bool,
    pub parent: AbilityScoreProps,
}

#[derive(Clone, Properties, PartialEq)]
pub struct SkillListProps {
    pub character: CharacterProps,
    pub acrobatics: SkillProps,
    pub animalhandling: SkillProps,
    pub arcana: SkillProps,
    pub athletics: SkillProps,
    pub deception: SkillProps,
    pub history: SkillProps,
    pub insight: SkillProps,
    pub intimidation: SkillProps,
    pub investigation: SkillProps,
    pub medicine: SkillProps,
    pub nature: SkillProps,
    pub perception: SkillProps,
    pub performance: SkillProps,
    pub persuasion: SkillProps,
    pub religion: SkillProps,
    pub sleightofhand: SkillProps,
    pub stealth: SkillProps,
    pub survival: SkillProps,
}

