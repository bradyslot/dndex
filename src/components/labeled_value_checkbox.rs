use stylist::css;
use yew::prelude::*;
use super::super::models::models::*;
use super::shared::utils::*;
use super::shared::icons::*;

#[function_component(LabeledValueCheckbox)]
pub fn labeled_value_checkbox(props: &LabeledValueCheckboxProps) -> Html {
    let s = random_alpha_string(8);
    let style = css!(
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
            height: 4rem;
            width: 4rem;
            font-size: 2rem;
            border-bottom: 2px solid black;
            line-height: 2.5;
        }

        .label-${s} {
            display: flex;
            flex-grow: 1;
            justify-content: flex-start;
            align-items: end;
            font-size: 1.5rem;
            transform: translate(0, -0.5rem);
        }

        .svg-${s} {
            display: block;
            margin: auto;
            width: 2rem;
            height: 2rem;
            transform: translate(0, 0.25rem);
        }
    );

    html! {
        <div class={style}>
            <div class={format!("container-{}", s)}>
                <div class={format!("label-{}", s)}>{ &props.label }</div>
                <div class={format!("value-{}", s)}>{ &props.value }</div>
                <div class={format!("svg-{}", s)}>{icon_shield(props.checked)}</div>
            </div>
        </div>
    }
}

