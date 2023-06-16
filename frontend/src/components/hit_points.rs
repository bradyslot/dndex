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

    let style = use_style!(
        r#"
            display: flex;
            height: 100%;
            width: 80%;
            padding: 0.5rem;

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

            .box {
                width: 8rem;
                height: 6rem;
                border: 2px solid black;
                background-color: var(--foreground);
                font-size: 2.5rem;
                right: 0;
            }

            .upper { top: 1.5rem; }
            .lower { bottom: 1.5rem; }

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
                transform: translate(0, 1rem);
            }
        "#
    );

    html! {
        <div class={style} style={vars}>
            <Rectangle>
                <div class="absolute center label"> {"Current Hit Points"} </div>
                <div class="absolute center middle"> {props.current} </div>
                <div class="absolute center box upper"> 
                    <div class="absolute center label"> {"Maximum"} </div>
                    <div class="adjust"> {props.max} </div>
                </div>
                <div class="absolute center box lower">
                    <div class="absolute center label"> {"Temporary"} </div>
                    <div class="adjust"> {props.temp} </div>
                </div>
                <div class="absolute center inspiration">
                    {"Inspiration: "}{props.inspiration}
                </div>
            </Rectangle>
        </div>
    }
}

