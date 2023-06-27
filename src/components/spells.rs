use stylist::css;
use yew::prelude::*;
use gloo_net::http::Request;
use log::info;
use super::super::models::models::*;
use super::shared::utils::*;
use super::super::components::spell_card::*;
use super::labeled_divider::*;
use super::super::constants::*;

// use anyhow::Error;
use std::cell::RefCell;
use std::rc::Rc;

#[function_component(Spells)]
pub fn spells(props: &Character) -> Html {
    let spells = props.spells.clone();
    let spell_card_data = use_state(|| vec![]);
    let spell_card_data_clone = spell_card_data.clone();

    use_effect_with_deps(
        move |_| {
            async fn fetch_spell_data(url: String) -> Result<Spell, anyhow::Error> {
                let response = Request::get(&url)
                    .send()
                    .await
                    .expect("could not send request")
                    .json::<Spell>()
                    .await
                    .expect("could not parse json response");
                Ok(response)
            }

            async fn fetch_spells(spells: Vec<String>, spell_card_data: Rc<RefCell<Vec<Spell>>>) {
                for spell in spells {
                    let url = format!("{}/spells/{}", API_URL, spell);
                    let fetched_spell_card = fetch_spell_data(url).await.expect("could not fetch spell");
                    spell_card_data.borrow_mut().push(fetched_spell_card);
                }
            }

            let spell_card_data_clone = spell_card_data_clone.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let spell_card_data = Rc::new(RefCell::new(vec![]));
                fetch_spells(spells, spell_card_data.clone()).await;

                spell_card_data_clone.set(spell_card_data.borrow().clone());
            });
        },
        props.spells.clone(),
    );

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
                info!("Spell: {:?}", spell_card_data.get(index));
                if let Some(spell) = spell_card_data.get(index) {
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
                            document__slug={spell.document__slug.clone()}
                            document__title={spell.document__title.clone()}
                            document__license_url={spell.document__license_url.clone()}
                            document__url={spell.document__url.clone()}
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
                            document__slug={"Loading..."}
                            document__title={"Loading..."}
                            document__license_url={"Loading..."}
                            document__url={"Loading..."}
                        />)
                }
            })}
            </div>
        </div>
    }
}
