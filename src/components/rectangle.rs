use stylist::css;
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;

#[function_component(Rectangle)]
pub fn rectangle(props: &Child) -> Html {
    let s = random_alpha_string(8);
    let style = css!(
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

        .radius-${s} {
            position: absolute;
            width: 1rem;
            height: 1rem;
            border-radius: 100%;
            background-color: var(--background);
            border: 2px solid black;
        }

        .top-${s} { top: -0.5rem; }
        .bottom-${s} { bottom: -0.5rem; }
        .left-${s} { left: -0.5rem; }
        .right-${s} { right: -0.5rem; }

        .children-${s} {
            height: 100%;
            width: 100%;
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
            <div class={format!("radius-{} top-{} left-{}", s, s, s)} />
            <div class={format!("radius-{} top-{} right-{}", s, s, s)} />
            <div class={format!("radius-{} bottom-{} left-{}", s, s, s)} />
            <div class={format!("radius-{} bottom-{} right-{}", s, s, s)} />
            <div class={format!("label-{} flex-center-{}", s, s)}>
                { props.label.clone() }
            </div>
            <div class={format!("children-{}", s)}>
                { for props.children.iter() }
            </div>
        </div>
    }
}

