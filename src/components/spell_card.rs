use stylist::css;
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;

#[function_component(SpellCard)]
pub fn spell_card(props: &Spell) -> Html {
    let s = random_alpha_string(8);
    let style = css!(
    );

    html! {
        <div class={style}>
            <div class={format!("{}", s)}>{props.name.clone()}</div>
            <div class={format!("{}", s)}>{props.level}</div>
            <div class={format!("{}", s)}>{props.casttime.clone()}</div>
            <div class={format!("{}", s)}>{props.range.clone()}</div>
            <div class={format!("{}", s)}>{props.duration.clone()}</div>
            <div class={format!("{}", s)}>{props.school.clone()}</div>
            <div class={format!("{}", s)}>{props.ritual}</div>
            <div class={format!("{}", s)}>{props.concentration}</div>
            <div class={format!("{}", s)}>{props.components.verbal}</div>
            <div class={format!("{}", s)}>{props.components.somatic}</div>
            <div class={format!("{}", s)}>{props.components.material}</div>
            <div class={format!("{}", s)}>{props.description.clone()}</div>
            <div class={format!("{}", s)}>{props.higherlevels.clone()}</div>
        </div>
    }
}

