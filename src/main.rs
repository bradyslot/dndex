use stylist::{css, global_style};
use yew::prelude::*;
use yew::props;
use log::info;
use serde_json;

mod data;
use data::classes::barbarian::*;
// use data::classes::bard::*;
// use data::classes::cleric::*;
// use data::classes::druid::*;
// use data::classes::fighter::*;
// use data::classes::monk::*;
// use data::classes::paladin::*;
// use data::classes::ranger::*;
// use data::classes::rogue::*;
// use data::classes::sorcerer::*;
// use data::classes::warlock::*;
// use data::classes::wizard::*;

mod models;
use models::models::*;
use models::classes::*;

mod components;
use components::shared::utils::*;
use components::player_stats::*;
use components::primary_abilities::*;
use components::passive_abilities::*;
use components::skills::*;
use components::spells::*;
use components::spell_slots::*;

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
    let character_ac = AC { base: 17, modifier: 1 };
    let character_class = Class {
        name: "Artificer".into(),
        subclass: "Battle Smith".into(),
        hitdice: Dice { count: character_level, sides: 8 },
        primary: character_abilities[INT].clone(),
        saves: vec![ character_abilities[CON].clone(), character_abilities[INT].clone() ]
    };
    let character_spells = vec![
        // "awaken-object".into(),
        // "booster-shot".into(),
        // "chaotic-vitality".into(),
        // "curse-of-incompetence".into(),
        // "dragon-breath".into(),
        // "fluctuating-alignment".into(),
        // "lovesick".into(),
        // "prismatic-ray".into(),
        // "snow-boulder".into(),
        // "time-vortex".into(),
        // "timely-distraction".into(),
        // "uncontrollable-transformation".into(),
        "conjure-greater-spectral-dead".into(),
        "frenzied-bolt".into(),
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
            spells: character_spells,
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

    let barbarian: DnDClass = serde_json::from_str(BARBARIAN_DATA).unwrap();
    // let bard_json: DnDClass = serde_json::from_str(BARD_DATA).unwrap();
    // let cleric_json: DnDClass = serde_json::from_str(CLERIC_DATA).unwrap();
    // let druid_json: DnDClass = serde_json::from_str(DRUID_DATA).unwrap();
    // let fighter_json: DnDClass = serde_json::from_str(FIGHTER_DATA).unwrap();
    // let monk_json: DnDClass = serde_json::from_str(MONK_DATA).unwrap();
    // let paladin_json: DnDClass = serde_json::from_str(PALADIN_DATA).unwrap();
    // let ranger_json: DnDClass = serde_json::from_str(RANGER_DATA).unwrap();
    // let rogue_json: DnDClass = serde_json::from_str(ROGUE_DATA).unwrap();
    // let sorcerer_json: DnDClass = serde_json::from_str(SORCERER_DATA).unwrap();
    // let warlock_json: DnDClass = serde_json::from_str(WARLOCK_DATA).unwrap();
    // let wizard_json: DnDClass = serde_json::from_str(WIZARD_DATA).unwrap();

    info!("{:?}", barbarian);
    // info!("{:?}", bard_json);
    // info!("{:?}", cleric_json);
    // info!("{:?}", druid_json);
    // info!("{:?}", fighter_json);
    // info!("{:?}", monk_json);
    // info!("{:?}", paladin_json);
    // info!("{:?}", ranger_json);
    // info!("{:?}", rogue_json);
    // info!("{:?}", sorcerer_json);
    // info!("{:?}", warlock_json);
    // info!("{:?}", wizard_json);


    html! {
        <div class={style}>
            <Spells ..props.clone() />
            <SpellSlots ..props.clone() />
            <PlayerStats ..props.clone() />
            <PrimaryAbilities ..props.clone() />
            <PassiveAbilities ..props.clone() />
            <Skills ..props.clone() />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
