use stylist::css;
use yew::prelude::*;
use super::super::models::models::*;
use super::shared::utils::*;
use super::labeled_value::*;
use super::labeled_divider::*;

#[function_component(PassiveAbilities)]
pub fn passive_abilities(props: &Character) -> Html {
    let s = random_alpha_string(8);
    let style = css!(
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        grid-gap: 0.5rem;
        margin: 1rem;

        .top-${s} {
            grid-column: 1 / span 3;
        }
    );

    html! {
        <div class={style}>
            <div class={format!("top-{}", s)}>
                <LabeledDivider text={"Passive Abilities"} />
            </div>
            <LabeledValue value={calc_proficiency_bonus(props.level)} label={"Proficiency Bonus"} />
            { for props.passives.iter().map(|p| html! { <LabeledValue label={p.name.clone()} value={p.value} /> }) }
        </div>
    }
}

