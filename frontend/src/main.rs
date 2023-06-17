use stylist::yew::use_style;
use stylist::GlobalStyle;
use yew::prelude::*;
use yew::props;
// use yew::Properties;
// use wasm_bindgen::prelude::*;

mod components;
use components::shared::models::*;

use components::passive_abilities::*;
use components::primary_abilities::*;
use components::skills::*;
use components::hit_points::*;
use components::saving_throws::*;

mod constants;
use constants::*;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

#[function_component]
fn App() -> Html {
    let _global_style = GlobalStyle::new(format!(r#"
        --background: {};
        --foreground: {};
        background: var(--background);
        font-family: {}; "#
        , BACKGROUND
        , FOREGROUND
        , FONT
    ));

    let abilities = Abilities {
        strength: Ability {
            name: "Strength".into(),
            value: 8,
            saving: false,
        },
        dexterity: Ability {
            name: "Dexterity".into(),
            value: 13,
            saving: false,
        },
        constitution: Ability {
            name: "Constitution".into(),
            value: 15,
            saving: true,
        },
        intelligence: Ability {
            name: "Intelligence".into(),
            value: 19,
            saving: true,
        },
        wisdom: Ability {
            name: "Wisdom".into(),
            value: 12,
            saving: false,
        },
        charisma: Ability {
            name: "Charisma".into(),
            value: 10,
            saving: false,
        },
    };

    let hp = Health {
        max: 52,
        current: 37,
        temp: 0,
        inspiration: false,
    };

    let skills = SkillList {
        acrobatics: Skill {
            name: "Acrobatics (Dex)".into(),
            proficiency: false,
            primary: abilities.dexterity.clone(),
        },
        animalhandling: Skill {
            name: "Animal Handling (Wis)".into(),
            proficiency: false,
            primary: abilities.wisdom.clone(),
        },
        arcana: Skill {
            name: "Arcana (Int)".into(),
            proficiency: true,
            primary: abilities.intelligence.clone(),
        },
        athletics: Skill {
            name: "Athletics (Str)".into(),
            proficiency: false,
            primary: abilities.strength.clone(),
        },
        deception: Skill {
            name: "Deception (Cha)".into(),
            proficiency: false,
            primary: abilities.charisma.clone(),
        },
        history: Skill {
            name: "History (Int)".into(),
            proficiency: false,
            primary: abilities.intelligence.clone(),
        },
        insight: Skill {
            name: "Insight (Wis)".into(),
            proficiency: true,
            primary: abilities.wisdom.clone(),
        },
        intimidation: Skill {
            name: "Intimidation (Cha)".into(),
            proficiency: false,
            primary: abilities.charisma.clone(),
        },
        investigation: Skill {
            name: "Investigation (Int)".into(),
            proficiency: false,
            primary: abilities.intelligence.clone(),
        },
        medicine: Skill {
            name: "Medicine (Wis)".into(),
            proficiency: false,
            primary: abilities.wisdom.clone(),
        },
        nature: Skill {
            name: "Nature (Int)".into(),
            proficiency: true,
            primary: abilities.intelligence.clone(),
        },
        perception: Skill {
            name: "Perception (Wis)".into(),
            proficiency: false,
            primary: abilities.wisdom.clone(),
        },
        performance: Skill {
            name: "Performance (Cha)".into(),
            proficiency: false,
            primary: abilities.charisma.clone(),
        },
        persuasion: Skill {
            name: "Persuasion (Cha)".into(),
            proficiency: true,
            primary: abilities.charisma.clone(),
        },
        religion: Skill {
            name: "Religion (Int)".into(),
            proficiency: false,
            primary: abilities.intelligence.clone(),
        },
        sleightofhand: Skill {
            name: "Sleight of Hand (Dex)".into(),
            proficiency: false,
            primary: abilities.dexterity.clone(),
        },
        stealth: Skill {
            name: "Stealth (Dex)".into(),
            proficiency: false,
            primary: abilities.dexterity.clone(),
        },
        survival: Skill {
            name: "Survival (Wis)".into(),
            proficiency: false,
            primary: abilities.wisdom.clone(),
        },
    };

    let style = use_style!(
        r#"
            display: flex;
            flex-direction: column;
            flex-grow: 1;

            .row {
                display: flex;
                flex-direction: row;
                flex-grow: 1;
            }
        "#
    );

    let props = props! {
        Character { 
            level: 5,
            hp: hp,
            abilities: abilities,
            skills: skills,
        }
    };

    html! {
        <div class={style}>
            <div class="row">
                <SavingThrows ..props.clone() />
                <HitPoints ..props.clone() />
            </div>
            <div class="row">
                <PassiveAbilities ..props.clone() />
            </div>
            <div class="row">
                <PrimaryAbilities ..props.clone() />
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

