use stylist::css;
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;
use super::shared::icons::*;
use super::rectangle_scooped::*;

#[function_component(DeathSaveRolls)]
pub fn death_save_rolls(props: &Character) -> Html {
    // grid-area: (row start) / (column start) / (row end) / (column end)
    let s = random_alpha_string(8);
    let style = css!(
        display: flex;
        flex-grow: 1;
        font-size: 1.5rem;
        min-height: 10rem;
        min-width: 10rem;

        .grid-${s} {
            margin-top: 2rem;
            display: grid;
            grid-template-rows: 1fr 1fr;
            grid-template-columns: 1fr 1fr 1fr 1fr;
        }

        .text-${s} {
            display: flex;
            text-align: center;
            justify-content: flex-end;
            align-items: center;
            margin: 0.5rem;
        }

        .svg-${s} {
            display: block;
            margin: auto;
            width: 2rem;
            height: 2rem;
        }
    );

    html! {
        <RectangleScooped label={"Death Saves"}>
            <div class={style}>
                <div class={format!("grid-{}", s)}>
                    <div class={format!("text-{}", s)}>{"Success: "}</div>
                    <div class={format!("svg-{}", s)}>{icon_heart(props.deathsaves.success[0])}</div>
                    <div class={format!("svg-{}", s)}>{icon_heart(props.deathsaves.success[1])}</div>
                    <div class={format!("svg-{}", s)}>{icon_heart(props.deathsaves.success[2])}</div>
                    <div class={format!("text-{}", s)}>{"Failure: "}</div>
                    <div class={format!("svg-{}", s)}>{icon_skull(props.deathsaves.failure[0])}</div>
                    <div class={format!("svg-{}", s)}>{icon_skull(props.deathsaves.failure[1])}</div>
                    <div class={format!("svg-{}", s)}>{icon_skull(props.deathsaves.failure[2])}</div>
                </div>
            </div>
        </RectangleScooped>
    }
}

