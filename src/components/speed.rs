use yew::prelude::*;
use super::super::models::models::*;
use super::labeled_value_modifier_box::*;

#[function_component(Speed)]
pub fn speed(props: &Character) -> Html {

    html! {
        <LabeledValueModiferBox label={"Speed"} value={props.speed.base} modifier={props.speed.modifier} />
    }
}
