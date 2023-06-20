use stylist::css;
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;

#[function_component(LabeledDivider)]
pub fn labeled_divider(props: &Label) -> Html {
    let s = random_alpha_string(8);
    let style = css!(
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 2rem;

        .line-${s} {
            display: flex;
            flex-grow: 1;
            align-items: center;
        }

        .line-${s}::before,
        .line-${s}::after {
            content: "";
            flex-grow: 1;
            height: 2px;
            background-color: black;
        }

        .label-${s} {
            margin: 0 1rem;
        }
    );

    html! {
        <div class={style}>
          <div class={format!("line-{}", s)}></div>
          <div class={format!("label-{}", s)}>{props.text.clone()}</div>
          <div class={format!("line-{}", s)}></div>
        </div>
    }
}
