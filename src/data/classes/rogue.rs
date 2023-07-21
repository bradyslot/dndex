#![allow(unused, non_upper_case_globals)]
use crate::models::classes::*;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref rogue: SRDClass<SRDRogueAttributes> = SRDClass::<SRDRogueAttributes> {
    hit_points: SRDClassHitPoints {
      hit_dice: 8,
      static_option: 5,
      desc: "**Hit Dice:** 1d8 per Rogue level\n**Hit Points at 1st Level:** 8 + your Constitution modifier\n**Hit Points at Higher Levels:** 1d8 (or 5) + your Constitution modifier per rogue level after 1st"
    },
    proficiencies: SRDClassProficiencies {
      armor: vec![
        SRDEquipment::Open5eCategory(SRDItem { key: "Light", source: "armor", qty: 0 }),
      ],
      weapons: vec![
        SRDEquipment::Open5eCategory(SRDItem { key: "Simple", source: "weapons", qty: 0 }),
        SRDEquipment::Open5eItem(SRDItem { key: "crossbow-hand", source: "weapons", qty: 0 }),
        SRDEquipment::Open5eItem(SRDItem { key: "longsword", source: "weapons", qty: 0 }),
        SRDEquipment::Open5eItem(SRDItem { key: "rapier", source: "weapons", qty: 0 }),
        SRDEquipment::Open5eItem(SRDItem { key: "shortsword", source: "weapons", qty: 0 }),
      ],
      tools: vec![
        vec![
          SRDEquipment::DnDexItem(SRDItem { key: "thieves_tools", source: "tools", qty: 0 }),
        ]
      ],
      saving_throws: vec![ "dexterity", "intelligence" ],
      skills: SRDClassProficientSkills {
        choices: 4,
        options: vec![ "acrobatics", "athletics", "deception", "insight", "intimidation", "investigation", "perception", "performance", "persuasion", "sleight_of_hand", "stealth" ]
      },
      desc: "**Armor:** Light armor\n**Weapons:** Simple weapons, hand crossbows, longswords, rapiers, shortswords\n**Tools:** Thieves' tools\n**Saving Throws:** Dexterity, Intelligence\n**Skills:** Choose four from Acrobatics, Athletics, Deception, Insight, Intimidation, Investigation, Perception, Performance, Persuasion, Sleight of Hand, and Stealth"
    },
    starting_equipment: SRDClassStartingEquipment {
      choice_1: vec![
        vec![
          SRDEquipment::Open5eItem(SRDItem { key: "rapier", source: "weapons", qty: 1 }),
        ],
        vec![
          SRDEquipment::Open5eItem(SRDItem { key: "shortsword", source: "weapons", qty: 1 }),
        ]
      ],
      choice_2: vec![
        vec![
          SRDEquipment::Open5eItem(SRDItem { key: "shortbow", source: "weapons", qty: 1 }),
          SRDEquipment::CustomItem(SRDCustomItem { name: "Arrows", qty: 20 })
        ],
        vec![
          SRDEquipment::Open5eItem(SRDItem { key: "shortsword", source: "weapons", qty: 1 }),
        ]
      ],
      choice_3: vec![
        vec![
          SRDEquipment::DnDexItem(SRDItem { key: "burglars_pack", source: "equipment_packs", qty: 1 }),
        ],
        vec![
          SRDEquipment::DnDexItem(SRDItem { key: "dungeoneers_pack", source: "equipment_packs", qty: 1 }),
        ],
        vec![
          SRDEquipment::DnDexItem(SRDItem { key: "explorers_pack", source: "equipment_packs", qty: 1 }),
        ]
      ],
      choice_4: vec![],
      defaults: vec![
        SRDEquipment::Open5eItem(SRDItem { key: "leather", source: "amor", qty: 1 }),
        SRDEquipment::Open5eItem(SRDItem { key: "dagger", source: "weapons", qty: 2 }),
        SRDEquipment::DnDexItem(SRDItem { key: "thieves_tools", source: "tools", qty: 1 }),
      ],
      desc: "You start with the following equipment, in addition to the equipment granted by your background:\n- *(a)* a rapier or *(b)* a shortsword\n- *(a)* a shortbow and quiver of 20 arrows or *(b)* a shortsword\n- *(a)* a burglar's pack, *(b)* a dungeoneer's pack, or *(c)* an explorer's pack\n- *(a)* Leather armor, two daggers, and thieves' tools"
    },
    spellcasting: None,
    levels: SRDClassLevels::<SRDRogueAttributes> {
      level_1: SRDRogueAttributes {
        level: 1,
        features: vec![
          SRDClassLevelFeature { key: "expertise", name: None },
          SRDClassLevelFeature { key: "sneak_attack", name: None },
          SRDClassLevelFeature { key: "thieves_cant", name: None }
        ],
        sneak_attack: 1
      },
      level_2: SRDRogueAttributes {
        level: 2,
        features: vec![
          SRDClassLevelFeature { key: "cunning_action", name: None }
        ],
        sneak_attack: 1
      },
      level_3: SRDRogueAttributes {
        level: 3,
        features: vec![
          SRDClassLevelFeature { key: "roguish_archetype", name: None }
        ],
        sneak_attack: 2
      },
      level_4: SRDRogueAttributes {
        level: 4,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        sneak_attack: 2
      },
      level_5: SRDRogueAttributes {
        level: 5,
        features: vec![
          SRDClassLevelFeature { key: "uncanny_dodge", name: None }
        ],
        sneak_attack: 3
      },
      level_6: SRDRogueAttributes {
        level: 6,
        features: vec![
          SRDClassLevelFeature { key: "expertise", name: Some("Expertise Improvement") }
        ],
        sneak_attack: 3
      },
      level_7: SRDRogueAttributes {
        level: 7,
        features: vec![
          SRDClassLevelFeature { key: "evasion", name: None }
        ],
        sneak_attack: 4
      },
      level_8: SRDRogueAttributes {
        level: 8,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        sneak_attack: 4
      },
      level_9: SRDRogueAttributes {
        level: 9,
        features: vec![
          SRDClassLevelFeature { key: "roguish_archetype", name: Some("Roguish Archetype Feature") }
        ],
        sneak_attack: 5
      },
      level_10: SRDRogueAttributes {
        level: 10,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        sneak_attack: 5
      },
      level_11: SRDRogueAttributes {
        level: 11,
        features: vec![
          SRDClassLevelFeature { key: "reliable_talent", name: None }
        ],
        sneak_attack: 6
      },
      level_12: SRDRogueAttributes {
        level: 12,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        sneak_attack: 6
      },
      level_13: SRDRogueAttributes {
        level: 13,
        features: vec![
          SRDClassLevelFeature { key: "roguish_archetype", name: Some("Roguish Archetype Feature") }
        ],
        sneak_attack: 7
      },
      level_14: SRDRogueAttributes {
        level: 14,
        features: vec![
          SRDClassLevelFeature { key: "blindsense", name: None }
        ],
        sneak_attack: 7
      },
      level_15: SRDRogueAttributes {
        level: 15,
        features: vec![
          SRDClassLevelFeature { key: "slippery_mind", name: None }
        ],
        sneak_attack: 8
      },
      level_16: SRDRogueAttributes {
        level: 16,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        sneak_attack: 8
      },
      level_17: SRDRogueAttributes {
        level: 17,
        features: vec![
          SRDClassLevelFeature { key: "roguish_archetype", name: Some("Roguish Archetype Feature") }
        ],
        sneak_attack: 9
      },
      level_18: SRDRogueAttributes {
        level: 18,
        features: vec![
          SRDClassLevelFeature { key: "elusive", name: None }
        ],
        sneak_attack: 9
      },
      level_19: SRDRogueAttributes {
        level: 19,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        sneak_attack: 10
      },
      level_20: SRDRogueAttributes {
        level: 20,
        features: vec![
          SRDClassLevelFeature { key: "stroke_of_luck", name: None }
        ],
        sneak_attack: 10
      }
    },
    features: HashMap::from([
      (
        "ability_score",
        SRDClassFeature {
          name: "Ability Score Improvement",
          desc: "When you reach 4th level, and again at 8th, 12th, 16th, and 19th level, you can increase one ability score of your choice by 2, or you can increase two ability scores of your choice by 1. As normal, you can't increase an ability score above 20 using this feature."
        },
      ),
      (
        "expertise",
        SRDClassFeature {
          name: "Expertise",
          desc: "At 1st level, choose two of your skill proficiencies, or one of your skill proficiencies and your proficiency with thieves' tools. Your proficiency bonus is doubled for any ability check you make that uses either of the chosen proficiencies.\nAt 6th level, you can choose two more of your proficiencies (in skills or with thieves' tools) to gain this benefit."
        },
      ),
      (
        "sneak_attack",
        SRDClassFeature {
          name: "Sneak Attack",
          desc: "Beginning at 1st level, you know how to strike subtly and exploit a foe's distraction. Once per turn, you can deal an extra 1d6 damage to one creature you hit with an attack if you have advantage on the attack roll. The attack must use a finesse or a ranged weapon.\nYou don't need advantage on the attack roll if another enemy of the target is within 5 feet of it, that enemy isn't incapacitated, and you don't have disadvantage on the attack roll.\nThe amount of the extra damage increases as you gain levels in this class, as shown in the Sneak Attack column of the Rogue table."
        },
      ),
      (
        "thieves_cant",
        SRDClassFeature {
          name: "Thieves' Cant",
          desc: "During your rogue training you learned thieves' cant, a secret mix of dialect, jargon, and code that allows you to hide messages in seemingly normal conversation. Only another creature that knows thieves' cant understands such messages. It takes four times longer to convey such a message than it does to speak the same idea plainly.\nIn addition, you understand a set of secret signs and symbols used to convey short, simple messages, such as whether an area is dangerous or the territory of a thieves' guild, whether loot is nearby, or whether the people in an area are easy marks or will provide a safe house for thieves on the run."
        },
      ),
      (
        "cunning_action",
        SRDClassFeature {
          name: "Cunning Action",
          desc: "Starting at 2nd level, your quick thinking and agility allow you to move and act quickly. You can take a bonus action on each of your turns in combat. This action can be used only to take the Dash, Disengage, or Hide action."
        },
      ),
      (
        "roguish_archetype",
        SRDClassFeature {
          name: "Roguish Archetype",
          desc: "At 3rd level, you choose an archetype that you emulate in the exercise of your rogue abilities: Thief, Assassin, or Arcane Trickster, all detailed at the end of the class description. Your archetype choice grants you features at 3rd level and then again at 9th, 13th, and 17th level.\nRogues have many features in common, including their emphasis on perfecting their skills, their precise and deadly approach to combat, and their increasingly quick reflexes. But different rogues steer those talents in varying directions, embodied by the rogue archetypes. Your choice of archetype is a reflection of your focus_not necessarily an indication of your chosen profession, but a description of your preferred techniques."
        },
      ),
      (
        "uncanny_dodge",
        SRDClassFeature {
          name: "Uncanny Dodge",
          desc: "Starting at 5th level, when an attacker that you can see hits you with an attack, you can use your reaction to halve the attack's damage against you."
        },
      ),
      (
        "evasion",
        SRDClassFeature {
          name: "Evasion",
          desc: "Beginning at 7th level, you can nimbly dodge out of the way of certain area effects, such as a red dragon's fiery breath or an ice storm spell. When you are subjected to an effect that allows you to make a Dexterity saving throw to take only half damage, you instead take no damage if you succeed on the saving throw, and only half damage if you fail."
        },
      ),
      (
        "reliable_talent",
        SRDClassFeature {
          name: "Reliable Talent",
          desc: "By 11th level, you have refined your chosen skills until they approach perfection. Whenever you make an ability check that lets you add your proficiency bonus, you can treat a d20 roll of 9 or lower as a 10."
        },
      ),
      (
        "blindsense",
        SRDClassFeature {
          name: "Blindsense",
          desc: "Starting at 14th level, if you are able to hear, you are aware of the location of any hidden or invisible creature within 10 feet of you."
        },
      ),
      (
        "slippery_mind",
        SRDClassFeature {
          name: "Slippery Mind",
          desc: "By 15th level, you have acquired greater mental strength. You gain proficiency in Wisdom saving throws."
        },
      ),
      (
        "elusive",
        SRDClassFeature {
          name: "Elusive",
          desc: "Beginning at 18th level, you are so evasive that attackers rarely gain the upper hand against you. No attack roll has advantage against you while you aren't incapacitated."
        },
      ),
      (
        "stroke_of_luck",
        SRDClassFeature {
          name: "Stroke of Luck",
          desc: "At 20th level, you have an uncanny knack for succeeding when you need to. If your attack misses a target within range, you can turn the miss into a hit. Alternatively, if you fail an ability check, you can treat the d20 roll as a 20.\nOnce you use this feature, you can't use it again until you finish a short or long rest."
        }
      ),
    ])
  };
}
