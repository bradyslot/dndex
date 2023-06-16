use stylist::yew::use_style;
use yew::prelude::*;
use super::shared::models::*;
use super::rectangle::*;
use super::super::constants::*;
use super::shared::utils::*;

#[function_component(AbilityScore)]
pub fn ability_score(props: &AbilityScoreProps) -> Html {
    let vars = format!(r#"
        --foreground: {}; "#
        , FOREGROUND
    );

    let style = use_style!(
        r#"
            display: flex;
            padding-bottom: 2rem;
            height: 15rem;
            width: 15rem;

            .absolute {
                display: flex;
                position: absolute;
                width: 100%;
            }

            .center {
                justify-content: center;
                align-items: center;
                text-align: center;
            }

            .label {
                top: 0.5rem;
                font-size: 1.5rem;
            }

            .modifier {
                align-self: center;
                font-size: 5rem;
            }

            .score {
                bottom: -2rem;
            }

            .circle {
                transform: translate(0, -2rem);
                font-size: 2rem;
                width: 4rem;
                height: 4rem;
                border-radius: 50%;
                border: 2px solid black;
                background-color: var(--foreground);
            }
        "#
    );

    html! {
        <div class={style} style={vars}>
            <Rectangle>
                <div class="absolute center label">{ &props.name }</div>
                <div class="absolute center modifier">{ calc_base_modifier(props.value) }</div>
                <div class="absolute center score">
                    <div class="absolute center circle">{ &props.value }</div>
                </div>
            </Rectangle>
        </div>
    }
}

