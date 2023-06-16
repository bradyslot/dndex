use stylist::yew::use_style;
use yew::prelude::*;
use super::shared::models::*;
use super::rectangle::*;
use super::super::constants::*;

#[function_component(HitPoints)]
pub fn hit_points(props: &HitPointProps) -> Html {
    let vars = format!(r#"
        --foreground: {}; "#
        , FOREGROUND
    );

    // grid-area: (row start) / (column start) / (row end) / (column end)
    let style = use_style!(
        r#"
            display: grid;
            height: 100%;
            width: 100%;
            padding: 0.5rem;
            grid-template-columns: 1fr;

            .grid-left {
                grid-area: 1 / 1 / 3 / 5;
            }

            .grid-right-top,
            .grid-right-bottom {
                border: 2px solid black;
                margin: 2rem 0 2rem 0;
                z-index: 1;
                background-color: var(--foreground);
                width: 10rem;
            }
            .grid-right-top {
                grid-area: 1 / 4 / 2 / 6;
            }
            .grid-right-bottom {
                grid-area: 2 / 4 / 3 / 6;
            }

            .absolute {
                display: flex;
                position: absolute;
                width: 100%;
            }

            .center {
                justify-content: center;
                align-items: center;
                text-align: center;
            }

            .label { 
                top: 0.5rem;
                font-size: 1.5rem;
            }

            .middle {
                align-self: center;
                font-size: 4rem;
            }

            .inspiration {
                width: 15rem;
                padding: 1rem;
                border-top: 2px solid black;
                border-right: 2px solid black;
                font-size: 1.5rem;
                bottom: 0;
                left: 0;
            }

            .adjust {
                transform: translate(0, 50%);
            }
        "#
    );

    html! {
        <div class={style} style={vars}>
            <div class="grid-left">
                <Rectangle>
                    <div class="absolute center label"> {"Current Hit Points"} </div>
                    <div class="absolute center middle"> {props.current} </div>
                    <div class="absolute center inspiration">
                        {"Inspiration: "}{props.inspiration}
                    </div>
                </Rectangle>
            </div>
            <div class="grid-right-top"> 
                <div class="center label"> {"Maximum"} </div>
                <div class="center middle adjust"> {props.max} </div>
            </div>
            <div class="grid-right-bottom">
                <div class="center label"> {"Temporary"} </div>
                <div class="center middle adjust"> {props.temp} </div>
            </div>
        </div>
    }
}

