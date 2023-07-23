#![allow(non_snake_case, non_camel_case_types, clippy::similar_names)]
use yew::prelude::Properties;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct SRDAdventuringGearItem {
    pub name: &'static str,
    pub desc: &'static str,
}

#[derive(PartialEq, Debug)]
pub struct SRDEquipmentPack {
    pub name: &'static str,
    pub value: i32,
    pub denom: &'static str,
    pub desc: &'static str,
    pub contents: Vec<SRDPackItem>,
}

#[derive(PartialEq, Debug)]
pub enum SRDPackItem {
    Gear(SRDKey),
    Tool(SRDKey),
    Custom(SRDCustom),
}

#[derive(PartialEq, Debug)]
pub struct SRDKey {
    pub key: &'static str,
    pub qty: i32,
}

#[derive(PartialEq, Debug)]
pub struct SRDCustom {
    pub name: &'static str,
    pub qty: i32,
}

#[derive(PartialEq, Debug)]
pub struct SRDTools {
    pub desc: &'static str,
    pub table: &'static str,
    pub artisans_tools: SRDToolSet,
    pub gaming_sets: SRDToolSet,
    pub musical_instruments: SRDToolSet,
    pub kits: HashMap<&'static str, SRDToolKit>,
}

#[derive(PartialEq, Debug)]
pub struct SRDToolSet {
    pub name: &'static str,
    pub desc: &'static str,
    pub subtypes: HashMap<&'static str, SRDToolSubtype>,
}

#[derive(PartialEq, Debug)]
pub struct SRDToolSubtype {
    pub name: &'static str,
    pub value: i32,
    pub denom: &'static str,
    pub weight: i32,
}

#[derive(PartialEq, Debug)]
pub struct SRDToolKit {
    pub name: &'static str,
    pub value: i32,
    pub denom: &'static str,
    pub weight: i32,
    pub desc: &'static str,
}

#[derive(PartialEq, Debug)]
pub struct SRDMountsAndVehicles {
    pub desc: &'static str,
    pub table: &'static str,
    pub mounts: HashMap<&'static str, SRDMount>,
    pub tack: HashMap<&'static str, SRDTack>,
    pub drawn_vehicles: HashMap<&'static str, SRDDrawnVehicle>,
    pub waterborne_vehicles: HashMap<&'static str, SRDWaterborneVehicle>,
}

#[derive(PartialEq, Debug)]
pub struct SRDMount {
    pub name: &'static str,
    pub cost: i32,
    pub denom: &'static str,
    pub speed: i32,
    pub speed_unit: &'static str,
    pub capacity: i32,
}

#[derive(PartialEq, Debug)]
pub struct SRDTack {
    pub name: &'static str,
    pub cost: i32,
    pub denom: &'static str,
    pub weight: i32,
}

#[derive(PartialEq, Debug)]
pub struct SRDDrawnVehicle {
    pub name: &'static str,
    pub cost: i32,
    pub denom: &'static str,
    pub weight: i32,
}

#[derive(PartialEq, Debug)]
pub struct SRDWaterborneVehicle {
    pub name: &'static str,
    pub cost: i32,
    pub denom: &'static str,
    pub speed: f32,
    pub speed_unit: &'static str,
}

