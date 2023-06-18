use stylist::{css, Style};
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;
use super::rectangle::*;

#[function_component(Speed)]
pub fn speed(props: &Character) -> Html {
    let s = random_alpha_string(8);
    let css = css!(
        display: grid;
        grid-template-rows: 1fr;
        width: 10rem;
        height: 10rem;

        .upper-${s} {
            grid-area: 1 / 1 / 3 / 1;
        }

        .lower-${s} {
            grid-area: 2 / 1 / 4 / 1;
            z-index: 1;
        }

        .absolute-${s} {
            display: flex;
            position: absolute;
            width: 100%;
        }
        
        .modifier-${s} {
            margin: 0 2rem 0 2rem;
            border: 2px solid black;
            background-color: var(--foreground);
        }

        .label-${s} {
            top: 0.5rem;
            font-size: 1.5rem;
        }

        .middle-${s} {
            align-self: center;
            font-size: 3rem;
            height: 100%;
        }

        .center-${s} {
            text-align: center;
            justify-content: center;
            align-items: center;
        }
    );
    let style = Style::new(css).expect("css no good");

    html! {
        <div class={style}>
            <div class={format!("upper-{}", s)}>
                <Rectangle>
                    <div class={format!("absolute-{} center-{} label-{}", s, s, s)}>{"Speed"}</div>
                    <div class={format!("absolute-{} center-{} middle-{}", s, s, s)}>{props.speed.base}</div>
                </Rectangle>
            </div>
            <div class={format!("lower-{}", s)}>
                <div class={format!("modifier-{} center-{} middle-{}", s, s, s)}>{props.speed.modifier}</div>
            </div>
        </div>
    }
}
