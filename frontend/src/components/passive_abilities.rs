use yew::prelude::*;
use stylist::yew::use_style;
use super::shared::models::*;
use super::shared::utils::*;
use super::labeled_value::*;
use super::labeled_divider::*;

#[function_component(PassiveAbilities)]
pub fn passive_abilities(props: &PrimaryAbilitiesProps) -> Html {

    let style = use_style! {
        r#"
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            grid-gap: 0.5rem;
            padding: 0.5rem;
            flex-grow: 1;

            .top-row {
                grid-column: 1 / span 3;
            }
        "#
    };

    html! {
        <div class={style}>
            <div class="top-row">
                <LabeledDivider text={"Passive Abilities"} />
            </div>
            <LabeledValue value={calc_proficiency_bonus(props.character.level)} label={"Proficiency Bonus"} />
            <LabeledValue value={10 + calc_base_modifier(props.wisdom.value)} label={"Perception (Passive)"} />
            <LabeledValue value={10 + calc_base_modifier(props.intelligence.value)} label={"Investigation (Passive)"} />
            <LabeledValue value={10 + calc_base_modifier(props.wisdom.value)} label={"Insight (Passive)"} />
            <LabeledValue value={10 + calc_base_modifier(props.intelligence.value)} label={"Arcana (Passive)"} />
            <LabeledValue value={10 + calc_base_modifier(props.intelligence.value)} label={"History (Passive)"} />
            <LabeledValue value={10 + calc_base_modifier(props.wisdom.value)} label={"Religion (Passive)"} />
            <LabeledValue value={10 + calc_base_modifier(props.wisdom.value)} label={"Nature (Passive)"} />
            <LabeledValue value={10 + calc_base_modifier(props.wisdom.value)} label={"Survival (Passive)"} />
        </div>
    }
}

