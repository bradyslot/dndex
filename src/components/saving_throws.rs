use stylist::{css, Style};
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;
use super::rectangle::*;
use super::labeled_value_checkbox::*;

#[function_component(SavingThrows)]
pub fn saving_throws(props: &Character) -> Html {
    let s = random_alpha_string(8);
    let css = css!(
        display: flex;

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

        .grid-${s} {
            display: grid;
            grid-template-rows: repeat(6, 1fr);
            margin: 1rem;
        }
    );
    let style = Style::new(css).expect("css no good");

    let modifier = |ability: Ability| -> i8 {
        let base_modifier = calc_base_modifier(ability.value);
        if ability.saving {
            base_modifier + calc_proficiency_bonus(props.level)
        } else { base_modifier }
    };

    html! {
        <div class={style}>
            <Rectangle>
                <div class={format!("absolute-{} center-{} label-{}", s, s, s)}>{"Saving Throws"}</div>
                <div class={format!("grid-{}", s)}>
                    { for props.abilities.iter().map(|a| html! { <LabeledValueCheckbox checked={a.saving} label={a.name.clone()} value={modifier(a.clone())} /> }) }
                </div>
            </Rectangle>
        </div>
    }
}

