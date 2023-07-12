#![allow(unused)]
pub const MOUNTS_AND_VEHICLES_DATA: &str = r#"
{
  "desc": "A good mount can help you move more quickly through the wilderness, but its primary purpose is to carry the gear that would otherwise slow you down. The Mounts and Other Animals table shows each animal's speed and base carrying capacity.\nAn animal pulling a carriage, cart, chariot, sled, or wagon can move weight up to five times its base carrying capacity, including the weight of the vehicle. If multiple animals pull the same vehicle, they can add their carrying capacity together.\nMounts other than those listed here are available in fantasy gaming worlds, but they are rare and not normally available for purchase. These include flying mounts (pegasi, griffons, hippogriffs, and similar animals) and even aquatic mounts (giant sea horses, for example). Acquiring such a mount often means securing an egg and raising the creature yourself, making a bargain with a powerful entity, or negotiating with the mount itself.\n***Barding.*** Barding is armor designed to protect an animal's head, neck, chest, and body. Any type of armor shown on the Armor table can be purchased as barding. The cost is four times the equivalent armor made for humanoids, and it weighs twice as much.\n**Saddles.** A military saddle braces the rider, helping you keep your seat on an active mount in battle. It gives you advantage on any check you make to remain mounted. An exotic saddle is required for riding any aquatic or flying mount.\n***Vehicle Proficiency.*** If you have proficiency with a certain kind of vehicle (land or water), you can add your proficiency bonus to any check you make to control that kind of vehicle in difficult circumstances.\n***Rowed Vessels.*** Keelboats and rowboats are used on lakes and rivers. If going downstream, add the speed of the current (typically 3 miles per hour) to the speed of the vehicle. These vehicles can't be rowed against any significant current, but they can be pulled upstream by draft animals on the shores. A rowboat weighs 100 pounds, in case adventurers carry it over land.",
  "table": "**Mounts and Other Animals (table)**\n\n| Item | Cost | Speed | Carrying Capacity |\n| - | - | - | - |\n| Camel | 50 gp | 50 ft. | 480 lb. |\n| Donkey or mule | 8 gp | 40 ft. | 420 lb. |\n| Elephant | 200 gp | 40 ft. | 1,320 lb. |\n| Horse, draft | 50 gp | 40 ft. | 540 lb. |\n| Horse, riding | 75 gp | 60 ft. | 480 lb. |\n| Mastiff | 25 gp | 40 ft. | 195 lb. |\n| Pony | 30 gp | 40 ft. | 225 lb. |\n| Warhorse | 400 gp | 60 ft. | 540 lb. |\n\n**Tack, Harness, and Drawn Vehicles (table)**\n\n| Item | Cost | Weight |\n| - | - | - |\n| Barding | ×4 | ×2 |\n| Bit and bridle | 2 gp | 1 lb. |\n| Carriage | 100 gp | 600 lb. |\n| Cart | 15 gp | 200 lb. |\n| Chariot | 250 gp | 100 lb. |\n| Feed (per day) | 5 cp | 10 lb. |\n| Saddle | | |\n| - Exotic | 60 gp | 40 lb. |\n| - Military | 20 gp | 30 lb. |\n| - Pack | 5 gp | 15 lb. |\n| - Riding | 10 gp | 25 lb. |\n| Saddlebags | 4 gp | 8 lb. |\n| Sled | 20 gp | 300 lb. |\n| Stabling (per day) | 5 sp | - |\n| Wagon | 35 gp | 400 lb. |\n\n**Waterborne Vehicles (table)**\n\n| Item | Cost | Speed |\n| - | - | - |\n| Galley | 30,000 gp | 4 mph |\n| Keelboat | 3,000 gp | 1 mph |\n| Longship | 10,000 gp | 3 mph |\n| Rowboat | 50 gp | 1.5 mph |\n| Sailing ship | 10,000 gp | 2 mph |\n| Warship | 25,000 gp | 2.5 mph |\n",
  "mounts": {
    "camel": {
      "name": "Camel",
      "cost": 50,
      "denom": "gp",
      "speed": 50,
      "speed_unit": "ft",
      "capacity": 480
    },
    "donkey_mule": {
      "name": "Donkey/Mule",
      "cost": 8,
      "denom": "gp",
      "speed": 40,
      "speed_unit": "ft",
      "capacity": 420
    },
    "elephant": {
      "name": "Elephant",
      "cost": 200,
      "denom": "gp",
      "speed": 40,
      "speed_unit": "ft",
      "capacity": 1320
    },
    "horse_draft": {
      "name": "Horse, draft",
      "cost": 50,
      "denom": "gp",
      "speed": 40,
      "speed_unit": "ft",
      "capacity": 540
    },
    "horse_riding": {
      "name": "Horse, riding",
      "cost": 75,
      "denom": "gp",
      "speed": 60,
      "speed_unit": "ft",
      "capacity": 480
    },
    "mastiff": {
      "name": "Mastiff",
      "cost": 25,
      "denom": "gp",
      "speed": 40,
      "speed_unit": "ft",
      "capacity": 195
    },
    "pony": {
      "name": "Pony",
      "cost": 30,
      "denom": "gp",
      "speed": 40,
      "speed_unit": "ft",
      "capacity": 225
    },
    "warhorse": {
      "name": "Warhorse",
      "cost": 400,
      "denom": "gp",
      "speed": 60,
      "speed_unit": "ft",
      "capacity": 540
    }
  },
  "tack": {
    "bit_and_bridle": {
      "name": "Bit and bridle",
      "cost": 2,
      "denom": "gp",
      "weight": 1
    },
    "feed": {
      "name": "Feed (per day)",
      "cost": 5,
      "denom": "cp",
      "weight": 10
    },
    "stabling": {
      "name": "Stabling (per day)",
      "cost": 5,
      "denom": "sp",
      "weight": 0
    },
    "saddle_exotic": {
      "name": "Saddle, exotic",
      "cost": 60,
      "denom": "gp",
      "weight": 40
    },
    "saddle_military": {
      "name": "Saddle, military",
      "cost": 20,
      "denom": "gp",
      "weight": 30
    },
    "saddle_pack": {
      "name": "Saddle, pack",
      "cost": 5,
      "denom": "gp",
      "weight": 15
    },
    "saddle_riding": {
      "name": "Saddle, riding",
      "cost": 10,
      "denom": "gp",
      "weight": 25
    },
    "saddlebags": {
      "name": "Saddlebags",
      "cost": 4,
      "denom": "gp",
      "weight": 8
    }
  },
  "drawn_vehicles": {
    "carriage": {
      "name": "Carriage",
      "cost": 100,
      "denom": "gp",
      "weight": 600
    },
    "cart": {
      "name": "Cart",
      "cost": 15,
      "denom": "gp",
      "weight": 200
    },
    "chariot": {
      "name": "Chariot",
      "cost": 250,
      "denom": "gp",
      "weight": 100
    },
    "sled": {
      "name": "Sled",
      "cost": 20,
      "denom": "gp",
      "weight": 300
    },
    "wagon": {
      "name": "Wagon",
      "cost": 35,
      "denom": "gp",
      "weight": 400
    }
  },
  "waterborne_vehicles": {
    "galley": {
      "name": "Galley",
      "cost": 30000,
      "denom": "gp",
      "speed": 4,
      "speed_unit": "mph"
    },
    "keelboat": {
      "name": "Keelboat",
      "cost": 3000,
      "denom": "gp",
      "speed": 1,
      "speed_unit": "mph"
    },
    "longship": {
      "name": "Longship",
      "cost": 10000,
      "denom": "gp",
      "speed": 3,
      "speed_unit": "mph"
    },
    "rowboat": {
      "name": "Rowboat",
      "cost": 50,
      "denom": "gp",
      "speed": 1.5,
      "speed_unit": "mph"
    },
    "sailing_ship": {
      "name": "Sailing ship",
      "cost": 10000,
      "denom": "gp",
      "speed": 2,
      "speed_unit": "mph"
    },
    "warship": {
      "name": "Warship",
      "cost": 25000,
      "denom": "gp",
      "speed": 2.5,
      "speed_unit": "mph"
    }
  }
}
"#;
