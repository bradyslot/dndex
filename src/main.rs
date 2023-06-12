use stylist::yew::use_style;
use stylist::GlobalStyle;
use yew::prelude::*;

const FOREGROUND: &str = "#F8F8F8";
const BACKGROUND: &str = "#C8C8C8";

#[derive(Clone, Properties, PartialEq)]
struct Dimensions {
    height: u16,
    width: u16,
}

#[function_component(BittenRectangle)]
fn bitten_rectangle(props: &Dimensions) -> Html {
    let vars = format!("
        --height: {}px;
        --width: {}px;
        --background: {};
        --foreground: {};
        "
        , props.height
        , props.width
        , BACKGROUND
        , FOREGROUND
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
    ability: u8,
}

#[function_component(AbilityScore)]
fn ability_score(props: &AbilityScoreProps) -> Html {
    let size = &Dimensions { height: 100, width: 100 };
    let vars = format!("
        --height: {}px;
        --width: {}px;
        --foreground: {};
        "
        , size.height
        , size.width
        , FOREGROUND
    );
    let ability = use_state(|| {props.ability});
    let modifier = props.ability as i8;
    let modifier = use_state(|| {(modifier - 10) / 2});

    // let onclick = {
    //     let ability_score = ability_score.clone();
    //     move |_| {
    //         let value = *ability_score + 1;
    //         ability_score.set(value);
    //     }
    // };

    let style = use_style!("
        display: flex;
        justify-content: center;
        margin: 4px;

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
            font-family: serif;
            line-height: 1;
        }

        .ability {
            top: 0;
            left: 0;
            height: var(--height);
            width: var(--width);
            font-size: calc(var(--height) / 2);
        }

        .modifier {
            bottom: -20%;
            left: 27.5%;
            height: 40%;
            width: 40%;
            border-radius: 100%;
            background-color: var(--foreground);
            border: 2px solid black;
            font-size: calc(var(--height) / 6);
        }

        .overlay {
            z-index: 9;
        }
    ");

    html! {
        <div class={style} style={vars}>
            <div class="container">
                <BittenRectangle height={size.height} width={size.width} />
                <div class="text ability overlay">{ *ability }</div>
                <div class="text modifier overlay">{ *modifier }</div>
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
    ");

    html! {
        <div class={style}>
            <AbilityScore ability={props.strength} />
            <AbilityScore ability={props.dexterity} />
            <AbilityScore ability={props.constitution} />
            <AbilityScore ability={props.intelligence} />
            <AbilityScore ability={props.wisdom} />
            <AbilityScore ability={props.charisma} />
        </div>
    }
}

#[function_component]
fn App() -> Html {
    let _global_style = GlobalStyle::new(format!("background: {};", BACKGROUND));

    let primary_abilities = PrimaryAbilitiesProps {
        strength: 8,
        dexterity: 13,
        constitution: 15,
        intelligence: 19,
        wisdom: 12,
        charisma: 10,
    };

    html! {
        <div>
            <PrimaryAbilities
                strength={primary_abilities.strength}
                dexterity={primary_abilities.dexterity}
                constitution={primary_abilities.constitution}
                intelligence={primary_abilities.intelligence}
                wisdom={primary_abilities.wisdom}
                charisma={primary_abilities.charisma}
            />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

