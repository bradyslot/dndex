use stylist::yew::use_style;
use stylist::GlobalStyle;
use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

const FOREGROUND: &str = "#F8F8F8";
const BACKGROUND: &str = "#F8F8F8";
const FONT: &str = "Serif";

fn calc_proficiency_bonus(level: u8) -> i8 {
    2 + (level as i8 - 1) / 4
}

fn calc_base_modifier(ability: u8) -> i8 {
    (ability as i8 - 10) / 2
}


#[derive(Clone, Properties, PartialEq)]
struct Dimensions {
    height: String,
    width: String,
}

#[function_component(BittenRectangle)]
fn bitten_rectangle(props: &Dimensions) -> Html {
    let vars = format!("
        --height: {}; --width: {};
        --background: {}; --foreground: {}; "
        , props.height , props.width
        , BACKGROUND , FOREGROUND
    );

    let style = use_style!("
        height: var(--height);
        width: var(--width);
        background-color: var(--foreground);
        position: relative;
        overflow: hidden;

        .border {
            position: absolute;
            width: calc(var(--width) - 4px);
            height: calc(var(--height) - 4px);
            border: 2px solid black;
        }

        .radius {
            position: absolute;
            width: 20px;
            height: 20px;
            border-radius: 100%;
            background-color: var(--background);
            border: 2px solid black;
        }

        .top { top: -10px; }
        .bottom { bottom: -10px; }
        .left { left: -10px; }
        .right { right: -10px; }
    ");

    html! {
        <div class={style} style={vars}>
            <div class="border" />
            <div class="radius top left" />
            <div class="radius top right" />
            <div class="radius bottom left" />
            <div class="radius bottom right" />
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
struct AbilityScoreProps {
    name: String,
    value: u8,
}

#[function_component(AbilityScore)]
fn ability_score(props: &AbilityScoreProps) -> Html {
    let size = &Dimensions { height: "15rem".to_string(), width: "15rem".to_string() };
    let vars = format!("
        --height: {}; --width: {};
        --foreground: {}; "
        , size.height , size.width
        , FOREGROUND
    );
    // let ability = use_state(|| {props.value});
    let modifier = props.value as i8;
    let modifier = use_state(|| {(modifier - 10) / 2});

    let style = use_style!("
        display: flex;
        flex-grow: 1;
        justify-content: center;
        padding: 4px;

        .container {
            display: flex;
            height: var(--height);
            width: var(--width);
            position: relative;
        }
        
        .text {
            display: flex;
            position: absolute;
            justify-content: center;
            align-items: center;
            text-align: center;
            line-height: 1;
        }

        .modifier {
            top: 0;
            left: 0;
            height: var(--height);
            width: var(--width);
            font-size: calc(var(--height) / 3);
        }

        .ability {
            bottom: -20%;
            left: 27.5%;
            height: 40%;
            width: 40%;
            border-radius: 100%;
            background-color: var(--foreground);
            border: 2px solid black;
            font-size: calc(var(--height) / 5);
        }

        .label {
            width: var(--width);
            top: 5%;
            font-size: calc(var(--height) / 8);
        }

        .overlay {
            z-index: 9;
        }
    ");

    html! {
        <div class={style} style={vars}>
            <div class="container">
                <BittenRectangle height={size.height.clone()} width={size.width.clone()} />
                <div class="text modifier overlay">{ *modifier }</div>
                <div class="text ability overlay">{ &props.value }</div>
                <div class="label text overlay">{ &props.name }</div>
            </div>
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
struct AbilityProps {
    name: String,
    value: u8,
}

#[derive(Clone, Properties, PartialEq)]
struct PrimaryAbilitiesProps {
    strength: AbilityProps,
    dexterity: AbilityProps,
    constitution: AbilityProps,
    intelligence: AbilityProps,
    wisdom: AbilityProps,
    charisma: AbilityProps,
}

#[function_component(PrimaryAbilities)]
fn primary_abilities(props: &PrimaryAbilitiesProps) -> Html {
    let style = use_style!("
        display: flex;
        flex-direction: row;
        flex-grow: 1;
        margin-bottom: 3rem;
    ");

    html! {
        <div class={style}>
            <AbilityScore value={props.strength.value.clone()} name={props.strength.name.clone()} />
            <AbilityScore value={props.dexterity.value.clone()} name={props.dexterity.name.clone()} />
            <AbilityScore value={props.constitution.value.clone()} name={props.constitution.name.clone()} />
            <AbilityScore value={props.intelligence.value.clone()} name={props.intelligence.name.clone()} />
            <AbilityScore value={props.wisdom.value.clone()} name={props.wisdom.name.clone()} />
            <AbilityScore value={props.charisma.value.clone()} name={props.charisma.name.clone()} />
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
struct LabeledValueProps {
    value: i8,
    label: String,
}

#[function_component(LabeledValue)]
fn labeled_value(props: &LabeledValueProps) -> Html {
    let vars = format!("
        --foreground: {};
        --size: 5rem; "
        ,
        FOREGROUND
    );
    let style = use_style!("
        display: flex;
        flex-direction: row;
        flex-grow: 1;
        justify-content: flex-start;
        text-align: center;
        padding: 4px;

        .container {
            display: flex;
            flex-grow: 1;
        }
        .circle {
            display: flex;
            border-radius: 50%;
            height: var(--size);
            width: var(--size);
            background-color: var(--foreground);
            border: 2px solid black;
        }
        .value {
            height: var(--size);
            width: var(--size);
            font-size: calc(var(--size) / 2);
            line-height: 2;
        }
        .label {
            display: flex;
            flex-grow: 1;
            justify-content: flex-start;
            align-items: center;
            font-size: calc(var(--size) / 2.5);
            padding: 4px;
        }
    ");

    html! {
        <div class={style} style={vars}>
            <div class="container">
                <div class="circle">
                    <div class="value">{ &props.value }</div>
                </div>
                <div class="label">{ &props.label }</div>
            </div>
        </div>
    }
}


#[derive(Clone, Properties, PartialEq)]
struct CharacterProps {
    level: u8,
}

#[function_component(ProficiencyBonus)]
fn proficiency_bonus(props: &CharacterProps) -> Html {
    let label = format!("Proficiency Bonus");

    html! {
        <LabeledValue value={calc_proficiency_bonus(props.level)} label={label} />
    }
}

#[function_component(PassiveAbilities)]
fn passive_abilities(props: &PrimaryAbilitiesProps) -> Html {
    fn modifier(ability: u8) -> i8 {
        (ability as i8 - 10) / 2
    }

    html! {
        <>
            <LabeledValue value={10 + modifier(props.wisdom.value)} label={"Perception (Passive)"} />
            <LabeledValue value={10 + modifier(props.intelligence.value)} label={"Investigation (Passive)"} />
            <LabeledValue value={10 + modifier(props.wisdom.value)} label={"Insight (Passive)"} />
            // <LabeledValue value={10 + modifier(props.intelligence)} label={"Arcana (Passive)"} />
            // <LabeledValue value={10 + modifier(props.intelligence)} label={"History (Passive)"} />
            // <LabeledValue value={10 + modifier(props.wisdom)} label={"Religion (Passive)"} />
            // <LabeledValue value={10 + modifier(props.wisdom)} label={"Nature (Passive)"} />
            // <LabeledValue value={10 + modifier(props.wisdom)} label={"Survival (Passive)"} />
        </>
    }
}

#[derive(Clone, Properties, PartialEq)]
struct LabeledValueCheckboxProps {
    value: i8,
    label: String,
    checked: bool,
}

#[function_component(LabeledValueCheckbox)]
fn labeled_value_checkbox(props: &LabeledValueCheckboxProps) -> Html {
    let vars = format!("
        --foreground: {};
        --size: 4rem; "
        ,
        FOREGROUND
    );
    let style = use_style!("
        display: flex;
        flex-direction: row;
        flex-grow: 1;
        justify-content: flex-start;
        text-align: center;
        padding: 4px;

        .container {
            display: flex;
            flex-grow: 1;
        }
        .checkbox {
            display: flex;
            border-radius: 50%;
            height: var(--size);
            width: var(--size);
            background-color: var(--foreground);
            border: 2px solid black;
        }
        .checked {

        }
        .value {
            height: var(--size);
            width: var(--size);
            font-size: calc(var(--size) / 2);
            line-height: 2;
            border-bottom: 2px solid black;
        }
        .label {
            display: flex;
            flex-grow: 1;
            justify-content: flex-start;
            align-items: center;
            font-size: calc(var(--size) / 2.5);
            padding: 4px;
        }
    ");

    html! {
        <div class={style} style={vars}>
            <div class="container">
                <div class="label">{ &props.label }</div>
                <div class="value">{ &props.value }</div>
                <div class="checkbox">{ &props.checked }</div>
            </div>
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
struct SkillProps {
    name: String,
    proficiency: bool,
    parent: AbilityProps,
}

#[derive(Clone, Properties, PartialEq)]
struct SkillListProps {
    character: CharacterProps,
    acrobatics: SkillProps,
    animalhandling: SkillProps,
    arcana: SkillProps,
    athletics: SkillProps,
    deception: SkillProps,
    history: SkillProps,
    insight: SkillProps,
    intimidation: SkillProps,
    investigation: SkillProps,
    medicine: SkillProps,
    nature: SkillProps,
    perception: SkillProps,
    performance: SkillProps,
    persuasion: SkillProps,
    religion: SkillProps,
    sleightofhand: SkillProps,
    stealth: SkillProps,
    survival: SkillProps,
}

#[function_component(Skills)]
fn skills(props: &SkillListProps) -> Html {

    let modifier = |skill: SkillProps| -> i8 {
        let base_modifier = calc_base_modifier(skill.parent.value);
        // let log_string = format!("{}: base:{}", skill.name, &base_modifier.to_string());
        // log(&log_string);
        if skill.proficiency {
            // log(&calc_proficiency_bonus(props.character.level).to_string());
            base_modifier + calc_proficiency_bonus(props.character.level)
        } else { base_modifier }
    };

    let style = use_style!("
        display: flex;
        flex-direction: row;
        margin: 4px;
        border: 2px solid black;

        .column {
            display: flex;
            flex-direction: column;
            margin: 4px;
            border: 2px solid black;
        }
    ");

    html! {
        <div class={style}>
            <div class="column">
                <LabeledValueCheckbox checked={props.acrobatics.proficiency.clone()} value={modifier(props.acrobatics.clone())} label={props.acrobatics.name.clone()} />
                <LabeledValueCheckbox checked={props.animalhandling.proficiency.clone()} value={modifier(props.animalhandling.clone())} label={props.animalhandling.name.clone()} />
                <LabeledValueCheckbox checked={props.arcana.proficiency.clone()} value={modifier(props.arcana.clone())} label={props.arcana.name.clone()} />
                <LabeledValueCheckbox checked={props.athletics.proficiency.clone()} value={modifier(props.athletics.clone())} label={props.athletics.name.clone()} />
                <LabeledValueCheckbox checked={props.deception.proficiency.clone()} value={modifier(props.deception.clone())} label={props.deception.name.clone()} />
                <LabeledValueCheckbox checked={props.history.proficiency.clone()} value={modifier(props.history.clone())} label={props.history.name.clone()} />
                <LabeledValueCheckbox checked={props.insight.proficiency.clone()} value={modifier(props.insight.clone())} label={props.insight.name.clone()} />
                <LabeledValueCheckbox checked={props.intimidation.proficiency.clone()} value={modifier(props.intimidation.clone())} label={props.intimidation.name.clone()} />
                <LabeledValueCheckbox checked={props.investigation.proficiency.clone()} value={modifier(props.investigation.clone())} label={props.investigation.name.clone()} />
            </div>
            <div class="column">
                <LabeledValueCheckbox checked={props.medicine.proficiency.clone()} value={modifier(props.medicine.clone())} label={props.medicine.name.clone()} />
                <LabeledValueCheckbox checked={props.nature.proficiency.clone()} value={modifier(props.nature.clone())} label={props.nature.name.clone()} />
                <LabeledValueCheckbox checked={props.perception.proficiency.clone()} value={modifier(props.perception.clone())} label={props.perception.name.clone()} />
                <LabeledValueCheckbox checked={props.performance.proficiency.clone()} value={modifier(props.performance.clone())} label={props.performance.name.clone()} />
                <LabeledValueCheckbox checked={props.persuasion.proficiency.clone()} value={modifier(props.persuasion.clone())} label={props.persuasion.name.clone()} />
                <LabeledValueCheckbox checked={props.religion.proficiency.clone()} value={modifier(props.religion.clone())} label={props.religion.name.clone()} />
                <LabeledValueCheckbox checked={props.sleightofhand.proficiency.clone()} value={modifier(props.sleightofhand.clone())} label={props.sleightofhand.name.clone()} />
                <LabeledValueCheckbox checked={props.stealth.proficiency.clone()} value={modifier(props.stealth.clone())} label={props.stealth.name.clone()} />
                <LabeledValueCheckbox checked={props.survival.proficiency.clone()} value={modifier(props.survival.clone())} label={props.survival.name.clone()} />
            </div>
        </div>
    }
}

#[function_component]
fn App() -> Html {
    let _global_style = GlobalStyle::new(format!("
        background: {};
        font-family: {};
        "
        , BACKGROUND
        , FONT
    ));

    let primary_abilities = PrimaryAbilitiesProps {
        strength: AbilityProps {
            name: "Strength".to_string(),
            value: 8,
        },
        dexterity: AbilityProps {
            name: "Dexterity".to_string(),
            value: 13,
        },
        constitution: AbilityProps {
            name: "Constitution".to_string(),
            value: 15,
        },
        intelligence: AbilityProps {
            name: "Intelligence".to_string(),
            value: 19,
        },
        wisdom: AbilityProps {
            name: "Wisdom".to_string(),
            value: 12,
        },
        charisma: AbilityProps {
            name: "Charisma".to_string(),
            value: 10,
        },
    };

    let character = CharacterProps {
        level: 5,
    };

    let skills = SkillListProps {
        character: character.clone(),
        acrobatics: SkillProps {
            name: "Acrobatics (Dex)".to_string(),
            proficiency: false,
            parent: primary_abilities.dexterity.clone(),
        },
        animalhandling: SkillProps {
            name: "Animal Handling (Wis)".to_string(),
            proficiency: false,
            parent: primary_abilities.wisdom.clone(),
        },
        arcana: SkillProps {
            name: "Arcana (Int)".to_string(),
            proficiency: true,
            parent: primary_abilities.intelligence.clone(),
        },
        athletics: SkillProps {
            name: "Athletics (Str)".to_string(),
            proficiency: false,
            parent: primary_abilities.strength.clone(),
        },
        deception: SkillProps {
            name: "Deception (Cha)".to_string(),
            proficiency: false,
            parent: primary_abilities.charisma.clone(),
        },
        history: SkillProps {
            name: "History (Int)".to_string(),
            proficiency: false,
            parent: primary_abilities.intelligence.clone(),
        },
        insight: SkillProps {
            name: "Insight (Wis)".to_string(),
            proficiency: true,
            parent: primary_abilities.wisdom.clone(),
        },
        intimidation: SkillProps {
            name: "Intimidation (Cha)".to_string(),
            proficiency: false,
            parent: primary_abilities.charisma.clone(),
        },
        investigation: SkillProps {
            name: "Investigation (Int)".to_string(),
            proficiency: false,
            parent: primary_abilities.intelligence.clone(),
        },
        medicine: SkillProps {
            name: "Medicine (Wis)".to_string(),
            proficiency: false,
            parent: primary_abilities.wisdom.clone(),
        },
        nature: SkillProps {
            name: "Nature (Int)".to_string(),
            proficiency: true,
            parent: primary_abilities.intelligence.clone(),
        },
        perception: SkillProps {
            name: "Perception (Wis)".to_string(),
            proficiency: false,
            parent: primary_abilities.wisdom.clone(),
        },
        performance: SkillProps {
            name: "Performance (Cha)".to_string(),
            proficiency: false,
            parent: primary_abilities.charisma.clone(),
        },
        persuasion: SkillProps {
            name: "Persuasion (Cha)".to_string(),
            proficiency: true,
            parent: primary_abilities.charisma.clone(),
        },
        religion: SkillProps {
            name: "Religion (Int)".to_string(),
            proficiency: false,
            parent: primary_abilities.intelligence.clone(),
        },
        sleightofhand: SkillProps {
            name: "Sleight of Hand (Dex)".to_string(),
            proficiency: false,
            parent: primary_abilities.dexterity.clone(),
        },
        stealth: SkillProps {
            name: "Stealth (Dex)".to_string(),
            proficiency: false,
            parent: primary_abilities.dexterity.clone(),
        },
        survival: SkillProps {
            name: "Survival (Wis)".to_string(),
            proficiency: false,
            parent: primary_abilities.wisdom.clone(),
        },
    };

    let style = use_style!("
        display: flex;
        flex-direction: column;
        flex-grow: 1;

        .row {
            display: flex;
            flex-direction: row;
            flex-grow: 1;
        }
    ");

    html! {
        <div class={style}>
            <div class="row">
                <ProficiencyBonus level={character.level} />
                <PassiveAbilities 
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
                    character={character.clone()}
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

