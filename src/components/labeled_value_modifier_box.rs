use stylist::css;
use yew::prelude::*;
use super::super::models::models::*;
use super::shared::utils::*;

#[function_component(LabeledValueModiferBox)]
pub fn labeled_value_modifier_box(props: &LabeledValueModiferBoxProps) -> Html {
    // grid-area: (row start) / (column start) / (row end) / (column end)
    let s = random_alpha_string(8);
    let style = css!(
        display: flex;
        position: relative;
        justify-content: center;
        align-items: center;
        text-align: center;

        .grid-${s} {
            display: grid;
        }

        .upper-${s} {
            grid-area: 1 / 1 / 4 / 1;
            outline: 2px solid black;
            outline-offset: -2px;
            height: 10rem;
            width: 10rem;
        }

        .lower-${s} {
            grid-area: 3 / 1 / 5 / 1;
            border: 2px solid black;
            background-color: var(--foreground);
            width: 70%;
            height: 4rem;
            margin: auto;
        }
        
        .modifier-${s} {
            font-size: 2rem;
            background-color: var(--foreground);
        }

        .flex-center-${s} {
            display: flex;
            justify-content: center;
            align-items: center;
            text-align: center;
        }

        .label-${s} {
            position: absolute;
            top: 0.5rem;
            width: 10rem;
            font-size: 1.5rem;
            line-height: 2rem;
        }

        .middle-${s} {
            font-size: 3rem;
            width: 100%;
            height: 50%;
        }
    );

    html! {
        <div class={style}>
            <div class={format!("label-{}", s)}>{props.label.clone()}</div>
            <div class={format!("grid-{}", s)}>
                <div class={format!("flex-center-{} upper-{}", s, s)}>
                    { if props.value.is_some() {html! (<div class={format!("flex-center-{} middle-{}", s, s)}>{props.value}</div>)} else {html!()} }
                    { if props.text.is_some() {html! (<div class={format!("flex-center-{} middle-{}", s, s)}>{props.text.clone()}</div>)} else {html!()} }
                </div>
                <div class={format!("flex-center-{} lower-{}", s, s)}>
                    <div class={format!("flex-center-{} modifier-{}", s, s)}>{props.modifier}</div>
                </div>
            </div>
        </div>
    }
}
