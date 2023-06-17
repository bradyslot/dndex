use stylist::yew::use_style;
use yew::prelude::*;
use super::shared::models::*;
use super::ability_score::*;
use super::labeled_divider::*;

#[function_component(PrimaryAbilities)]
pub fn primary_abilities(props: &Character) -> Html {
    let style = use_style!(
        r#"
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            grid-gap: 1rem;
            padding: 0.5rem;
            justify-items: center;
            flex-grow: 1;
            margin-bottom: 1rem;

            .top-row {
                grid-column: 1 / span 3;
                width: 100%;
            }
        "#
    );

    html! {
        <div class={style}>
            <div class="top-row">
                <LabeledDivider text={"Primary Abilities"} />
            </div>
            <AbilityScore saving={false} value={props.abilities.strength.value.clone()} name={props.abilities.strength.name.clone()} />
            <AbilityScore saving={false} value={props.abilities.dexterity.value.clone()} name={props.abilities.dexterity.name.clone()} />
            <AbilityScore saving={false} value={props.abilities.constitution.value.clone()} name={props.abilities.constitution.name.clone()} />
            <AbilityScore saving={false} value={props.abilities.intelligence.value.clone()} name={props.abilities.intelligence.name.clone()} />
            <AbilityScore saving={false} value={props.abilities.wisdom.value.clone()} name={props.abilities.wisdom.name.clone()} />
            <AbilityScore saving={false} value={props.abilities.charisma.value.clone()} name={props.abilities.charisma.name.clone()} />
        </div>
    }
}
