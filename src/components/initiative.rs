use stylist::css;
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;
use super::rectangle::*;

#[function_component(Initiative)]
pub fn initiative(props: &Character) -> Html {
    let s = random_alpha_string(8);
    let style = css!(
        display: flex;
        height: 10rem;
        width: 10rem;

        .flex-center-${s} {
            display: flex;
            text-align: center;
            justify-content: center;
            align-items: center;
            font-size: 3rem;
            height: 100%;
            width: 100%;
        }
    );

    html! {
        <div class={style}>
            <Rectangle label={"Initiative"}>
                <div class={format!("flex-center-{}", s)}>{props.initiative}</div>
            </Rectangle>
        </div>
    }
}

