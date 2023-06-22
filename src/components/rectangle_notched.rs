use stylist::css;
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;

#[function_component(RectangleNotched)]
pub fn rectangle_notched(props: &Child) -> Html {
    let s = random_alpha_string(8);
    let style = css!(
        display: flex;
        background-color: var(--foreground);
        position: relative;
        overflow: hidden;
        width: 100%;
        height: 100%;

        .border-${s} {
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            border: 2px solid black;
        }

        .notch-${s} {
            content: "";
            position: absolute;
            margin: -2px;
            width: 2rem;
            height: 2rem;
            background-color: var(--background);
            transform: rotate(45deg);
            border: 2px solid black;
        }

        .top-${s} { top: -1rem; }
        .bottom-${s} { bottom: -1rem; }
        .left-${s} { left: -1rem; }
        .right-${s} { right: -1rem; }

        .children-${s} {
            display: flex;
            flex-grow: 1;
        }

        .label-${s} {
            position: absolute;
            top: 0.5rem;
            width: 100%;
            font-size: 1.5rem;
            line-height: 2rem;
        }

        .flex-center-${s} {
            display: flex;
            justify-content: center;
            align-items: center;
            text-align: center;
        }
    );

    html! {
        <div class={style}>
            <div class={format!("border-{}", s)} />
            <div class={format!("notch-{} top-{} left-{}", s, s, s)} />
            <div class={format!("notch-{} top-{} right-{}", s, s, s)} />
            <div class={format!("notch-{} bottom-{} left-{}", s, s, s)} />
            <div class={format!("notch-{} bottom-{} right-{}", s, s, s)} />
            <div class={format!("label-{} flex-center-{}", s, s)}>
                { props.label.clone() }
            </div>
            <div class={format!("children-{}", s)}>
                { props.children.clone().unwrap_or_default() } 
            </div>
        </div>
    }
}

