use stylist::yew::use_style;
use stylist::GlobalStyle;
use yew::prelude::*;
// use wasm_bindgen::prelude::*;

mod components;
use components::shared::models::*;

use components::passive_abilities::*;
use components::primary_abilities::*;
// use components::proficiency_bonus::*;
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

    let character = CharacterProps {
        level: 5,
        hp: HitPointProps {
            max: 52,
            current: 37,
            temp: 0,
            inspiration: false,
        },
    };

    let primary_abilities = PrimaryAbilitiesProps {
        character: character.clone(),
        strength: AbilityScoreProps {
            name: "Strength".into(),
            value: 8,
            saving: false,
        },
        dexterity: AbilityScoreProps {
            name: "Dexterity".into(),
            value: 13,
            saving: false,
        },
        constitution: AbilityScoreProps {
            name: "Constitution".into(),
            value: 15,
            saving: true,
        },
        intelligence: AbilityScoreProps {
            name: "Intelligence".into(),
            value: 19,
            saving: true,
        },
        wisdom: AbilityScoreProps {
            name: "Wisdom".into(),
            value: 12,
            saving: false,
        },
        charisma: AbilityScoreProps {
            name: "Charisma".into(),
            value: 10,
            saving: false,
        },
    };

    let skills = SkillListProps {
        character: primary_abilities.character.clone(),
        acrobatics: SkillProps {
            name: "Acrobatics (Dex)".into(),
            proficiency: false,
            parent: primary_abilities.dexterity.clone(),
        },
        animalhandling: SkillProps {
            name: "Animal Handling (Wis)".into(),
            proficiency: false,
            parent: primary_abilities.wisdom.clone(),
        },
        arcana: SkillProps {
            name: "Arcana (Int)".into(),
            proficiency: true,
            parent: primary_abilities.intelligence.clone(),
        },
        athletics: SkillProps {
            name: "Athletics (Str)".into(),
            proficiency: false,
            parent: primary_abilities.strength.clone(),
        },
        deception: SkillProps {
            name: "Deception (Cha)".into(),
            proficiency: false,
            parent: primary_abilities.charisma.clone(),
        },
        history: SkillProps {
            name: "History (Int)".into(),
            proficiency: false,
            parent: primary_abilities.intelligence.clone(),
        },
        insight: SkillProps {
            name: "Insight (Wis)".into(),
            proficiency: true,
            parent: primary_abilities.wisdom.clone(),
        },
        intimidation: SkillProps {
            name: "Intimidation (Cha)".into(),
            proficiency: false,
            parent: primary_abilities.charisma.clone(),
        },
        investigation: SkillProps {
            name: "Investigation (Int)".into(),
            proficiency: false,
            parent: primary_abilities.intelligence.clone(),
        },
        medicine: SkillProps {
            name: "Medicine (Wis)".into(),
            proficiency: false,
            parent: primary_abilities.wisdom.clone(),
        },
        nature: SkillProps {
            name: "Nature (Int)".into(),
            proficiency: true,
            parent: primary_abilities.intelligence.clone(),
        },
        perception: SkillProps {
            name: "Perception (Wis)".into(),
            proficiency: false,
            parent: primary_abilities.wisdom.clone(),
        },
        performance: SkillProps {
            name: "Performance (Cha)".into(),
            proficiency: false,
            parent: primary_abilities.charisma.clone(),
        },
        persuasion: SkillProps {
            name: "Persuasion (Cha)".into(),
            proficiency: true,
            parent: primary_abilities.charisma.clone(),
        },
        religion: SkillProps {
            name: "Religion (Int)".into(),
            proficiency: false,
            parent: primary_abilities.intelligence.clone(),
        },
        sleightofhand: SkillProps {
            name: "Sleight of Hand (Dex)".into(),
            proficiency: false,
            parent: primary_abilities.dexterity.clone(),
        },
        stealth: SkillProps {
            name: "Stealth (Dex)".into(),
            proficiency: false,
            parent: primary_abilities.dexterity.clone(),
        },
        survival: SkillProps {
            name: "Survival (Wis)".into(),
            proficiency: false,
            parent: primary_abilities.wisdom.clone(),
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

    html! {
        <div class={style}>
            <div class="row">
                <SavingThrows 
                    character={primary_abilities.character.clone()}
                    strength={primary_abilities.strength.clone()}
                    dexterity={primary_abilities.dexterity.clone()}
                    constitution={primary_abilities.constitution.clone()}
                    intelligence={primary_abilities.intelligence.clone()}
                    wisdom={primary_abilities.wisdom.clone()}
                    charisma={primary_abilities.charisma.clone()}
                />
                <HitPoints 
                    current={character.hp.current.clone()}
                    max={character.hp.max.clone()}
                    temp={character.hp.temp.clone()}
                    inspiration={character.hp.inspiration.clone()}
                />
            </div>
            <div class="row">
                <PassiveAbilities 
                    character={primary_abilities.character.clone()}
                    strength={primary_abilities.strength.clone()}
                    dexterity={primary_abilities.dexterity.clone()}
                    constitution={primary_abilities.constitution.clone()}
                    intelligence={primary_abilities.intelligence.clone()}
                    wisdom={primary_abilities.wisdom.clone()}
                    charisma={primary_abilities.charisma.clone()}
                />
            </div>
            <div class="row">
                <PrimaryAbilities
                    character={primary_abilities.character.clone()}
                    strength={primary_abilities.strength.clone()}
                    dexterity={primary_abilities.dexterity.clone()}
                    constitution={primary_abilities.constitution.clone()}
                    intelligence={primary_abilities.intelligence.clone()}
                    wisdom={primary_abilities.wisdom.clone()}
                    charisma={primary_abilities.charisma.clone()}
                />
            </div>
            <div class="row">
                <Skills
                    character={primary_abilities.character.clone()}
                    acrobatics={skills.acrobatics.clone()}
                    animalhandling={skills.animalhandling.clone()}
                    arcana={skills.arcana.clone()}
                    athletics={skills.athletics.clone()}
                    deception={skills.deception.clone()}
                    history={skills.history.clone()}
                    insight={skills.insight.clone()}
                    intimidation={skills.intimidation.clone()}
                    investigation={skills.investigation.clone()}
                    medicine={skills.medicine.clone()}
                    nature={skills.nature.clone()}
                    perception={skills.perception.clone()}
                    performance={skills.performance.clone()}
                    persuasion={skills.persuasion.clone()}
                    religion={skills.religion.clone()}
                    sleightofhand={skills.sleightofhand.clone()}
                    stealth={skills.stealth.clone()}
                    survival={skills.survival.clone()}
                />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

