use stylist::css;
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;
use super::super::components::spell_card::*;
use super::labeled_divider::*;

#[function_component(Spells)]
pub fn spells(props: &Character) -> Html {
    let s = random_alpha_string(8);
    let style = css!(
        display: flex;
        flex-direction: column;
        margin: 1rem;

        .spell-card-${s} {
            display: flex;
            flex-direction: row;
            overflow-x: scroll;
        }
    );

    html! {
        <div class={style}>
            <LabeledDivider text={"Spell Cards"}/>
            <div class={format!("spell-card-{}", s)}>
            { for props.spells.iter().map(|spell| { html!(
                <SpellCard
                        name={spell.name.clone()}
                        level={spell.level}
                        casttime={spell.casttime.clone()}
                        range={spell.range.clone()}
                        duration={spell.duration.clone()}
                        school={spell.school.clone()}
                        ritual={spell.ritual}
                        concentration={spell.concentration}
                        components={spell.components.clone()}
                        description={spell.description.clone()}
                        classes={spell.classes.clone()}
                        higherlevels={spell.higherlevels.clone()}
                    />)
                })
            }
            </div>
        </div>
    }
}
