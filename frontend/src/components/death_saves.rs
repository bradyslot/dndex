use stylist::yew::use_style;
use yew::prelude::*;
use super::shared::models::*;
// use super::shared::utils::*;
use super::rectangle::*;

#[function_component(DeathSaves)]
pub fn death_saves(props: &Character) -> Html {
    // grid-area: (row start) / (column start) / (row end) / (column end)
    let style = use_style!(
        r#"
            display: flex;
            flex-grow: 1;

            .grid {
                margin-top: 1.5rem;
                display: grid;
                padding: 0.5rem;
                grid-template-columns: repeat(4, 25%);
            }

            .absolute {
                display: flex;
                position: absolute;
                width: 100%;
            }

            .label {
                top: 0.5rem;
                font-size: 1.5rem;
            }

            .center {
                text-align: center;
                justify-content: center;
                align-items: center;
            }

        "#
    );

    html! {
        <div class={style}>
            <Rectangle>
                <div class="absolute center label">{"Death Saves"}</div>
                <div class="grid">
                    <div class="center">{"Success: "}</div>
                    <div class="center">{props.saves.success[0]}</div>
                    <div class="center">{props.saves.success[1]}</div>
                    <div class="center">{props.saves.success[2]}</div>
                    <div class="center">{"Failure: "}</div>
                    <div class="center">{props.saves.failure[0]}</div>
                    <div class="center">{props.saves.failure[1]}</div>
                    <div class="center">{props.saves.failure[2]}</div>
                </div>
            </Rectangle>
        </div>
    }
}

