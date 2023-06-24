use stylist::css;
use yew::prelude::*;
use super::super::models::models::*;
use super::shared::utils::*;
use super::super::components::hit_points::*;
use super::super::components::saving_throws::*;
use super::super::components::speed::*;
use super::super::components::death_save_rolls::*;
use super::super::components::initiative::*;
use super::super::components::hit_dice::*;
use super::super::components::armor_class::*;
use super::super::components::labeled_divider::*;

#[function_component(PlayerStats)]
pub fn player_stats(props: &Character) -> Html {
    let s = random_alpha_string(8);
    let style = css!(
        display: grid;
        grid-gap: 1rem;
        margin: 1rem;
        grid-template-areas:
            "divider divider divider divider divider"
            "speed initiative hitdice armorclass deathsaverolls"
            "savingthrows savingthrows hitpoints hitpoints hitpoints";

        .divider-${s} {
            grid-area: divider;
        }
        .speed-${s} {
            grid-area: speed;
        }
        .initiative-${s} {
            grid-area: initiative;
            height: 10rem;
            width: 10rem;
        }
        .hitdice-${s} {
            grid-area: hitdice;
        }
        .armorclass-${s} {
            grid-area: armorclass;
            height: 10rem;
            width: 10rem;
        }
        .deathsaverolls-${s} {
            grid-area: deathsaverolls;
            height: 10rem;
        }
        .savingthrows-${s} {
            grid-area: savingthrows;
        }
        .hitpoints-${s} {
            grid-area: hitpoints;
        }
    );

    html! {
        <div class={style}>
            <div class={format!("divider-{}", s)}><LabeledDivider text={"Player Stats"} /></div>
            <div class={format!("speed-{}", s)}><Speed ..props.clone() /></div>
            <div class={format!("initiative-{}", s)}><Initiative ..props.clone() /></div>
            <div class={format!("hitdice-{}", s)}><HitDice ..props.clone() /></div>
            <div class={format!("armorclass-{}", s)}><ArmorClass ..props.clone() /></div>
            <div class={format!("deathsaverolls-{}", s)}><DeathSaveRolls ..props.clone() /></div>
            <div class={format!("savingthrows-{}", s)}><SavingThrows ..props.clone() /></div>
            <div class={format!("hitpoints-{}", s)}><HitPoints ..props.clone() /></div>
        </div>
    }
}

