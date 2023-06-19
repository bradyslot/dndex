use stylist::{css, Style};
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;
use super::rectangle::*;

#[function_component(AbilityScore)]
pub fn ability_score(props: &Ability) -> Html {
    // grid-area: (row start) / (column start) / (row end) / (column end)
    let s = random_alpha_string(8);
    let css = css!(
        display: grid;
        padding: 0.5rem;
        grid-template-rows: 1fr;

        .upper-${s} {
            width: 15rem;
            height: 15rem;
            grid-area: 1 / 1 / 3 / 1;
        }

        .lower-${s} {
            grid-area: 2 / 1 / 4 / 1;
            z-index: 1;
        }

        .absolute-${s} {
            display: flex;
            position: absolute;
            width: 100%;
        }

        .center-${s} {
            justify-content: center;
            align-items: center;
            text-align: center;
        }

        .label-${s} {
            top: 0.5rem;
            font-size: 1.5rem;
        }

        .modifier-${s} {
            align-self: center;
            font-size: 5rem;
            height: 100%;
        }

        .circle-${s} {
            font-size: 2rem;
            width: 4rem;
            height: 4rem;
            border-radius: 50%;
            border: 2px solid black;
            background-color: var(--foreground);
            line-height: 4rem;
            display: block;
            margin: auto;
        }
    );
    let style = Style::new(css).expect("css no good");

    html! {
        <div class={style}>
            <div class={format!("upper-{}", s)}>
                <Rectangle>
                    <div class={format!("absolute-{} center-{} label-{}", s, s, s)}>{ &props.name }</div>
                    <div class={format!("absolute-{} center-{} modifier-{}", s, s, s)}>{ calc_base_modifier(props.value) }</div>
                </Rectangle>
            </div>
            <div class={format!("lower-{}", s)}>
                <div class={format!("center-{} circle-{}", s, s)}>{ &props.value }</div>
            </div>
        </div>
    }
}
