use stylist::{css, Style};
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;
use super::shared::icons::*;

#[function_component(LabeledValueCheckbox)]
pub fn labeled_value_checkbox(props: &LabeledValueCheckboxProps) -> Html {
    let s = random_alpha_string(8);
    let css = css!(
        display: flex;
        flex-direction: row;
        flex-grow: 1;
        justify-content: flex-start;
        text-align: center;
        --size: 4rem;

        .container-${s} {
            display: flex;
            flex-grow: 1;
        }

        .value-${s} {
            height: var(--size);
            width: var(--size);
            font-size: calc(var(--size) / 2);
            border-bottom: 2px solid black;
            line-height: 2.5;
        }

        .label-${s} {
            display: flex;
            flex-grow: 1;
            justify-content: flex-start;
            align-items: end;
            font-size: calc(var(--size) / 2.5);
            padding: 4px;
        }

        .svg-${s} {
            display: block;
            margin: 1rem;
            width: 2rem;
            height: 2rem;
        }
    );
    let style = Style::new(css).expect("Failed to create style");

    html! {
        <div class={style}>
            <div class={format!("container-{}", s)}>
                <div class={format!("label-{}", s)}>{ &props.label }</div>
                <div class={format!("value-{}", s)}>{ &props.value }</div>
                <div class={format!("svg-{}", s)}>{icon_checkbox(props.checked)}</div>
            </div>
        </div>
    }
}

