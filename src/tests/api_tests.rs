use wasm_bindgen_test::*;
use crate::api::open5e::*;
use crate::models::open5e::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_spell() {
    let result = fetch_spell("fire-bolt".into()).await;
    // console_log!("spell: {:?}", result);
    assert_eq!(result.name, "Fire Bolt");
}

#[wasm_bindgen_test]
async fn test_monster() {
    let result = fetch_monster("aboleth".into()).await;
    console_log!("monster: {:?}", result);
    assert_eq!(result.name, "Aboleth");
}
