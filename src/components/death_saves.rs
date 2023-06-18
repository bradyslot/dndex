use stylist::{css, Style};
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;
use super::shared::icons::*;
use super::rectangle::*;

#[function_component(DeathSaves)]
pub fn death_saves(props: &Character) -> Html {
    // grid-area: (row start) / (column start) / (row end) / (column end)
    let s = random_alpha_string(8);
    let css = css!(
        display: flex;
        flex-grow: 1;
        font-size: 1.5rem;

        .grid-${s} {
            margin-top: 1.5rem;
            display: grid;
            padding: 0.5rem;
            grid-template-columns: repeat(4, auto);
        }

        .absolute-${s} {
            display: flex;
            position: absolute;
            width: 100%;
        }

        .label-${s} {
            top: 0.5rem;
            font-size: 1.5rem;
        }

        .center-${s} {
            text-align: center;
            justify-content: center;
            align-items: center;
        }

        .svg-${s} {
            display: block;
            margin: auto;
            width: 1.5rem;
            height: 1.5rem;
        }
    );
    let style = Style::new(css).expect("css no good");

    html! {
        <div class={style}>
            <Rectangle>
                <div class={format!("absolute-{} center-{} label-{}", s, s, s)}>{"Death Saves"}</div>
                <div class={format!("grid-{}", s)}>
                    <div class={format!("center-{}", s)}>{"Success: "}</div>
                    <div class={format!("center-{} svg-{}", s, s)}>{icon_heart(props.saves.success[0])}</div>
                    <div class={format!("center-{} svg-{}", s, s)}>{icon_heart(props.saves.success[1])}</div>
                    <div class={format!("center-{} svg-{}", s, s)}>{icon_heart(props.saves.success[2])}</div>
                    <div class={format!("center-{}", s)}>{"Failure: "}</div>
                    <div class={format!("center-{} svg-{}", s, s)}>{icon_skull(props.saves.failure[0])}</div>
                    <div class={format!("center-{} svg-{}", s, s)}>{icon_skull(props.saves.failure[1])}</div>
                    <div class={format!("center-{} svg-{}", s, s)}>{icon_skull(props.saves.failure[2])}</div>
                </div>
            </Rectangle>
        </div>
    }
}

