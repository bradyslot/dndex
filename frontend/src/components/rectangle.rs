use stylist::yew::use_style;
use yew::prelude::*;
use super::shared::models::*;

#[function_component(Rectangle)]
pub fn rectangle(props: &RectangleProps) -> Html {
    let style = use_style!(
        r#"
            display: flex;
            flex-grow: 1;
            background-color: var(--foreground);
            position: relative;
            overflow: hidden;
            width: 100%;
            height: 100%;

            .border {
                position: absolute;
                width: calc(100% - 4px);
                height: calc(100% - 4px);
                border: 2px solid black;
            }

            .radius {
                position: absolute;
                width: 1rem;
                height: 1rem;
                border-radius: 100%;
                background-color: var(--background);
                border: 2px solid black;
            }

            .top { top: -0.5rem; }
            .bottom { bottom: -0.5rem; }
            .left { left: -0.5rem; }
            .right { right: -0.5rem; }

            .children {
                display: flex;
                position: relative;
                overflow: visible;
                width: 100%;
                height: 100%;
            }
        "#
    );

    html! {
        <div class={style}>
            <div class="border" />
            <div class="radius top left" />
            <div class="radius top right" />
            <div class="radius bottom left" />
            <div class="radius bottom right" />
            <div class="children">{for props.children.iter()}</div>
        </div>
    }
}

