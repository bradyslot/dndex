use stylist::css;
use stylist::Style;
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;

#[function_component(LabeledValueCheckbox)]
pub fn labeled_value_checkbox(props: &LabeledValueCheckboxProps) -> Html {
    let s = random_alpha_string(8);
    let css = css!(
        r#"
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
            .checkbox-${s} {
                display: flex;
                border-radius: 50%;
                height: calc(var(--size) / 1.5);
                width: calc(var(--size) / 1.5);
                background-color: var(--foreground);
                border: 2px solid black;
                margin: 1rem;
            }
            .checked-${s} {
                background-color: black;
            }
            .value-${s} {
                height: var(--size);
                width: var(--size);
                font-size: calc(var(--size) / 2);
                line-height: 2;
                border-bottom: 2px solid black;
            }
            .label-${s} {
                display: flex;
                flex-grow: 1;
                justify-content: flex-start;
                align-items: center;
                font-size: calc(var(--size) / 2.5);
                padding: 4px;
            }
        "#, s = s,
    );
    let style = Style::new(css).expect("Failed to create style");

    let checkbox = if props.checked {
        classes!(format!("checkbox-{}", s), format!("checked-{}", s))
    } else {
        classes!(format!("checkbox-{}", s))
    };

    html! {
        <div class={style}>
            <div class={format!("container-{}", s)}>
                <div class={format!("label-{}", s)}>{ &props.label }</div>
                <div class={format!("value-{}", s)}>{ &props.value }</div>
                <div class={checkbox} />
            </div>
        </div>
    }
}

