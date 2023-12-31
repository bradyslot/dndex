use stylist::css;
use yew::prelude::*;
use super::super::models::models::*;
use super::shared::utils::*;
use super::rectangle_scooped::*;
use super::labeled_value_checkbox::*;

#[function_component(SavingThrows)]
pub fn saving_throws(props: &Character) -> Html {
    let s = random_alpha_string(8);
    let style = css!(
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
            margin: 1rem 2rem 1rem 2rem;
            width: 100%;
        }
    );

    let proficiency_bonus = calc_proficiency_bonus(props.level);
    let modifier = |ability: &Ability| -> i32 {
        let base_modifier = calc_base_modifier(ability.value);
        if !props.class.saves.contains(ability) {
            return base_modifier;
        }
        base_modifier + proficiency_bonus
    };

    html! {
        <div class={style}>
            <RectangleScooped>
                <div class={format!("absolute-{} center-{} label-{}", s, s, s)}>{"Saving Throws"}</div>
                <div class={format!("grid-{}", s)}>
                    { for props.abilities.iter().map(|a| html! { 
                        <LabeledValueCheckbox 
                            checked={props.class.saves.contains(a)}
                            label={a.name.clone()}
                            value={modifier(&a)} 
                        /> })
                    }
                </div>
            </RectangleScooped>
        </div>
    }
}

