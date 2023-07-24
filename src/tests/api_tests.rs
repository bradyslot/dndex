use wasm_bindgen_test::*;
use crate::api::open5e::*;
use crate::models::open5e::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_search_spells() {
    let results = search_spells("".into(), Some(5), Some(1), Some(vec!["document__slug=wotc-srd".into()])).await;
    assert_eq!(results.len(), 5);
}

#[wasm_bindgen_test]
async fn test_search_monsters() {
    let results = search_monsters("".into(), Some(5), Some(1), Some(vec!["document__slug=wotc-srd".into()])).await;
    assert_eq!(results.len(), 5);
}

#[wasm_bindgen_test]
async fn test_search_backgrounds() {
    let results = search_backgrounds("".into(), Some(5), Some(1), None).await;
    assert_eq!(results.len(), 5);
}

#[wasm_bindgen_test]
async fn test_search_magicitems() {
    let results = search_magicitems("".into(), Some(5), Some(1), Some(vec!["document__slug=wotc-srd".into()])).await;
    assert_eq!(results.len(), 5);
}

#[wasm_bindgen_test]
async fn test_search_weapons() {
    let results = search_weapons("".into(), Some(5), Some(1), Some(vec!["document__slug=wotc-srd".into()])).await;
    assert_eq!(results.len(), 5);
}

#[wasm_bindgen_test]
async fn test_search_armor() {
    let results = search_weapons("".into(), Some(5), Some(1), Some(vec!["document__slug=wotc-srd".into()])).await;
    assert_eq!(results.len(), 5);
}

#[wasm_bindgen_test]
async fn test_armor_category() {
    let light_armor = fetch_armor_category("Light".into()).await;
    assert!(light_armor.len() > 0);
}

#[wasm_bindgen_test]
async fn test_weapon_category() {
    let martial_ranged = fetch_weapon_category("Martial Ranged".into()).await;
    assert!(martial_ranged.len() > 0);
}

#[wasm_bindgen_test]
async fn test_spells() {
    let spell = fetch_spell("fire-bolt".into()).await;
    let vec_spells = fetch_vec_spells(vec!["fire-bolt".into(), "cure-wounds".into()]).await;
    let all_spells = fetch_all_spells().await;
    assert_eq!(spell.name, "Fire Bolt");
}

#[wasm_bindgen_test]
async fn test_monsters() {
    let monster = fetch_monster("aboleth".into()).await;
    let vec_monsters = fetch_vec_monsters(vec!["aboleth".into(), "bandit-captain".into()]).await;
    let all_monsters = fetch_all_monsters().await;
    assert_eq!(monster.name, "Aboleth");
}

#[wasm_bindgen_test]
async fn test_backgrounds() {
    let background = fetch_background("acolyte".into()).await;
    let vec_backgrounds = fetch_vec_backgrounds(vec!["acolyte".into(), "artisan".into()]).await;
    let all_backgrounds = fetch_all_backgrounds().await;
    assert_eq!(background.name, "Acolyte");
}

#[wasm_bindgen_test]
async fn test_feats() {
    let feat = fetch_feat("grappler".into()).await;
    let vec_feats = fetch_vec_feats(vec!["grappler".into(), "athletic".into()]).await;
    let all_feats = fetch_all_feats().await;
    assert_eq!(feat.name, "Grappler");
}

#[wasm_bindgen_test]
async fn test_conditions() {
    let condition = fetch_condition("blinded".into()).await;
    let vec_conditions = fetch_vec_conditions(vec!["blinded".into(), "charmed".into()]).await;
    let all_conditions = fetch_all_conditions().await;
    assert_eq!(condition.name, "Blinded");
}

#[wasm_bindgen_test]
async fn test_races() {
    let race = fetch_race("dwarf".into()).await;
    let vec_races = fetch_vec_races(vec!["dwarf".into(), "elf".into()]).await;
    let all_races = fetch_all_races().await;
    assert_eq!(race.name, "Dwarf");
}

#[wasm_bindgen_test]
async fn test_classes() {
    let class = fetch_class("barbarian".into()).await;
    let vec_classes = fetch_vec_classes(vec!["barbarian".into(), "bard".into()]).await;
    let all_classes = fetch_all_classes().await;
    assert_eq!(class.name, "Barbarian");
}

#[wasm_bindgen_test]
async fn test_magicitems() {
    let magicitem = fetch_magicitem("adamantine-armor".into()).await;
    let vec_magicitems = fetch_vec_magicitems(vec!["adamantine-armor".into(), "amulet-of-health".into()]).await;
    let all_magicitems = fetch_all_magicitems().await;
    assert_eq!(magicitem.name, "Adamantine Armor");
}

#[wasm_bindgen_test]
async fn test_weapons() {
    let weapon = fetch_weapon("dagger".into()).await;
    let vec_weapons = fetch_vec_weapons(vec!["dagger".into(), "crossbow-light".into()]).await;
    let all_weapons = fetch_all_weapons().await;
    assert_eq!(weapon.name, "Dagger");
}

#[wasm_bindgen_test]
async fn test_armor() {
    let armor = fetch_armor("leather".into()).await;
    let vec_armor = fetch_vec_armor(vec!["unarmored".into(), "padded".into()]).await;
    let all_armor = fetch_all_armor().await;
    assert_eq!(armor.name, "Leather");
}
