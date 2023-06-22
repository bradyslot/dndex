use stylist::css;
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;
use super::ability_score::*;
use super::labeled_divider::*;

#[function_component(PrimaryAbilities)]
pub fn primary_abilities(props: &Character) -> Html {
    let s = random_alpha_string(8);
    let style = css!(
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        grid-gap: 1rem;
        justify-items: center;
        flex-grow: 1;
        margin: 1rem;

        .top-${s} {
            grid-column: 1 / span 3;
            width: 100%;
        }
    );

    html! {
        <div class={style}>
            <div class={format!("top-{}", s)}>
                <LabeledDivider text={"Primary Abilities"} />
            </div>
            { for props.abilities.iter().map(|a| html! { <AbilityScore value={a.value} name={a.name.clone()} /> }) }
        </div>
    }
}
