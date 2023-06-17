use stylist::yew::use_style;
use yew::prelude::*;
use super::shared::models::*;

#[function_component(LabeledDivider)]
pub fn labeled_divider(props: &Label) -> Html {
    let style = use_style!(
        r#"
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 2em;

            .line {
                display: flex;
                flex-grow: 1;
                align-items: center;
            }

            .line::before,
            .line::after {
                content: "";
                flex-grow: 1;
                height: 2px;
                background-color: black;
            }

            .label {
                margin: 0 1em;
            }
        "#
    );

    html! {
        <div class={style}>
          <div class="line"></div>
          <div class="label">{props.text.clone()}</div>
          <div class="line"></div>
        </div>
    }
}
