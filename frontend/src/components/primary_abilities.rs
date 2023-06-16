use stylist::yew::use_style;
use yew::prelude::*;
use super::shared::models::*;
use super::ability_score::*;
use super::labeled_divider::*;

#[function_component(PrimaryAbilities)]
pub fn primary_abilities(props: &PrimaryAbilitiesProps) -> Html {
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
            <AbilityScore saving={false} value={props.strength.value.clone()} name={props.strength.name.clone()} />
            <AbilityScore saving={false} value={props.dexterity.value.clone()} name={props.dexterity.name.clone()} />
            <AbilityScore saving={false} value={props.constitution.value.clone()} name={props.constitution.name.clone()} />
            <AbilityScore saving={false} value={props.intelligence.value.clone()} name={props.intelligence.name.clone()} />
            <AbilityScore saving={false} value={props.wisdom.value.clone()} name={props.wisdom.name.clone()} />
            <AbilityScore saving={false} value={props.charisma.value.clone()} name={props.charisma.name.clone()} />
        </div>
    }
}

