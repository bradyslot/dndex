use stylist::css;
use yew::prelude::*;
use super::shared::models::*;
use super::super::components::hit_points::*;
use super::super::components::saving_throws::*;
use super::super::components::speed::*;
use super::super::components::death_save_rolls::*;
use super::super::components::initiative::*;
use super::super::components::hit_dice::*;
use super::super::components::armor_class::*;

#[function_component(CharacterStats)]
pub fn character_stats(props: &Character) -> Html {
    let style = css!(
        display: flex;
    );

    html! {
        <div class={style}>
            <Speed ..props.clone() />
            <Initiative ..props.clone() />
            <HitDice ..props.clone() />
            <ArmorClass ..props.clone() />
            <DeathSaveRolls ..props.clone() />
            <SavingThrows ..props.clone() />
            <HitPoints ..props.clone() />
        </div>
    }
}

