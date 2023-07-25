use wasm_bindgen_test::*;
use crate::models::equipment::*;

wasm_bindgen_test_configure!(run_in_browser);

// #[wasm_bindgen_test]
// async fn test_fetch_weapon_contents() {
//     let item = SRDEquipment::Open5eItem(SRDItem { key: "dagger", source: "weapons", qty: 1 });
//     let fetched_item = item.fetch_contents().await;
//     let fetched_slug = match fetched_item {
//         FetchResult::Weapon(weapon) => weapon.slug,
//         _ => "".into(),
//     };
//     let expected_key = match item {
//         SRDEquipment::Open5eItem(item) => item.key,
//         _ => "".into(),
//     };
//     assert_eq!(expected_key, fetched_slug);
// }

// #[wasm_bindgen_test]
// async fn test_fetch_armor_contents() {
//     let item = SRDEquipment::Open5eItem(SRDItem { key: "leather", source: "armor", qty: 1 });
//     let fetched_item = item.fetch_contents().await;
//     let fetched_slug = match fetched_item {
//         FetchResult::Armor(armor) => armor.slug,
//         _ => "".into(),
//     };
//     let expected_key = match item {
//         SRDEquipment::Open5eItem(item) => item.key,
//         _ => "".into(),
//     };
//     assert_eq!(expected_key, fetched_slug);
// }

// #[wasm_bindgen_test]
// async fn test_fetch_weapon_category_contents() {
//     let category = SRDEquipment::Open5eCategory(SRDItem { key: "Martial Ranged", source: "weapons", qty: 1 });
//     let fetched_category = category.fetch_contents().await;
//     let fetched_category_length = match fetched_category {
//         FetchResult::WeaponCategory(weapons) => weapons.len(),
//         _ => 0,
//     };
//     assert!(fetched_category_length > 0);
// }

// #[wasm_bindgen_test]
// async fn test_fetch_armor_category_contents() {
//     let category = SRDEquipment::Open5eCategory(SRDItem { key: "Light", source: "armor", qty: 1 });
//     let fetched_category = category.fetch_contents().await;
//     let fetched_category_length = match fetched_category {
//         FetchResult::ArmorCategory(armor) => armor.len(),
//         _ => 0,
//     };
//     assert!(fetched_category_length > 0);
// }

// #[wasm_bindgen_test]
// async fn test_fetch_adventuring_gear_contents() {
//     let item = SRDEquipment::Open5eItem(SRDItem { key: "acid", source: "adventuring_gear", qty: 1 });
//     let fetched_item = item.fetch_contents().await;
//     console_log!("fetched_item: {:?}", fetched_item);
//     let fetched_name = match fetched_item {
//         FetchResult::AdventuringGear(Some(item)) => item.name,
//         _ => "".into(),
//     };
//     let expected_name = "Acid";
//     assert_eq!(expected_name, fetched_name);
// }

// #[derive(PartialEq, Debug)]
// pub enum SRDEquipment {
//     Open5eItem(SRDItem),
//     Open5eCategory(SRDItem),
//     DnDexItem(SRDItem),
//     DnDexCategory(SRDCategory),
//     CustomItem(SRDCustomItem),
// }

// #[derive(PartialEq, Debug)]
// pub enum FetchResult {
//     AdventuringGear(Option<&'static SRDAdventuringGearItem>),
//     EquipmentPack(Option<&'static SRDEquipmentPack>),
//     ArtisansTools(Option<&'static SRDToolSubtype>),
//     MusicalInstrument(Option<&'static SRDToolSubtype>),
//     GamingSet(Option<&'static SRDToolSubtype>),
//     ToolKit(Option<&'static SRDToolKit>),
//     Mount(Option<&'static SRDMount>),
//     Tack(Option<&'static SRDTack>),
//     DrawnVehicle(Option<&'static SRDDrawnVehicle>),
//     WaterborneVehicle(Option<&'static SRDWaterborneVehicle>),
//     MusicalInstrumentCategory(HashMap<&'static str, SRDToolSubtype>),
//     ArtisansToolsCategory(HashMap<&'static str, SRDToolSubtype>),
//     Custom(SRDCustomItem),
//     Empty,
// }
