use stylist::{css, Style};
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;
use super::rectangle::*;

#[function_component(HitPoints)]
pub fn hit_points(props: &Character) -> Html {
    // grid-area: (row start) / (column start) / (row end) / (column end)
    let s = random_alpha_string(8);
    let css = css!(
        r#"
            display: grid;
            height: 100%;
            width: 100%;
            padding: 0.5rem;
            grid-template-columns: 1fr;

            .grid-left-${s} {
                grid-area: 1 / 1 / 3 / 5;
            }

            .grid-right-top-${s},
            .grid-right-bottom-${s} {
                border: 2px solid black;
                margin: 2rem 0 2rem 0;
                z-index: 1;
                background-color: var(--foreground);
                width: 10rem;
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
                font-size: 4rem;
                height: 100%;
            }

            .inspiration-${s} {
                width: 15rem;
                padding: 1rem;
                border-top: 2px solid black;
                border-right: 2px solid black;
                font-size: 1.5rem;
                bottom: 0;
                left: 0;
            }

            .adjust-${s} {
                transform: translate(0, 15%);
            }
        "#, s = s,
    );
    let style = Style::new(css).expect("css no good");

    html! {
        <div class={style}>
            <div class={format!("grid-left-{}", s)}>
                <Rectangle>
                    <div class={format!("absolute-{} center-{} label-{}", s, s, s)}> {"Current Hit Points"} </div>
                    <div class={format!("absolute-{} center-{} middle-{}", s, s, s)}> {props.hp.current} </div>
                    <div class={format!("absolute-{} center-{} inspiration-{}", s, s, s)}>
                        {"Inspiration: "}{props.hp.inspiration}
                    </div>
                </Rectangle>
            </div>
            <div class={format!("grid-right-top-{}", s)}> 
                <div class={format!("center-{} label-{}", s, s)}> {"Maximum"} </div>
                <div class={format!("center-{} middle-{} adjust-{}", s, s, s)}> {props.hp.max} </div>
            </div>
            <div class={format!("grid-right-bottom-{}", s)}>
                <div class={format!("center-{} label-{}", s, s)}> {"Temporary"} </div>
                <div class={format!("center-{} middle-{} adjust-{}", s, s, s)}> {props.hp.temp} </div>
            </div>
        </div>
    }
}

