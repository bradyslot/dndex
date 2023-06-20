use stylist::{css, Style};
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;
use super::labeled_value::*;
use super::labeled_divider::*;

#[function_component(PassiveAbilities)]
pub fn passive_abilities(props: &Character) -> Html {
    let s = random_alpha_string(8);
    let css = css!(
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        grid-gap: 0.5rem;
        margin: 1rem;

        .top-${s} {
            grid-column: 1 / span 3;
        }
    );
    let style = Style::new(css).expect("css no good");

    html! {
        <div class={style}>
            <div class={format!("top-{}", s)}>
                <LabeledDivider text={"Passive Abilities"} />
            </div>
            <LabeledValue value={calc_proficiency_bonus(props.level)} label={"Proficiency Bonus"} />
            <LabeledValue value={10 + calc_base_modifier(props.abilities[props.abilities.iter().position(|a| a.name == "Wisdom").unwrap()].value)} label={"Perception (Passive)"} />
            <LabeledValue value={10 + calc_base_modifier(props.abilities[props.abilities.iter().position(|a| a.name == "Intelligence").unwrap()].value)} label={"Investigation (Passive)"} />
            <LabeledValue value={10 + calc_base_modifier(props.abilities[props.abilities.iter().position(|a| a.name == "Wisdom").unwrap()].value)} label={"Insight (Passive)"} />
            <LabeledValue value={10 + calc_base_modifier(props.abilities[props.abilities.iter().position(|a| a.name == "Intelligence").unwrap()].value)} label={"Arcana (Passive)"} />
            <LabeledValue value={10 + calc_base_modifier(props.abilities[props.abilities.iter().position(|a| a.name == "Intelligence").unwrap()].value)} label={"History (Passive)"} />
            <LabeledValue value={10 + calc_base_modifier(props.abilities[props.abilities.iter().position(|a| a.name == "Wisdom").unwrap()].value)} label={"Religion (Passive)"} />
            <LabeledValue value={10 + calc_base_modifier(props.abilities[props.abilities.iter().position(|a| a.name == "Wisdom").unwrap()].value)} label={"Nature (Passive)"} />
            <LabeledValue value={10 + calc_base_modifier(props.abilities[props.abilities.iter().position(|a| a.name == "Wisdom").unwrap()].value)} label={"Survival (Passive)"} />
        </div>
    }
}

