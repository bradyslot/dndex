use stylist::yew::use_style;
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;
use super::rectangle::*;
use super::labeled_value_checkbox::*;

#[function_component(SavingThrows)]
pub fn saving_throws(props: &PrimaryAbilitiesProps) -> Html {
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
                margin-top: 1rem;
            }
        "#
    );

    let modifier = |ability: AbilityScoreProps| -> i8 {
        let base_modifier = calc_base_modifier(ability.value);
        if ability.saving {
            base_modifier + calc_proficiency_bonus(props.character.level)
        } else { base_modifier }
    };


    html! {
        <div class={style}>
            <Rectangle>
                <div class="absolute center label">{"Saving Throws"}</div>
                <div class="grid">
                    <LabeledValueCheckbox checked={props.strength.saving.clone()} label="Strength" value={modifier(props.strength.clone())} />
                    <LabeledValueCheckbox checked={props.dexterity.saving.clone()} label="Dexterity" value={modifier(props.dexterity.clone())} />
                    <LabeledValueCheckbox checked={props.constitution.saving.clone()} label="Constitution" value={modifier(props.constitution.clone())} />
                    <LabeledValueCheckbox checked={props.intelligence.saving.clone()} label="Intelligence" value={modifier(props.intelligence.clone())} />
                    <LabeledValueCheckbox checked={props.wisdom.saving.clone()} label="Wisdom" value={modifier(props.wisdom.clone())} />
                    <LabeledValueCheckbox checked={props.charisma.saving.clone()} label="Charisma" value={modifier(props.charisma.clone())} />
                </div>
            </Rectangle>
        </div>
    }
}

