#![allow(unused, non_upper_case_globals)]
use crate::models::classes::*;
use crate::models::equipment::*;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref fighter: SRDClass<SRDFighterAttributes> = SRDClass::<SRDFighterAttributes> {
    hit_points: SRDClassHitPoints {
      hit_dice: 10,
      static_option: 6,
      desc: "**Hit Dice:** 1d10 per Fighter level\n**Hit Points at 1st Level:** 10 + your Constitution modifier\n**Hit Points at Higher Levels:** 1d10 (or 6) + your Constitution modifier per fighter level after 1st"
    },
    proficiencies: SRDClassProficiencies {
      armor: vec![
        Equipment::Open5e(Open5eEquipment::Open5eCategory(SRDItem { key: "Armor", source: "armor", qty: 0 })),
      ],
      weapons: vec![
        Equipment::Open5e(Open5eEquipment::Open5eCategory(SRDItem { key: "Simple", source: "weapons", qty: 0 })),
        Equipment::Open5e(Open5eEquipment::Open5eCategory(SRDItem { key: "Martial", source: "weapons", qty: 0 })),
      ],
      tools: vec![],
      saving_throws: vec![ "strength", "constitution" ],
      skills: SRDClassProficientSkills {
        choices: 2,
        options: vec![ "acrobatics", "animal_handling", "athletics", "history", "insight", "intimidation", "perception", "survival" ] },
      desc: "**Armor:** All armor, shields\n**Weapons:** Simple weapons, martial weapons\n**Tools:** None\n**Saving Throws:** Strength, Constitution\n**Skills:** Choose two skills from Acrobatics, Animal, Handling, Athletics, History, Insight, Intimidation, Perception, and Survival"
    },
    starting_equipment: SRDClassStartingEquipment {
      choice_1: vec![
        vec![
          Equipment::Open5e(Open5eEquipment::Open5eItem(SRDItem { key: "chain-mail", source: "armor", qty: 1 })),
        ],
        vec![
          Equipment::Open5e(Open5eEquipment::Open5eItem(SRDItem { key: "leather", source: "armor", qty: 1 })),
          Equipment::Open5e(Open5eEquipment::Open5eItem(SRDItem { key: "longbow", source: "weapons", qty: 1 })),
          Equipment::DnDex(DnDexEquipment::CustomItem(SRDCustomItem { name: "Arrows", qty: 20 }))
        ]
      ],
      choice_2: vec![
        vec![
          Equipment::Open5e(Open5eEquipment::Open5eCategory(SRDItem { key: "Martial", source: "weapons", qty: 1 })),
          Equipment::Open5e(Open5eEquipment::Open5eItem(SRDItem { key: "shield", source: "armor", qty: 1 }))
        ],
        vec![
          Equipment::Open5e(Open5eEquipment::Open5eCategory(SRDItem { key: "Martial", source: "weapons", qty: 2 })),
        ]
      ],
      choice_3: vec![
        vec![
          Equipment::Open5e(Open5eEquipment::Open5eItem(SRDItem { key: "crossbow-light", source: "weapons", qty: 1 })),
          Equipment::DnDex(DnDexEquipment::CustomItem(SRDCustomItem { name: "Bolts", qty: 20 }))
        ],
        vec![
          Equipment::Open5e(Open5eEquipment::Open5eItem(SRDItem { key: "handaxe", source: "weapons", qty: 2 })),
        ]
      ],
      choice_4: vec![
        vec![
          Equipment::DnDex(DnDexEquipment::DnDexItem(SRDItem { key: "dungeoneers_pack", source: "equipment_packs", qty: 1 })),
        ],
        vec![
          Equipment::DnDex(DnDexEquipment::DnDexItem(SRDItem { key: "explorers_pack", source: "equipment_packs", qty: 1 })),
        ]
      ],
      defaults: vec![],
      desc: "You start with the following equipment, in addition to the equipment granted by your background:\n- *(a)* chain mail or *(b)* leather armor, longbow, and 20 arrows\n- *(a)* a martial weapon and a shield or *(b)* two martial weapons\n- *(a)* a light crossbow and 20 bolts or *(b)* two handaxes\n- *(a)* a dungeoneer's pack or *(b)* an explorer's pack"
    },
    spellcasting: None,
    levels: SRDClassLevels::<SRDFighterAttributes> {
      level_1: SRDFighterAttributes {
        level: 1,
        features: vec![
          SRDClassLevelFeature { key: "fighter_fighting_style", name: None },
          SRDClassLevelFeature { key: "second_wind", name: None }
        ]
      },
      level_2: SRDFighterAttributes {
        level: 2,
        features: vec![
          SRDClassLevelFeature { key: "action_surge", name: Some("Action Surge (1 use)") }
        ]
      },
      level_3: SRDFighterAttributes {
        level: 3,
        features: vec![
          SRDClassLevelFeature { key: "martial_archetype", name: None }
        ]
      },
      level_4: SRDFighterAttributes {
        level: 4,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ]
      },
      level_5: SRDFighterAttributes {
        level: 5,
        features: vec![
          SRDClassLevelFeature { key: "extra_attack", name: None }
        ]
      },
      level_6: SRDFighterAttributes {
        level: 6,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ]
      },
      level_7: SRDFighterAttributes {
        level: 7,
        features: vec![
          SRDClassLevelFeature { key: "martial_archetype", name: Some("Martial Archetype Feature") }
        ]
      },
      level_8: SRDFighterAttributes {
        level: 8,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ]
      },
      level_9: SRDFighterAttributes {
        level: 9,
        features: vec![
          SRDClassLevelFeature { key: "indomitable", name: Some("Indomitable (1 use)") }
        ]
      },
      level_10: SRDFighterAttributes {
        level: 10,
        features: vec![
          SRDClassLevelFeature { key: "martial_archetype", name: Some("Martial Archetype Feature") }
        ]
      },
      level_11: SRDFighterAttributes {
        level: 11,
        features: vec![
          SRDClassLevelFeature { key: "extra_attack", name: Some("Extra Attack (2)") }
        ]
      },
      level_12: SRDFighterAttributes {
        level: 12,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ]
      },
      level_13: SRDFighterAttributes {
        level: 13,
        features: vec![
          SRDClassLevelFeature { key: "indomitable", name: Some("Indomitable (2 uses)") }
        ]
      },
      level_14: SRDFighterAttributes {
        level: 14,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ]
      },
      level_15: SRDFighterAttributes {
        level: 15,
        features: vec![
          SRDClassLevelFeature { key: "martial_archetype", name: Some("Martial Archetype Feature") }
        ]
      },
      level_16: SRDFighterAttributes {
        level: 16,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ]
      },
      level_17: SRDFighterAttributes {
        level: 17,
        features: vec![
          SRDClassLevelFeature { key: "action_surge", name: Some("Action Surge (2 uses)") },
          SRDClassLevelFeature { key: "indomitable", name: Some("Indomitable (3 uses)") }
        ]
      },
      level_18: SRDFighterAttributes {
        level: 18,
        features: vec![
          SRDClassLevelFeature { key: "martial_archetype", name: Some("Martial Archetype Feature") }
        ]
      },
      level_19: SRDFighterAttributes {
        level: 19,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ]
      },
      level_20: SRDFighterAttributes {
        level: 20,
        features: vec![
          SRDClassLevelFeature { key: "extra_attack", name: Some("Extra Attack (3)") }
        ]
      }
    },
    features: HashMap::from([
      (
        "fighting_style",
        SRDClassFeature {
          name: "Fighting Style",
          desc: "You adopt a particular style of fighting as your specialty. Choose one of the following options. You can't take a Fighting Style option more than once, even if you later get to choose again.\n## Archery You gain a +2 bonus to attack rolls you make with ranged weapons.\n## Defense While you are wearing armor, you gain a +1 bonus to AC.\n## Dueling When you are wielding a melee weapon in one hand and no other weapons, you gain a +2 bonus to damage rolls with that weapon.\n## Great Weapon Fighting When you roll a 1 or 2 on a damage die for an attack you make with a melee weapon that you are wielding with two hands, you can reroll the die and must use the new roll, even if the new roll is a 1 or a 2. The weapon must have the two_handed or versatile property for you to gain this benefit.\n## Protection When a creature you can see attacks a target other than you that is within 5 feet of you, you can use your reaction to impose disadvantage on the attack roll. You must be wielding a shield.\n## Two_Weapon Fighting When you engage in two_weapon fighting, you can add your ability modifier to the damage of the second attack. "
        },
      ),
      (
        "second_wind",
        SRDClassFeature {
          name: "Second Wind",
          desc: "You have a limited well of stamina that you can draw on to protect yourself from harm. On your turn, you can use a bonus action to regain hit points equal to 1d10 + your fighter level. Once you use this feature, you must finish a short or long rest before you can use it again."
        },
      ),
      (
        "action_surge",
        SRDClassFeature {
          name: "Action Surge",
          desc: "Starting at 2nd level, you can push yourself beyond your normal limits for a moment. On your turn, you can take one additional action on top of your regular action and a possible bonus action.\nOnce you use this feature, you must finish a short or long rest before you can use it again. Starting at 17th level, you can use it twice before a rest, but only once on the same turn. "
        },
      ),
      (
        "martial_archetype",
        SRDClassFeature {
          name: "Martial Archetype",
          desc: "At 3rd level, you choose an archetype that you strive to emulate in your combat styles and techniques. Choose Champion, Battle Master, or Eldritch Knight, all detailed at the end of the class description. The archetype you choose grants you features at 3rd level and again at 7th, 10th, 15th, and 18th level.\nDifferent fighters choose different approaches to perfecting their fighting prowess. The martial archetype you choose to emulate reflects your approach."
        },
      ),
      (
        "ability_score",
        SRDClassFeature {
          name: "Ability Score Improvement",
          desc: "When you reach 4th level, and again at 6th, 8th, 12th, 14th, 16th, and 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature."
        },
      ),
      (
        "extra_attack",
        SRDClassFeature {
          name: "Extra Attack",
          desc: "Beginning at 5th level, you can attack twice, instead of once, whenever you take the Attack action on your turn.\nThe number of attacks increases to three when you reach 11th level in this class and to four when you reach 20th level in this class."
        },
      ),
      (
        "indomitable",
        SRDClassFeature {
          name: "Indomitable",
          desc: "Beginning at 9th level, you can reroll a saving throw that you fail. If you do so, you must use the new roll, and you can't use this feature again until you finish a long rest.\nYou can use this feature twice between long rests starting at 13th level and three times between long rests starting at 17th level."
        }
      )
    ])
  };
}
