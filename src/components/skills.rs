use stylist::css;
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;
use super::labeled_value_checkbox::*;
use super::labeled_divider::*;

#[function_component(Skills)]
pub fn skills(props: &Character) -> Html {
    let s = random_alpha_string(8);
    let style = css!(
        display: grid;
        flex-grow: 1;
        grid-template-columns: repeat(2, 1fr);
        grid-gap: 1rem;
        margin: 1rem;

        .top-${s} {
            grid-column: 1 / span 2;
            width: 100%;
        }
    );

    let modifier = |skill: Skill| -> i8 {
        let base_modifier = calc_base_modifier(skill.primary.value);
        if skill.proficiency {
            base_modifier + calc_proficiency_bonus(props.level)
        } else { base_modifier }
    };

    html! {
        <div class={style}>
            <div class={format!("top-{}", s)}>
                <LabeledDivider text={"Skills"}/>
            </div>
            { for props.skills.iter().map(|skill| html! {
                <LabeledValueCheckbox checked={skill.proficiency} value={modifier(skill.clone())} label={skill.name.clone()} />
            }) }
        </div>
    }
}

