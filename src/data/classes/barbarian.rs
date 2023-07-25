#![allow(unused, non_upper_case_globals)]
use crate::models::classes::*;
use crate::models::equipment::*;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref barbarian: SRDClass<SRDBarbarianAttributes> = SRDClass::<SRDBarbarianAttributes> {
    hit_points: SRDClassHitPoints {
      hit_dice: 12,
      static_option: 7,
      desc: "**Hit Dice:** 1d12 per barbarian level\n**Hit Points at 1st Level:** 12 + your Constitution modifier\n**Hit Points at Higher Levels:** 1d12 (or 7) + your Constitution modifier per barbarian level after 1st"
    },
    proficiencies: SRDClassProficiencies {
      armor: vec![
        SRDEquipment::Open5eCategory(SRDItem { key: "Light", source: "armor", qty: 0 }),
        SRDEquipment::Open5eCategory(SRDItem { key: "Medium", source: "armor", qty: 0 }),
        SRDEquipment::Open5eCategory(SRDItem { key: "Shield", source: "armor", qty: 0 })
      ],
      weapons: vec![
        SRDEquipment::Open5eCategory(SRDItem { key: "Simple", source: "weapons", qty: 0 }),
        SRDEquipment::Open5eCategory(SRDItem { key: "Martial", source: "weapons", qty: 0 })
      ],
      tools: vec![],
      saving_throws: vec![ "strength", "constitution" ],
      skills: SRDClassProficientSkills {
        choices: 2,
        options: vec![ "animal_handling", "athletics", "intimidation", "nature", "perception", "survival" ]
      },
      desc: "**Armor:** Light armor, medium armor, shields\n**Weapons:** Simple weapons, martial weapons\n**Tools:** None\n**Saving Throws:** Strength, Constitution\n**Skills:** Choose two from Animal Handling, Athletics, Intimidation, Nature, Perception, and Survival"
    },
    starting_equipment: SRDClassStartingEquipment {
      choice_1: vec![
        vec![
          SRDEquipment::Open5eItem(SRDItem { key: "greataxe", source: "weapons", qty: 1 })
        ],
        vec![
          SRDEquipment::Open5eCategory(SRDItem { key: "Martial Melee", source: "weapons", qty: 1 }),
        ]
      ],
      choice_2: vec![
        vec![
          SRDEquipment::Open5eItem(SRDItem { key: "handaxe", source: "weapons", qty: 2 })
        ],
        vec![
          SRDEquipment::Open5eCategory(SRDItem { key: "Simple Melee", source: "weapons", qty: 1 }),
        ]
      ],
      choice_3: vec![],
      choice_4: vec![],
      defaults: vec![
        SRDEquipment::DnDexItem(SRDItem { key: "explorers_pack", source: "equipment_packs", qty: 1 }),
        SRDEquipment::Open5eCategory(SRDItem { key: "javelin", source: "weapons", qty: 4 }),
      ],
      desc: "You start with the following equipment, in addition to the equipment granted by your background:\n- *(a)* a greataxe or *(b)* any martial melee weapon \n- *(a)* two handaxes or *(b)* any simple weapon\nAn explorer's pack and four javelins"
    },
    spellcasting: None,
    levels: SRDClassLevels::<SRDBarbarianAttributes> {
      level_1: SRDBarbarianAttributes {
        level: 1,
        features: vec![
          SRDClassLevelFeature { key: "rage", name: None },
          SRDClassLevelFeature { key: "unarmored_defense", name: None }
        ],
        rages: 2,
        rage_damage: 2
      },
      level_2: SRDBarbarianAttributes {
        level: 2,
        features: vec![
          SRDClassLevelFeature { key: "reckless_attack", name: None },
          SRDClassLevelFeature { key: "danger_sense", name: None }
        ],
        rages: 2,
        rage_damage: 2
      },
      level_3: SRDBarbarianAttributes {
        level: 3,
        features: vec![
          SRDClassLevelFeature { key: "primal_path", name: None }
        ],
        rages: 3,
        rage_damage: 2
      },
      level_4: SRDBarbarianAttributes {
        level: 4,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        rages: 3,
        rage_damage: 2
      },
      level_5: SRDBarbarianAttributes {
        level: 5,
        features: vec![
          SRDClassLevelFeature { key: "extra_attack", name: None },
          SRDClassLevelFeature { key: "fast_movement", name: None }
        ],
        rages: 4,
        rage_damage: 2
      },
      level_6: SRDBarbarianAttributes {
        level: 6,
        features: vec![
          SRDClassLevelFeature { key: "primal_path", name: Some("Path Feature") }
        ],
        rages: 4,
        rage_damage: 2
      },
      level_7: SRDBarbarianAttributes {
        level: 7,
        features: vec![
          SRDClassLevelFeature { key: "feral_instinct", name: None }
        ],
        rages: 4,
        rage_damage: 2
      },
      level_8: SRDBarbarianAttributes {
        level: 8,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        rages: 4,
        rage_damage: 2
      },
      level_9: SRDBarbarianAttributes {
        level: 9,
        features: vec![
          SRDClassLevelFeature { key: "brutal_critical", name: Some("Brutal Critical (1 die)") }
        ],
        rages: 4,
        rage_damage: 3
      },
      level_10: SRDBarbarianAttributes {
        level: 10,
        features: vec![
          SRDClassLevelFeature { key: "primal_path", name: Some("Path Feature") }
        ],
        rages: 4,
        rage_damage: 3
      },
      level_11: SRDBarbarianAttributes {
        level: 11,
        features: vec![
          SRDClassLevelFeature { key: "relentless_rage", name: None }
        ],
        rages: 4,
        rage_damage: 3
      },
      level_12: SRDBarbarianAttributes {
        level: 12,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        rages: 5,
        rage_damage: 3
      },
      level_13: SRDBarbarianAttributes {
        level: 13,
        features: vec![
          SRDClassLevelFeature { key: "brutal_critical", name: Some("Brutal Critical (2 dice)") }
        ],
        rages: 5,
        rage_damage: 3
      },
      level_14: SRDBarbarianAttributes {
        level: 14,
        features: vec![
          SRDClassLevelFeature { key: "primal_path", name: Some("Path Feature") }
        ],
        rages: 5,
        rage_damage: 3
      },
      level_15: SRDBarbarianAttributes {
        level: 15,
        features: vec![
          SRDClassLevelFeature { key: "persistent_rage", name: None }
        ],
        rages: 5,
        rage_damage: 3
      },
      level_16: SRDBarbarianAttributes {
        level: 16,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        rages: 5,
        rage_damage: 4
      },
      level_17: SRDBarbarianAttributes {
        level: 17,
        features: vec![
          SRDClassLevelFeature { key: "brutal_critical", name: Some("Brutal Critical (3 dice)") }
        ],
        rages: 5,
        rage_damage: 4
      },
      level_18: SRDBarbarianAttributes {
        level: 18,
        features: vec![
          SRDClassLevelFeature { key: "indomitable_might", name: None }
        ],
        rages: 5,
        rage_damage: 4
      },
      level_19: SRDBarbarianAttributes {
        level: 19,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        rages: 5,
        rage_damage: 4
      },
      level_20: SRDBarbarianAttributes {
        level: 20,
        features: vec![
          SRDClassLevelFeature { key: "primal_champion", name: None }
        ],
        rages: 0,
        rage_damage: 4
      }
    },
    features: HashMap::from([
      (
        "ability_score",
        SRDClassFeature {
          name: "Ability Score Improvement",
          desc: "When you reach 4th level, and again at 8th, 12th, 16th, and 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature."
        }
      ),
      (
        "rage",
        SRDClassFeature {
          name: "Rage",
          desc: "In battle, you fight with primal ferocity. On your turn, you can enter a rage as a bonus action.\nWhile raging, you gain the following benefits if you aren't wearing heavy armor:\nYou have advantage on Strength checks and Strength saving throws.\nWhen you make a melee weapon attack using Strength, you gain a bonus to the damage roll that increases as you gain levels as a barbarian, as shown in the Rage Damage column of the Barbarian table.\nYou have resistance to bludgeoning, piercing, and slashing damage.\nIf you are able to cast spells, you can't cast them or concentrate on them while raging.\nYour rage lasts for 1 minute. It ends early if you are knocked srd:unconscious or if your turn ends and you haven't attacked a hostile creature since your last turn or taken damage since then. You can also end your rage on your turn as a bonus action.\nOnce you have raged the number of times shown for your barbarian level in the Rages column of the Barbarian table, you must finish a long rest before you can rage again."
        }
      ),
      (
        "unarmored_defense",
        SRDClassFeature {
          name: "Unarmored Defense",
          desc: "While you are not wearing any armor, your Armor Class equals 10 + your Dexterity modifier + your Constitution modifier. You can use a shield and still gain this benefit."
        }
      ),
      (
        "reckless_attack",
        SRDClassFeature {
          name: "Reckless Attack",
          desc: "Starting at 2nd level, you can throw aside all concern for defense to attack with fierce desperation. When you make your first attack on your turn, you can decide to attack recklessly. Doing so gives you advantage on melee weapon attack rolls using Strength during this turn, but attack rolls against you have advantage until your next turn."
        }
      ),
      (
        "danger_sense",
        SRDClassFeature {
          name: "Danger Sense",
          desc: "At 2nd level, you gain an uncanny sense of when things nearby aren't as they should be, giving you an edge when you dodge away from danger.\nYou have advantage on Dexterity saving throws against effects that you can see, such as traps and spells. To gain this benefit, you can't be [blinded]({{ base_url }}/conditions/blinded), [deafened]({{ base_url }}/conditions/deafened), or [incapacitated]({{ base_url }}/conditions/incapacitated)."
        }
      ),
      (
        "primal_path",
        SRDClassFeature {
          name: "Primal Path",
          desc: "At 3rd level, you choose a path that shapes the nature of your rage. Choose the Path of the Berserker or the Path of the Totem Warrior, both detailed at the end of the class description. Your choice grants you features at 3rd level and again at 6th, 10th, and 14th levels."
        }
      ),
      (
        "extra_attack",
        SRDClassFeature {
          name: "Extra Attack",
          desc: "Beginning at 5th level, you can attack twice, instead of once, whenever you take the Attack action on your turn."
        }
      ),
      (
        "fast_movement",
        SRDClassFeature {
          name: "Fast Movement",
          desc: "Starting at 5th level, your speed increases by 10 feet while you aren't wearing heavy armor."
        }
      ),
      (
        "feral_instinct",
        SRDClassFeature {
          name: "Feral Instinct",
          desc: "By 7th level, your instincts are so honed that you have advantage on initiative rolls.\nAdditionally, if you are surprised at the beginning of combat and aren't srd:incapacitated, you can act normally on your first turn, but only if you enter your rage before doing anything else on that turn."
        }
      ),
      (
        "brutal_critical",
        SRDClassFeature {
          name: "Brutal Critical",
          desc: "Beginning at 9th level, you can roll one additional weapon damage die when determining the extra damage for a critical hit with a melee attack.\nThis increases to two additional dice at 13th level and three additional dice at 17th level."
        }
      ),
      (
        "relentless_rage",
        SRDClassFeature {
          name: "Relentless Rage",
          desc: "Starting at 11th level, your rage can keep you fighting despite grievous wounds. If you drop to 0 hit points while you're raging and don't die outright, you can make a DC 10 Constitution saving throw. If you succeed, you drop to 1 hit point instead.\nEach time you use this feature after the first, the DC increases by 5. When you finish a short or long rest, the DC resets to 10."
        }
      ),
      (
        "persistent_rage",
        SRDClassFeature {
          name: "Persistent Rage",
          desc: "Beginning at 15th level, your rage is so fierce that it ends early only if you fall srd:unconscious or if you choose to end it."
        }
      ),
      (
        "indomitable_might",
        SRDClassFeature {
          name: "Indomitable Might",
          desc: "Beginning at 18th level, if your total for a Strength check is less than your Strength score, you can use that score in place of the total."
        }
      ),
      (
        "primal_champion",
        SRDClassFeature {
          name: "Primal Champion",
          desc: "At 20th level, you embody the power of the wilds. Your Strength and Constitution scores increase by 4. Your maximum for those scores is now 24."
        }
      )
    ])
  };
}
