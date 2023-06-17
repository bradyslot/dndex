use stylist::yew::use_style;
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;
use super::labeled_value_checkbox::*;
use super::labeled_divider::*;

#[function_component(Skills)]
pub fn skills(props: &Character) -> Html {

    let modifier = |skill: Skill| -> i8 {
        let base_modifier = calc_base_modifier(skill.primary.value);
        if skill.proficiency {
            base_modifier + calc_proficiency_bonus(props.level)
        } else { base_modifier }
    };

    let style = use_style!(
        r#"
            display: grid;
            grid-template-columns: 1fr 1fr;
            grid-gap: 1rem;
            flex-grow: 1;

            .top-row {
                grid-column: 1 / span 2;
            }
        "#
    );

    let label = Label { text: "Skills".into() };

    html! {
        <div class={style}>
            <div class="top-row">
                <LabeledDivider text={label.text}/>
            </div>
            <div>
                <LabeledValueCheckbox checked={props.skills.acrobatics.proficiency.clone()} value={modifier(props.skills.acrobatics.clone())} label={props.skills.acrobatics.name.clone()} />
                <LabeledValueCheckbox checked={props.skills.animalhandling.proficiency.clone()} value={modifier(props.skills.animalhandling.clone())} label={props.skills.animalhandling.name.clone()} />
                <LabeledValueCheckbox checked={props.skills.arcana.proficiency.clone()} value={modifier(props.skills.arcana.clone())} label={props.skills.arcana.name.clone()} />
                <LabeledValueCheckbox checked={props.skills.athletics.proficiency.clone()} value={modifier(props.skills.athletics.clone())} label={props.skills.athletics.name.clone()} />
                <LabeledValueCheckbox checked={props.skills.deception.proficiency.clone()} value={modifier(props.skills.deception.clone())} label={props.skills.deception.name.clone()} />
                <LabeledValueCheckbox checked={props.skills.history.proficiency.clone()} value={modifier(props.skills.history.clone())} label={props.skills.history.name.clone()} />
                <LabeledValueCheckbox checked={props.skills.insight.proficiency.clone()} value={modifier(props.skills.insight.clone())} label={props.skills.insight.name.clone()} />
                <LabeledValueCheckbox checked={props.skills.intimidation.proficiency.clone()} value={modifier(props.skills.intimidation.clone())} label={props.skills.intimidation.name.clone()} />
                <LabeledValueCheckbox checked={props.skills.investigation.proficiency.clone()} value={modifier(props.skills.investigation.clone())} label={props.skills.investigation.name.clone()} />
            </div>
            <div>
                <LabeledValueCheckbox checked={props.skills.medicine.proficiency.clone()} value={modifier(props.skills.medicine.clone())} label={props.skills.medicine.name.clone()} />
                <LabeledValueCheckbox checked={props.skills.nature.proficiency.clone()} value={modifier(props.skills.nature.clone())} label={props.skills.nature.name.clone()} />
                <LabeledValueCheckbox checked={props.skills.perception.proficiency.clone()} value={modifier(props.skills.perception.clone())} label={props.skills.perception.name.clone()} />
                <LabeledValueCheckbox checked={props.skills.performance.proficiency.clone()} value={modifier(props.skills.performance.clone())} label={props.skills.performance.name.clone()} />
                <LabeledValueCheckbox checked={props.skills.persuasion.proficiency.clone()} value={modifier(props.skills.persuasion.clone())} label={props.skills.persuasion.name.clone()} />
                <LabeledValueCheckbox checked={props.skills.religion.proficiency.clone()} value={modifier(props.skills.religion.clone())} label={props.skills.religion.name.clone()} />
                <LabeledValueCheckbox checked={props.skills.sleightofhand.proficiency.clone()} value={modifier(props.skills.sleightofhand.clone())} label={props.skills.sleightofhand.name.clone()} />
                <LabeledValueCheckbox checked={props.skills.stealth.proficiency.clone()} value={modifier(props.skills.stealth.clone())} label={props.skills.stealth.name.clone()} />
                <LabeledValueCheckbox checked={props.skills.survival.proficiency.clone()} value={modifier(props.skills.survival.clone())} label={props.skills.survival.name.clone()} />
            </div>
        </div>
    }
}

