use stylist::css;
use stylist::Style;
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;

#[function_component(LabeledValue)]
pub fn labeled_value(props: &LabeledValueProps) -> Html {
    let s = random_alpha_string(8);
    let css = css!(
        r#"
            display: flex;
            flex-direction: row;
            flex-grow: 1;
            justify-content: flex-start;
            text-align: center;
            padding: 4px;
            --size: 5rem;

            .container-${s} {
                display: flex;
                flex-grow: 1;
            }
            .circle-${s} {
                display: flex;
                border-radius: 50%;
                height: var(--size);
                width: var(--size);
                background-color: var(--foreground);
                border: 2px solid black;
            }
            .value-${s} {
                height: var(--size);
                width: var(--size);
                font-size: calc(var(--size) / 2);
                line-height: 2;
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

    html! {
        <div class={style}>
            <div class={format!("container-{}", s)}>
                <div class={format!("circle-{}", s)}>
                    <div class={format!("value-{}", s)}>{ &props.value }</div>
                </div>
                <div class={format!("label-{}", s)}>{ &props.label }</div>
            </div>
        </div>
    }
}

