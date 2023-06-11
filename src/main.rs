use stylist::yew::use_style;
use stylist::global_style;
use yew::prelude::*;

#[function_component]
fn AbilityScore() -> Html {
    let ability_score = use_state(|| {let x: u8 = 0; x});
    let ability_modifier = use_state(|| {let x: i8 = 0; x});
    // let onclick = {
    //     let ability_score = ability_score.clone();
    //     move |_| {
    //         let value = *ability_score + 1;
    //         ability_score.set(value);
    //     }
    // };

    let container_style = use_style!("
        border: 2px solid black;
        border-radius: 10px;
        height: 100px;
        width: 100px;
        justify-content: center;
        align-items: center;
    ");

    let ability_score_style = use_style!("
        display: flex;
        justify-content: center;
        align-items: center;
        font-size: 50px;
        font-align: center;
        font-family: serif;
    ");

    let ability_modifier_style = use_style!("
        display: flex;
        justify-content: center;
        align-items: center;
        position: relative;
        bottom: 0px;
        left: 25px;
        border: 2px solid black;
        border-radius: 20px;
        height: 50px;
        width: 50px;
        font-size: 30px;
        font-align: center;
        font-family: serif;
        background: #F8F8F8;
    ");

    html! {
        <div class={container_style}>
            <div class={ability_score_style}>{ *ability_score }</div>
            <div class={ability_modifier_style}>{ *ability_modifier }</div>
        </div>
    }
}

#[function_component]
fn App() -> Html {
    let _global_syle = global_style!("background: #F8F8F8;");

    html! {
        <div>
            <AbilityScore />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

