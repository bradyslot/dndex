use stylist::yew::use_style;
use stylist::global_style;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
struct Dimensions {
    height: u16,
    width: u16,
}

#[function_component(BittenRectangle)]
fn bitten_rectangle(props: &Dimensions) -> Html {
    let size = format!("--height: {}px; --width: {}px;", props.height, props.width);

    let style = use_style!("
        display: flex;
        height: var(--height);
        width: var(--width);
        background-color: #F8F8F8;
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
            background-color: #C8C8C8;
            border: 2px solid black;
        }

        .top { top: -10px; }
        .bottom { bottom: -10px; }
        .left { left: -10px; }
        .right { right: -10px; }
    ");

    html! {
        <div class={style} style={size}>
            <div class="border" />
            <div class="radius top left" />
            <div class="radius top right" />
            <div class="radius bottom left" />
            <div class="radius bottom right" />
        </div>
    }
}

#[function_component(AbilityScore)]
fn ability_score() -> Html {
    let _ability_score = use_state(|| {let x: u8 = 0; x});
    let _ability_modifier = use_state(|| {let x: i8 = 0; x});
    let props = Dimensions { height: 100, width: 100 };

    // let onclick = {
    //     let ability_score = ability_score.clone();
    //     move |_| {
    //         let value = *ability_score + 1;
    //         ability_score.set(value);
    //     }
    // };

    html! {
        <div>
            <BittenRectangle height={props.height} width={props.width} />
            // <div class={ability_score_style}>{ *ability_score }</div>
            // <div class={modifier_style}>{ *ability_modifier }</div>
        </div>
    }
}

#[function_component]
fn App() -> Html {
    let _global_syle = global_style!("background: #C8C8C8;");

    html! {
        <div>
            <AbilityScore />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

