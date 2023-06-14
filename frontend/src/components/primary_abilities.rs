use stylist::yew::use_style;
use yew::prelude::*;
use super::shared::models::*;
use super::ability_score::*;

#[function_component(PrimaryAbilities)]
pub fn primary_abilities(props: &PrimaryAbilitiesProps) -> Html {
    let style = use_style!(
        r#"
            display: flex;
            flex-direction: row;
            flex-grow: 1;
            margin-bottom: 3rem;
        "#
    );

    html! {
        <div class={style}>
            <AbilityScore value={props.strength.value.clone()} name={props.strength.name.clone()} />
            <AbilityScore value={props.dexterity.value.clone()} name={props.dexterity.name.clone()} />
            <AbilityScore value={props.constitution.value.clone()} name={props.constitution.name.clone()} />
            <AbilityScore value={props.intelligence.value.clone()} name={props.intelligence.name.clone()} />
            <AbilityScore value={props.wisdom.value.clone()} name={props.wisdom.name.clone()} />
            <AbilityScore value={props.charisma.value.clone()} name={props.charisma.name.clone()} />
        </div>
    }
}

