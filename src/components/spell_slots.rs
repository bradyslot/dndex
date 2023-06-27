use stylist::css;
use yew::prelude::*;
// use gloo_net::http::Request;
// use log::info;
use super::super::models::models::*;
use super::shared::utils::*;
// use super::super::components::spell_card::*;
// use super::super::constants::*;
use super::super::components::rectangle_scooped::*;

#[function_component(SpellSlots)]
pub fn spell_slots(props: &Character) -> Html {

    let s = random_alpha_string(8);
    let style = css!(
        display: flex;

        .grid-${s} {
            display: grid;
            grid-template-areas:
                "spell-slots-1 spell-slots-2 spell-slots-3 spell-slots-4 spell-slots-5 spell-slots-6 spell-slots-7 spell-slots-8 spell-slots-9";
            grid-template-columns: repeat(9, 1fr);
            grid-gap: 0.25rem;
            height: 100%;
        }
    );

    html! {
        <div class={style}>
            <RectangleScooped label={"Spell Slots"}>
                {"Hello world"}
            </RectangleScooped>
        </div>
    }
}
