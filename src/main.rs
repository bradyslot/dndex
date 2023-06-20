use stylist::{css, global_style};
use yew::prelude::*;
use yew::props;

mod components;
use components::shared::models::*;
use components::shared::utils::*;

use components::passive_abilities::*;
use components::primary_abilities::*;
use components::skills::*;
use components::hit_points::*;
use components::saving_throws::*;
use components::speed::*;
use components::death_save_rolls::*;
use components::initiative::*;
use components::hit_dice::*;

mod constants;
use constants::*;

#[function_component]
fn App() -> Html {
    let abilities = vec![
        Ability { name: "Strength".into(),     value: 8,  saving: false },
        Ability { name: "Dexterity".into(),    value: 13, saving: false },
        Ability { name: "Constitution".into(), value: 15, saving: true  },
        Ability { name: "Intelligence".into(), value: 19, saving: true  },
        Ability { name: "Wisdom".into(),       value: 12, saving: false },
        Ability { name: "Charisma".into(),     value: 10, saving: false },
    ];

    let hp = Health { max: 52, current: 37, temp: 0 };

    let skills = vec![
        Skill { name: "Acrobatics (Dex)".into(),      proficiency: false, primary: abilities[DEX].clone() },
        Skill { name: "Animal Handling (Wis)".into(), proficiency: false, primary: abilities[WIS].clone() },
        Skill { name: "Arcana (Int)".into(),          proficiency: true,  primary: abilities[INT].clone() },
        Skill { name: "Athletics (Str)".into(),       proficiency: false, primary: abilities[STR].clone() },
        Skill { name: "Deception (Cha)".into(),       proficiency: false, primary: abilities[CHA].clone() },
        Skill { name: "History (Int)".into(),         proficiency: false, primary: abilities[INT].clone() },
        Skill { name: "Insight (Wis)".into(),         proficiency: true,  primary: abilities[WIS].clone() },
        Skill { name: "Intimidation (Cha)".into(),    proficiency: false, primary: abilities[CHA].clone() },
        Skill { name: "Investigation (Int)".into(),   proficiency: false, primary: abilities[INT].clone() },
        Skill { name: "Medicine (Wis)".into(),        proficiency: false, primary: abilities[WIS].clone() },
        Skill { name: "Nature (Int)".into(),          proficiency: true,  primary: abilities[INT].clone() },
        Skill { name: "Perception (Wis)".into(),      proficiency: false, primary: abilities[WIS].clone() },
        Skill { name: "Performance (Cha)".into(),     proficiency: false, primary: abilities[CHA].clone() },
        Skill { name: "Persuasion (Cha)".into(),      proficiency: false, primary: abilities[CHA].clone() },
        Skill { name: "Religion (Int)".into(),        proficiency: false, primary: abilities[INT].clone() },
        Skill { name: "Sleight of Hand (Dex)".into(), proficiency: false, primary: abilities[DEX].clone() },
        Skill { name: "Stealth (Dex)".into(),         proficiency: false, primary: abilities[DEX].clone() },
        Skill { name: "Survival (Wis)".into(),        proficiency: false, primary: abilities[WIS].clone() },
    ];

    let passives = vec![
        Passive { name: "Passive Perception (Wis)".into(),    value: 10 + calc_base_modifier(abilities[WIS].value) },
        Passive { name: "Passive Investigation (Int)".into(), value: 10 + calc_base_modifier(abilities[INT].value) },
        Passive { name: "Passive Insight (Wis)".into(),       value: 10 + calc_base_modifier(abilities[WIS].value) },
        Passive { name: "Passive Stealth (Dex)".into(),       value: 10 + calc_base_modifier(abilities[DEX].value) },
    ];

    let deathsaves = DeathSaves {
        success: [true, false, false],
        failure: [true, false, false],
    };

    let speed = Movement { base: 25, modifier: -5 };
    
    let initiative = calc_base_modifier(abilities[1].value);

    let props = props! {
        Character { 
            abilities: abilities,
            deathsaves: deathsaves,
            hp: hp,
            initiative: initiative,
            inspiration: false,
            level: 5,
            passives: passives,
            skills: skills,
            speed: speed,
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
        flex-grow: 1;

        .row {
            display: flex;
            flex-direction: row;
            flex-grow: 1;
        }
    );

    html! {
        <div class={style}>
            <div class="row">
                <Speed ..props.clone() />
                <Initiative ..props.clone() />
                <HitDice ..props.clone() />
                <DeathSaveRolls ..props.clone() />
            </div>
            <div class="row">
                <SavingThrows ..props.clone() />
                <HitPoints ..props.clone() />
            </div>
            <div class="row">
                <PrimaryAbilities ..props.clone() />
            </div>
            <div class="row">
                <PassiveAbilities ..props.clone() />
            </div>
            <div class="row">
                <Skills ..props.clone() />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
