use stylist::yew::use_style;
use stylist::GlobalStyle;
use yew::prelude::*;
use yew::props;
// use yew::Properties;
// use wasm_bindgen::prelude::*;

mod components;
use components::shared::models::*;
use components::shared::utils::*;

use components::passive_abilities::*;
use components::primary_abilities::*;
use components::skills::*;
use components::hit_points::*;
use components::saving_throws::*;
use components::speed::*;
use components::death_saves::*;

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

    let abilities = vec![
        Ability { name: "Strength".into(),     value: 8,  saving: false },
        Ability { name: "Dexterity".into(),    value: 13, saving: false },
        Ability { name: "Constitution".into(), value: 15, saving: true  },
        Ability { name: "Intelligence".into(), value: 19, saving: true  },
        Ability { name: "Wisdom".into(),       value: 12, saving: false },
        Ability { name: "Charisma".into(),     value: 10, saving: false },
    ];

    let hp = Health { max: 52, current: 37, temp: 0 };

    // str, dex, con, int, wis, cha
    // 0  , 1  , 2  , 3  , 4  , 5
    let skills = vec![
        Skill { name: "Acrobatics (Dex)".into(),      proficiency: false, primary: abilities[1].clone() },
        Skill { name: "Animal Handling (Wis)".into(), proficiency: false, primary: abilities[4].clone() },
        Skill { name: "Arcana (Int)".into(),          proficiency: true,  primary: abilities[3].clone() },
        Skill { name: "Athletics (Str)".into(),       proficiency: false, primary: abilities[0].clone() },
        Skill { name: "Deception (Cha)".into(),       proficiency: false, primary: abilities[5].clone() },
        Skill { name: "History (Int)".into(),         proficiency: false, primary: abilities[3].clone() },
        Skill { name: "Insight (Wis)".into(),         proficiency: true,  primary: abilities[4].clone() },
        Skill { name: "Intimidation (Cha)".into(),    proficiency: false, primary: abilities[5].clone() },
        Skill { name: "Investigation (Int)".into(),   proficiency: false, primary: abilities[3].clone() },
        Skill { name: "Medicine (Wis)".into(),        proficiency: false, primary: abilities[4].clone() },
        Skill { name: "Nature (Int)".into(),          proficiency: true,  primary: abilities[3].clone() },
        Skill { name: "Perception (Wis)".into(),      proficiency: false, primary: abilities[4].clone() },
        Skill { name: "Performance (Cha)".into(),     proficiency: false, primary: abilities[5].clone() },
        Skill { name: "Persuasion (Cha)".into(),      proficiency: false, primary: abilities[5].clone() },
        Skill { name: "Religion (Int)".into(),        proficiency: false, primary: abilities[3].clone() },
        Skill { name: "Sleight of Hand (Dex)".into(), proficiency: false, primary: abilities[1].clone() },
        Skill { name: "Stealth (Dex)".into(),         proficiency: false, primary: abilities[1].clone() },
        Skill { name: "Survival (Wis)".into(),        proficiency: false, primary: abilities[4].clone() },
    ];

    let passives = vec![
        Passive { name: "Passive Perception (Wis)".into(),    value: 10 + calc_base_modifier(abilities[4].value) },
        Passive { name: "Passive Investigation (Int)".into(), value: 10 + calc_base_modifier(abilities[3].value) },
        Passive { name: "Passive Insight (Wis)".into(),       value: 10 + calc_base_modifier(abilities[4].value) },
        Passive { name: "Passive Stealth (Dex)".into(),       value: 10 + calc_base_modifier(abilities[1].value) },
    ];

    let saves = Saves {
        success: [true, false, false],
        failure: [true, false, false],
    };

    let speed = Movement {
        base: 25,
        modifier: -5,
    };

    let props = props! {
        Character { 
            level: 5,
            hp: hp,
            abilities: abilities,
            skills: skills,
            saves: saves,
            inspiration: false,
            speed: speed,
            passives: passives,
        }
    };

    // grid-area: (row start) / (column start) / (row end) / (column end)
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

    html! {
        <div class={style}>
            <div class="row">
                <Speed ..props.clone() />
                <DeathSaves ..props.clone() />
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

