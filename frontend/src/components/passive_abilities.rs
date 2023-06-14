use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;
use super::labeled_value::*;

#[function_component(PassiveAbilities)]
pub fn passive_abilities(props: &PrimaryAbilitiesProps) -> Html {

    html! {
        <>
            <LabeledValue value={10 + calc_base_modifier(props.wisdom.value)} label={"Perception (Passive)"} />
            <LabeledValue value={10 + calc_base_modifier(props.intelligence.value)} label={"Investigation (Passive)"} />
            <LabeledValue value={10 + calc_base_modifier(props.wisdom.value)} label={"Insight (Passive)"} />
            // <LabeledValue value={10 + calc_base_modifier(props.intelligence)} label={"Arcana (Passive)"} />
            // <LabeledValue value={10 + calc_base_modifier(props.intelligence)} label={"History (Passive)"} />
            // <LabeledValue value={10 + calc_base_modifier(props.wisdom)} label={"Religion (Passive)"} />
            // <LabeledValue value={10 + calc_base_modifier(props.wisdom)} label={"Nature (Passive)"} />
            // <LabeledValue value={10 + calc_base_modifier(props.wisdom)} label={"Survival (Passive)"} />
        </>
    }
}

