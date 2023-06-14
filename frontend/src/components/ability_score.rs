use stylist::yew::use_style;
use yew::prelude::*;
use super::shared::models::*;
use super::rectangle::*;
use super::super::constants::*;

#[function_component(AbilityScore)]
pub fn ability_score(props: &AbilityScoreProps) -> Html {
    let size = &Dimensions { height: "15rem".into(), width: "15rem".into() };
    let vars = format!(r#"
        --height: {}; --width: {};
        --foreground: {}; "#
        , size.height , size.width
        , FOREGROUND
    );
    // let ability = use_state(|| {props.value});
    let modifier = props.value as i8;
    let modifier = use_state(|| {(modifier - 10) / 2});

    let style = use_style!(
        r#"
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
        "#
    );

    html! {
        <div class={style} style={vars}>
            <div class="container">
                <Rectangle height={size.height.clone()} width={size.width.clone()} />
                <div class="text modifier overlay">{ *modifier }</div>
                <div class="text ability overlay">{ &props.value }</div>
                <div class="label text overlay">{ &props.name }</div>
            </div>
        </div>
    }
}

