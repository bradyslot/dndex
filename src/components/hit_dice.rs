use yew::prelude::*;
use super::super::models::models::*;
use super::labeled_value_modifier_box::*;
use super::super::constants::*;
use super::shared::utils::*;

#[function_component(HitDice)]
pub fn hit_dice(props: &Character) -> Html {

    html! {
        <LabeledValueModiferBox
            label={"HitDice"}
            text={format!("{}d{}", props.class.hitdice.count, props.class.hitdice.sides)}
            modifier={calc_base_modifier(props.abilities[CON].value)}
        />
    }
}
