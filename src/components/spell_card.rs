use stylist::css;
use yew::prelude::*;
use comrak::{markdown_to_html, ComrakOptions};
use minijinja::render;
// use log::info;

use crate::models::models::*;
use crate::models::open5e::*;
use crate::components::shared::utils::*;
use crate::components::shared::icons::*;

#[function_component(SpellCard)]
pub fn spell_card(props: &Open5eSpell) -> Html {
    let s = random_alpha_string(8);
    let style = css!(
        height: 40rem;
        width: 25rem;
        border: 2px solid black;
        border-radius: 1rem;
        font-size: 1.25rem;
        padding: 0.5rem;
        margin: 0.5rem;
        flex-shrink: 0;

        .grid-${s} {
            display: grid;
            grid-template-areas:
                "name             name             name             name             name           level"
                "name-label       name-label       name-label       name-label       name-label     level-label"
                "casttime         casttime         range            range            duration       duration"
                "casttime-label   casttime-label   range-label      range-label      duration-label duration-label"
                "components       components       components       components       school         school"
                "components-label components-label components-label components-label school-label   school-label"
                "description      description      description      description      description    description"
                "higherlevels     higherlevels     higherlevels     higherlevels     higherlevels   higherlevels";
            grid-template-rows: auto auto auto auto auto auto 1fr auto;
            grid-gap: 0.25rem;
            height: 100%;
        }
        .name-${s} {
            display: flex;
            justify-content: center;
            grid-area: name;
            border-bottom: 2px solid black;
            font-size: 2rem;
        }
        .name-label-${s} {
            display: flex;
            justify-content: center;
            grid-area: name-label;
            font-size: 1rem;
        }
        .level-${s} {
            display: flex;
            justify-content: center;
            grid-area: level;
            border-bottom: 2px solid black;
            font-size: 2rem;
        }
        .level-label-${s} {
            display: flex;
            justify-content: center;
            grid-area: level-label;
            font-size: 1rem;
        }
        .casttime-${s} {
            display: flex;
            justify-content: space-around;
            grid-area: casttime;
            border-bottom: 2px solid black;
        }
        .casttime-label-${s} {
            display: flex;
            justify-content: center;
            grid-area: casttime-label;
            font-size: 1rem;
        }
        .range-${s} {
            display: flex;
            justify-content: center;
            grid-area: range;
            border-bottom: 2px solid black;
        }
        .range-label-${s} {
            display: flex;
            justify-content: center;
            grid-area: range-label;
            font-size: 1rem;
        }
        .duration-${s} {
            display: flex;
            justify-content: space-around;
            grid-area: duration;
            border-bottom: 2px solid black;
        }
        .duration-label-${s} {
            display: flex;
            justify-content: center;
            grid-area: duration-label;
            font-size: 1rem;
        }
        .components-${s} {
            display: flex;
            justify-content: space-around;
            grid-area: components;
            border-bottom: 2px solid black;
        }
        .components-label-${s} {
            display: flex;
            justify-content: center;
            grid-area: components-label;
            font-size: 1rem;
        }
        .school-${s} {
            display: flex;
            justify-content: center;
            grid-area: school;
            border-bottom: 2px solid black;
        }
        .school-label-${s} {
            display: flex;
            justify-content: center;
            grid-area: school-label;
            font-size: 1rem;
        }
        .description-${s} {
            grid-area: description;
            overflow-y: auto;
        }
        .higherlevels-${s} {
            border-top: 2px dashed black;
            padding-top: 0.5rem;
            grid-area: higherlevels;
            overflow-y: auto;
            max-height: 12rem;
        }
        .svg-${s} {
            height: 1.25rem;
            width: 1.25rem;
        }
        .flex-${s} {
            display: flex;
        }
        table {
            width: 100%;
        }
        h2 {
            font-size: 1.5rem;
            font-weight: bold;
            margin: 0;
        }
        p {
            margin: 0;
        }
        th {
            text-align: center;
            border-bottom: 2px solid black;
        }
        td {
            border-bottom: 1px solid black;
        }
    );

    let mut options = ComrakOptions::default();
    options.extension.table = true;

    let base_url = "https://open5e.com";
    let desc = render!(&props.desc.clone(), base_url);
    let higher_level = render!(&props.higher_level.clone(), base_url);
    // info!("desc: {}", desc);

    let desc_string = markdown_to_html(&desc, &options).into();
    let desc_html = Html::from_html_unchecked(desc_string);

    let higher_level_string = markdown_to_html(&higher_level, &options).into();
    let higher_level_html = Html::from_html_unchecked(higher_level_string);

    html! {
        <div class={style}>
            <div class={format!("grid-{}", s)}>
                <div class={format!("name-{}", s)}>{props.name.clone()}</div>
                <div class={format!("name-label-{}", s)}>{"Name"}</div>
                <div class={format!("level-{}", s)}>{if props.level_int > 0 {html!({props.level_int})} else {html!({"C"})}}</div>
                <div class={format!("level-label-{}", s)}>{"Level"}</div>
                <div class={format!("casttime-{}", s)}>
                    <div class={format!("flex-{}", s)}>{props.casting_time.clone()}</div>
                    <div class={format!("flex-{}", s)}>
                        <div class={format!("svg-{}", s)}>{icon_checkbox(is_yes_or_no(props.ritual.clone()))}</div>
                        {" R "}
                    </div>
                </div>
                <div class={format!("casttime-label-{}", s)}>{"Casting Time"}</div>
                <div class={format!("range-{}", s)}>{props.range.clone()}</div>
                <div class={format!("range-label-{}", s)}>{"Range"}</div>
                <div class={format!("duration-{}", s)}>
                    <div class={format!("flex-{}", s)}>{props.duration.clone()}</div>
                    <div class={format!("flex-{}", s)}>
                        <div class={format!("svg-{}", s)}>{icon_checkbox(is_yes_or_no(props.concentration.clone()))}</div>
                        {" C "}
                    </div>
                </div>
                <div class={format!("duration-label-{}", s)}>{"Duration"}</div>
                <div class={format!("school-{}", s)}>{props.school.clone()}</div>
                <div class={format!("school-label-{}", s)}>{"School"}</div>
                <div class={format!("components-{}", s)}>
                    <div class={format!("flex-{}", s)}><div class={format!("svg-{}", s)}>{icon_checkbox(props.requires_verbal_components)}</div>{" V "}</div>
                    <div class={format!("flex-{}", s)}><div class={format!("svg-{}", s)}>{icon_checkbox(props.requires_somatic_components)}</div>{" S "}</div>
                    <div class={format!("flex-{}", s)}><div class={format!("svg-{}", s)}>{icon_checkbox(props.requires_material_components)}</div>{" M "}</div>
                </div>
                <div class={format!("components-label-{}", s)}>{"Components"}</div>
                <div class={format!("description-{}", s)}>{desc_html}</div>
                { if props.higher_level.len() > 0 {html!(<div class={format!("higherlevels-{}", s)}>{higher_level_html}</div>) } else {html!(<div></div>)} }
            </div>
        </div>
    }
}

