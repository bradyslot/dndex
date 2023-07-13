#![allow(non_snake_case, non_camel_case_types, clippy::similar_names)]
use yew::prelude::Properties;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct SRDEquipment {
    pub adventuring_gear: HashMap<String, AdventuringGear>,
    pub equipment_packs: HashMap<String, EquipmentPack>,
    pub mounts_and_vehicles: MountsAndVehicles,
    pub tools: Tools,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct AdventuringGear {
    pub name: String,
    pub desc: String,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct EquipmentPack {
    pub name: String,
    pub value: i32,
    pub denom: String,
    pub desc: String,
    pub contents: Vec<PackItem>,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct PackItem {
    pub name: Option<String>,
    pub key: Option<String>,
    pub qty: Option<i32>,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct MountsAndVehicles {
    pub desc: String,
    pub table: String,
    pub mounts: HashMap<String, Mount>,
    pub tack: HashMap<String, Tack>,
    pub drawn_vehicles: HashMap<String, DrawnVehicles>,
    pub waterborne_vehicles: HashMap<String, WaterborneVehicles>,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct Mount {
    pub name: String,
    pub cost: i32,
    pub denom: String,
    pub speed: i32,
    pub capacity: i32,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct Tack {
    pub name: String,
    pub cost: i32,
    pub denom: String,
    pub weight: i32,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct DrawnVehicles {
    pub name: String,
    pub cost: i32,
    pub denom: String,
    pub weight: i32,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct WaterborneVehicles {
    pub name: String,
    pub cost: i32,
    pub denom: String,
    pub speed: f32,
    pub speed_unit: String,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct Tools {
    pub desc: String,
    pub table: String,
    pub artisans_tools: ToolSets,
    pub gaming_sets: ToolSets,
    pub musical_instruments: ToolSets,
    pub kits: HashMap<String, Kits>,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct ToolSets {
    pub name: String,
    pub desc: String,
    pub subtypes: HashMap<String, ToolSubtypes>,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct ToolSubtypes {
    pub name: String,
    pub value: i32,
    pub denom: String,
    pub weight: i32,
}

#[derive(Deserialize, Clone, Properties, PartialEq, Debug)]
pub struct Kits {
    pub name: String,
    pub value: i32,
    pub denom: String,
    pub weight: i32,
    pub desc: String,
}
