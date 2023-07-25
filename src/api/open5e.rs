use gloo_net::http::Request;
use log::info;
use serde::de::DeserializeOwned;
use std::any::type_name;
use wasm_bindgen_test::console_log;

use crate::models::open5e::*;
use crate::constants::*;

pub async fn fetch_results<T: DeserializeOwned + PartialEq>(
    endpoint: &str,
    search: Option<String>,
    limit: Option<u32>,
    page: Option<u32>,
    filters: Option<Vec<String>>
) -> Vec<T> {
    let url = format!("{}/{}/", API_URL, endpoint);
    let mut query = String::new();
    match search {
        Some(search) => query.push_str(&format!("&search={}", search)),
        None => {}
    }
    match limit {
        Some(limit) => query.push_str(&format!("&limit={}", limit)),
        None => {}
    }
    match page {
        Some(page) => query.push_str(&format!("&page={}", page)),
        None => {}
    }
    match filters {
        Some(filters) => {
            for filter in filters {
                query.push_str(&format!("&{}", filter));
            }
        },
        None => {}
    }
    let url = format!("{}?{}", url, query);
    let result = Request::get(&url)
        .send()
        .await;
    match result {
        Ok(response) => match response.json::<Open5eResults<T>>().await {
            Err(e) => {
                info!("Error deserializing: <{}>", type_name::<T>());
                info!("{}", e);
                console_log!("Error deserializing: <{}>", type_name::<T>());
                console_log!("{}", e);
                vec![]
            },
            Ok(api) => api.results
        },
        Err(e) => {
            info!("Error fetching <{}> data from: {}", type_name::<T>(), url);
            info!("{}", e);
            console_log!("Error fetching <{}> data from: {}", type_name::<T>(), url);
            console_log!("{}", e);
            vec![]
        }
    }
}

pub async fn fetch_vec_slugs<T: DeserializeOwned + PartialEq>(endpoint: &str, slugs: Vec<String>) -> Vec<T> {
    let mut results = vec![];

    for slug in slugs {
        let endpoint = format!("{}/?slug={}", endpoint, slug);
        let fetched_data = fetch_results::<T>(&endpoint, None, None, None, None).await;
        results.extend(fetched_data);
    }

    results
}

pub async fn fetch_slug<T: DeserializeOwned + PartialEq + Clone + Default>(endpoint: &str, slug: String) -> T {
    let url = format!("{}/{}/{}/", API_URL, endpoint, slug);
    let result = Request::get(&url)
        .send()
        .await;
    match result {
        Ok(response) => match response.json::<T>().await {
            Err(e) => {
                info!("Error deserializing: {}<{}>", slug, type_name::<T>());
                info!("{}", e);
                console_log!("Error deserializing: {}<{}>", slug, type_name::<T>());
                console_log!("{}", e);
                // panic!("Deserializing {}<{}>", slug, type_name::<T>())
                T::default()
            },
            Ok(api) => api
        },
        Err(e) => {
            info!("Error fetching {}<{}> data from API", slug, type_name::<T>());
            info!("{}", e);
            console_log!("Error fetching: {}<{}>", slug, type_name::<T>());
            console_log!("{}", e);
            // panic!("Fetching {}<{}>", slug, type_name::<T>())
            T::default()
        }
    }
}

// GENERAL SEARCH
// searches everything using full terms

// example of alternative to endpoint searching by specifying route filter
// filters: vec![
//     "route=spells".to_string(),
//     "document_slug=wotc-srd".to_string(),
// ]
// this is likely more useful than endpoint searching
pub async fn search_general(search: String, limit: Option<u32>, page: Option<u32>, filters: Option<Vec<String>>) -> Vec<Open5eSearch> {
    fetch_results::<Open5eSearch>("search", Some(search), limit, page, filters).await
}

// SEARCH ENDPOINT
// only these endpoints implement the search feature at this time
// spells, monsters, backgrounds, magicitems, weapons, armor
// case insensitive and partial matches are supported

pub async fn search_spells(search: String, limit: Option<u32>, page: Option<u32>, filters: Option<Vec<String>>) -> Vec<Open5eSpell> {
    fetch_results::<Open5eSpell>("spells", Some(search), limit, page, filters).await
}

pub async fn search_monsters(search: String, limit: Option<u32>, page: Option<u32>, filters: Option<Vec<String>>) -> Vec<Open5eMonster> {
    fetch_results::<Open5eMonster>("monsters", Some(search), limit, page, filters).await
}

pub async fn search_backgrounds(search: String, limit: Option<u32>, page: Option<u32>, filters: Option<Vec<String>>) -> Vec<Open5eBackground> {
    fetch_results::<Open5eBackground>("backgrounds", Some(search), limit, page, filters).await
}

pub async fn search_magicitems(search: String, limit: Option<u32>, page: Option<u32>, filters: Option<Vec<String>>) -> Vec<Open5eMagicItem> {
    fetch_results::<Open5eMagicItem>("magicitems", Some(search), limit, page, filters).await
}

pub async fn search_weapons(search: String, limit: Option<u32>, page: Option<u32>, filters: Option<Vec<String>>) -> Vec<Open5eWeapon> {
    fetch_results::<Open5eWeapon>("weapons", Some(search), limit, page, filters).await
}

pub async fn search_armor(search: String, limit: Option<u32>, page: Option<u32>, filters: Option<Vec<String>>) -> Vec<Open5eArmor> {
    fetch_results::<Open5eArmor>("armor", Some(search), limit, page, filters).await
}

// FETCH SINGLE

pub async fn fetch_spell(spell: String) -> Open5eSpell {
    fetch_slug::<Open5eSpell>("spells", spell).await
}

pub async fn fetch_monster(monster: String) -> Open5eMonster {
    fetch_slug::<Open5eMonster>("monsters", monster).await
}

pub async fn fetch_background(background: String) -> Open5eBackground {
    fetch_slug::<Open5eBackground>("backgrounds", background).await
}

pub async fn fetch_feat(feat: String) -> Open5eFeat {
    fetch_slug::<Open5eFeat>("feats", feat).await
}

pub async fn fetch_condition(condition: String) -> Open5eCondition {
    fetch_slug::<Open5eCondition>("conditions", condition).await
}

pub async fn fetch_race(race: String) -> Open5eRace {
    fetch_slug::<Open5eRace>("races", race).await
}

pub async fn fetch_class(class: String) -> Open5eClass {
    fetch_slug::<Open5eClass>("classes", class).await
}

pub async fn fetch_magicitem(magicitem: String) -> Open5eMagicItem {
   fetch_slug::<Open5eMagicItem>("magicitems", magicitem).await
}

pub async fn fetch_weapon(weapon: String) -> Open5eWeapon {
   fetch_slug::<Open5eWeapon>("weapons", weapon).await
}

pub async fn fetch_armor(armor: String) -> Open5eArmor {
  fetch_slug::<Open5eArmor>("armor", armor).await
}

// FETCH VEC

pub async fn fetch_vec_spells(spells: Vec<String>) -> Vec<Open5eSpell> {
    fetch_vec_slugs::<Open5eSpell>("spells", spells).await
}

pub async fn fetch_vec_monsters(monsters: Vec<String>) -> Vec<Open5eMonster> {
    fetch_vec_slugs::<Open5eMonster>("monsters", monsters).await
}

pub async fn fetch_vec_backgrounds(backgrounds: Vec<String>) -> Vec<Open5eBackground> {
    fetch_vec_slugs::<Open5eBackground>("backgrounds", backgrounds).await
}

pub async fn fetch_vec_feats(feats: Vec<String>) -> Vec<Open5eFeat> {
    fetch_vec_slugs::<Open5eFeat>("feats", feats).await
}

pub async fn fetch_vec_conditions(conditions: Vec<String>) -> Vec<Open5eCondition> {
    fetch_vec_slugs::<Open5eCondition>("conditions", conditions).await
}

pub async fn fetch_vec_races(races: Vec<String>) -> Vec<Open5eRace> {
    fetch_vec_slugs::<Open5eRace>("races", races).await
}

pub async fn fetch_vec_classes(classes: Vec<String>) -> Vec<Open5eClass> {
    fetch_vec_slugs::<Open5eClass>("classes", classes).await
}

pub async fn fetch_vec_magicitems(magicitems: Vec<String>) -> Vec<Open5eMagicItem> {
   fetch_vec_slugs::<Open5eMagicItem>("magicitems", magicitems).await
}

pub async fn fetch_vec_weapons(weapons: Vec<String>) -> Vec<Open5eWeapon> {
   fetch_vec_slugs::<Open5eWeapon>("weapons", weapons).await
}

pub async fn fetch_vec_armor(armor: Vec<String>) -> Vec<Open5eArmor> {
  fetch_vec_slugs::<Open5eArmor>("armor", armor).await
}

// FETCH ALL

pub async fn fetch_all_spells() -> Vec<Open5eSpell> {
    fetch_results::<Open5eSpell>("spells", None, None, None, None).await
}

pub async fn fetch_all_monsters() -> Vec<Open5eMonster> {
    fetch_results::<Open5eMonster>("monsters", None, None, None, None).await
}

pub async fn fetch_all_documents() -> Vec<Open5eDocument> {
    fetch_results::<Open5eDocument>("documents", None, None, None, None).await
}

pub async fn fetch_all_backgrounds() -> Vec<Open5eBackground> {
    fetch_results::<Open5eBackground>("backgrounds", None, None, None, None).await
}

pub async fn fetch_all_feats() -> Vec<Open5eFeat> {
    fetch_results::<Open5eFeat>("feats", None, None, None, None).await
}

pub async fn fetch_all_conditions() -> Vec<Open5eCondition> {
    fetch_results::<Open5eCondition>("conditions", None, None, None, None).await
}

pub async fn fetch_all_races() -> Vec<Open5eRace> {
    fetch_results::<Open5eRace>("races", None, None, None, None).await
}

pub async fn fetch_all_classes() -> Vec<Open5eClass> {
    fetch_results::<Open5eClass>("classes", None, None, None, None).await
}

pub async fn fetch_all_magicitems() -> Vec<Open5eMagicItem> {
    fetch_results::<Open5eMagicItem>("magicitems", None, None, None, None).await
}

pub async fn fetch_all_weapons() -> Vec<Open5eWeapon> {
    fetch_results::<Open5eWeapon>("weapons", None, None, None, None).await
}

pub async fn fetch_all_armor() -> Vec<Open5eArmor> {
    fetch_results::<Open5eArmor>("armor", None, None, None, None).await
}

// FETCH BY CATEGORY
// these calls are expensive because they fetch all items and then filter them
// open5e-api v1 provides no way to filter by category

pub async fn fetch_armor_category(category: String) -> Vec<Open5eArmor> {
    let mut all_armor = fetch_all_armor().await;
    all_armor.iter().filter(|item| item.category.contains(&category)).cloned().collect()
}

pub async fn fetch_weapon_category(category: String) -> Vec<Open5eWeapon> {
    let mut all_weapons = fetch_all_weapons().await;
    all_weapons.iter().filter(|item| item.category.contains(&category)).cloned().collect()
}
