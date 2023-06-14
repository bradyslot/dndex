use stylist::yew::use_style;
use yew::prelude::*;
use super::shared::models::*;
use super::super::constants::*;

#[function_component(Rectangle)]
pub fn rectangle(props: &Dimensions) -> Html {
    let vars = format!(r#"
        --height: {}; --width: {};
        --background: {}; --foreground: {}; "#
        , props.height , props.width
        , BACKGROUND , FOREGROUND
    );

    let style = use_style!(
        r#"
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
        "#
    );

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

