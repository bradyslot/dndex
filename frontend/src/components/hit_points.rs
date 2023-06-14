use stylist::yew::use_style;
use yew::prelude::*;
use super::shared::models::*;
use super::rectangle::*;

#[function_component(HitPoints)]
pub fn hit_points(props: &HitPointProps) -> Html {
    let style = use_style!(
        r#"
            display: flex;
            flex-direction: column;
            flex-grow: 1;
            margin-bottom: 3rem;
        "#
    );

    html! {
        <div class={style}>
            <Rectangle height={"100%"} width={"100%"} />
            <div> {props.current} </div>
            <div> {props.max} </div>
            <div> {props.temp} </div>
            <div> {props.inspiration} </div>
        </div>
    }
}

