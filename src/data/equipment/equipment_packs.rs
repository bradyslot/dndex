#![allow(unused, non_upper_case_globals)]
use crate::models::equipment::*;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref equipment_packs: HashMap<&'static str, SRDEquipmentPack> = {
    HashMap::from([
      (
        "burglars_pack",
        SRDEquipmentPack {
          name: "Burglar's Pack",
          value: 16,
          denom: "gp",
          desc: "Burglar's Pack (16 gp). Includes a backpack, a bag of 1,000 ball bearings, 10 feet of string, a bell, 5 candles, a crowbar, a hammer, 10 pitons, a hooded lantern, 2 flasks of oil, 5 days rations, a tinderbox, and a waterskin. The pack also has 50 feet of hempen rope strapped to the side of it.",
          contents: vec![
            SRDEquipment::CustomItem(SRDCustomItem { name: "Backpack", qty: 1, }),
            SRDEquipment::DnDexItem(SRDItem { key: "ball_bearings", source: "adventuring_gear", qty: 1000, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "String (feet)", qty: 10, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Bell", qty: 1, }),
            SRDEquipment::DnDexItem(SRDItem { key: "candle", source: "adventuring_gear", qty: 5, }),
            SRDEquipment::DnDexItem(SRDItem { key: "crowbar", source: "adventuring_gear", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Hammer", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Piton", qty: 10, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Hooded lantern", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Flasks of Oil", qty: 2, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Days of rations", qty: 5, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Tinderbox", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Waterskin", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Hempen rope (feet)", qty: 50, }),
          ]
        },
      ),
      (
        "diplomats_pack",
        SRDEquipmentPack {
          name: "Diplomat's Pack",
          value: 39,
          denom: "gp",
          desc: "Diplomat's Pack (39 gp). Includes a chest, 2 cases for maps and scrolls, a set of fine clothes, a bottle of ink, an ink pen, a lamp, 2 flasks of oil, 5 sheets of paper, a vial of perfume, sealing wax, and soap.",
          contents: vec![
            SRDEquipment::CustomItem(SRDCustomItem { name: "Chest", qty: 1, }),
            SRDEquipment::DnDexItem(SRDItem { key: "case_map_or_scroll", source: "adventuring_gear", qty: 2, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Set of fine clothes", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Bottle of ink", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Ink pen", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Lamp", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Flasks of Oil", qty: 2, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Sheets of paper", qty: 5, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Vial of perfume", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Sealing wax", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Soap", qty: 1, }),
          ]
        },
      ),
      (
        "dungeoneers_pack",
        SRDEquipmentPack {
          name: "Dungeoneer's Pack",
          value: 12,
          denom: "gp",
          desc: "Dungeoneer's Pack (12 gp). Includes a backpack, a crowbar, a hammer, 10 pitons, 10 torches, a tinderbox, 10 days of rations, and a waterskin. The pack also has 50 feet of hempen rope strapped to the side of it.",
          contents: vec![
            SRDEquipment::CustomItem(SRDCustomItem { name: "Backpack", qty: 1, }),
            SRDEquipment::DnDexItem(SRDItem { key: "Crowbar", source: "adventuring_gear", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Hammer", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Piton", qty: 10, }),
            SRDEquipment::DnDexItem(SRDItem { key: "Torch", source: "adventuring_gear", qty: 10, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Tinderbox", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Days of rations", qty: 10, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Waterskin", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Hempen rope (feet)", qty: 50, }),
          ]
        },
      ),
      (
        "entertainers_pack",
        SRDEquipmentPack {
          name: "Entertainer's Pack",
          value: 40,
          denom: "gp",
          desc: "Entertainer's Pack (40 gp). Includes a backpack, a bedroll, 2 costumes, 5 candles, 5 days of rations, a waterskin, and a disguise kit.",
          contents: vec![
            SRDEquipment::CustomItem(SRDCustomItem { name: "Backpack", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Bedroll", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Costumes", qty: 2, }),
            SRDEquipment::DnDexItem(SRDItem { key: "candle", source: "adventuring_gear", qty: 5, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Days of rations", qty: 5, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Waterskin", qty: 1, }),
            SRDEquipment::DnDexItem(SRDItem { key: "disguise_kit", source: "tools", qty: 1, }),
          ]
        },
      ),
      (
        "explorers_pack",
        SRDEquipmentPack {
          name: "Explorer's Pack",
          value: 10,
          denom: "gp",
          desc: "Explorer's Pack (10 gp). Includes a backpack, a bedroll, a mess kit, a tinderbox, 10 torches, 10 days of rations, and a waterskin. The pack also has 50 feet of hempen rope strapped to the side of it.",
          contents: vec![
            SRDEquipment::CustomItem(SRDCustomItem { name: "Backpack", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Bedroll", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Mess kit", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Tinderbox", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Torch", qty: 10, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Days of rations", qty: 10, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Waterskin", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Hempen rope (feet)", qty: 50, }),
          ]
        },
      ),
      (
        "priests_pack",
        SRDEquipmentPack {
          name: "Priest's Pack",
          value: 19,
          denom: "gp",
          desc: "Priest's Pack (19 gp). Includes a backpack, a blanket, 10 candles, a tinderbox, an alms box, 2 blocks of incense, a censer, vestments, 2 days of rations, and a waterskin.",
          contents: vec![
            SRDEquipment::CustomItem(SRDCustomItem { name: "Backpack", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Blanket", qty: 1, }),
            SRDEquipment::DnDexItem(SRDItem { key: "candle", source: "adventuring_gear", qty: 10, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Tinderbox", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Alms box", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Blocks of incense", qty: 2, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Censer", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Vestments", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Days of rations", qty: 2, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Waterskin", qty: 1, }),
          ]
        },
      ),
      (
        "scholars_pack",
        SRDEquipmentPack {
          name: "Scholar's Pack",
          value: 40,
          denom: "gp",
          desc: "Scholar's Pack (40 gp). Includes a backpack, a book of lore, a bottle of ink, an ink pen, 10 sheet of parchment, a little bag of sand, and a small knife.",
          contents: vec![
            SRDEquipment::CustomItem(SRDCustomItem { name: "Backpack", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Book of lore", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Bottle of ink", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Ink pen", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Sheet of parchment", qty: 10, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Little bag of sand", qty: 1, }),
            SRDEquipment::CustomItem(SRDCustomItem { name: "Small knife", qty: 1, }),
          ]
        }
      ),
    ])
  };
}
