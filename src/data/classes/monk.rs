#![allow(unused, non_upper_case_globals)]
use crate::models::classes::*;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
  pub static ref monk: SRDClass<SRDMonkAttributes> = SRDClass::<SRDMonkAttributes> {
    hit_points: SRDClassHitPoints {
      hit_dice: 8,
      static_option: 5,
      desc: "**Hit Dice:** 1d8 per Monk level\n**Hit Points at 1st Level:** 8 + your Constitution modifier\n**Hit Points at Higher Levels:** 1d8 (or 5) + your Constitution modifier per monk level after 1st"
    },
    proficiencies: SRDClassProficiencies {
      armor: vec![],
      weapons: vec![
        SRDEquipment::Open5eCategory(SRDItem { key: "Simple", source: "weapons", qty: 0 }),
        SRDEquipment::Open5eItem(SRDItem { key: "shortsword", source: "weapons", qty: 0 })
      ],
      tools: vec![
        vec![
          SRDEquipment::DnDexItem(SRDItem { key: "artisans_tools", source: "tools", qty: 0 })
        ],
        vec![
          SRDEquipment::DnDexItem(SRDItem { key: "musical_instrument", source: "tools", qty: 0 })
        ]
      ],
      saving_throws: vec![ "strength", "dexterity" ],
      skills: SRDClassProficientSkills {
        choices: 2,
        options: vec![ "acrobatics", "athletics", "history", "insight", "religion", "stealth" ]
      },
      desc: "**Armor:** None\n**Weapons:** Simple weapons, shortswords\n**Tools:** Choose one type of artisan's tools or one musical instrument\n**Saving Throws:** Strength, Dexterity\n**Skills:** Choose two from Acrobatics, Athletics, History, Insight, Religion, and Stealth"
    },
    starting_equipment: SRDClassStartingEquipment {
      choice_1: vec![
        vec![
          SRDEquipment::Open5eItem(SRDItem { key: "shortsword", source: "weapons", qty: 1 })
        ],
        vec![
          SRDEquipment::Open5eCategory(SRDItem { key: "Simple", source: "weapons", qty: 1 })
        ]
      ],
      choice_2: vec![
        vec![
          SRDEquipment::DnDexItem(SRDItem { key: "dungeoneers_pack", source: "equipment_packs", qty: 1 }),
        ],
        vec![
          SRDEquipment::DnDexItem(SRDItem { key: "explorers_pack", source: "equipment_packs", qty: 1 }),
        ]
      ],
      choice_3: vec![],
      choice_4: vec![],
      defaults: vec![
        SRDEquipment::Open5eItem(SRDItem { key: "dart", source: "weapons", qty: 10 })
      ],
      desc: "You start with the following equipment, in addition to the equipment granted by your background:\n- *(a)* a shortsword or *(b)* any simple weapon\n- *(a)* a dungeoneer's pack or *(b)* an explorer’s pack\n- 10 darts"
    },
    spellcasting: None,
    levels: SRDClassLevels::<SRDMonkAttributes> {
      level_1: SRDMonkAttributes {
        level: 1,
        features: vec![
          SRDClassLevelFeature { key: "unarmored_defense", name: None },
          SRDClassLevelFeature { key: "martial_arts", name: None }
        ],
        martial_arts: 4,
        ki_points: 0,
        unarmored_movement: 0
      },
      level_2: SRDMonkAttributes {
        level: 2,
        features: vec![
          SRDClassLevelFeature { key: "ki", name: None },
          SRDClassLevelFeature { key: "unarmored_movement", name: None }
        ],
        martial_arts: 4,
        ki_points: 2,
        unarmored_movement: 10
      },
      level_3: SRDMonkAttributes {
        level: 3,
        features: vec![
          SRDClassLevelFeature { key: "monastic_traditions", name: None },
          SRDClassLevelFeature { key: "deflect_missiles", name: None }
        ],
        martial_arts: 4,
        ki_points: 3,
        unarmored_movement: 10
      },
      level_4: SRDMonkAttributes {
        level: 4,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None },
          SRDClassLevelFeature { key: "slow_fall", name: None }
        ],
        martial_arts: 4,
        ki_points: 4,
        unarmored_movement: 10
      },
      level_5: SRDMonkAttributes {
        level: 5,
        features: vec![
          SRDClassLevelFeature { key: "extra_attack", name: None },
          SRDClassLevelFeature { key: "stunning_strike", name: None }
        ],
        martial_arts: 6,
        ki_points: 5,
        unarmored_movement: 10
      },
      level_6: SRDMonkAttributes {
        level: 6,
        features: vec![
          SRDClassLevelFeature { key: "ki_empowered_strikes", name: None },
          SRDClassLevelFeature { key: "monastic_traditions", name: Some("Monastic Tradition Feature") }
        ],
        martial_arts: 6,
        ki_points: 6,
        unarmored_movement: 15
      },
      level_7: SRDMonkAttributes {
        level: 7,
        features: vec![
          SRDClassLevelFeature { key: "evasion", name: None },
          SRDClassLevelFeature { key: "stillness_of_mind", name: None }
        ],
        martial_arts: 6,
        ki_points: 7,
        unarmored_movement: 15
      },
      level_8: SRDMonkAttributes {
        level: 8,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        martial_arts: 6,
        ki_points: 8,
        unarmored_movement: 15
      },
      level_9: SRDMonkAttributes {
        level: 9,
        features: vec![
          SRDClassLevelFeature { key: "unarmored_movement", name: Some("Unarmored Movement Improvement") }
        ],
        martial_arts: 6,
        ki_points: 9,
        unarmored_movement: 15
      },
      level_10: SRDMonkAttributes {
        level: 10,
        features: vec![
          SRDClassLevelFeature { key: "purity_of_body", name: None }
        ],
        martial_arts: 6,
        ki_points: 10,
        unarmored_movement: 20
      },
      level_11: SRDMonkAttributes {
        level: 11,
        features: vec![
          SRDClassLevelFeature { key: "monastic_traditions", name: Some("Monastic Tradition Feature") }
        ],
        martial_arts: 8,
        ki_points: 11,
        unarmored_movement: 20
      },
      level_12: SRDMonkAttributes {
        level: 12,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        martial_arts: 8,
        ki_points: 12,
        unarmored_movement: 20
      },
      level_13: SRDMonkAttributes {
        level: 13,
        features: vec![
          SRDClassLevelFeature { key: "tongue_of_the_sun_and_moon", name: None }
        ],
        martial_arts: 8,
        ki_points: 13,
        unarmored_movement: 20
      },
      level_14: SRDMonkAttributes {
        level: 14,
        features: vec![
          SRDClassLevelFeature { key: "diamond_soul", name: None }
        ],
        martial_arts: 8,
        ki_points: 14,
        unarmored_movement: 25
      },
      level_15: SRDMonkAttributes {
        level: 15,
        features: vec![
          SRDClassLevelFeature { key: "timeless_body", name: None }
        ],
        martial_arts: 8,
        ki_points: 15,
        unarmored_movement: 25
      },
      level_16: SRDMonkAttributes {
        level: 16,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        martial_arts: 8,
        ki_points: 16,
        unarmored_movement: 25
      },
      level_17: SRDMonkAttributes {
        level: 17,
        features: vec![
          SRDClassLevelFeature { key: "monastic_traditions", name: Some("Monastic Tradition Feature") }
        ],
        martial_arts: 10,
        ki_points: 17,
        unarmored_movement: 25
      },
      level_18: SRDMonkAttributes {
        level: 18,
        features: vec![
          SRDClassLevelFeature { key: "empty_body", name: None }
        ],
        martial_arts: 10,
        ki_points: 18,
        unarmored_movement: 30
      },
      level_19: SRDMonkAttributes {
        level: 19,
        features: vec![
          SRDClassLevelFeature { key: "ability_score", name: None }
        ],
        martial_arts: 10,
        ki_points: 19,
        unarmored_movement: 30
      },
      level_20: SRDMonkAttributes {
        level: 20,
        features: vec![
          SRDClassLevelFeature { key: "perfect_self", name: None }
        ],
        martial_arts: 10,
        ki_points: 20,
        unarmored_movement: 30
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
        "unarmored_defense",
        SRDClassFeature {
          name: "Unarmored Defense",
          desc: "Beginning at 1st level, while you are wearing no armor and not wielding a shield, your AC equals 10 + your Dexterity modifier + your Wisdom modifier."
        },
      ),
      (
        "martial_arts",
        SRDClassFeature {
          name: "Martial Arts",
          desc: "At 1st level, your practice of martial arts gives you mastery of combat styles that use unarmed strikes and monk weapons, which are shortswords and any simple melee weapons that don't have the two_ handed or heavy property.\nYou gain the following benefits while you are unarmed or wielding only monk weapons and you aren't wearing armor or wielding a shield:\nYou can use Dexterity instead of Strength for the attack and damage rolls of your unarmed strikes and monk weapons.\nYou can roll a d4 in place of the normal damage of your unarmed strike or monk weapon. This die changes as you gain monk levels, as shown in the Martial Arts column of the Monk table.\nWhen you use the Attack action with an unarmed strike or a monk weapon on your turn, you can make one unarmed strike as a bonus action. For example, if you take the Attack action and attack with a quarterstaff, you can also make an unarmed strike as a bonus action, assuming you haven't already taken a bonus action this turn.\nCertain monasteries use specialized forms of the monk weapons. For example, you might use a club that is two lengths of wood connected by a short chain (called a nunchaku) or a sickle with a shorter, straighter blade (called a kama). Whatever name you use for a monk weapon, you can use the game statistics provided for the weapon."
        },
      ),
      (
        "ki",
        SRDClassFeature {
          name: "Ki",
          desc: "Starting at 2nd level, your training allows you to harness the mystic energy of ki. Your access to this energy is represented by a number of ki points. Your monk level determines the number of points you have, as shown in the Ki Points column of the Monk table.\nYou can spend these points to fuel various ki features. You start knowing three such features: Flurry of Blows, Patient Defense, and Step of the Wind. You learn more ki features as you gain levels in this class.\nWhen you spend a ki point, it is unavailable until you finish a short or long rest, at the end of which you draw all of your expended ki back into yourself. You must spend at least 30 minutes of the rest meditating to regain your ki points.\nSome of your ki features require your target to make a saving throw to resist the feature's effects. The saving throw DC is calculated as follows:\n**Ki save DC** = 8 + your proficiency bonus + your Wisdom modifier\n## Flurry of Blows\nImmediately after you take the Attack action on your turn, you can spend 1 ki point to make two unarmed strikes as a bonus action.\n## Patient Defense\nYou can spend 1 ki point to take the Dodge action as a bonus action on your turn.\n## Step of the Wind\nYou can spend 1 ki point to take the Disengage or Dash action as a bonus action on your turn, and your jump distance is doubled for the turn."
        },
      ),
      (
        "unarmored_movement",
        SRDClassFeature {
          name: "Unarmored Movement",
          desc: "Starting at 2nd level, your speed increases by 10 feet while you are not wearing armor or wielding a shield. This bonus increases when you reach certain monk levels, as shown in the Monk table.\nAt 9th level, you gain the ability to move along vertical surfaces and across liquids on your turn without falling during the move."
        },
      ),
      (
        "monastic_tradition",
        SRDClassFeature {
          name: "Monastic Tradition",
          desc: "When you reach 3rd level, you commit yourself to a monastic tradition: the Way of the Open Hand, the Way of Shadow, or the Way of the Four Elements, all detailed at the end of the class description. Your tradition grants you features at 3rd level and again at 6th, 11th, and 17th level. Three traditions of monastic pursuit are common in the monasteries scattered across the multiverse. Most monasteries practice one tradition exclusively, but a few honor the three traditions and instruct each monk according to his or her aptitude and interest. All three traditions rely on the same basic techniques, diverging as the student grows more adept. Thus, a monk need choose a tradition only upon reaching 3rd level."
        },
      ),
      (
        "deflect_missiles",
        SRDClassFeature {
          name: "Deflect Missiles",
          desc: "Starting at 3rd level, you can use your reaction to deflect or catch the missile when you are hit by a ranged weapon attack. When you do so, the damage you take from the attack is reduced by 1d10 + your Dexterity modifier + your monk level.\nIf you reduce the damage to 0, you can catch the missile if it is small enough for you to hold in one hand and you have at least one hand free. If you catch a missile in this way, you can spend 1 ki point to make a ranged attack with the weapon or piece of ammunition you just caught, as part of the same reaction. You make this attack with proficiency, regardless of your weapon proficiencies, and the missile counts as a monk weapon for the attack, which has a normal range of 20 feet and a long range of 60 feet."
        },
      ),
      (
        "slow_fall",
        SRDClassFeature {
          name: "Slow Fall",
          desc: "Beginning at 4th level, you can use your reaction when you fall to reduce any falling damage you take by an amount equal to five times your monk level."
        },
      ),
      (
        "extra_attack",
        SRDClassFeature {
          name: "Extra Attack",
          desc: "Beginning at 5th level, you can attack twice, instead of once, whenever you take the Attack action on your turn."
        },
      ),
      (
        "stunning_strike",
        SRDClassFeature {
          name: "Stunning Strike",
          desc: "Starting at 5th level, you can interfere with the flow of ki in an opponent's body. When you hit another creature with a melee weapon attack, you can spend 1 ki point to attempt a stunning strike. The target must succeed on a Constitution saving throw or be stunned until the end of your next turn."
        },
      ),
      (
        "ki_empowered_strikes",
        SRDClassFeature {
          name: "Ki Empowered Strikes",
          desc: "Starting at 6th level, your unarmed strikes count as magical for the purpose of overcoming resistance and immunity to nonmagical attacks and damage."
        },
      ),
      (
        "evasion",
        SRDClassFeature {
          name: "Evasion",
          desc: "At 7th level, your instinctive agility lets you dodge out of the way of certain area effects, such as a blue dragon's lightning breath or a *fireball* spell. When you are subjected to an effect that allows you to make a Dexterity saving throw to take only half damage, you instead take no damage if you succeed on the saving throw, and only half damage if you fail."
        },
      ),
      (
        "stillness_of_mind",
        SRDClassFeature {
          name: "Stillness of Mind",
          desc: "Starting at 7th level, you can use your action to end one effect on yourself that is causing you to be charmed or frightened."
        },
      ),
      (
        "purity_of_body",
        SRDClassFeature {
          name: "Purity of Body",
          desc: "At 10th level, your mastery of the ki flowing through you makes you immune to disease and poison."
        },
      ),
      (
        "tongue_of_the_sun_and_moon",
        SRDClassFeature {
          name: "Tongue of the Sun and Moon",
          desc: "Starting at 13th level, you learn to touch the ki of other minds so that you understand all spoken languages. Moreover, any creature that can understand a language can understand what you say."
        },
      ),
      (
        "diamond_soul",
        SRDClassFeature {
          name: "Diamond Soul",
          desc: "Beginning at 14th level, your mastery of ki grants you proficiency in all saving throws.\nAdditionally, whenever you make a saving throw and fail, you can spend 1 ki point to reroll it and take the second result."
        },
      ),
      (
        "timeless_body",
        SRDClassFeature {
          name: "Timeless Body",
          desc: "At 15th level, your ki sustains you so that you suffer none of the frailty of old age, and you can't be aged magically. You can still die of old age, however. In addition, you no longer need food or water."
        },
      ),
      (
        "empty_body",
        SRDClassFeature {
          name: "Empty Body",
          desc: "Beginning at 18th level, you can use your action to spend 4 ki points to become invisible for 1 minute. During that time, you also have resistance to all damage but force damage.\nAdditionally, you can spend 8 ki points to cast the *astral projection* spell, without needing material components. When you do so, you can't take any other creatures with you."
        },
      ),
      (
        "perfect_self",
        SRDClassFeature {
          name: "Perfect Self",
          desc: "At 20th level, when you roll for initiative and have no ki points remaining, you regain 4 ki points."
        }
      ),
    ])
  };
}
