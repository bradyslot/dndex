#![allow(non_snake_case, non_camel_case_types, clippy::similar_names)]
use yew::prelude::Properties;
use serde::Deserialize;

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct SRDMechanics {
    pub advancement: Vec<EachLevel>,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct EachLevel {
    pub level: u8,
    pub experience: u32,
    pub proficiency_bonus: u8,
}
