use stylist::css;
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;
use super::shared::icons::*;
use super::rectangle::*;

#[function_component(DeathSaveRolls)]
pub fn death_save_rolls(props: &Character) -> Html {
    // grid-area: (row start) / (column start) / (row end) / (column end)
    let s = random_alpha_string(8);
    let style = css!(
        display: flex;
        flex-grow: 1;
        font-size: 1.5rem;

        .grid-${s} {
            display: grid;
            grid-template-columns: repeat(4, auto);
            height: 100%;
            width: 100%;
        }

        .absolute-${s} {
            display: flex;
            position: absolute;
            width: 100%;
        }

        .flex-center-${s} {
            display: flex;
            text-align: center;
            justify-content: center;
            align-items: center;
        }

        .svg-${s} {
            display: block;
            margin: auto;
            width: 2rem;
            height: 2rem;
        }
    );

    html! {
        <div class={style}>
            <Rectangle label={"Death Saves"}>
                <div class={format!("grid-{}", s)}>
                    <div class={format!("flex-center-{}", s)}>{"Success: "}</div>
                    <div class={format!("flex-center-{} svg-{}", s, s)}>{icon_heart(props.deathsaves.success[0])}</div>
                    <div class={format!("flex-center-{} svg-{}", s, s)}>{icon_heart(props.deathsaves.success[1])}</div>
                    <div class={format!("flex-center-{} svg-{}", s, s)}>{icon_heart(props.deathsaves.success[2])}</div>
                    <div class={format!("flex-center-{}", s)}>{"Failure: "}</div>
                    <div class={format!("flex-center-{} svg-{}", s, s)}>{icon_skull(props.deathsaves.failure[0])}</div>
                    <div class={format!("flex-center-{} svg-{}", s, s)}>{icon_skull(props.deathsaves.failure[1])}</div>
                    <div class={format!("flex-center-{} svg-{}", s, s)}>{icon_skull(props.deathsaves.failure[2])}</div>
                </div>
            </Rectangle>
        </div>
    }
}

