use stylist::css;
use yew::prelude::*;
use super::shared::models::*;
use super::rectangle_notched::*;

#[function_component(ArmorClass)]
pub fn armor_class(props: &Character) -> Html {
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
        <RectangleNotched label={"AC"}>
            <div class={style}>{props.ac}</div>
        </RectangleNotched>
    }
}

