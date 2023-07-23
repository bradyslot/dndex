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
            SRDPackItem::Custom(SRDCustom { name: "Backpack", qty: 1, }),
            SRDPackItem::Gear(SRDKey { key: "ball_bearings", qty: 1000, }),
            SRDPackItem::Custom(SRDCustom { name: "String (feet)", qty: 10, }),
            SRDPackItem::Custom(SRDCustom { name: "Bell", qty: 1, }),
            SRDPackItem::Gear(SRDKey { key: "candle", qty: 5, }),
            SRDPackItem::Gear(SRDKey { key: "crowbar", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Hammer", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Piton", qty: 10, }),
            SRDPackItem::Custom(SRDCustom { name: "Hooded lantern", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Flasks of Oil", qty: 2, }),
            SRDPackItem::Custom(SRDCustom { name: "Days of rations", qty: 5, }),
            SRDPackItem::Custom(SRDCustom { name: "Tinderbox", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Waterskin", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Hempen rope (feet)", qty: 50, }),
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
            SRDPackItem::Custom(SRDCustom { name: "Chest", qty: 1, }),
            SRDPackItem::Gear(SRDKey { key: "case_map_or_scroll", qty: 2, }),
            SRDPackItem::Custom(SRDCustom { name: "Set of fine clothes", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Bottle of ink", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Ink pen", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Lamp", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Flasks of Oil", qty: 2, }),
            SRDPackItem::Custom(SRDCustom { name: "Sheets of paper", qty: 5, }),
            SRDPackItem::Custom(SRDCustom { name: "Vial of perfume", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Sealing wax", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Soap", qty: 1, }),
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
            SRDPackItem::Custom(SRDCustom { name: "Backpack", qty: 1, }),
            SRDPackItem::Gear(SRDKey { key: "Crowbar", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Hammer", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Piton", qty: 10, }),
            SRDPackItem::Gear(SRDKey { key: "Torch", qty: 10, }),
            SRDPackItem::Custom(SRDCustom { name: "Tinderbox", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Days of rations", qty: 10, }),
            SRDPackItem::Custom(SRDCustom { name: "Waterskin", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Hempen rope (feet)", qty: 50, }),
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
            SRDPackItem::Custom(SRDCustom { name: "Backpack", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Bedroll", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Costumes", qty: 2, }),
            SRDPackItem::Gear(SRDKey { key: "candle", qty: 5, }),
            SRDPackItem::Custom(SRDCustom { name: "Days of rations", qty: 5, }),
            SRDPackItem::Custom(SRDCustom { name: "Waterskin", qty: 1, }),
            SRDPackItem::Tool(SRDKey { key: "disguise_kit", qty: 1, }),
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
            SRDPackItem::Custom(SRDCustom { name: "Backpack", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Bedroll", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Mess kit", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Tinderbox", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Torch", qty: 10, }),
            SRDPackItem::Custom(SRDCustom { name: "Days of rations", qty: 10, }),
            SRDPackItem::Custom(SRDCustom { name: "Waterskin", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Hempen rope (feet)", qty: 50, }),
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
            SRDPackItem::Custom(SRDCustom { name: "Backpack", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Blanket", qty: 1, }),
            SRDPackItem::Gear(SRDKey { key: "candle", qty: 10, }),
            SRDPackItem::Custom(SRDCustom { name: "Tinderbox", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Alms box", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Blocks of incense", qty: 2, }),
            SRDPackItem::Custom(SRDCustom { name: "Censer", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Vestments", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Days of rations", qty: 2, }),
            SRDPackItem::Custom(SRDCustom { name: "Waterskin", qty: 1, }),
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
            SRDPackItem::Custom(SRDCustom { name: "Backpack", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Book of lore", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Bottle of ink", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Ink pen", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Sheet of parchment", qty: 10, }),
            SRDPackItem::Custom(SRDCustom { name: "Little bag of sand", qty: 1, }),
            SRDPackItem::Custom(SRDCustom { name: "Small knife", qty: 1, }),
          ]
        }
      ),
    ])
  };
}
