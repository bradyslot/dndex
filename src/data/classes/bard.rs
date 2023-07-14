#![allow(unused, non_upper_case_globals)]
use crate::models::classes::*;
use std::collections::HashMap; 
use lazy_static::lazy_static;

lazy_static! {
  pub static ref bard: SRDClass<SRDBardAttributes> = SRDClass::<SRDBardAttributes> {
    hit_points: SRDClassHitPoints {
      hit_dice: 8,
      static_option: 5,
      desc: "**Hit Dice:** 1d8 per bard level\n**Hit Points at 1st Level:** 8 + your Constitution modifier\n**Hit Points at Higher Levels:** 1d8 (or 5) + your Constitution modifier per bard level after 1st".to_string()
    },
    proficiencies: SRDClassProficiencies {
      armor: vec![
        SRDClassEquipmentItem {
          category: Some("Light".to_string()), source: Some("open5e".to_string()), location: Some("armor".to_string()),
          name: None, key: None, qty: None
        }
      ],
      weapons: vec![
        SRDClassEquipmentItem {
          category: Some("Simple".to_string()), source: Some("open5e".to_string()), location: Some("weapons".to_string()),
          name: None, key: None, qty: None
        },
        SRDClassEquipmentItem {
          key: Some("crossbow-hand".to_string()), source: Some("open5e".to_string()), location: Some("weapons".to_string()),
          name: None, category: None, qty: None
        },
        SRDClassEquipmentItem {
          key: Some("longsword".to_string()), source: Some("open5e".to_string()), location: Some("weapons".to_string()),
          name: None, category: None, qty: None
        },
        SRDClassEquipmentItem {
          key: Some("rapier".to_string()), source: Some("open5e".to_string()), location: Some("weapons".to_string()),
          name: None, category: None, qty: None
        },
        SRDClassEquipmentItem {
          key: Some("shortsword".to_string()), source: Some("open5e".to_string()), location: Some("weapons".to_string()),
          name: None, category: None, qty: None
        },
      ],
      tools: vec![
        vec![
          SRDClassEquipmentItem {
            key: Some("musical_instrument".to_string()), source: Some("data".to_string()), location: Some("tools".to_string()),
            name: None, category: None, qty: None
          },
        ]
      ],
      saving_throws: vec![ "dexterity".to_string(), "charisma".to_string() ],
      skills: SRDClassSkills {
        choices: 3,
        options: vec![]
      },
      desc: "**Armor:** Light armor\n**Weapons:** Simple weapons, hand crossbows, longswords, rapiers, shortswords\n**Tools:** Three musical instruments of your choice\n**Saving Throws:** Dexterity, Charisma\n**Skills:** Choose any three".to_string()
    },
    equipment: SRDClassStartingEquipment {
      choice_1: vec![
        vec![
          SRDClassEquipmentItem {
            source: Some("open5e".to_string()), location: Some("weapons".to_string()), key: Some("rapier".to_string()),
            name: None, category: None, qty: None
          }
        ],
        vec![
          SRDClassEquipmentItem {
            source: Some("open5e".to_string()), location: Some("weapons".to_string()), key: Some("longsword".to_string()),
            name: None, category: None, qty: None
          }
        ],
        vec![
          SRDClassEquipmentItem {
            source: Some("open5e".to_string()), location: Some("weapons".to_string()), category: Some("Simple Melee".to_string()),
            name: None, key: None, qty: None
          }
        ]
      ],
      choice_2: vec![
        vec![
          SRDClassEquipmentItem {
            source: Some("data".to_string()), location: Some("equipment_packs".to_string()), key: Some("diplomats_pack".to_string()),
            name: None, category: None, qty: None
          }
        ],
        vec![
          SRDClassEquipmentItem {
            source: Some("data".to_string()), location: Some("equipment_packs".to_string()), key: Some("entertainers_pack".to_string()),
            name: None, category: None, qty: None
          }
        ]
      ],
      choice_3: vec![
        vec![
          SRDClassEquipmentItem {
            source: Some("data".to_string()), location: Some("tools".to_string()), key: Some("lute".to_string()),
            name: None, category: None, qty: None
          }
        ],
        vec![
          SRDClassEquipmentItem {
            source: Some("data".to_string()), location: Some("tools".to_string()), key: Some("musical_instrument".to_string()),
            name: None, category: None, qty: None
          }
        ]
      ],
      choice_4: vec![],
      defaults: vec![
        SRDClassEquipmentItem {
          source: Some("open5e".to_string()), location: Some("weapons".to_string()), key: Some("dagger".to_string()),
          name: None, category: None, qty: None
        },
        SRDClassEquipmentItem {
          source: Some("open5e".to_string()), location: Some("weapons".to_string()), key: Some("leather".to_string()),
          name: None, category: None, qty: None
        },
      ],
      desc: "You start with the following equipment, in addition to the equipment granted by your background:\n- *(a)* a rapier, *(b)* a longsword, or *(c)* any simple weapon\n- *(a)* a diplomat's pack or *(b)* an entertainer's pack\n- *(a)* a lute or *(b)* any other musical instrument\n- Leather armor and a dagger".to_string()
    },
    spellcasting: None,
    levels: SRDClassLevels::<SRDBardAttributes> {
      level_1: SRDBardAttributes {
        level: 1,
        features: vec![
          SRDClassLevelFeature { key: "spellcasting".to_string(), name: None },
          SRDClassLevelFeature { key: "bardic_inspiration".to_string(), name: Some("Bardic Inspiration (d6)".to_string()) }
        ],
        cantrips_known: 2,
        spells_known: 4,
        spell_slots: [ 2, 0, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_2: SRDBardAttributes {
        level: 2,
        features: vec![
          SRDClassLevelFeature { key: "jack_of_all_trades".to_string(), name: None },
          SRDClassLevelFeature { key: "song_of_rest".to_string(), name: Some("Song of Rest (d6)".to_string()) }
        ],
        cantrips_known: 2,
        spells_known: 5,
        spell_slots: [ 3, 0, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_3: SRDBardAttributes {
        level: 3,
        features: vec![
          SRDClassLevelFeature { key: "bard_college".to_string(), name: None },
          SRDClassLevelFeature { key: "expertise".to_string(), name: None }
        ],
        cantrips_known: 3,
        spells_known: 6,
        spell_slots: [ 4, 2, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_4: SRDBardAttributes {
        level: 4,
        features: vec![
          SRDClassLevelFeature { key: "ability_score".to_string(), name: None }
        ],
        cantrips_known: 3,
        spells_known: 7,
        spell_slots: [ 4, 3, 0, 0, 0, 0, 0, 0, 0 ]
      },
      level_5: SRDBardAttributes {
        level: 5,
        features: vec![
          SRDClassLevelFeature { key: "bardic_inspiration".to_string(), name: Some("Bardic Inspiration (d8)".to_string()) },
          SRDClassLevelFeature { key: "font_of_inspiration".to_string(), name: None }
        ],
        cantrips_known: 3,
        spells_known: 8,
        spell_slots: [ 4, 3, 2, 0, 0, 0, 0, 0, 0 ]
      },
      level_6: SRDBardAttributes {
        level: 6,
        features: vec![
          SRDClassLevelFeature { key: "countercharm".to_string(), name: None },
          SRDClassLevelFeature { key: "bard_college".to_string(), name: Some("Bard College Feature".to_string()) }
        ],
        cantrips_known: 3,
        spells_known: 9,
        spell_slots: [ 4, 3, 3, 0, 0, 0, 0, 0, 0 ]
      },
      level_7: SRDBardAttributes {
        level: 7,
        features: vec![],
        cantrips_known: 3,
        spells_known: 10,
        spell_slots: [ 4, 3, 3, 1, 0, 0, 0, 0, 0 ]
      },
      level_8: SRDBardAttributes {
        level: 8,
        features: vec![
          SRDClassLevelFeature { key: "ability_score".to_string(), name: None }
        ],
        cantrips_known: 3,
        spells_known: 11,
        spell_slots: [ 4, 3, 3, 2, 0, 0, 0, 0, 0 ]
      },
      level_9: SRDBardAttributes {
        level: 9,
        features: vec![
          SRDClassLevelFeature { key: "song_of_rest".to_string(), name: Some("Song of Rest (d8)".to_string()) }
        ],
        cantrips_known: 3,
        spells_known: 12,
        spell_slots: [ 4, 3, 3, 3, 1, 0, 0, 0, 0 ]
      },
      level_10: SRDBardAttributes {
        level: 10,
        features: vec![
          SRDClassLevelFeature { key: "bardic_inspiration".to_string(), name: Some("Bardic Inspiration (d10)".to_string()) },
          SRDClassLevelFeature { key: "expertise".to_string(), name: Some("Expertise Upgrade".to_string()) },
          SRDClassLevelFeature { key: "magical_secrets".to_string(), name: None }
        ],
        cantrips_known: 4,
        spells_known: 14,
        spell_slots: [ 4, 3, 3, 3, 2, 0, 0, 0, 0 ]
      },
      level_11: SRDBardAttributes {
        level: 11,
        features: vec![],
        cantrips_known: 4,
        spells_known: 15,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 0, 0, 0 ]
      },
      level_12: SRDBardAttributes {
        level: 12,
        features: vec![
          SRDClassLevelFeature { key: "ability_score".to_string(), name: None }
        ],
        cantrips_known: 4,
        spells_known: 15,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 0, 0, 0 ]
      },
      level_13: SRDBardAttributes {
        level: 13,
        features: vec![
          SRDClassLevelFeature { key: "song_of_rest".to_string(), name: Some("Song of Rest (d10)".to_string()) }
        ],
        cantrips_known: 4,
        spells_known: 16,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 1, 0, 0 ]
      },
      level_14: SRDBardAttributes {
        level: 14,
        features: vec![
          SRDClassLevelFeature { key: "magical_secrets".to_string(), name: None },
          SRDClassLevelFeature { key: "bard_college".to_string(), name: Some("Bard College Feature".to_string()) }
        ],
        cantrips_known: 4,
        spells_known: 18,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 1, 0, 0 ]
      },
      level_15: SRDBardAttributes {
        level: 15,
        features: vec![
          SRDClassLevelFeature { key: "bardic_inspiration".to_string(), name: Some("Bardic Inspiration (d12)".to_string()) }
        ],
        cantrips_known: 4,
        spells_known: 19,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 1, 1, 0 ]
      },
      level_16: SRDBardAttributes {
        level: 16,
        features: vec![
          SRDClassLevelFeature { key: "ability_score".to_string(), name: None }
        ],
        cantrips_known: 4,
        spells_known: 19,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 1, 1, 0 ]
      },
      level_17: SRDBardAttributes {
        level: 17,
        features: vec![
          SRDClassLevelFeature { key: "song_of_rest".to_string(), name: Some("Song of Rest (d12)".to_string()) }
        ],
        cantrips_known: 4,
        spells_known: 20,
        spell_slots: [ 4, 3, 3, 3, 2, 1, 1, 1, 1 ]
      },
      level_18: SRDBardAttributes {
        level: 18,
        features: vec![
          SRDClassLevelFeature { key: "magical_secrets".to_string(), name: Some("Magical Secrets Upgrade".to_string()) }
        ],
        cantrips_known: 4,
        spells_known: 22,
        spell_slots: [ 4, 3, 3, 3, 3, 1, 1, 1, 1 ]
      },
      level_19: SRDBardAttributes {
        level: 19,
        features: vec![
          SRDClassLevelFeature { key: "ability_score".to_string(), name: None }
        ],
        cantrips_known: 4,
        spells_known: 22,
        spell_slots: [ 4, 3, 3, 3, 3, 2, 1, 1, 1 ]
      },
      level_20: SRDBardAttributes {
        level: 20,
        features: vec![
          SRDClassLevelFeature { key: "superior_inspiration".to_string(), name: None }
        ],
        cantrips_known: 4,
        spells_known: 22,
        spell_slots: [ 4, 3, 3, 3, 3, 2, 2, 1, 1 ]
      }
    },
    features: HashMap::from([
      (
        "ability_score".to_string(),
        SRDClassFeatures {
          name: "Ability Score Improvement".to_string(),
          desc: "When you reach 4th level, and again at 8th, 12th, 16th, and 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature.".to_string()
        }
      ),
      (
        "bardic_inspiration".to_string(),
        SRDClassFeatures {
          name: "Bardic Inspiration".to_string(),
          desc: "You can inspire others through stirring words or music. To do so, you use a bonus action on your turn to choose one creature other than yourself within 60 feet of you who can hear you. That creature gains one Bardic Inspiration die, a d6.\nOnce within the next 10 minutes, the creature can roll the die and add the number rolled to one ability check, attack roll, or saving throw it makes. The creature can wait until after it rolls the d20 before deciding to use the Bardic Inspiration die, but must decide before the GM says whether the roll succeeds or fails. Once the Bardic Inspiration die is rolled, it is lost. A creature can have only one Bardic Inspiration die at a time.\nYou can use this feature a number of times equal to your Charisma modifier (a minimum of once). You regain any expended uses when you finish a long rest.\nYour Bardic Inspiration die changes when you reach certain levels in this class. The die becomes a d8 at 5th level, a d10 at 10th level, and a d12 at 15th level.".to_string()
        }
      ),
      (
        "jack_of_all_trades".to_string(),
        SRDClassFeatures {
          name: "Jack of All Trades".to_string(),
          desc: "Jack of all trades applies only to ability checks, not attack rolls or saving throws. However since initiative rolls are Dexterity checks Jack of All Trades does apply to initiative rolls as long as it is not already benefiting from the character's proficiency bonus.\nStarting at 2nd level, you can add half your proficiency bonus, rounded down, to any ability check you make that doesn't already include your proficiency bonus.".to_string()
        }
      ),
      (
        "song_of_rest".to_string(),
        SRDClassFeatures {
          name: "Song of Rest".to_string(),
          desc: "Beginning at 2nd level, you can use soothing music or oration to help revitalize your wounded allies during a short rest. If you or any friendly creatures who can hear your performance regain hit points at the end of the short rest by spending one or more Hit Dice, each of those creatures regains an extra 1d6 hit points.\nThe extra hit points increase when you reach certain levels in this class: to 1d8 at 9th level, to 1d10 at 13th level, and to 1d12 at 17th level.".to_string()
        }
      ),
      (
        "bard_college".to_string(),
        SRDClassFeatures {
          name: "Bard College".to_string(),
          desc: "At 3rd level, you delve into the advanced techniques of a bard college of your choice, such as the College of Lore. Your choice grants you features at 3rd level and again at 6th and 14th level.".to_string()
        }
      ),
      (
        "expertise".to_string(),
        SRDClassFeatures {
          name: "Expertise".to_string(),
          desc: "At 3rd level, choose two of your skill proficiencies. Your proficiency bonus is doubled for any ability check you make that uses either of the chosen proficiencies.\nAt 10th level, you can choose another two skill proficiencies to gain this benefit.".to_string()
        }
      ),
      (
        "font_of_inspiration".to_string(),
        SRDClassFeatures {
          name: "Font of Inspiration".to_string(),
          desc: "Beginning when you reach 5th level, you regain all of your expended uses of Bardic Inspiration when you finish a short or long rest.".to_string()
        }
      ),
      (
        "countercharm".to_string(),
        SRDClassFeatures {
          name: "Countercharm".to_string(),
          desc: "At 6th level, you gain the ability to use musical notes or words of power to disrupt mind-influencing effects. As an action, you can start a performance that lasts until the end of your next turn. During that time, you and any friendly creatures within 30 feet of you have advantage on saving throws against being [frightened]({{ base_url }}/conditions/frightened) or [charmed]({{ base_url }}/conditions/charmed). A creature must be able to hear you to gain this benefit. The performance ends early if you are [incapacitated]({{ base_url }}/conditions/incapacitated) or silenced or if you voluntarily end it (no action required).".to_string()
        }
      ),
      (
        "magical_secrets".to_string(),
        SRDClassFeatures {
          name: "Magical Secrets".to_string(),
          desc: "By 10th level, you have plundered magical knowledge from a wide spectrum of disciplines. Choose two spells from any class, including this one. A spell you choose must be of a level you can cast, as shown on the Bard table, or a cantrip.\nThe chosen spells count as bard spells for you and are included in the number in the Spells Known column of the Bard table.\nYou learn two additional spells from any class at 14th level and again at 18th level.".to_string()
        }
      ),
      (
        "superior_inspiration".to_string(),
        SRDClassFeatures {
          name: "Superior Inspiration".to_string(),
          desc: "At 20th level, when you roll initiative and have no uses of Bardic Inspiration left, you regain one use.".to_string()
        }
      ),
    ])
  };
}
