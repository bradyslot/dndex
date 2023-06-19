use stylist::{css, Style};
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;

#[function_component(LabeledValue)]
pub fn labeled_value(props: &LabeledValueProps) -> Html {
    let s = random_alpha_string(8);
    let css = css!(
        display: flex;
        flex-direction: row;
        padding: 0.5rem;

        .container-${s} {
            display: flex;
            flex-grow: 1;
        }
        .flex-center-${s} {
            justify-content: center;
            align-items: center;
            text-align: center;
        }
        .circle-${s} {
            display: flex;
            border-radius: 50%;
            height: 4rem;
            width: 4rem;
            background-color: var(--foreground);
            border: 2px solid black;
        }
        .value-${s} {
            display: flex;
            height: 4rem;
            width: 4rem;
            font-size: 2rem;
            line-height: 2;
        }
        .label-${s} {
            display: flex;
            font-size: 2rem;
            margin-left: 1rem;
        }
    );
    let style = Style::new(css).expect("Failed to create style");

    html! {
        <div class={style}>
            <div class={format!("container-{}", s)}>
                <div class={format!("circle-{}", s)}>
                    <div class={format!("flex-center-{} value-{}", s, s)}>{ &props.value }</div>
                </div>
                <div class={format!("label-{}", s)}>{ &props.label }</div>
            </div>
        </div>
    }
}

