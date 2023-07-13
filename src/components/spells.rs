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
                    let fetched_spells = fetch_spells(spells).await;
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

    let placeholder_document = Open5eDocument {
        slug: "".into(),
        title: "".into(),
        license_url: "".into(),
        url: "".into(),
    };

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
                    html! (<SpellCard
                            slug={"Loading..."}
                            name={"Loading..."}
                            desc={"Loading..."}
                            higher_level={"Loading..."}
                            page={"Loading..."}
                            range={"Loading..."}
                            target_range_sort={0}
                            components={"Loading..."}
                            requires_verbal_components={false}
                            requires_somatic_components={false}
                            requires_material_components={false}
                            material={"Loading..."}
                            can_be_cast_as_ritual={false}
                            ritual={"Loading..."}
                            duration={"Loading..."}
                            concentration={"Loading..."}
                            requires_concentration={false}
                            casting_time={"Loading..."}
                            level={"Loading..."}
                            level_int={0}
                            spell_level={0}
                            school={"Loading..."}
                            dnd_class={"Loading..."}
                            spell_lists={vec![]}
                            archetype={"Loading..."}
                            circles={"Loading..."}
                            document={placeholder_document.clone()}
                        />)
                }
            })}
            </div>
        </div>
    }
}
