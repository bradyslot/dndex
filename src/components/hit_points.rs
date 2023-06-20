use stylist::{css, Style};
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;
use super::shared::icons::*;
use super::rectangle::*;

#[function_component(HitPoints)]
pub fn hit_points(props: &Character) -> Html {
    // grid-area: (row start) / (column start) / (row end) / (column end)
    let s = random_alpha_string(8);
    let css = css!(
        display: grid;
        width: 100%;
        grid-template-columns: 1fr;

        .grid-left-${s} {
            grid-area: 1 / 1 / 3 / 5;
        }

        .grid-right-top-${s},
        .grid-right-bottom-${s} {
            display: block;
            margin: auto;
            border: 2px solid black;
            background-color: var(--foreground);
            width: 10rem;
            z-index: 1;
        }
        .grid-right-top-${s} {
            grid-area: 1 / 4 / 2 / 6;
        }
        .grid-right-bottom-${s} {
            grid-area: 2 / 4 / 3 / 6;
        }

        .absolute-${s} {
            display: flex;
            position: absolute;
            width: 100%;
        }

        .center-${s} {
            justify-content: center;
            align-items: center;
            text-align: center;
        }

        .label-${s} {
            top: 0.5rem;
            font-size: 1.5rem;
            line-height: 2rem;
        }

        .middle-${s} {
            align-self: center;
            font-size: 5rem;
            height: 100%;
        }

        .bottom-left-${s} {
            display: flex;
            position: absolute;
            bottom: 0;
            left: 0;
            border-right: 2px solid black;
            border-top: 2px solid black;
        }

        .inspiration-${s} {
            display: flex;
            flex-grow: 2;
            padding: 2rem 2rem 2rem 0;
            font-size: 1.5rem;
            align-self: flex-end;
            line-height: 1.5rem;
        }

        .svg-${s} {
            display: flex;
            flex-grow: 1;
            padding: 2rem;
            width: 2rem;
            height: 2rem;
        }
    );
    let style = Style::new(css).expect("css no good");

    html! {
        <div class={style}>
            <div class={format!("grid-left-{}", s)}>
                <Rectangle label={"Current Hit Points"}>
                    <div class={format!("absolute-{} center-{} middle-{}", s, s, s)}> {props.hp.current} </div>
                    <div class={format!("bottom-left-{}", s)}>
                        <div class={format!("svg-{}", s)}>{icon_star(props.inspiration)}</div>
                        <div class={format!("inspiration-{}", s)}>{"Inspiration"}</div>
                    </div>
                </Rectangle>
            </div>
            <div class={format!("grid-right-top-{}", s)}> 
                <div class={format!("center-{} label-{}", s, s)}> {"Maximum"} </div>
                <div class={format!("center-{} middle-{}", s, s)}> {props.hp.max} </div>
            </div>
            <div class={format!("grid-right-bottom-{}", s)}>
                <div class={format!("center-{} label-{}", s, s)}> {"Temporary"} </div>
                <div class={format!("center-{} middle-{}", s, s)}> {props.hp.temp} </div>
            </div>
        </div>
    }
}

