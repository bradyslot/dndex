use stylist::yew::use_style;
use yew::prelude::*;
use super::shared::models::*;
use super::rectangle::*;
use super::shared::utils::*;

#[function_component(AbilityScore)]
pub fn ability_score(props: &Ability) -> Html {
    // grid-area: (row start) / (column start) / (row end) / (column end)
    let style = use_style!(
        r#"
            display: grid;
            padding: 0.5rem;
            grid-template-rows: 1fr;
            width: 15rem;
            height: 15rem;

            .grid-upper {
                grid-area: 1 / 1 / 3 / 1;
            }

            .grid-lower {
                grid-area: 2 / 1 / 4 / 1;
                z-index: 1;
                margin-left: calc(50% - 2rem);
            }

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
                height: 100%;
            }

            .circle {
                font-size: 2rem;
                width: 4rem;
                height: 4rem;
                border-radius: 50%;
                border: 2px solid black;
                background-color: var(--foreground);
                line-height: 4rem;
            }
        "#
    );

    html! {
        <div class={style}>
            <div class="grid-upper">
                <Rectangle>
                    <div class="absolute center label">{ &props.name }</div>
                    <div class="absolute center modifier">{ calc_base_modifier(props.value) }</div>
                </Rectangle>
            </div>
            <div class="grid-lower">
                <div class="center circle">{ &props.value }</div>
            </div>
        </div>
    }
}

