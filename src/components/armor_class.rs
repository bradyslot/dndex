use stylist::css;
use yew::prelude::*;
use super::super::models::models::*;
use super::shared::utils::*;
use super::rectangle_notched::*;

#[function_component(ArmorClass)]
pub fn armor_class(props: &Character) -> Html {
    let s = random_alpha_string(8);
    let style = css!(
        display: flex;

        .grid-${s} {
            grid-template-areas: "rectangle"
                                 "modifier";
        }
        .rectangle-${s} {
            grid-area: rectangle;
            height: 10rem;
            width: 10rem;
            font-size: 3rem;
        }
        .base-${s} {
            font-size: 3rem;
            width: 100%;
            height: 100%;
            display: flex;
            justify-content: center;
            align-items: center;
        }
        .modifier-${s} {
            display: flex;
            justify-content: center;
            align-items: center;
            grid-area: modifier;
            height: 4rem;
            width: 70%;
            font-size: 2rem;
            border: 2px solid black;
            margin: auto;
            transform: translateY(-50%);
            background-color: var(--foreground);
        }
    );

    html! {
        <div class={style}>
            <div class={format!("grid-{}", s)}>
                <div class={format!("rectangle-{}", s)}>
                    <RectangleNotched label={"AC"}>
                        <div class={format!("base-{}", s)}>{props.ac.base}</div>
                    </RectangleNotched>
                </div>
                <div class={format!("modifier-{}", s)}>{props.ac.modifier}</div>
            </div>
        </div>
    }
}

