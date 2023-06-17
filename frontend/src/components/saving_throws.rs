use stylist::yew::use_style;
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;
use super::rectangle::*;
use super::labeled_value_checkbox::*;

#[function_component(SavingThrows)]
pub fn saving_throws(props: &Character) -> Html {
    let style = use_style!(
        r#"
            display: flex;
            padding: 0.5rem;
            height: 100%;

            .absolute {
                display: flex;
                position: absolute;
                width: 100%;
            }

            .center {
                justify-content: center;
                align-items: center;
                text-align: center;
            }

            .label { 
                top: 0.5rem;
                font-size: 1.5rem;
            }

            .grid {
                display: grid;
                grid-template-rows: repeat(6, 1fr);
                margin-left: 1rem;
                margin-top: 1.5rem;
            }
        "#
    );

    let modifier = |ability: Ability| -> i8 {
        let base_modifier = calc_base_modifier(ability.value);
        if ability.saving {
            base_modifier + calc_proficiency_bonus(props.level)
        } else { base_modifier }
    };

    html! {
        <div class={style}>
            <Rectangle>
                <div class="absolute center label">{"Saving Throws"}</div>
                <div class="grid">
                    <LabeledValueCheckbox checked={props.abilities.strength.saving} label="Strength" value={modifier(props.abilities.strength.clone())} />
                    <LabeledValueCheckbox checked={props.abilities.dexterity.saving} label="Dexterity" value={modifier(props.abilities.dexterity.clone())} />
                    <LabeledValueCheckbox checked={props.abilities.constitution.saving} label="Constitution" value={modifier(props.abilities.constitution.clone())} />
                    <LabeledValueCheckbox checked={props.abilities.intelligence.saving} label="Intelligence" value={modifier(props.abilities.intelligence.clone())} />
                    <LabeledValueCheckbox checked={props.abilities.wisdom.saving} label="Wisdom" value={modifier(props.abilities.wisdom.clone())} />
                    <LabeledValueCheckbox checked={props.abilities.charisma.saving} label="Charisma" value={modifier(props.abilities.charisma.clone())} />
                </div>
            </Rectangle>
        </div>
    }
}

