use stylist::{css, global_style};
use yew::prelude::*;
use yew::props;

mod components;
use components::shared::models::*;
use components::shared::utils::*;
use components::player_stats::*;
use components::primary_abilities::*;
use components::passive_abilities::*;
use components::skills::*;
use components::spell_card::*;

mod constants;
use constants::*;

#[function_component]
fn App() -> Html {
    let character_abilities = vec![
        Ability { name: "Strength".into(),     value: 8  },
        Ability { name: "Dexterity".into(),    value: 13 },
        Ability { name: "Constitution".into(), value: 15 },
        Ability { name: "Intelligence".into(), value: 19 },
        Ability { name: "Wisdom".into(),       value: 12 },
        Ability { name: "Charisma".into(),     value: 10 },
    ];
    let character_skills = vec![
        Skill { name: "Acrobatics (Dex)".into(),      proficiency: false, primary: character_abilities[DEX].clone() },
        Skill { name: "Animal Handling (Wis)".into(), proficiency: false, primary: character_abilities[WIS].clone() },
        Skill { name: "Arcana (Int)".into(),          proficiency: true,  primary: character_abilities[INT].clone() },
        Skill { name: "Athletics (Str)".into(),       proficiency: false, primary: character_abilities[STR].clone() },
        Skill { name: "Deception (Cha)".into(),       proficiency: false, primary: character_abilities[CHA].clone() },
        Skill { name: "History (Int)".into(),         proficiency: false, primary: character_abilities[INT].clone() },
        Skill { name: "Insight (Wis)".into(),         proficiency: true,  primary: character_abilities[WIS].clone() },
        Skill { name: "Intimidation (Cha)".into(),    proficiency: false, primary: character_abilities[CHA].clone() },
        Skill { name: "Investigation (Int)".into(),   proficiency: false, primary: character_abilities[INT].clone() },
        Skill { name: "Medicine (Wis)".into(),        proficiency: false, primary: character_abilities[WIS].clone() },
        Skill { name: "Nature (Int)".into(),          proficiency: true,  primary: character_abilities[INT].clone() },
        Skill { name: "Perception (Wis)".into(),      proficiency: false, primary: character_abilities[WIS].clone() },
        Skill { name: "Performance (Cha)".into(),     proficiency: false, primary: character_abilities[CHA].clone() },
        Skill { name: "Persuasion (Cha)".into(),      proficiency: false, primary: character_abilities[CHA].clone() },
        Skill { name: "Religion (Int)".into(),        proficiency: false, primary: character_abilities[INT].clone() },
        Skill { name: "Sleight of Hand (Dex)".into(), proficiency: false, primary: character_abilities[DEX].clone() },
        Skill { name: "Stealth (Dex)".into(),         proficiency: false, primary: character_abilities[DEX].clone() },
        Skill { name: "Survival (Wis)".into(),        proficiency: false, primary: character_abilities[WIS].clone() },
    ];
    let character_passives = vec![
        Passive { name: "Passive Perception (Wis)".into(),    value: 10 + calc_base_modifier(character_abilities[WIS].value) },
        Passive { name: "Passive Investigation (Int)".into(), value: 10 + calc_base_modifier(character_abilities[INT].value) },
        Passive { name: "Passive Insight (Wis)".into(),       value: 10 + calc_base_modifier(character_abilities[WIS].value) },
        Passive { name: "Passive Stealth (Dex)".into(),       value: 10 + calc_base_modifier(character_abilities[DEX].value) },
    ];
    let character_deathsaves = DeathSaves {
        success: [true, false, false],
        failure: [true, false, false],
    };
    let character_name: AttrValue = "Howard".into();
    let character_hp = Health { max: 52, current: 37, temp: 0 };
    let character_speed = Movement { base: 25, modifier: -5 };
    let character_initiative = calc_base_modifier(character_abilities[DEX].value);
    let character_level = 6;
    let character_ac = 17;
    let character_class = Class {
        name: "Artificer".into(),
        subclass: "Battle Smith".into(),
        hitdice: Dice { count: character_level, sides: 8 },
        primary: character_abilities[INT].clone(),
        saves: vec![ character_abilities[CON].clone(), character_abilities[INT].clone() ]
    };
    let character_spells = vec![
        Spell {
            name: "Fireball".into(),
            level: 0,
            casttime: "1 action".into(),
            range: "120\"".into(),
            duration: "Instant".into(),
            school: "Evocation".into(),
            ritual: false,
            concentration: false,
            components: SpellComponents { verbal: true, somatic: true, material: false },
            description: "1d10 fire damage. Flammable objects ignite if not worn or carried.".into(),
            classes: vec![],
            higherlevels: None,
        }
    ];

    let props = props! {
        Character { 
            abilities: character_abilities,
            ac: character_ac,
            class: character_class,
            deathsaves: character_deathsaves,
            hp: character_hp,
            initiative: character_initiative,
            inspiration: false,
            level: character_level,
            name: character_name,
            passives: character_passives,
            skills: character_skills,
            speed: character_speed,
            spells: character_spells.clone(),
        }
    };

    let _global_style = global_style!(
        --background: ${BACKGROUND};
        --foreground: ${FOREGROUND};
        background: ${BACKGROUND};
        font-family: ${FONT};
    );

    let style = css!(
        display: flex;
        flex-direction: column;
    );

    html! {
        <div class={style}>
            <SpellCard ..character_spells[0].clone() />
            <PlayerStats ..props.clone() />
            <PrimaryAbilities ..props.clone() />
            <PassiveAbilities ..props.clone() />
            <Skills ..props.clone() />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
