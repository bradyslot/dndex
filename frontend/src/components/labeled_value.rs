use stylist::yew::use_style;
use yew::prelude::*;
use super::shared::models::*;

#[function_component(LabeledValue)]
pub fn labeled_value(props: &LabeledValueProps) -> Html {
    let style = use_style!(
        r#"
            display: flex;
            flex-direction: row;
            flex-grow: 1;
            justify-content: flex-start;
            text-align: center;
            padding: 4px;
            --size: 5rem;

            .container {
                display: flex;
                flex-grow: 1;
            }
            .circle {
                display: flex;
                border-radius: 50%;
                height: var(--size);
                width: var(--size);
                background-color: var(--foreground);
                border: 2px solid black;
            }
            .value {
                height: var(--size);
                width: var(--size);
                font-size: calc(var(--size) / 2);
                line-height: 2;
            }
            .label {
                display: flex;
                flex-grow: 1;
                justify-content: flex-start;
                align-items: center;
                font-size: calc(var(--size) / 2.5);
                padding: 4px;
            }
        "#
    );

    html! {
        <div class={style}>
            <div class="container">
                <div class="circle">
                    <div class="value">{ &props.value }</div>
                </div>
                <div class="label">{ &props.label }</div>
            </div>
        </div>
    }
}

