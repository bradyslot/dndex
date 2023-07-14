use gloo_net::http::Request;
use log::info;
use serde::de::DeserializeOwned;
use std::any::type_name;

use crate::models::open5e::*;
use crate::constants::*;

async fn fetch_endpoint<T: DeserializeOwned + PartialEq>(endpoint: &str) -> Vec<T> {
    let url = format!("{}/{}", API_URL, endpoint);
    let result = Request::get(&url)
        .send()
        .await;
    match result {
        Ok(response) => match response.json::<Open5eResults<T>>().await {
            Ok(api) => api.results,
            Err(e) => {
                info!("Error deserializing <{}>", type_name::<T>());
                info!("{}", e);
                vec![]
            }
        },
        Err(e) => {
            info!("Error fetching {} data from API", type_name::<T>());
            info!("{}", e);
            vec![]
        }
    }
}

pub async fn fetch_slugs<T: DeserializeOwned + PartialEq>(endpoint: &str, slugs: Vec<String>) -> Vec<T> {
    let mut results = vec![];

    for slug in slugs {
        let endpoint = format!("{}/?slug={}", endpoint, slug);
        let fetched_data = fetch_endpoint::<T>(&endpoint).await;
        results.extend(fetched_data);
    }

    results
}

// FETCH INDIVIDUAL ITEMS

pub async fn fetch_spells(spells: Vec<String>) -> Vec<Open5eSpell> {
    fetch_slugs::<Open5eSpell>("spells", spells).await
}

// pub async fn fetch_monsters(monsters: Vec<String>) -> Vec<Open5eMonster> {
//     fetch_slugs::<Open5eMonster>("monsters", monsters).await
// }

// pub async fn fetch_documents(document: Vec<String>) -> Vec<Open5eDocument> {
//     fetch_slugs::<Open5eDocument>("documents", documents).await
// }

pub async fn fetch_backgrounds(backgrounds: Vec<String>) -> Vec<Open5eBackground> {
    fetch_slugs::<Open5eBackground>("backgrounds", backgrounds).await
}

// pub async fn fetch_planes(planes: Vec<String>) -> Vec<Open5ePlane> {
//     fetch_slugs::<Open5ePlane>("planes", planes).await
// }

// pub async fn fetch_sections(sections: Vec<String>) -> Vec<Open5eSection> {
//     fetch_slugs::<Open5eSection>("sections", sections).await
// }

pub async fn fetch_feats(feats: Vec<String>) -> Vec<Open5eFeat> {
    fetch_slugs::<Open5eFeat>("feats", feats).await
}

// pub async fn fetch_conditions(conditions: String) -> Vec<Open5eCondition> {
//     fetch_slugs::<Open5eCondition>("conditions", conditions).await
// }

pub async fn fetch_races(races: Vec<String>) -> Vec<Open5eRace> {
    fetch_slugs::<Open5eRace>("races", races).await
}

pub async fn fetch_classes(classes: Vec<String>) -> Vec<Open5eClass> {
    fetch_slugs::<Open5eClass>("classes", classes).await
}

// pub async fn fetch_magicitems(magicitems: Vec<String>) -> Vec<Open5eMagicItem> {
//    fetch_slugs::<Open5eMagicItem>("magicitems", magicitems).await
// }

// pub async fn fetch_weapons(weapons: Vec<String>) -> Vec<Open5eWeapon> {
//    fetch_slugs::<Open5eWeapon>("weapons", weapons).await
// }

// pub async fn fetch_armor(armor: Vec<String>) -> Vec<Open5eArmor> {
//   fetch_slugs::<Open5eArmor>("armor", armor).await
// }

// FETCH ENTIRE ENDPOINTS

pub async fn fetch_all_spells() -> Vec<Open5eSpell> {
    fetch_endpoint::<Open5eSpell>("spells").await
}

// pub async fn fetch_all_monsters() -> Vec<Open5eMonster> {
//     fetch_endpoint::<Open5eMonster>("monsters").await
// }

// pub async fn fetch_all_documents() -> Vec<Open5eDocument> {
//     fetch_endpoint::<Open5eDocument>("documents").await
// }

pub async fn fetch_all_backgrounds() -> Vec<Open5eBackground> {
    fetch_endpoint::<Open5eBackground>("backgrounds").await
}

// pub async fn fetch_all_planes() -> Vec<Open5ePlane> {
//     fetch_endpoint::<Open5ePlane>("planes").await
// }

// pub async fn fetch_all_sections() -> Vec<Open5eSection> {
//    fetch_endpoint::<Open5eSection>("sections").await
// }

pub async fn fetch_all_feats() -> Vec<Open5eFeat> {
    fetch_endpoint::<Open5eFeat>("feats").await
}

// pub async fn fetch_all_conditions() -> Vec<Open5eCondition> {
//     fetch_endpoint::<Open5eCondition>("conditions").await
// }

pub async fn fetch_all_races() -> Vec<Open5eRace> {
    fetch_endpoint::<Open5eRace>("races").await
}

pub async fn fetch_all_classes() -> Vec<Open5eClass> {
    fetch_endpoint::<Open5eClass>("classes").await
}

// pub async fn fetch_all_magicitems() -> Vec<Open5eMagicItem> {
//     fetch_endpoint::<Open5eMagicItem>("magicitems").await
// }

// pub async fn fetch_all_weapons() -> Vec<Open5eWeapon> {
//     fetch_endpoint::<Open5eWeapon>("weapons").await
// }

// pub async fn fetch_all_armor() -> Vec<Open5eArmor> {
//     fetch_endpoint::<Open5eArmor>("armors").await
// }
