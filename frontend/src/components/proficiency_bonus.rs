use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;
use super::labeled_value::*;

#[function_component(ProficiencyBonus)]
pub fn proficiency_bonus(props: &CharacterProps) -> Html {
    let label = format!("Proficiency Bonus");

    html! {
        <LabeledValue value={calc_proficiency_bonus(props.level)} label={label} />
    }
}

