use stylist::yew::use_style;
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;
use super::labeled_value_checkbox::*;
use super::labeled_devider::*;

#[function_component(Skills)]
pub fn skills(props: &SkillListProps) -> Html {

    let modifier = |skill: SkillProps| -> i8 {
        let base_modifier = calc_base_modifier(skill.parent.value);
        if skill.proficiency {
            base_modifier + calc_proficiency_bonus(props.character.level)
        } else { base_modifier }
    };

    let style = use_style!(
        r#"
            display: grid;
            grid-template-rows: auto;
            grid-template-columns: 1fr 1fr;
            width: 100%;

            .top-row {
                grid-column: 1 / span 2;
            }
        "#
    );

    let label = Label { text: "Skills".into() };

    html! {
        <div class={style}>
            <div class="top-row">
                <LabeledDevider text={label.text}/>
            </div>
            <div>
                <LabeledValueCheckbox checked={props.acrobatics.proficiency.clone()} value={modifier(props.acrobatics.clone())} label={props.acrobatics.name.clone()} />
                <LabeledValueCheckbox checked={props.animalhandling.proficiency.clone()} value={modifier(props.animalhandling.clone())} label={props.animalhandling.name.clone()} />
                <LabeledValueCheckbox checked={props.arcana.proficiency.clone()} value={modifier(props.arcana.clone())} label={props.arcana.name.clone()} />
                <LabeledValueCheckbox checked={props.athletics.proficiency.clone()} value={modifier(props.athletics.clone())} label={props.athletics.name.clone()} />
                <LabeledValueCheckbox checked={props.deception.proficiency.clone()} value={modifier(props.deception.clone())} label={props.deception.name.clone()} />
                <LabeledValueCheckbox checked={props.history.proficiency.clone()} value={modifier(props.history.clone())} label={props.history.name.clone()} />
                <LabeledValueCheckbox checked={props.insight.proficiency.clone()} value={modifier(props.insight.clone())} label={props.insight.name.clone()} />
                <LabeledValueCheckbox checked={props.intimidation.proficiency.clone()} value={modifier(props.intimidation.clone())} label={props.intimidation.name.clone()} />
                <LabeledValueCheckbox checked={props.investigation.proficiency.clone()} value={modifier(props.investigation.clone())} label={props.investigation.name.clone()} />
            </div>
            <div>
                <LabeledValueCheckbox checked={props.medicine.proficiency.clone()} value={modifier(props.medicine.clone())} label={props.medicine.name.clone()} />
                <LabeledValueCheckbox checked={props.nature.proficiency.clone()} value={modifier(props.nature.clone())} label={props.nature.name.clone()} />
                <LabeledValueCheckbox checked={props.perception.proficiency.clone()} value={modifier(props.perception.clone())} label={props.perception.name.clone()} />
                <LabeledValueCheckbox checked={props.performance.proficiency.clone()} value={modifier(props.performance.clone())} label={props.performance.name.clone()} />
                <LabeledValueCheckbox checked={props.persuasion.proficiency.clone()} value={modifier(props.persuasion.clone())} label={props.persuasion.name.clone()} />
                <LabeledValueCheckbox checked={props.religion.proficiency.clone()} value={modifier(props.religion.clone())} label={props.religion.name.clone()} />
                <LabeledValueCheckbox checked={props.sleightofhand.proficiency.clone()} value={modifier(props.sleightofhand.clone())} label={props.sleightofhand.name.clone()} />
                <LabeledValueCheckbox checked={props.stealth.proficiency.clone()} value={modifier(props.stealth.clone())} label={props.stealth.name.clone()} />
                <LabeledValueCheckbox checked={props.survival.proficiency.clone()} value={modifier(props.survival.clone())} label={props.survival.name.clone()} />
            </div>
        </div>
    }
}

