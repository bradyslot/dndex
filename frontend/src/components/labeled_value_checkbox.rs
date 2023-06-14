use stylist::yew::use_style;
use yew::prelude::*;
use super::shared::models::*;
use super::super::constants::*;

#[function_component(LabeledValueCheckbox)]
pub fn labeled_value_checkbox(props: &LabeledValueCheckboxProps) -> Html {
    let vars = format!(r#"
        --foreground: {};
        --size: 4rem; "#
        ,
        FOREGROUND
    );
    let style = use_style!(
        r#"
            display: flex;
            flex-direction: row;
            flex-grow: 1;
            justify-content: flex-start;
            text-align: center;

            .container {
                display: flex;
                flex-grow: 1;
            }
            .checkbox {
                display: flex;
                border-radius: 50%;
                height: calc(var(--size) / 1.5);
                width: calc(var(--size) / 1.5);
                background-color: var(--foreground);
                border: 2px solid black;
                margin: 1rem;
            }
            .checked {
                background-color: black;
            }
            .value {
                height: var(--size);
                width: var(--size);
                font-size: calc(var(--size) / 2);
                line-height: 2;
                border-bottom: 2px solid black;
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

    let checkbox_class = if props.checked {
        classes!("checkbox", "checked")
    } else {
        classes!("checkbox")
    };

    html! {
        <div class={style} style={vars}>
            <div class="container">
                <div class="label">{ &props.label }</div>
                <div class="value">{ &props.value }</div>
                <div class={checkbox_class} />
            </div>
        </div>
    }
}

