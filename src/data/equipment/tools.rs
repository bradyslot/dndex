#![allow(unused, non_upper_case_globals)]
use crate::models::equipment::*;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref tools: SRDTools = SRDTools {
    desc: "A tool helps you to do something you couldn't otherwise do, such as craft or repair an item, forge a document, or pick a lock. Your race, class, background, or feats give you proficiency with certain tools. Proficiency with a tool allows you to add your proficiency bonus to any ability check you make using that tool. Tool use is not tied to a single ability, since proficiency with a tool represents broader knowledge of its use. For example, the GM might ask you to make a Dexterity check to carve a fine detail with your woodcarver's tools, or a Strength check to make something out of particularly hard wood.",
    table: "| Item | Cost | Weight |\n|_|-|-|\n| Artisan's tools | | |\n| - Alchemist's supplies | 50 gp | 8 lb. |\n| - Brewer's supplies | 20 gp | 9 lb. |\n| - Calligrapher's supplies | 10 gp | 5 lb. |\n| - Carpenter's tools | 8 gp | 6 lb. |\n| - Cartographer's tools | 15 gp | 6 lb. |\n| - Cobbler's tools | 5 gp | 5 lb. |\n| - Cook's utensils | 1 gp | 8 lb. |\n| - Glassblower's tools | 30 gp | 5 lb. |\n| - Jeweler's tools | 25 gp | 2 lb. |\n| - Leatherworker's tools | 5 gp | 5 lb. |\n| - Mason's tools | 10 gp | 8 lb. |\n| - Painter's supplies | 10 gp | 5 lb. |\n| - Potter's tools | 10 gp | 3 lb. |\n| - Smith's tools | 20 gp | 8 lb. |\n| - Tinker's tools | 50 gp | 10 lb. |\n| - Weaver's tools | 1 gp | 5 lb. |\n| - Woodcarver's tools | 1 gp | 5 lb. |\n| Disguise kit | 25 gp | 3 lb. |\n| Forgery kit | 15 gp | 5 lb. |\n| Gaming set | | |\n| - Dice set | 1 sp | |\n| - Playing card set | 5 sp | |\n| Herbalism kit | 5 gp | 3 lb. |\n| Musical instrument | | |\n| - Bagpipes | 30 gp | 6 lb. |\n| - Drum | 6 gp | 3 lb. |\n| - Dulcimer | 25 gp | 10 lb. |\n| - Flute | 2 gp | 1 lb. |\n| - Lute | 35 gp | 2 lb. |\n| - Lyre | 30 gp | 2 lb. |\n| - Horn | 3 gp | 2 lb. |\n| - Pan flute | 12 gp | 2 lb. |\n| - Shawm | 2 gp | 1 lb. |\n| - Viol | 30 gp | 1 lb. |\n| Navigator's tools | 25 gp | 2 lb. |\n| Poisoner's kit | 50 gp | 2 lb. |\n| Thieves' tools | 25 gp | 1 lb. |\n",
    artisans_tools: SRDToolSet {
      name: "Atisan's Tools",
      desc: "These special tools include the items needed to pursue a craft or trade. The table shows examples of the most common types of tools, each providing items related to a single craft. Proficiency with a set of artisan's tools lets you add your proficiency bonus to any ability checks you make using the tools in your craft. Each type of artisan's tools requires a separate proficiency.",
      subtypes: HashMap::from([
        (
          "alchemists_supplies",
          SRDToolSubtype {
            name: "Alchemist's Supplies",
            value: 50,
            denom: "gp",
            weight: 8
          },
        ),
        (
          "brewers_supplies",
          SRDToolSubtype {
            name: "Brewer's Supplies",
            value: 20,
            denom: "gp",
            weight: 9
          },
        ),
        (
          "calligraphers_supplies",
          SRDToolSubtype {
            name: "Calligrapher's Supplies",
            value: 10,
            denom: "gp",
            weight: 5
          },
        ),
        (
          "carpenters_tools",
          SRDToolSubtype {
            name: "Carpenter's Tools",
            value: 8,
            denom: "gp",
            weight: 6
          },
        ),
        (
          "cartographers_tools",
          SRDToolSubtype {
            name: "Cartographer's Tools",
            value: 15,
            denom: "gp",
            weight: 6
          },
        ),
        (
          "cobblers_tools",
          SRDToolSubtype {
            name: "Cobbler's Tools",
            value: 5,
            denom: "gp",
            weight: 5
          },
        ),
        (
          "cooks_utensils",
          SRDToolSubtype {
            name: "Cook's Utensils",
            value: 1,
            denom: "gp",
            weight: 8
          },
        ),
        (
          "glassblowers_tools",
          SRDToolSubtype {
            name: "Glassblower's Tools",
            value: 30,
            denom: "gp",
            weight: 5
          },
        ),
        (
          "jewelers_tools",
          SRDToolSubtype {
            name: "Jeweler's Tools",
            value: 25,
            denom: "gp",
            weight: 2
          },
        ),
        (
          "leatherworkers_tools",
          SRDToolSubtype {
            name: "Leatherworker's Tools",
            value: 5,
            denom: "gp",
            weight: 5
          },
        ),
        (
          "masons_tools",
          SRDToolSubtype {
            name: "Mason's Tools",
            value: 10,
            denom: "gp",
            weight: 8
          },
        ),
        (
          "painters_supplies",
          SRDToolSubtype {
            name: "Painter's Supplies",
            value: 10,
            denom: "gp",
            weight: 5
          },
        ),
        (
          "potters_tools",
          SRDToolSubtype {
            name: "Potter's Tools",
            value: 10,
            denom: "gp",
            weight: 3
          },
        ),
        (
          "smiths_tools",
          SRDToolSubtype {
            name: "Smith's Tools",
            value: 20,
            denom: "gp",
            weight: 8
          },
        ),
        (
          "tinkers_tools",
          SRDToolSubtype {
            name: "Tinker's Tools",
            value: 50,
            denom: "gp",
            weight: 10
          },
        ),
        (
          "weavers_tools",
          SRDToolSubtype {
            name: "Weaver's Tools",
            value: 1,
            denom: "gp",
            weight: 5
          },
        ),
        (
          "woodcarvers_tools",
          SRDToolSubtype {
            name: "Woodcarver's Tools",
            value: 1,
            denom: "gp",
            weight: 5
          }
        ),
      ])
    },
    gaming_sets: SRDToolSet {
      name: "Gaming Set",
      desc: "This item encompasses a wide range of game pieces, including dice and decks of cards (for games such as Three_Dragon Ante). A few common examples appear on the Tools table, but other kinds of gaming sets exist. If you are proficient with a gaming set, you can add your proficiency bonus to ability checks you make to play a game with that set. Each type of gaming set requires a separate proficiency.",
      subtypes: HashMap::from([
        (
          "dice_set",
          SRDToolSubtype {
            name: "Dice Set",
            value: 1,
            denom: "sp",
            weight: 0
          },
        ),
        (
          "playing_card_set",
          SRDToolSubtype {
            name: "Playing Card Set",
            value: 5,
            denom: "sp",
            weight: 0
          }
        ),
      ])
    },
    musical_instruments: SRDToolSet {
      name: "Musical Instrument",
      desc: "Several of the most common types of musical instruments are shown on the table as examples. If you have proficiency with a given musical instrument, you can add your proficiency bonus to any ability checks you make to play music with the instrument. A bard can use a musical instrument as a spellcasting focus. Each type of musical instrument requires a separate proficiency.",
      subtypes: HashMap::from([
        (
          "bagpipes",
          SRDToolSubtype {
            name: "Bagpipes",
            value: 30,
            denom: "gp",
            weight: 6
          },
        ),
        (
          "drum",
          SRDToolSubtype {
            name: "Drum",
            value: 6,
            denom: "gp",
            weight: 3
          },
        ),
        (
          "dulcimer",
          SRDToolSubtype {
            name: "Dulcimer",
            value: 25,
            denom: "gp",
            weight: 10
          },
        ),
        (
          "flute",
          SRDToolSubtype {
            name: "Flute",
            value: 2,
            denom: "gp",
            weight: 1
          },
        ),
        (
          "lute",
          SRDToolSubtype {
            name: "Lute",
            value: 35,
            denom: "gp",
            weight: 2
          },
        ),
        (
          "lyre",
          SRDToolSubtype {
            name: "Lyre",
            value: 30,
            denom: "gp",
            weight: 2
          },
        ),
        (
          "horn",
          SRDToolSubtype {
            name: "Horn",
            value: 3,
            denom: "gp",
            weight: 2
          },
        ),
        (
          "pan_flute",
          SRDToolSubtype {
            name: "Pan Flute",
            value: 12,
            denom: "gp",
            weight: 2
          },
        ),
        (
          "shawm",
          SRDToolSubtype {
            name: "Shawm",
            value: 2,
            denom: "gp",
            weight: 1
          },
        ),
        (
          "viol",
          SRDToolSubtype {
            name: "Viol",
            value: 30,
            denom: "gp",
            weight: 1
          }
        ),
      ])
    },
    kits: HashMap::from([
      (
        "disguise_kit",
        SRDToolKit {
          name: "Disguise Kit",
          value: 25,
          denom: "gp",
          weight: 3,
          desc: "This pouch of cosmetics, hair dye, and small props lets you create disguises that change your physical appearance. Proficiency with this kit lets you add your proficiency bonus to any ability checks you make to create a visual disguise."
        },
      ),
      (
        "forgery_kit",
        SRDToolKit {
          name: "Forgery Kit",
          value: 15,
          denom: "gp",
          weight: 5,
          desc: "This small box contains a variety of papers and parchments, pens and inks, seals and sealing wax, gold and silver leaf, and other supplies necessary to create convincing forgeries of physical documents. Proficiency with this kit lets you add your proficiency bonus to any ability checks you make to create a physical forgery of a document."
        },
      ),
      (
        "herbalism_kit",
        SRDToolKit {
          name: "Herbalism Kit",
          value: 5,
          denom: "gp",
          weight: 3,
          desc: "This kit contains a variety of instruments such as clippers, mortar and pestle, and pouches and vials used by herbalists to create remedies and potions. Proficiency with this kit lets you add your proficiency bonus to any ability checks you make to identify or apply herbs. Also, proficiency with this kit is required to create antitoxin and potions of healing."
        },
      ),
      (
        "navigator_tools",
        SRDToolKit {
          name: "Navigator's Tools",
          value: 25,
          denom: "25",
          weight: 2,
          desc: "This set of instruments is used for navigation at sea. Proficiency with navigator's tools lets you chart a ship's course and follow navigation charts. In addition, these tools allow you to add your proficiency bonus to any ability check you make to avoid getting lost at sea."
        },
      ),
      (
        "poisoner_kit",
        SRDToolKit {
          name: "Poisoner's Kit",
          value: 50,
          denom: "gp",
          weight: 2,
          desc: "A poisoner's kit includes the vials, chemicals, and other equipment necessary for the creation of poisons. Proficiency with this kit lets you add your proficiency bonus to any ability checks you make to craft or use poisons."
        },
      ),
      (
        "thieves_tools",
        SRDToolKit {
          name: "Thieves' Tools",
          value: 25,
          denom: "gp",
          weight: 1,
          desc: "This set of tools includes a small file, a set of lock picks, a small mirror mounted on a metal handle, a set of narrow-bladed scissors, and a pair of pliers. Proficiency with these tools lets you add your proficiency bonus to any ability checks you make to disarm traps or open locks."
        }
      )
    ])
  };
}
