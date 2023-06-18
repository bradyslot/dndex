use stylist::css;
use stylist::Style;
use yew::prelude::*;
use super::shared::models::*;
use super::shared::utils::*;

#[function_component(Rectangle)]
pub fn rectangle(props: &Child) -> Html {
    let s = random_alpha_string(8);
    let css = css!(
        r#"
            display: flex;
            flex-grow: 1;
            background-color: var(--foreground);
            position: relative;
            overflow: hidden;
            width: 100%;
            height: 100%;

            .border-${s} {
                position: absolute;
                width: calc(100% - 4px);
                height: calc(100% - 4px);
                border: 2px solid black;
            }

            .radius-${s} {
                position: absolute;
                width: 1rem;
                height: 1rem;
                border-radius: 100%;
                background-color: var(--background);
                border: 2px solid black;
            }

            .top-${s} { top: -0.5rem; }
            .bottom-${s} { bottom: -0.5rem; }
            .left-${s} { left: -0.5rem; }
            .right-${s} { right: -0.5rem; }

            .children-${s} {
                height: 100%;
                width: 100%;
            }
        "#, s = s,
    );
    let style = Style::new(css).expect("css no good");

    html! {
        <div class={style}>
            <div class={format!("border-{}", s)} />
            <div class={format!("radius-{} top-{} left-{}", s, s, s)} />
            <div class={format!("radius-{} top-{} right-{}", s, s, s)} />
            <div class={format!("radius-{} bottom-{} left-{}", s, s, s)} />
            <div class={format!("radius-{} bottom-{} right-{}", s, s, s)} />
            <div class={format!("children-{}", s)}>{for props.children.iter()}</div>
        </div>
    }
}

