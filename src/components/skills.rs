use stylist::{css, Style};
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;
use super::labeled_value_checkbox::*;
use super::labeled_divider::*;

#[function_component(Skills)]
pub fn skills(props: &Character) -> Html {
    let s = random_alpha_string(8);
    let css = css!(
        display: grid;
        flex-grow: 1;
        grid-template-columns: repeat(2, 1fr);
        grid-gap: 1rem;
        margin: 1rem;

        .top-${s} {
            grid-column: 1 / span 2;
            width: 100%;
        }
    );
    let style = Style::new(css).expect("css no good");

    let modifier = |skill: Skill| -> i8 {
        let base_modifier = calc_base_modifier(skill.primary.value);
        if skill.proficiency {
            base_modifier + calc_proficiency_bonus(props.level)
        } else { base_modifier }
    };

    html! {
        <div class={style}>
            <div class={format!("top-{}", s)}>
                <LabeledDivider text={"Skills"}/>
            </div>
            <LabeledValueCheckbox checked={props.skills.acrobatics.proficiency} value={modifier(props.skills.acrobatics.clone())} label={props.skills.acrobatics.name.clone()} />
            <LabeledValueCheckbox checked={props.skills.animalhandling.proficiency} value={modifier(props.skills.animalhandling.clone())} label={props.skills.animalhandling.name.clone()} />
            <LabeledValueCheckbox checked={props.skills.arcana.proficiency} value={modifier(props.skills.arcana.clone())} label={props.skills.arcana.name.clone()} />
            <LabeledValueCheckbox checked={props.skills.athletics.proficiency} value={modifier(props.skills.athletics.clone())} label={props.skills.athletics.name.clone()} />
            <LabeledValueCheckbox checked={props.skills.deception.proficiency} value={modifier(props.skills.deception.clone())} label={props.skills.deception.name.clone()} />
            <LabeledValueCheckbox checked={props.skills.history.proficiency} value={modifier(props.skills.history.clone())} label={props.skills.history.name.clone()} />
            <LabeledValueCheckbox checked={props.skills.insight.proficiency} value={modifier(props.skills.insight.clone())} label={props.skills.insight.name.clone()} />
            <LabeledValueCheckbox checked={props.skills.intimidation.proficiency} value={modifier(props.skills.intimidation.clone())} label={props.skills.intimidation.name.clone()} />
            <LabeledValueCheckbox checked={props.skills.investigation.proficiency} value={modifier(props.skills.investigation.clone())} label={props.skills.investigation.name.clone()} />
            <LabeledValueCheckbox checked={props.skills.medicine.proficiency} value={modifier(props.skills.medicine.clone())} label={props.skills.medicine.name.clone()} />
            <LabeledValueCheckbox checked={props.skills.nature.proficiency} value={modifier(props.skills.nature.clone())} label={props.skills.nature.name.clone()} />
            <LabeledValueCheckbox checked={props.skills.perception.proficiency} value={modifier(props.skills.perception.clone())} label={props.skills.perception.name.clone()} />
            <LabeledValueCheckbox checked={props.skills.performance.proficiency} value={modifier(props.skills.performance.clone())} label={props.skills.performance.name.clone()} />
            <LabeledValueCheckbox checked={props.skills.persuasion.proficiency} value={modifier(props.skills.persuasion.clone())} label={props.skills.persuasion.name.clone()} />
            <LabeledValueCheckbox checked={props.skills.religion.proficiency} value={modifier(props.skills.religion.clone())} label={props.skills.religion.name.clone()} />
            <LabeledValueCheckbox checked={props.skills.sleightofhand.proficiency} value={modifier(props.skills.sleightofhand.clone())} label={props.skills.sleightofhand.name.clone()} />
            <LabeledValueCheckbox checked={props.skills.stealth.proficiency} value={modifier(props.skills.stealth.clone())} label={props.skills.stealth.name.clone()} />
            <LabeledValueCheckbox checked={props.skills.survival.proficiency} value={modifier(props.skills.survival.clone())} label={props.skills.survival.name.clone()} />
        </div>
    }
}

