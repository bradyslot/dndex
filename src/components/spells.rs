use stylist::css;
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
// use log::info;

use crate::components::shared::utils::*;
use crate::components::labeled_divider::*;
use crate::components::spell_card::*;
use crate::models::models::*;
use crate::models::open5e::*;
use crate::constants::*;
use crate::api::open5e::*;

#[function_component(Spells)]
pub fn spells(props: &Character) -> Html {

    let spell_cards = use_state(|| vec![]);
    let spells = props.spells.clone();
    {
        let spell_cards = spell_cards.clone();
        use_effect_with_deps(move |_| {
                let spell_cards = spell_cards.clone();
                spawn_local(async move {
                    let fetched_spells = fetch_vec_spells(spells).await;
                    spell_cards.set(fetched_spells);
                });
            },
            props.spells.clone(),
        );
    }

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

    // TODO: add materials
    html! {
        <div class={style}>
            <LabeledDivider text={"Spell Cards"}/>
            <div class={format!("spell-card-{}", s)}>
            { for props.spells.iter().enumerate().map(|(index, _)| {
                // info!("Spell: {:?}", spell_cards.get(index));
                if let Some(spell) = spell_cards.get(index) {
                    html! (<SpellCard
                            slug={spell.slug.clone()}
                            name={spell.name.clone()}
                            desc={spell.desc.clone()}
                            higher_level={spell.higher_level.clone()}
                            page={spell.page.clone()}
                            range={spell.range.clone()}
                            target_range_sort={spell.target_range_sort.clone()}
                            components={spell.components.clone()}
                            requires_verbal_components={spell.requires_verbal_components.clone()}
                            requires_somatic_components={spell.requires_somatic_components.clone()}
                            requires_material_components={spell.requires_material_components.clone()}
                            material={spell.material.clone()}
                            can_be_cast_as_ritual={spell.can_be_cast_as_ritual.clone()}
                            ritual={spell.ritual.clone()}
                            duration={spell.duration.clone()}
                            concentration={spell.concentration.clone()}
                            requires_concentration={spell.requires_concentration.clone()}
                            casting_time={spell.casting_time.clone()}
                            level={spell.level.clone()}
                            level_int={spell.level_int.clone()}
                            spell_level={spell.spell_level.clone()}
                            school={spell.school.clone()}
                            dnd_class={spell.dnd_class.clone()}
                            spell_lists={spell.spell_lists.clone()}
                            archetype={spell.archetype.clone()}
                            circles={spell.circles.clone()}
                            document={spell.document.clone()}
                        />)
                } else {
                    let default_spell: Open5eSpell = Default::default();
                    html! (<SpellCard
                            slug={default_spell.slug.clone()}
                            name={default_spell.name.clone()}
                            desc={default_spell.desc.clone()}
                            higher_level={default_spell.higher_level.clone()}
                            page={default_spell.page.clone()}
                            range={default_spell.range.clone()}
                            target_range_sort={default_spell.target_range_sort.clone()}
                            components={default_spell.components.clone()}
                            requires_verbal_components={default_spell.requires_verbal_components.clone()}
                            requires_somatic_components={default_spell.requires_somatic_components.clone()}
                            requires_material_components={default_spell.requires_material_components.clone()}
                            material={default_spell.material.clone()}
                            can_be_cast_as_ritual={default_spell.can_be_cast_as_ritual.clone()}
                            ritual={default_spell.ritual.clone()}
                            duration={default_spell.duration.clone()}
                            concentration={default_spell.concentration.clone()}
                            requires_concentration={default_spell.requires_concentration.clone()}
                            casting_time={default_spell.casting_time.clone()}
                            level={default_spell.level.clone()}
                            level_int={default_spell.level_int.clone()}
                            spell_level={default_spell.spell_level.clone()}
                            school={default_spell.school.clone()}
                            dnd_class={default_spell.dnd_class.clone()}
                            spell_lists={default_spell.spell_lists.clone()}
                            archetype={default_spell.archetype.clone()}
                            circles={default_spell.circles.clone()}
                            document={default_spell.document.clone()}
                        />)
                }
            })}
            </div>
        </div>
    }
}
