use stylist::css;
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;
use super::rectangle::*;

#[function_component(AbilityScore)]
pub fn ability_score(props: &Ability) -> Html {
    // grid-area: (row start) / (column start) / (row end) / (column end)
    let s = random_alpha_string(8);
    let style = css!(
        display: grid;
        padding: 0.5rem;
        grid-template-rows: 1fr;

        .upper-${s} {
            width: 14rem;
            height: 14rem;
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

        .modifier-${s} {
            align-self: center;
            font-size: 5rem;
            height: 100%;
        }

        .circle-${s} {
            font-size: 3rem;
            width: 5rem;
            height: 5rem;
            border-radius: 50%;
            border: 2px solid black;
            background-color: var(--foreground);
            line-height: 5rem;
            display: block;
            margin: auto;
        }
    );

    html! {
        <div class={style}>
            <div class={format!("upper-{}", s)}>
                <Rectangle label={ &props.name }>
                    <div class={format!("absolute-{} center-{} modifier-{}", s, s, s)}>{ calc_base_modifier(props.value) }</div>
                </Rectangle>
            </div>
            <div class={format!("lower-{}", s)}>
                <div class={format!("center-{} circle-{}", s, s)}>{ &props.value }</div>
            </div>
        </div>
    }
}
