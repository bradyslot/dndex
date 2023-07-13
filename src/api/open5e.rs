use crate::models::open5e::*;
use crate::constants::*;
use log::info;
use gloo_net::http::Request;

pub async fn fetch_races() -> Vec<Open5eRace> {
    let url = format!("{}/races/", API_URL);
    let result = Request::get(&url)
        .send()
        .await;
    match result {
        Ok(response) => match response.json::<Open5eEndpoint<Open5eRace>>().await {
            Ok(api) => api.results,
            Err(e) => {
                info!("Error deserializing Open5eEndpoint");
                info!("{}", e);
                vec![]
            }
        },
        Err(e) => {
            info!("Error fetching race data from API");
            info!("{}", e);
            vec![]
        }
    }
}

pub async fn fetch_classes() -> Vec<Open5eClass> {
    let url = format!("{}/classes/", API_URL);
    let result = Request::get(&url)
        .send()
        .await;
    match result {
        Ok(response) => match response.json::<Open5eEndpoint<Open5eClass>>().await {
            Ok(api) => api.results,
            Err(e) => {
                info!("Error deserializing Open5eEndpoint");
                info!("{}", e);
                vec![]
            }
        },
        Err(e) => {
            info!("Error fetching class data from API");
            info!("{}", e);
            vec![]
        }
    }
}

pub async fn fetch_spells(spells: Vec<String>) -> Vec<Open5eSpell> {
    let mut result_spells = vec![];

    for spell in spells {
        let url = format!("{}/spells/{}", API_URL, spell);
        let result = Request::get(&url)
            .send()
            .await;
        match result {
            Ok(response) => match response.json::<Open5eSpell>().await {
                Ok(api) => result_spells.push(api),
                Err(e) => {
                    info!("Error deserializing Open5eSpell");
                    info!("{}", e);
                }
            },
            Err(e) => {
                info!("Error fetching spell data from API");
                info!("{}", e);
            }
        }
    }

    result_spells
}
