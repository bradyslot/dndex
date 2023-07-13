#![allow(unused)]
pub const EQUIPMENT_PACKS_DATA: &str = r#"
{
  "burglars_pack": {
    "name": "Burglar's Pack",
    "value": 16,
    "denom": "gp",
    "desc": "Burglar's Pack (16 gp). Includes a backpack, a bag of 1,000 ball bearings, 10 feet of string, a bell, 5 candles, a crowbar, a hammer, 10 pitons, a hooded lantern, 2 flasks of oil, 5 days rations, a tinderbox, and a waterskin. The pack also has 50 feet of hempen rope strapped to the side of it.",
    "contents": [
      { "name": "Backpack" },
      { "key": "ball_bearings", "qty": 1000 },
      { "name": "String (feet)", "qty": 10 },
      { "name": "Bell" },
      { "key": "candle", "qty": 5 },
      { "key": "crowbar" },
      { "name": "Hammer" },
      { "name": "Piton", "qty": 10 },
      { "name": "Hooded lantern" },
      { "name": "Flasks of Oil", "qty": 2 },
      { "name": "Days of rations", "qty": 5 },
      { "name": "Tinderbox" },
      { "name": "Waterskin" },
      { "name": "Hempen rope (feet)", "qty": 50 }
    ]
  },
  "diplomats_pack": {
    "name": "Diplomat's Pack",
    "value": 39,
    "denom": "gp",
    "desc": "Diplomat's Pack (39 gp). Includes a chest, 2 cases for maps and scrolls, a set of fine clothes, a bottle of ink, an ink pen, a lamp, 2 flasks of oil, 5 sheets of paper, a vial of perfume, sealing wax, and soap.",
    "contents": [
      { "name": "Chest" },
      { "key": "case_map_or_scroll", "qty": 2 },
      { "name": "Set of fine clothes" },
      { "name": "Bottle of ink" },
      { "name": "Ink pen" },
      { "name": "Lamp" },
      { "name": "Flasks of Oil", "qty": 2 },
      { "name": "Sheets of paper", "qty": 5 },
      { "name": "Vial of perfume" },
      { "name": "Sealing wax" },
      { "name": "Soap" }
    ]
  },
  "dungeoneers_pack": {
    "name": "Dungeoneer's Pack",
    "value": 12,
    "denom": "gp",
    "desc": "Dungeoneer's Pack (12 gp). Includes a backpack, a crowbar, a hammer, 10 pitons, 10 torches, a tinderbox, 10 days of rations, and a waterskin. The pack also has 50 feet of hempen rope strapped to the side of it.",
    "contents": [
      { "name": "Backpack" },
      { "key": "crowbar" },
      { "name": "Hammer" },
      { "name": "Piton", "qty": 10 },
      { "name": "Torches", "qty": 10 },
      { "name": "Tinderbox" },
      { "name": "Days of rations", "qty": 10 },
      { "name": "Waterskin" },
      { "name": "Hempen rope (feet)", "qty": 50 }
    ]
  },
  "entertainers_pack": {
    "name": "Entertainer's Pack",
    "value": 40,
    "denom": "gp",
    "desc": "Entertainer's Pack (40 gp). Includes a backpack, a bedroll, 2 costumes, 5 candles, 5 days of rations, a waterskin, and a disguise kit.",
    "contents": [
      { "name": "Backpack" },
      { "name": "Bedroll" },
      { "name": "Costumes", "qty": 2 },
      { "name": "Candles", "qty": 5 },
      { "name": "Days of rations", "qty": 5 },
      { "name": "Waterskin" },
      { "name": "Disguise kit" }
    ]
  },
  "explorers_pack": {
    "name": "Explorer's Pack",
    "value": 10,
    "denom": "gp",
    "desc": "Explorer's Pack (10 gp). Includes a backpack, a bedroll, a mess kit, a tinderbox, 10 torches, 10 days of rations, and a waterskin. The pack also has 50 feet of hempen rope strapped to the side of it.",
    "contents": [
      { "name": "Backpack" },
      { "name": "Bedroll" },
      { "name": "Mess kit" },
      { "name": "Tinderbox" },
      { "name": "Torches", "qty": 10 },
      { "name": "Days of rations", "qty": 10 },
      { "name": "Waterskin" },
      { "name": "Hempen rope (feet)", "qty": 50 }
    ]
  },
  "priests_pack": {
    "name": "Priest's Pack",
    "value": 19,
    "denom": "gp",
    "desc": "Priest's Pack (19 gp). Includes a backpack, a blanket, 10 candles, a tinderbox, an alms box, 2 blocks of incense, a censer, vestments, 2 days of rations, and a waterskin.",
    "contents": [
      { "name": "Backpack" },
      { "name": "Blanket" },
      { "name": "Candles", "qty": 10 },
      { "name": "Tinderbox" },
      { "name": "Alms box" },
      { "name": "Blocks of incense", "qty": 2 },
      { "name": "Censer" },
      { "name": "Vestments" },
      { "name": "Days of rations", "qty": 2 },
      { "name": "Waterskin" }
    ]
  },
  "scholars_pack": {
    "name": "Scholar's Pack",
    "value": 40,
    "denom": "gp",
    "desc": "Scholar's Pack (40 gp). Includes a backpack, a book of lore, a bottle of ink, an ink pen, 10 sheet of parchment, a little bag of sand, and a small knife.",
    "contents": [
      { "name": "Backpack" },
      { "name": "Book of lore" },
      { "name": "Bottle of ink" },
      { "name": "Ink pen" },
      { "name": "Sheet of parchment", "qty": 10 },
      { "name": "Little bag of sand" },
      { "name": "Small knife" }
    ]
  }
}
"#;