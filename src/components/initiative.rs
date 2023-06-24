use stylist::css;
use yew::prelude::*;
use super::super::models::models::*;
use super::rectangle_scooped::*;

#[function_component(Initiative)]
pub fn initiative(props: &Character) -> Html {
    let style = css!(
        display: flex;
        flex-grow: 1;
        text-align: center;
        justify-content: center;
        align-items: center;
        min-height: 10rem;
        min-width: 10rem;
        font-size: 3rem;
    );

    html! {
        <RectangleScooped label={"Initiative"}>
            <div class={style}>{props.initiative}</div>
        </RectangleScooped>
    }
}

