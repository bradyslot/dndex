use stylist::yew::use_style;
use stylist::GlobalStyle;
use yew::prelude::*;

const FOREGROUND: &str = "#F8F8F8";
const BACKGROUND: &str = "#F8F8F8";
const FONT: &str = "Serif";

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
struct PrimaryAbilitiesProps {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
}

#[function_component(PrimaryAbilities)]
fn primary_abilities(props: &PrimaryAbilitiesProps) -> Html {
    let style = use_style!("
        display: flex;
        flex-direction: row;
        flex-grow: 1;
    ");

    html! {
        <div class={style}>
            <AbilityScore value={props.strength} name={"Strength"} />
            <AbilityScore value={props.dexterity} name={"Dexterity"} />
            <AbilityScore value={props.constitution} name={"Constitution"} />
            <AbilityScore value={props.intelligence} name={"Intelligence"} />
            <AbilityScore value={props.wisdom} name={"Wisdom"} />
            <AbilityScore value={props.charisma} name={"Charisma"} />
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
struct CharacterLevelProps {
    level: u8,
}

#[function_component(ProficiencyBonus)]
fn proficiency_bonus(props: &CharacterLevelProps) -> Html {
    let label = format!("Proficiency Bonus");

    // let style = use_style!("
    //     display: flex;
    //     flex-grow: 1;
    // ");

    html! {
        // <div class={style}>
        <LabeledValue value={2 + (props.level as i8 - 1) / 4} label={label} />
        // </div>
    }
}

#[function_component(PassiveAbilities)]
fn passive_abilities(props: &PrimaryAbilitiesProps) -> Html {
    fn modifier(ability: u8) -> i8 {
        (ability as i8 - 10) / 2
    }

    html! {
        <>
            <LabeledValue value={10 + modifier(props.wisdom)} label={"Perception (Passive)"} />
            <LabeledValue value={10 + modifier(props.intelligence)} label={"Investigation (Passive)"} />
            <LabeledValue value={10 + modifier(props.wisdom)} label={"Insight (Passive)"} />
            // <LabeledValue value={10 + modifier(props.intelligence)} label={"Arcana (Passive)"} />
            // <LabeledValue value={10 + modifier(props.intelligence)} label={"History (Passive)"} />
            // <LabeledValue value={10 + modifier(props.wisdom)} label={"Religion (Passive)"} />
            // <LabeledValue value={10 + modifier(props.wisdom)} label={"Nature (Passive)"} />
            // <LabeledValue value={10 + modifier(props.wisdom)} label={"Survival (Passive)"} />
        </>
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
        strength: 8,
        dexterity: 13,
        constitution: 15,
        intelligence: 19,
        wisdom: 12,
        charisma: 10,
    };

    let character_level = CharacterLevelProps {
        level: 5,
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
                <ProficiencyBonus level={character_level.level} />
                <PassiveAbilities 
                    strength={primary_abilities.strength}
                    dexterity={primary_abilities.dexterity}
                    constitution={primary_abilities.constitution}
                    intelligence={primary_abilities.intelligence}
                    wisdom={primary_abilities.wisdom}
                    charisma={primary_abilities.charisma}
                />
            </div>
            <div class="row">
                <PrimaryAbilities
                    strength={primary_abilities.strength}
                    dexterity={primary_abilities.dexterity}
                    constitution={primary_abilities.constitution}
                    intelligence={primary_abilities.intelligence}
                    wisdom={primary_abilities.wisdom}
                    charisma={primary_abilities.charisma}
                />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

