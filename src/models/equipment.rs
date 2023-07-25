#![allow(non_snake_case, non_camel_case_types, clippy::similar_names)]
use yew::prelude::Properties;
use serde::Deserialize;
use std::collections::HashMap;
use crate::models::open5e::*;
use crate::api::open5e::*;
use crate::data::equipment::adventuring_gear::*;
use crate::data::equipment::equipment_packs::*;
use crate::data::equipment::tools::*;
use crate::data::equipment::mounts_and_vehicles::*;

#[derive(PartialEq, Debug)]
pub enum SRDEquipment {
    Open5eItem(SRDItem),
    Open5eCategory(SRDItem),
    DnDexItem(SRDItem),
    DnDexCategory(SRDItem),
    CustomItem(SRDCustomItem),
}

#[derive(PartialEq, Debug)]
pub enum FetchResult {
    Weapon(Open5eWeapon),
    Armor(Open5eArmor),
    WeaponCategory(Vec<Open5eWeapon>),
    ArmorCategory(Vec<Open5eArmor>),
    AdventuringGear(Option<&'static SRDAdventuringGearItem>),
    EquipmentPack(Option<&'static SRDEquipmentPack>),
    ToolKit(Option<&'static SRDToolKit>),
    MusicalInstrument(Option<&'static SRDToolSubtype>),
    GamingSet(Option<&'static SRDToolSubtype>),
    Mount(Option<&'static SRDMount>),
    Tack(Option<&'static SRDTack>),
    DrawnVehicle(Option<&'static SRDDrawnVehicle>),
    WaterborneVehicle(Option<&'static SRDWaterborneVehicle>),
    Empty(),
}

impl SRDEquipment {
    pub async fn fetch_contents(&self) -> FetchResult {
        match self {
            SRDEquipment::Open5eItem(item) => {
                match item.source {
                    "weapons" => FetchResult::Weapon(fetch_weapon(item.key.into()).await),
                    "armor" => FetchResult::Armor(fetch_armor(item.key.into()).await),
                    _ => FetchResult::Empty(),
                }
            }
            SRDEquipment::Open5eCategory(item) => {
                match item.source {
                    "weapons" => FetchResult::WeaponCategory(fetch_weapon_category(item.key.into()).await),
                    "armor" => FetchResult::ArmorCategory(fetch_armor_category(item.key.into()).await),
                    _ => FetchResult::Empty(),
                }
            }
            SRDEquipment::DnDexItem(item) => {
                match item.source {
                    "adventuring_gear" => FetchResult::AdventuringGear(adventuring_gear.get(item.key.into())),
                    "equipment_packs" => FetchResult::EquipmentPack(equipment_packs.get(item.key.into())),
                    "kits" => FetchResult::ToolKit(tools.kits.get(item.key.into())),
                    "musical_instruments" => FetchResult::MusicalInstrument(tools.musical_instruments.subtypes.get(item.key.into())),
                    "gaming_sets" => FetchResult::GamingSet(tools.gaming_sets.subtypes.get(item.key.into())),
                    "mount" => FetchResult::Mount(mounts_and_vehicles.mounts.get(item.key.into())),
                    "tack" => FetchResult::Tack(mounts_and_vehicles.tack.get(item.key.into())),
                    "drawn_vehicles" => FetchResult::DrawnVehicle(mounts_and_vehicles.drawn_vehicles.get(item.key.into())),
                    "waterborne_vehicles" => FetchResult::WaterborneVehicle(mounts_and_vehicles.waterborne_vehicles.get(item.key.into())),
                    _ => FetchResult::Empty(),
                }
            }
            SRDEquipment::DnDexCategory(item) => {
                match item.source {
                    // "musical_instruments" => FetchResult::MusicalInstrument(tools.musical_instruments.subtypes.iter().map(instrument).collect()),
                    _ => FetchResult::Empty(),
                }
            }
            SRDEquipment::CustomItem(item) => {
                unimplemented!()
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct SRDCustomItem {
    pub name: &'static str,
    pub qty: i32,
}

#[derive(PartialEq, Debug)]
pub struct SRDItem {
    pub key: &'static str,
    pub source: &'static str,
    pub qty: i32,
}

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
    pub contents: Vec<SRDEquipment>,
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

